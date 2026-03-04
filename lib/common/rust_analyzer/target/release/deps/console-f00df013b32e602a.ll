; ModuleID = 'console.72671f79b508dbd-cgu.0'
source_filename = "console.72671f79b508dbd-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"core::sync::atomic::AtomicUsize" = type { i64 }
%"std::sync::reentrant_lock::ReentrantLock<core::cell::RefCell<std::io::stdio::StderrRaw>>" = type { %"std::sys::sync::mutex::pthread::Mutex", %"std::sync::reentrant_lock::Tid", i32, [1 x i32], i64 }
%"std::sys::sync::mutex::pthread::Mutex" = type { %"std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>" }
%"std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>" = type { %"core::sync::atomic::AtomicPtr<std::sys::pal::unix::sync::mutex::Mutex>" }
%"core::sync::atomic::AtomicPtr<std::sys::pal::unix::sync::mutex::Mutex>" = type { ptr }
%"std::sync::reentrant_lock::Tid" = type { %"core::sync::atomic::AtomicU64" }
%"core::sync::atomic::AtomicU64" = type { i64 }
%"unicode_width::tables::Align128<[u8; 256]>" = type { [256 x i8] }
%"unicode_width::tables::Align64<[[u8; 64]; 20]>" = type { [20 x [64 x i8]] }
%"unicode_width::tables::Align32<[[u8; 32]; 186]>" = type { [186 x [32 x i8]] }
%"unicode_width::tables::Align128<[[u8; 128]; 7]>" = type { [7 x [128 x i8]] }

@alloc_a51357dfbcca0800220965a98e80088f = private unnamed_addr constant [132 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/collections/btree/node.rs\00", align 1
@alloc_36e9d3034d1040a1b97b71beadbdf183 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a51357dfbcca0800220965a98e80088f, [16 x i8] c"\83\00\00\00\00\00\00\00\F0\00\00\00M\00\00\00" }>, align 8
@alloc_5c50b0573939a5b3b419995fc3e13e19 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a51357dfbcca0800220965a98e80088f, [16 x i8] c"\83\00\00\00\00\00\00\00\13\05\00\00$\00\00\00" }>, align 8
@alloc_69009fdc319497586282719e739ab5f8 = private unnamed_addr constant [136 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/collections/btree/navigate.rs\00", align 1
@alloc_1df1e5171bffdf21494df69d159bd444 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00\C6\00\00\00'\00\00\00" }>, align 8
@alloc_2bd3b8890b9559f1d5fa47166d6aa71d = private unnamed_addr constant [114 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/io/mod.rs\00", align 1
@alloc_92437324c5298c66e2abee23d7039388 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_2bd3b8890b9559f1d5fa47166d6aa71d, [16 x i8] c"q\00\00\00\00\00\00\00\D2\08\00\005\00\00\00" }>, align 8
@alloc_61571fe18316b4282550a450ed10f289 = private unnamed_addr constant [34 x i8] c"stream did not contain valid UTF-8", align 1
@alloc_a679944945e039118b94625869e7de57 = private unnamed_addr constant <{ ptr, [9 x i8], [7 x i8] }> <{ ptr @alloc_61571fe18316b4282550a450ed10f289, [9 x i8] c"\22\00\00\00\00\00\00\00\15", [7 x i8] undef }>, align 8
@alloc_930f3883875d82d5043e839301ab22e6 = private unnamed_addr constant [40 x i8] c"assertion failed: src.len() == dst.len()", align 1
@alloc_3d1b356cbfc916b26f15e548d269cb74 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a51357dfbcca0800220965a98e80088f, [16 x i8] c"\83\00\00\00\00\00\00\00T\07\00\00\05\00\00\00" }>, align 8
@alloc_93816f04728d387347072ad30618ff9c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00X\02\00\000\00\00\00" }>, align 8
@anon.554535de7657ca3c38ed6e2115135a2e.0 = private unnamed_addr constant [64 x i8] c"\A7\AB\AA2\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", align 8
@alloc_87b741302675a09f531e93859aa1d77d = private unnamed_addr constant [105 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/console-0.15.11/src/unix_term.rs\00", align 1
@alloc_a5020ff128eafee240ced9e9d62411d1 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_87b741302675a09f531e93859aa1d77d, [16 x i8] c"h\00\00\00\00\00\00\00\8E\00\00\00\0C\00\00\00" }>, align 8
@alloc_84da818df01979596a9e7a52ed4fd1e4 = private unnamed_addr constant [48 x i8] c"assertion failed: self.is_char_boundary(new_len)", align 1
@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEEB1Z_, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1q_ }>, align 8
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant [43 x i8] c"called `Result::unwrap()` on an `Err` value", align 1
@vtable.1 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1q_ }>, align 8
@vtable.2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1q_ }>, align 8
@vtable.3 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEEECsC3JuwEIQwb_7console, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1q_ }>, align 8
@vtable.4 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEEECsC3JuwEIQwb_7console, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1q_ }>, align 8
@_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT = external local_unnamed_addr global %"core::sync::atomic::AtomicUsize"
@alloc_df3352094b5ea906c434c84ffe49cca7 = private unnamed_addr constant [100 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/console-0.15.11/src/term.rs\00", align 1
@alloc_10ae8e1c7422912565b0a725bff29948 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00\F6\00\00\00/\00\00\00" }>, align 8
@alloc_34a907ce632321878f08de7a619b60ed = private unnamed_addr constant [5 x i8] c"\0D\1B[2K", align 1
@alloc_71ab024f63e05fa8c544dae9bff3eae8 = private unnamed_addr constant [5 x i8] c"\C0\01\0A\C0\00", align 1
@alloc_7954641d60551ca0a31844d45868012f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00\FC\00\00\00/\00\00\00" }>, align 8
@_RNvNvNtNtCs5sEH5CPMdak_3std2io5stdio6stderr8INSTANCE = external global %"std::sync::reentrant_lock::ReentrantLock<core::cell::RefCell<std::io::stdio::StderrRaw>>"
@alloc_be34cf22961c134b4f22f36d9b4f226c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\003\02\00\00.\00\00\00" }>, align 8
@alloc_cae42f98ba52838b2ed93bf93f07b323 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00E\01\00\00$\00\00\00" }>, align 8
@alloc_1bd3c013d602be6e35682a3072b8f24e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00G\01\00\005\00\00\00" }>, align 8
@alloc_d1c5991a811577b5a75f5444621d6d5a = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00l\01\00\00$\00\00\00" }>, align 8
@alloc_e6817151e95acf2e0d9d4b0416a3fb0b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00\89\01\00\00,\00\00\00" }>, align 8
@alloc_a42bcd32b1b51ab8fb3130fb27019cbe = private unnamed_addr constant [14 x i8] c"Not a terminal", align 1
@alloc_118a6c87c06c70363f5ebdfdf2788916 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00\EF\00\00\00/\00\00\00" }>, align 8
@alloc_f46351429e0b545e52edab17c91e045b = private unnamed_addr constant [5 x i8] c"black", align 1
@alloc_940496d3cd18caebe90765385b962abe = private unnamed_addr constant [3 x i8] c"red", align 1
@alloc_b8ebeef8965b067fea87e4e40f7a2696 = private unnamed_addr constant [5 x i8] c"green", align 1
@alloc_761ec3f22b4eea6465689eb413e86232 = private unnamed_addr constant [6 x i8] c"yellow", align 1
@alloc_d11e62a55d3bc6b15beecd4f0707fbb1 = private unnamed_addr constant [4 x i8] c"blue", align 1
@alloc_3d9fd8ba1c29d9f6085e0294be30b396 = private unnamed_addr constant [7 x i8] c"magenta", align 1
@alloc_6fc1c32c52b5575ce6b85d0118457658 = private unnamed_addr constant [4 x i8] c"cyan", align 1
@alloc_16c9842326e8d04943b3c4e15a061707 = private unnamed_addr constant [5 x i8] c"white", align 1
@alloc_0e40d41a90d9449ef3c0a2d4ebecf25e = private unnamed_addr constant [6 x i8] c"bright", align 1
@alloc_988dbb775df289ae5d387217d1326fbb = private unnamed_addr constant [8 x i8] c"on_black", align 1
@alloc_00afc89fad6ce0b69c03ae44a4c57e63 = private unnamed_addr constant [6 x i8] c"on_red", align 1
@alloc_d3f73fbdad6723a6f7e70241cc5ffd7b = private unnamed_addr constant [8 x i8] c"on_green", align 1
@alloc_b2d76a187fec1d5904bbbaf8cd5a45f9 = private unnamed_addr constant [9 x i8] c"on_yellow", align 1
@alloc_0ea3ed158ba78f67fb80c06310d3d976 = private unnamed_addr constant [7 x i8] c"on_blue", align 1
@alloc_93fbedf531e254d87a372581b818455c = private unnamed_addr constant [10 x i8] c"on_magenta", align 1
@alloc_70cd957082e55e3219d604e7774b3a2a = private unnamed_addr constant [7 x i8] c"on_cyan", align 1
@alloc_28c707092aa47c9e62a5453e9825cf31 = private unnamed_addr constant [8 x i8] c"on_white", align 1
@alloc_d0d4965f28f12d18e1eb5434656a1634 = private unnamed_addr constant [9 x i8] c"on_bright", align 1
@alloc_8004f499ef1e85ed18420cba06aae287 = private unnamed_addr constant [4 x i8] c"bold", align 1
@alloc_bc56aad9843e15c95b8ae131ad5c2648 = private unnamed_addr constant [3 x i8] c"dim", align 1
@alloc_35da8c1274638488887234a29631ac8a = private unnamed_addr constant [10 x i8] c"underlined", align 1
@alloc_343238722d8106d5e592de156f114b52 = private unnamed_addr constant [5 x i8] c"blink", align 1
@alloc_94ec8616ca31a77937e20e985e657532 = private unnamed_addr constant [10 x i8] c"blink_fast", align 1
@alloc_6f7f7c851d095ea6be0620cebf08a8c7 = private unnamed_addr constant [7 x i8] c"reverse", align 1
@alloc_a49a2be336e4029d0e1a46cd0ad31a8b = private unnamed_addr constant [6 x i8] c"hidden", align 1
@alloc_4a97ccfb756c0a73db16da629adb6fac = private unnamed_addr constant [13 x i8] c"strikethrough", align 1
@alloc_607574f98762a7642d8175dbddfc81da = private unnamed_addr constant [3 x i8] c"on_", align 1
@alloc_9b41339eb38873415d2e9ba71f6a1bf6 = private unnamed_addr constant [101 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/console-0.15.11/src/utils.rs\00", align 1
@alloc_9749b5a9842556100920529e544f7dc1 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9b41339eb38873415d2e9ba71f6a1bf6, [16 x i8] c"d\00\00\00\00\00\00\00\E2\00\00\00(\00\00\00" }>, align 8
@alloc_ebf3b6b7067b219708a558a8386bf721 = private unnamed_addr constant [100 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/console-0.15.11/src/ansi.rs\00", align 1
@alloc_01955e5c48a2e8a6df036a754cf3097b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ebf3b6b7067b219708a558a8386bf721, [16 x i8] c"c\00\00\00\00\00\00\00\EB\00\00\00\10\00\00\00" }>, align 8
@alloc_a8c1ea329d6664f53eac78246d452a6c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ebf3b6b7067b219708a558a8386bf721, [16 x i8] c"c\00\00\00\00\00\00\00\E6\00\00\00\10\00\00\00" }>, align 8
@alloc_b0d6f4768d6dbcaaba9bf9a6c0de2b5f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a51357dfbcca0800220965a98e80088f, [16 x i8] c"\83\00\00\00\00\00\00\00\D0\04\00\00#\00\00\00" }>, align 8
@alloc_890f94c692f51df5fc72c5e0dfe1823b = private unnamed_addr constant [8 x i8] c"/dev/tty", align 1
@alloc_50679f783fca94dab7457816f752eff0 = private unnamed_addr constant [48 x i8] c"assertion failed: edge.height == self.height - 1", align 1
@alloc_6f083584960555ed8732225b7b53e5b1 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a51357dfbcca0800220965a98e80088f, [16 x i8] c"\83\00\00\00\00\00\00\00\B6\02\00\00\09\00\00\00" }>, align 8
@alloc_80b5bcfc6b71c11e412d8b752ae375dc = private unnamed_addr constant [11 x i8] c"\02\1B[\C0\05D\1B[0K\00", align 1
@alloc_26f3afb675a2033575e507bda4f7709d = private unnamed_addr constant [7 x i8] c"\02\1B[\C0\01A\00", align 1
@alloc_8041b981a0c9f2271efbceca9d03160b = private unnamed_addr constant [7 x i8] c"\02\1B[\C0\01B\00", align 1
@alloc_91eb3b305a07e4e7da5a5c0c180f5643 = private unnamed_addr constant [7 x i8] c"\02\1B[\C0\01D\00", align 1
@alloc_e721d4fe15bdeb41f351b3369d7e026f = private unnamed_addr constant [7 x i8] c"\02\1B[\C0\01C\00", align 1
@alloc_7ba7d7c5f08be500d53e5ec9430a471e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9b41339eb38873415d2e9ba71f6a1bf6, [16 x i8] c"d\00\00\00\00\00\00\00\0A\03\00\00-\00\00\00" }>, align 8
@_RNvNtCsC3JuwEIQwb_7console5utils13STDERR_COLORS = local_unnamed_addr global <{ [9 x i8], [7 x i8], ptr }> <{ [9 x i8] zeroinitializer, [7 x i8] undef, ptr @_RNvYNCNvNtCsC3JuwEIQwb_7console5utils13STDERR_COLORS0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_onceB8_ }>, align 8
@_RNvNtCsC3JuwEIQwb_7console5utils13STDOUT_COLORS = local_unnamed_addr global <{ [9 x i8], [7 x i8], ptr }> <{ [9 x i8] zeroinitializer, [7 x i8] undef, ptr @_RNvYNCNvNtCsC3JuwEIQwb_7console5utils13STDOUT_COLORS0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_onceB8_ }>, align 8
@alloc_5c8c5330792b76d76bc036d6b9372d18 = private unnamed_addr constant [8 x i8] c"CLICOLOR", align 1
@alloc_787344c135fb1ca228327e24d16f41dc = private unnamed_addr constant [14 x i8] c"CLICOLOR_FORCE", align 1
@alloc_7ef65f9252ea7966a1063c1ab22af5b9 = private unnamed_addr constant [19 x i8] c"Reached end of file", align 1
@alloc_50f3f282355a7f8c5c8e6d622c64bbaf = private unnamed_addr constant [16 x i8] c"read interrupted", align 1
@alloc_9e8821ebf06809bdf585485e81a043ea = private unnamed_addr constant [8 x i8] c"NO_COLOR", align 1
@alloc_c940f1872184b67533cde325d4eb7ceb = private unnamed_addr constant [4 x i8] c"TERM", align 1
@alloc_0cb071d804372b212169612c3e5a40ce = private unnamed_addr constant [4 x i8] c"dumb", align 1
@_RNvNtCsDJOD2kcAir_13unicode_width6tables10WIDTH_ROOT = external local_unnamed_addr global %"unicode_width::tables::Align128<[u8; 256]>"
@alloc_aaca2516fbe9b04517984be67db863ea = private unnamed_addr constant [106 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/unicode-width-0.2.2/src/tables.rs\00", align 1
@_RNvNtCsDJOD2kcAir_13unicode_width6tables12WIDTH_MIDDLE = external local_unnamed_addr global %"unicode_width::tables::Align64<[[u8; 64]; 20]>"
@alloc_3c235b06addb9c2230a3beb294b85791 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_aaca2516fbe9b04517984be67db863ea, [16 x i8] c"i\00\00\00\00\00\00\00\B8\00\00\00\15\00\00\00" }>, align 8
@_RNvNtCsDJOD2kcAir_13unicode_width6tables12WIDTH_LEAVES = external local_unnamed_addr global %"unicode_width::tables::Align32<[[u8; 32]; 186]>"
@alloc_54facc433374646a466a7fffff6dc929 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_aaca2516fbe9b04517984be67db863ea, [16 x i8] c"i\00\00\00\00\00\00\00\BE\00\00\00\19\00\00\00" }>, align 8
@_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_0 = external local_unnamed_addr global [2 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_1 = external local_unnamed_addr global [1 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_2 = external local_unnamed_addr global [4 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_3 = external local_unnamed_addr global [9 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_4 = external local_unnamed_addr global [4 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_5 = external local_unnamed_addr global [6 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_6 = external local_unnamed_addr global [12 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_7 = external local_unnamed_addr global [2 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables25EMOJI_PRESENTATION_LEAVES = external local_unnamed_addr global %"unicode_width::tables::Align128<[[u8; 128]; 7]>"
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_0 = external local_unnamed_addr global [4 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_1 = external local_unnamed_addr global [1 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_2 = external local_unnamed_addr global [15 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_3 = external local_unnamed_addr global [10 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_4 = external local_unnamed_addr global [3 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_5 = external local_unnamed_addr global [1 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_6 = external local_unnamed_addr global [13 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_7 = external local_unnamed_addr global [22 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_8 = external local_unnamed_addr global [4 x { i8, i8 }]
@_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_9 = external local_unnamed_addr global [10 x { i8, i8 }]
@alloc_bf8d9f537a4fba6a5267343a2274f175 = private unnamed_addr constant [105 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.177/src/unix/bsd/mod.rs\00", align 1
@alloc_e5974023cf134bdbad09697d09348ac9 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_bf8d9f537a4fba6a5267343a2274f175, [16 x i8] c"h\00\00\00\00\00\00\00W\02\00\00\09\00\00\00" }>, align 8
@alloc_afa71497e4832d4e825454760abdae23 = private unnamed_addr constant [4 x i8] c"\01\0A\C0\00", align 1
@alloc_9ead3bbeb6a5cfcbf0840e30f987d290 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00X\02\00\00\1E\00\00\00" }>, align 8
@alloc_da7d8ace005fc641b5ea7745ba5bdcfd = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00q\02\00\00/\00\00\00" }>, align 8
@alloc_0878d7ad62607610ca18347eb669fd5d = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_df3352094b5ea906c434c84ffe49cca7, [16 x i8] c"c\00\00\00\00\00\00\00\7F\02\00\00/\00\00\00" }>, align 8
@alloc_0c812808379efded5a4fb82d2790b556 = private unnamed_addr constant [2 x i8] c"\C0\00", align 1
@alloc_ca4d880a0cd8b0a81e2f00dd95ebe9c4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ebf3b6b7067b219708a558a8386bf721, [16 x i8] c"c\00\00\00\00\00\00\00\F7\00\00\00\1C\00\00\00" }>, align 8
@alloc_1b354bc18ec1773a84dec3c89f38b3b8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ebf3b6b7067b219708a558a8386bf721, [16 x i8] c"c\00\00\00\00\00\00\00\81\00\00\00\13\00\00\00" }>, align 8
@alloc_173e90a6bd2787cc8478b93b2c321f76 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ebf3b6b7067b219708a558a8386bf721, [16 x i8] c"c\00\00\00\00\00\00\00\02\01\00\00\1D\00\00\00" }>, align 8
@alloc_8e2410b80645266732854088d21653bc = private unnamed_addr constant [11 x i8] c"PoisonError", align 1
@switch.table._RNvNtCsC3JuwEIQwb_7console5utils9str_width = private unnamed_addr constant [7 x i16] [i16 0, i16 poison, i16 poison, i16 poison, i16 33, i16 32, i16 0], align 2

@_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EE9drop_slowB1A_ = unnamed_addr alias void (ptr), ptr @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EE9drop_slowB1A_

; <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
; Function Attrs: cold uwtable
define internal fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 captures(none) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %x.i = alloca [64 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %x.i)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %x.i, ptr noundef nonnull align 8 dereferenceable(64) @anon.554535de7657ca3c38ed6e2115135a2e.0, i64 64, i1 false)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !2
; call __rustc::__rust_alloc
  %0 = tail call noundef align 8 dereferenceable_or_null(64) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 64, i64 noundef 8) #29, !noalias !2
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb2.i.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCsC3JuwEIQwb_7console.exit.i, !prof !5

bb2.i.i:                                          ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 64) #30
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
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

common.resume:                                    ; preds = %cleanup.i.i, %cleanup.i, %bb1.i
  %common.resume.op = phi { ptr, i32 } [ %9, %bb1.i ], [ %2, %cleanup.i.i ], [ %4, %cleanup.i ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCsC3JuwEIQwb_7console.exit.i: ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %0, ptr noundef nonnull align 8 dereferenceable(64) @anon.554535de7657ca3c38ed6e2115135a2e.0, i64 64, i1 false)
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %x.i)
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::init
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4init(ptr noundef nonnull align 8 %0)
          to label %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0CsC3JuwEIQwb_7console.exit unwind label %cleanup.i

cleanup.i:                                        ; preds = %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCsC3JuwEIQwb_7console.exit.i
  %4 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<core::pin::Pin<alloc::boxed::Box<std::sys::pal::unix::sync::mutex::Mutex>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console(ptr %0) #32
          to label %common.resume unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0CsC3JuwEIQwb_7console.exit: ; preds = %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCsC3JuwEIQwb_7console.exit.i
  %6 = cmpxchg ptr %self, ptr null, ptr %0 release acquire, align 8
  %7 = extractvalue { ptr, i1 } %6, 1
  %8 = extractvalue { ptr, i1 } %6, 0
  br i1 %7, label %bb5, label %bb3

bb3:                                              ; preds = %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0CsC3JuwEIQwb_7console.exit
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %0)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console.exit unwind label %bb1.i

bb1.i:                                            ; preds = %bb3
  %9 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 64, i64 noundef 8) #29
  br label %common.resume

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console.exit: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 64, i64 noundef 8) #29
  br label %bb5

bb5:                                              ; preds = %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0CsC3JuwEIQwb_7console.exit, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console.exit
  %_0.sroa.0.0 = phi ptr [ %8, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console.exit ], [ %0, %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0CsC3JuwEIQwb_7console.exit ]
  %10 = icmp ne ptr %_0.sroa.0.0, null
  tail call void @llvm.assume(i1 %10)
  ret ptr %_0.sroa.0.0
}

; core::ptr::drop_in_place::<core::pin::Pin<alloc::boxed::Box<std::sys::pal::unix::sync::mutex::Mutex>>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console(ptr nonnull %_1.0.val) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %_1.0.val)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console.exit unwind label %bb1.i

bb1.i:                                            ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef 64, i64 noundef 8) #29
  resume { ptr, i32 } %0

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console.exit: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef 64, i64 noundef 8) #29
  ret void
}

; core::ptr::drop_in_place::<core::option::Option<std::sync::poison::mutex::Mutex<alloc::vec::Vec<u8>>>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(48) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_2 = load i64, ptr %_1, align 8, !range !6, !noundef !7
  %0 = icmp eq i64 %_2, 0
  br i1 %0, label %bb1, label %bb2

bb1:                                              ; preds = %bb2.i.i.i4.i.i4.i, %bb4.i, %start
  ret void

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
; invoke <std::sys::sync::mutex::pthread::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(40) %1)
          to label %bb4.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %bb2
  %2 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(40) %1) #32
          to label %cleanup.body.i unwind label %terminate.i.i

bb4.i.i:                                          ; preds = %bb2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11)
  %ptr.i.i.i.i = load ptr, ptr %1, align 8, !alias.scope !14, !noundef !7
  store ptr null, ptr %1, align 8, !alias.scope !14
  %3 = icmp eq ptr %ptr.i.i.i.i, null
  br i1 %3, label %bb4.i, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb4.i.i
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %ptr.i.i.i.i)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i unwind label %bb1.i.i.i.i.i.i.i, !noalias !19

bb1.i.i.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i.i
  %4 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef 64, i64 noundef 8) #29, !noalias !19
  br label %cleanup.body.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i: ; preds = %bb2.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef 64, i64 noundef 8) #29, !noalias !19
  br label %bb4.i

terminate.i.i:                                    ; preds = %cleanup.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

cleanup.body.i:                                   ; preds = %bb1.i.i.i.i.i.i.i, %cleanup.i.i
  %eh.lpad-body.i = phi { ptr, i32 } [ %4, %bb1.i.i.i.i.i.i.i ], [ %2, %cleanup.i.i ]
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val.i = load i64, ptr %6, align 8, !alias.scope !20
  %7 = icmp eq i64 %.val.i, 0
  br i1 %7, label %bb1.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.body.i
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val1.i = load ptr, ptr %8, align 8, !alias.scope !20, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1.i, i64 noundef %.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb1.i

bb4.i:                                            ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i, %bb4.i.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val2.i = load i64, ptr %9, align 8, !alias.scope !20
  %10 = icmp eq i64 %.val2.i, 0
  br i1 %10, label %bb1, label %bb2.i.i.i4.i.i4.i

bb2.i.i.i4.i.i4.i:                                ; preds = %bb4.i
  %11 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val3.i = load ptr, ptr %11, align 8, !alias.scope !20, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i, i64 noundef %.val2.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb1

bb1.i:                                            ; preds = %bb2.i.i.i4.i.i.i, %cleanup.body.i
  resume { ptr, i32 } %eh.lpad-body.i
}

; core::ptr::drop_in_place::<core::result::Result<console::kb::Key, std::io::error::Error>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsC3JuwEIQwb_7console2kb3KeyNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEEB16_(i64 %_1.0.val, ptr %_1.8.val) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %.not = icmp eq i64 %_1.0.val, -9223372036854775787
  br i1 %.not, label %bb3, label %bb2

bb2:                                              ; preds = %start
  %0 = icmp ne i64 %_1.0.val, -9223372036854775807
  tail call void @llvm.assume(i1 %0)
  %or.cond.i = icmp slt i64 %_1.0.val, 1
  br i1 %or.cond.i, label %bb1, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb2
  %alloc_size.i.i.i.i5.i.i = shl nuw i64 %_1.0.val, 2
  %1 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %1)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.8.val, i64 noundef %alloc_size.i.i.i.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #29
  br label %bb1

bb3:                                              ; preds = %start
  %2 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %2)
  %bits.i.i.i.i = ptrtoint ptr %_1.8.val to i64
  %_5.i.i.i.i = and i64 %bits.i.i.i.i, 3
  %switch.i.i.i = icmp eq i64 %_5.i.i.i.i, 1
  br i1 %switch.i.i.i, label %bb2.i2.i.i.i, label %bb1, !prof !21

bb2.i2.i.i.i:                                     ; preds = %bb3
  %3 = getelementptr i8, ptr %_1.8.val, i64 -1
  %4 = icmp ne ptr %3, null
  tail call void @llvm.assume(i1 %4)
  %_6.val.i.i.i.i.i = load ptr, ptr %3, align 8
  %5 = getelementptr i8, ptr %_1.8.val, i64 7
  %_6.val1.i.i.i.i.i = load ptr, ptr %5, align 8, !nonnull !7, !align !22, !noundef !7
  %6 = load ptr, ptr %_6.val1.i.i.i.i.i, align 8, !invariant.load !7
  %.not.i.i.i.i.i.i.i = icmp eq ptr %6, null
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i:                        ; preds = %bb2.i2.i.i.i
  %7 = icmp ne ptr %_6.val.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %7)
  invoke void %6(ptr noundef nonnull %_6.val.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %is_not_null.i.i.i.i.i.i.i, %bb2.i2.i.i.i
  %8 = icmp ne ptr %_6.val.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %8)
  %9 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %10 = load i64, ptr %9, align 8, !range !23, !invariant.load !7
  %11 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %12 = load i64, ptr %11, align 8, !range !24, !invariant.load !7
  %13 = add i64 %12, -1
  %14 = icmp sgt i64 %13, -1
  tail call void @llvm.assume(i1 %14)
  %15 = icmp eq i64 %10, 0
  br i1 %15, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %10, i64 noundef range(i64 1, -9223372036854775807) %12) #29
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i

cleanup.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i
  %16 = landingpad { ptr, i32 }
          cleanup
  %17 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %18 = load i64, ptr %17, align 8, !range !23, !invariant.load !7
  %19 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %20 = load i64, ptr %19, align 8, !range !24, !invariant.load !7
  %21 = add i64 %20, -1
  %22 = icmp sgt i64 %21, -1
  tail call void @llvm.assume(i1 %22)
  %23 = icmp eq i64 %18, 0
  br i1 %23, label %bb1.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %18, i64 noundef range(i64 1, -9223372036854775807) %20) #29
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %3, i64 noundef 24, i64 noundef 8) #29
  resume { ptr, i32 } %16

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %3, i64 noundef 24, i64 noundef 8) #29
  br label %bb1

bb1:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i, %bb3, %bb2.i.i.i4.i.i, %bb2
  ret void
}

; core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::io::error::Error>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsC3JuwEIQwb_7console(i64 %_1.0.val, ptr %_1.8.val) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  switch i64 %_1.0.val, label %bb2.i.i.i4.i.i [
    i64 -9223372036854775808, label %bb3
    i64 0, label %bb1
  ]

bb2.i.i.i4.i.i:                                   ; preds = %start
  %0 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %0)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.8.val, i64 noundef %_1.0.val, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb1

bb3:                                              ; preds = %start
  %1 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %1)
  %bits.i.i.i.i = ptrtoint ptr %_1.8.val to i64
  %_5.i.i.i.i = and i64 %bits.i.i.i.i, 3
  %switch.i.i.i = icmp eq i64 %_5.i.i.i.i, 1
  br i1 %switch.i.i.i, label %bb2.i2.i.i.i, label %bb1, !prof !21

bb2.i2.i.i.i:                                     ; preds = %bb3
  %2 = getelementptr i8, ptr %_1.8.val, i64 -1
  %3 = icmp ne ptr %2, null
  tail call void @llvm.assume(i1 %3)
  %_6.val.i.i.i.i.i = load ptr, ptr %2, align 8
  %4 = getelementptr i8, ptr %_1.8.val, i64 7
  %_6.val1.i.i.i.i.i = load ptr, ptr %4, align 8, !nonnull !7, !align !22, !noundef !7
  %5 = load ptr, ptr %_6.val1.i.i.i.i.i, align 8, !invariant.load !7
  %.not.i.i.i.i.i.i.i = icmp eq ptr %5, null
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i:                        ; preds = %bb2.i2.i.i.i
  %6 = icmp ne ptr %_6.val.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %6)
  invoke void %5(ptr noundef nonnull %_6.val.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %is_not_null.i.i.i.i.i.i.i, %bb2.i2.i.i.i
  %7 = icmp ne ptr %_6.val.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %7)
  %8 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %9 = load i64, ptr %8, align 8, !range !23, !invariant.load !7
  %10 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %11 = load i64, ptr %10, align 8, !range !24, !invariant.load !7
  %12 = add i64 %11, -1
  %13 = icmp sgt i64 %12, -1
  tail call void @llvm.assume(i1 %13)
  %14 = icmp eq i64 %9, 0
  br i1 %14, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %9, i64 noundef range(i64 1, -9223372036854775807) %11) #29
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i

cleanup.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i
  %15 = landingpad { ptr, i32 }
          cleanup
  %16 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %17 = load i64, ptr %16, align 8, !range !23, !invariant.load !7
  %18 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %19 = load i64, ptr %18, align 8, !range !24, !invariant.load !7
  %20 = add i64 %19, -1
  %21 = icmp sgt i64 %20, -1
  tail call void @llvm.assume(i1 %21)
  %22 = icmp eq i64 %17, 0
  br i1 %22, label %bb1.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %17, i64 noundef range(i64 1, -9223372036854775807) %19) #29
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %2, i64 noundef 24, i64 noundef 8) #29
  resume { ptr, i32 } %15

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %2, i64 noundef 24, i64 noundef 8) #29
  br label %bb1

bb1:                                              ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i, %bb3, %bb2.i.i.i4.i.i
  ret void
}

; core::ptr::drop_in_place::<core::result::Result<bool, std::io::error::Error>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultbNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsC3JuwEIQwb_7console(i8 %_1.0.val, ptr %_1.8.val) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = icmp eq i8 %_1.0.val, 0
  br i1 %0, label %bb1, label %bb2

bb1:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i, %bb2, %start
  ret void

bb2:                                              ; preds = %start
  %1 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %1)
  %bits.i.i.i.i = ptrtoint ptr %_1.8.val to i64
  %_5.i.i.i.i = and i64 %bits.i.i.i.i, 3
  %switch.i.i.i = icmp eq i64 %_5.i.i.i.i, 1
  br i1 %switch.i.i.i, label %bb2.i2.i.i.i, label %bb1, !prof !21

bb2.i2.i.i.i:                                     ; preds = %bb2
  %2 = getelementptr i8, ptr %_1.8.val, i64 -1
  %3 = icmp ne ptr %2, null
  tail call void @llvm.assume(i1 %3)
  %_6.val.i.i.i.i.i = load ptr, ptr %2, align 8
  %4 = getelementptr i8, ptr %_1.8.val, i64 7
  %_6.val1.i.i.i.i.i = load ptr, ptr %4, align 8, !nonnull !7, !align !22, !noundef !7
  %5 = load ptr, ptr %_6.val1.i.i.i.i.i, align 8, !invariant.load !7
  %.not.i.i.i.i.i.i.i = icmp eq ptr %5, null
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i:                        ; preds = %bb2.i2.i.i.i
  %6 = icmp ne ptr %_6.val.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %6)
  invoke void %5(ptr noundef nonnull %_6.val.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %is_not_null.i.i.i.i.i.i.i, %bb2.i2.i.i.i
  %7 = icmp ne ptr %_6.val.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %7)
  %8 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %9 = load i64, ptr %8, align 8, !range !23, !invariant.load !7
  %10 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %11 = load i64, ptr %10, align 8, !range !24, !invariant.load !7
  %12 = add i64 %11, -1
  %13 = icmp sgt i64 %12, -1
  tail call void @llvm.assume(i1 %13)
  %14 = icmp eq i64 %9, 0
  br i1 %14, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %9, i64 noundef range(i64 1, -9223372036854775807) %11) #29
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i

cleanup.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i
  %15 = landingpad { ptr, i32 }
          cleanup
  %16 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %17 = load i64, ptr %16, align 8, !range !23, !invariant.load !7
  %18 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %19 = load i64, ptr %18, align 8, !range !24, !invariant.load !7
  %20 = add i64 %19, -1
  %21 = icmp sgt i64 %20, -1
  tail call void @llvm.assume(i1 %21)
  %22 = icmp eq i64 %17, 0
  br i1 %22, label %bb1.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %17, i64 noundef range(i64 1, -9223372036854775807) %19) #29
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %2, i64 noundef 24, i64 noundef 8) #29
  resume { ptr, i32 } %15

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %2, i64 noundef 24, i64 noundef 8) #29
  br label %bb1
}

; core::ptr::drop_in_place::<core::result::Result<usize, std::io::error::Error>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultjNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsC3JuwEIQwb_7console(i64 %_1.0.val, ptr %_1.8.val) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = icmp eq i64 %_1.0.val, 0
  br i1 %0, label %bb1, label %bb2

bb1:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i, %bb2, %start
  ret void

bb2:                                              ; preds = %start
  %1 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %1)
  %bits.i.i.i.i = ptrtoint ptr %_1.8.val to i64
  %_5.i.i.i.i = and i64 %bits.i.i.i.i, 3
  %switch.i.i.i = icmp eq i64 %_5.i.i.i.i, 1
  br i1 %switch.i.i.i, label %bb2.i2.i.i.i, label %bb1, !prof !21

bb2.i2.i.i.i:                                     ; preds = %bb2
  %2 = getelementptr i8, ptr %_1.8.val, i64 -1
  %3 = icmp ne ptr %2, null
  tail call void @llvm.assume(i1 %3)
  %_6.val.i.i.i.i.i = load ptr, ptr %2, align 8
  %4 = getelementptr i8, ptr %_1.8.val, i64 7
  %_6.val1.i.i.i.i.i = load ptr, ptr %4, align 8, !nonnull !7, !align !22, !noundef !7
  %5 = load ptr, ptr %_6.val1.i.i.i.i.i, align 8, !invariant.load !7
  %.not.i.i.i.i.i.i.i = icmp eq ptr %5, null
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i:                        ; preds = %bb2.i2.i.i.i
  %6 = icmp ne ptr %_6.val.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %6)
  invoke void %5(ptr noundef nonnull %_6.val.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %is_not_null.i.i.i.i.i.i.i, %bb2.i2.i.i.i
  %7 = icmp ne ptr %_6.val.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %7)
  %8 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %9 = load i64, ptr %8, align 8, !range !23, !invariant.load !7
  %10 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %11 = load i64, ptr %10, align 8, !range !24, !invariant.load !7
  %12 = add i64 %11, -1
  %13 = icmp sgt i64 %12, -1
  tail call void @llvm.assume(i1 %13)
  %14 = icmp eq i64 %9, 0
  br i1 %14, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %9, i64 noundef range(i64 1, -9223372036854775807) %11) #29
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i

cleanup.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i
  %15 = landingpad { ptr, i32 }
          cleanup
  %16 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %17 = load i64, ptr %16, align 8, !range !23, !invariant.load !7
  %18 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %19 = load i64, ptr %18, align 8, !range !24, !invariant.load !7
  %20 = add i64 %19, -1
  %21 = icmp sgt i64 %20, -1
  tail call void @llvm.assume(i1 %21)
  %22 = icmp eq i64 %17, 0
  br i1 %22, label %bb1.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %17, i64 noundef range(i64 1, -9223372036854775807) %19) #29
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %2, i64 noundef 24, i64 noundef 8) #29
  resume { ptr, i32 } %15

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %2, i64 noundef 24, i64 noundef 8) #29
  br label %bb1
}

; core::ptr::drop_in_place::<console::unix_term::Input<std::io::buffered::bufreader::BufReader<std::fs::File>>>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsC3JuwEIQwb_7console9unix_term5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtB1v_2fs4FileEEEBL_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !noundef !7
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console.exit, %start
  ret void

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !25)
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val2.i = load i64, ptr %2, align 8, !alias.scope !25, !noundef !7
  %3 = icmp eq i64 %_1.val2.i, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef %_1.val2.i, i64 noundef 1) #29, !noalias !25
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console.exit: ; preds = %bb2, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 40
  %.val1.i = load i32, ptr %4, align 8, !range !28, !alias.scope !25, !noundef !7
  %_5.i.i.i.i.i3.i = tail call noundef i32 @close(i32 noundef %.val1.i) #29, !noalias !25
  br label %bb1
}

; core::ptr::drop_in_place::<alloc::sync::Weak<std::sync::poison::mutex::Mutex<dyn console::term::TermRead>, &alloc::alloc::Global>>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtBL_5alloc6GlobalEEB29_(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #2 {
start:
  %0 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %0)
  %_16.i = icmp eq ptr %_1.0.val, inttoptr (i64 -1 to ptr)
  br i1 %_16.i, label %_RNvXsO_NtCsdJPVW0sQgAG_5alloc4syncINtB5_4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtB7_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1B_.exit, label %bb8.i

bb8.i:                                            ; preds = %start
  %_20.i = getelementptr inbounds nuw i8, ptr %_1.0.val, i64 8
  %1 = atomicrmw sub ptr %_20.i, i64 1 release, align 8
  %2 = icmp eq i64 %1, 1
  br i1 %2, label %bb1.i, label %_RNvXsO_NtCsdJPVW0sQgAG_5alloc4syncINtB5_4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtB7_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1B_.exit

bb1.i:                                            ; preds = %bb8.i
  fence acquire
  %3 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %3)
  %4 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %5 = load i64, ptr %4, align 8, !range !23, !invariant.load !7
  %6 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %7 = load i64, ptr %6, align 8, !range !24, !invariant.load !7
  %8 = add nsw i64 %5, -1
  %9 = add i64 %8, %7
  %10 = sub i64 0, %7
  %11 = and i64 %9, %10
  %12 = tail call i64 @llvm.umax.i64(i64 %7, i64 8)
  %13 = add i64 %12, 8
  %14 = add i64 %13, %11
  %15 = sub i64 0, %12
  %16 = and i64 %14, %15
  %17 = add i64 %12, 15
  %18 = add i64 %17, %16
  %19 = and i64 %18, %15
  %20 = icmp eq i64 %19, 0
  br i1 %20, label %_RNvXsO_NtCsdJPVW0sQgAG_5alloc4syncINtB5_4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtB7_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1B_.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb1.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %19, i64 noundef range(i64 1, -9223372036854775807) %12) #29
  br label %_RNvXsO_NtCsdJPVW0sQgAG_5alloc4syncINtB5_4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtB7_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1B_.exit

_RNvXsO_NtCsdJPVW0sQgAG_5alloc4syncINtB5_4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtB7_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1B_.exit: ; preds = %start, %bb8.i, %bb1.i, %bb1.i.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEEB1Z_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #1 {
start:
  %_1.val = load ptr, ptr %_1, align 8, !nonnull !7, !align !22, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i8, ptr %0, align 8, !range !29, !noundef !7
  %_3.i.i = getelementptr inbounds nuw i8, ptr %_1.val, i64 8
  %_3.i.i.i = trunc nuw i8 %_1.val1 to i1
  br i1 %_3.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %start
  %1 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %1, 9223372036854775807
  %2 = icmp eq i64 %_7.i.i.i, 0
  br i1 %2, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_3.i.i monotonic, align 8
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit: ; preds = %start, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %3 = load atomic ptr, ptr %_1.val monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %3)
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<u8>>>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %_1) unnamed_addr #1 {
start:
  %_1.val = load ptr, ptr %_1, align 8, !nonnull !7, !align !22, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load i8, ptr %0, align 8, !range !29, !noundef !7
  %_3.i.i = getelementptr inbounds nuw i8, ptr %_1.val, i64 8
  %_3.i.i.i = trunc nuw i8 %_1.val1 to i1
  br i1 %_3.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %start
  %1 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %1, 9223372036854775807
  %2 = icmp eq i64 %_7.i.i.i, 0
  br i1 %2, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_3.i.i monotonic, align 8
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit: ; preds = %start, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %3 = load atomic ptr, ptr %_1.val monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %3)
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockReadGuard<alloc::string::String>>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEEECsC3JuwEIQwb_7console(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !34)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self1.i.i = load ptr, ptr %0, align 8, !alias.scope !37, !nonnull !7, !align !22, !noundef !7
  %1 = load atomic ptr, ptr %self1.i.i acquire, align 8, !noalias !37
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb3.i.i.i, %start
  %prev.sroa.0.0.i.i.i = phi ptr [ %1, %start ], [ %_8.sroa.0.0.i.i.i.i, %bb3.i.i.i ]
  %_4.i.i.i.i = ptrtoint ptr %prev.sroa.0.0.i.i.i to i64
  %_3.i.i.i.i = and i64 %_4.i.i.i.i, 2
  %2 = icmp eq i64 %_3.i.i.i.i, 0
  br i1 %2, label %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i, label %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i

_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i: ; preds = %bb1.i.i.i
  %count.i.i.i.i = add i64 %_4.i.i.i.i, -17
  %_7.not.i.i.i.i = icmp eq i64 %count.i.i.i.i, 0
  %addr.i.i.i.i = or i64 %count.i.i.i.i, 1
  %3 = inttoptr i64 %addr.i.i.i.i to ptr
  %_6.sroa.0.0.i.i.i.i = select i1 %_7.not.i.i.i.i, ptr null, ptr %3
  br label %bb3.i.i.i

_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i: ; preds = %bb1.i.i.i
  %_9.i.i.i.i = and i64 %_4.i.i.i.i, 8
  %.not.i.i.i.i = icmp eq i64 %_9.i.i.i.i, 0
  %4 = tail call nonnull align 2 ptr @llvm.ptrmask.p0.i64(ptr %prev.sroa.0.0.i.i.i, i64 -10)
  br i1 %.not.i.i.i.i, label %bb3.i.i, label %bb3.i.i.i, !prof !38

bb3.i.i.i:                                        ; preds = %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i
  %_0.sroa.4.0.i7.i.i.i = phi ptr [ %_6.sroa.0.0.i.i.i.i, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i ], [ %4, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i ]
  %5 = cmpxchg weak ptr %self1.i.i, ptr %prev.sroa.0.0.i.i.i, ptr %_0.sroa.4.0.i7.i.i.i release acquire, align 8, !noalias !37
  %_8.sroa.18.0.in.i.i.i.i = extractvalue { ptr, i1 } %5, 1
  %_8.sroa.0.0.i.i.i.i = extractvalue { ptr, i1 } %5, 0
  br i1 %_8.sroa.18.0.in.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit, label %bb1.i.i.i

bb3.i.i:                                          ; preds = %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i
; call <std::sys::sync::rwlock::queue::RwLock>::read_unlock_contended
  tail call void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock21read_unlock_contended(ptr noundef nonnull align 8 %self1.i.i, ptr noundef %prev.sroa.0.0.i.i.i), !noalias !37
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit: ; preds = %bb3.i.i.i, %bb3.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<alloc::string::String>>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEEECsC3JuwEIQwb_7console(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %_1) unnamed_addr #1 {
start:
  %_1.val = load ptr, ptr %_1, align 8, !nonnull !7, !align !22, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load i8, ptr %0, align 8, !range !29, !noundef !7
  %_3.i.i = getelementptr inbounds nuw i8, ptr %_1.val, i64 8
  %_3.i.i.i = trunc nuw i8 %_1.val1 to i1
  br i1 %_3.i.i.i, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %start
  %1 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %1, 9223372036854775807
  %2 = icmp eq i64 %_7.i.i.i, 0
  br i1 %2, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_3.i.i monotonic, align 8
  br label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i

_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i: ; preds = %bb2.i.i.i, %bb6.i.i.i, %bb1.i.i.i, %start
  %3 = cmpxchg ptr %_1.val, ptr inttoptr (i64 1 to ptr), ptr null release monotonic, align 8
  %4 = extractvalue { ptr, i1 } %3, 1
  br i1 %4, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit, label %bb3.i.i, !prof !30

bb3.i.i:                                          ; preds = %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i
  %5 = extractvalue { ptr, i1 } %3, 0
; call <std::sys::sync::rwlock::queue::RwLock>::unlock_contended
  tail call void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock16unlock_contended(ptr noundef nonnull align 8 %_1.val, ptr noundef %5)
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit: ; preds = %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i, %bb3.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39)
  %ptr.i = load ptr, ptr %_1, align 8, !alias.scope !39, !noundef !7
  store ptr null, ptr %_1, align 8, !alias.scope !39
  %0 = icmp eq ptr %ptr.i, null
  br i1 %0, label %_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit, label %bb2.i.i

bb2.i.i:                                          ; preds = %start
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %ptr.i)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i unwind label %bb1.i.i.i.i, !noalias !39

bb1.i.i.i.i:                                      ; preds = %bb2.i.i
  %1 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i, i64 noundef 64, i64 noundef 8) #29, !noalias !39
  resume { ptr, i32 } %1

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i: ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i, i64 noundef 64, i64 noundef 8) #29, !noalias !39
  br label %_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit

_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit: ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr captures(address_is_null) %_1.0.val, i8 %_1.16.val) unnamed_addr #1 {
start:
  %0 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %0)
  %_3.i = getelementptr inbounds nuw i8, ptr %_1.0.val, i64 8
  %_3.i.i = trunc nuw i8 %_1.16.val to i1
  br i1 %_3.i.i, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_.exit, label %bb1.i.i

bb1.i.i:                                          ; preds = %start
  %1 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i = and i64 %1, 9223372036854775807
  %2 = icmp eq i64 %_7.i.i, 0
  br i1 %2, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_.exit, label %bb6.i.i, !prof !30

bb6.i.i:                                          ; preds = %bb1.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_.exit, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb6.i.i
  store atomic i8 1, ptr %_3.i monotonic, align 1
  br label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_.exit

_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_.exit: ; preds = %start, %bb1.i.i, %bb6.i.i, %bb2.i.i
  %3 = load atomic ptr, ptr %_1.0.val monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %3)
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::mutex::Mutex<()>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
; invoke <std::sys::sync::mutex::pthread::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(8) %_1)
          to label %bb4.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(8) %_1) #32
          to label %common.resume.i unwind label %terminate.i

bb4.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !42)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !45)
  %ptr.i.i.i = load ptr, ptr %_1, align 8, !alias.scope !48, !noundef !7
  store ptr null, ptr %_1, align 8, !alias.scope !48
  %1 = icmp eq ptr %ptr.i.i.i, null
  br i1 %1, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console.exit, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb4.i
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %ptr.i.i.i)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i unwind label %bb1.i.i.i.i.i.i, !noalias !51

common.resume.i:                                  ; preds = %bb1.i.i.i.i.i.i, %cleanup.i
  %common.resume.op.i = phi { ptr, i32 } [ %2, %bb1.i.i.i.i.i.i ], [ %0, %cleanup.i ]
  resume { ptr, i32 } %common.resume.op.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb2.i.i.i.i
  %2 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i, i64 noundef 64, i64 noundef 8) #29, !noalias !51
  br label %common.resume.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i: ; preds = %bb2.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i, i64 noundef 64, i64 noundef 8) #29, !noalias !51
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console.exit

terminate.i:                                      ; preds = %cleanup.i
  %3 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console.exit: ; preds = %bb4.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::rwlock::RwLockReadGuard<alloc::string::String>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(16) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !52)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self1.i = load ptr, ptr %0, align 8, !alias.scope !52, !nonnull !7, !align !22, !noundef !7
  %1 = load atomic ptr, ptr %self1.i acquire, align 8, !noalias !52
  br label %bb1.i.i

bb1.i.i:                                          ; preds = %bb3.i.i, %start
  %prev.sroa.0.0.i.i = phi ptr [ %1, %start ], [ %_8.sroa.0.0.i.i.i, %bb3.i.i ]
  %_4.i.i.i = ptrtoint ptr %prev.sroa.0.0.i.i to i64
  %_3.i.i.i = and i64 %_4.i.i.i, 2
  %2 = icmp eq i64 %_3.i.i.i, 0
  br i1 %2, label %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i, label %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i

_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i: ; preds = %bb1.i.i
  %count.i.i.i = add i64 %_4.i.i.i, -17
  %_7.not.i.i.i = icmp eq i64 %count.i.i.i, 0
  %addr.i.i.i = or i64 %count.i.i.i, 1
  %3 = inttoptr i64 %addr.i.i.i to ptr
  %_6.sroa.0.0.i.i.i = select i1 %_7.not.i.i.i, ptr null, ptr %3
  br label %bb3.i.i

_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i: ; preds = %bb1.i.i
  %_9.i.i.i = and i64 %_4.i.i.i, 8
  %.not.i.i.i = icmp eq i64 %_9.i.i.i, 0
  %4 = tail call nonnull align 2 ptr @llvm.ptrmask.p0.i64(ptr %prev.sroa.0.0.i.i, i64 -10)
  br i1 %.not.i.i.i, label %bb3.i, label %bb3.i.i, !prof !38

bb3.i.i:                                          ; preds = %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i
  %_0.sroa.4.0.i7.i.i = phi ptr [ %_6.sroa.0.0.i.i.i, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i ], [ %4, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i ]
  %5 = cmpxchg weak ptr %self1.i, ptr %prev.sroa.0.0.i.i, ptr %_0.sroa.4.0.i7.i.i release acquire, align 8, !noalias !52
  %_8.sroa.18.0.in.i.i.i = extractvalue { ptr, i1 } %5, 1
  %_8.sroa.0.0.i.i.i = extractvalue { ptr, i1 } %5, 0
  br i1 %_8.sroa.18.0.in.i.i.i, label %_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit, label %bb1.i.i

bb3.i:                                            ; preds = %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i
; call <std::sys::sync::rwlock::queue::RwLock>::read_unlock_contended
  tail call void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock21read_unlock_contended(ptr noundef nonnull align 8 %self1.i, ptr noundef %prev.sroa.0.0.i.i), !noalias !52
  br label %_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit

_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit: ; preds = %bb3.i.i, %bb3.i
  ret void
}

; core::ptr::drop_in_place::<console::term::TermInner>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term9TermInnerEBK_(ptr noalias noundef nonnull align 8 dereferenceable(168) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_2.i.i.i.i.i.i.i.i = alloca [24 x i8], align 8
  %_x.i.i.i.i.i.i = alloca [72 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 48
  tail call void @llvm.experimental.noalias.scope.decl(metadata !55)
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 111
  %2 = load i8, ptr %1, align 1, !range !58, !alias.scope !55, !noundef !7
  %switch1.i = icmp samesign ugt i8 %2, 1
  br i1 %switch1.i, label %bb8, label %bb2.i

bb2.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !59)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !62)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !65)
  %_10.0.i.i.i.i = load ptr, ptr %0, align 8, !alias.scope !68, !nonnull !7, !noundef !7
  %3 = atomicrmw sub ptr %_10.0.i.i.i.i, i64 1 release, align 8, !noalias !68
  %4 = icmp eq i64 %3, 1
  br i1 %4, label %bb2.i.i.i.i, label %bb6.i.i

bb2.i.i.i.i:                                      ; preds = %bb2.i
  fence acquire
; invoke <alloc::sync::Arc<std::sync::poison::mutex::Mutex<dyn console::term::TermRead>>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EE9drop_slowB1A_(ptr noalias noundef nonnull readonly align 8 dereferenceable(64) %0) #33
          to label %bb6.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %bb2.i.i.i.i
  %5 = landingpad { ptr, i32 }
          cleanup
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  tail call void @llvm.experimental.noalias.scope.decl(metadata !69)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !72)
  %_10.0.i.i3.i.i = load ptr, ptr %6, align 8, !alias.scope !75, !nonnull !7, !noundef !7
  %7 = atomicrmw sub ptr %_10.0.i.i3.i.i, i64 1 release, align 8, !noalias !75
  %8 = icmp eq i64 %7, 1
  br i1 %8, label %bb2.i.i4.i.i, label %bb3.i.i

bb2.i.i4.i.i:                                     ; preds = %cleanup.i.i
  fence acquire
; invoke <alloc::sync::Arc<std::sync::poison::mutex::Mutex<dyn console::term::TermRead>>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EE9drop_slowB1A_(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %6) #33
          to label %bb3.i.i unwind label %terminate.i.i

bb6.i.i:                                          ; preds = %bb2.i.i.i.i, %bb2.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  tail call void @llvm.experimental.noalias.scope.decl(metadata !76)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !79)
  %_10.0.i.i6.i.i = load ptr, ptr %9, align 8, !alias.scope !82, !nonnull !7, !noundef !7
  %10 = atomicrmw sub ptr %_10.0.i.i6.i.i, i64 1 release, align 8, !noalias !82
  %11 = icmp eq i64 %10, 1
  br i1 %11, label %bb2.i.i7.i.i, label %bb5.i.i

bb2.i.i7.i.i:                                     ; preds = %bb6.i.i
  fence acquire
; invoke <alloc::sync::Arc<std::sync::poison::mutex::Mutex<dyn console::term::TermRead>>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EE9drop_slowB1A_(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %9) #33
          to label %bb5.i.i unwind label %cleanup1.i.i

bb3.i.i:                                          ; preds = %cleanup1.i.i, %bb2.i.i4.i.i, %cleanup.i.i
  %.pn.i.i = phi { ptr, i32 } [ %13, %cleanup1.i.i ], [ %5, %bb2.i.i4.i.i ], [ %5, %cleanup.i.i ]
  %12 = getelementptr inbounds nuw i8, ptr %_1, i64 80
; invoke core::ptr::drop_in_place::<console::utils::Style>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console5utils5StyleEBK_(ptr noalias noundef readonly align 8 dereferenceable(32) %12) #32
          to label %cleanup.body unwind label %terminate.i.i

cleanup1.i.i:                                     ; preds = %bb2.i.i7.i.i
  %13 = landingpad { ptr, i32 }
          cleanup
  br label %bb3.i.i

bb5.i.i:                                          ; preds = %bb2.i.i7.i.i, %bb6.i.i
  %14 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  tail call void @llvm.experimental.noalias.scope.decl(metadata !83)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !86)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !89)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !92)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_x.i.i.i.i.i.i), !noalias !95
  %self1.sroa.0.0.copyload.i.i.i.i.i.i = load ptr, ptr %14, align 8, !alias.scope !95
  %.not.i.i.i.i.i.i = icmp eq ptr %self1.sroa.0.0.copyload.i.i.i.i.i.i, null
  br i1 %.not.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb5.i.i
  %self1.sroa.5.0.self.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 96
  %self1.sroa.5.0.copyload.i.i.i.i.i.i = load i64, ptr %self1.sroa.5.0.self.sroa_idx.i.i.i.i.i.i, align 8, !alias.scope !95
  %self1.sroa.4.0.self.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %self1.sroa.4.0.copyload.i.i.i.i.i.i = load i64, ptr %self1.sroa.4.0.self.sroa_idx.i.i.i.i.i.i, align 8, !alias.scope !95
  %full_range.sroa.4.0._x.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i.i.i.i, i64 8
  store ptr null, ptr %full_range.sroa.4.0._x.sroa_idx.i.i.i.i.i.i, align 8, !noalias !95
  %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i.i.i.i, i64 16
  store ptr %self1.sroa.0.0.copyload.i.i.i.i.i.i, ptr %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i.i.i.i.i, align 8, !noalias !95
  %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i.i.i.i, i64 24
  store i64 %self1.sroa.4.0.copyload.i.i.i.i.i.i, ptr %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i.i.i.i.i, align 8, !noalias !95
  %full_range.sroa.6.0._x.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i.i.i.i, i64 40
  store ptr null, ptr %full_range.sroa.6.0._x.sroa_idx.i.i.i.i.i.i, align 8, !noalias !95
  %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i.i.i.i, i64 48
  store ptr %self1.sroa.0.0.copyload.i.i.i.i.i.i, ptr %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i.i.i.i.i, align 8, !noalias !95
  %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i.i.i.i, i64 56
  store i64 %self1.sroa.4.0.copyload.i.i.i.i.i.i, ptr %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i.i.i.i.i, align 8, !noalias !95
  br label %bb3.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i.i, %bb5.i.i
  %.sink9.i.i.i.i.i.i = phi i64 [ 1, %bb1.i.i.i.i.i.i ], [ 0, %bb5.i.i ]
  %self1.sroa.5.0.copyload.sink.i.i.i.i.i.i = phi i64 [ %self1.sroa.5.0.copyload.i.i.i.i.i.i, %bb1.i.i.i.i.i.i ], [ 0, %bb5.i.i ]
  store i64 %.sink9.i.i.i.i.i.i, ptr %_x.i.i.i.i.i.i, align 8, !noalias !95
  %15 = getelementptr inbounds nuw i8, ptr %_x.i.i.i.i.i.i, i64 32
  store i64 %.sink9.i.i.i.i.i.i, ptr %15, align 8, !noalias !95
  %16 = getelementptr inbounds nuw i8, ptr %_x.i.i.i.i.i.i, i64 64
  store i64 %self1.sroa.5.0.copyload.sink.i.i.i.i.i.i, ptr %16, align 8, !noalias !95
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i.i.i.i.i), !noalias !96
; invoke <alloc::collections::btree::map::IntoIter<console::utils::Attribute, alloc::collections::btree::set_val::SetValZST>>::dying_next
  invoke fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10dying_nextB1b_(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i.i.i.i.i)
          to label %.noexc unwind label %cleanup.loopexit.split-lp

.noexc:                                           ; preds = %bb3.i.i.i.i.i.i
  %17 = load ptr, ptr %_2.i.i.i.i.i.i.i.i, align 8, !noalias !96, !noundef !7
  %.not1.i.i.i.i.i.i.i.i = icmp eq ptr %17, null
  br i1 %.not1.i.i.i.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term13ReadWritePairEBK_.exit.i, label %bb4.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i:                              ; preds = %.noexc, %.noexc9
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i.i.i.i.i), !noalias !96
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i.i.i.i.i), !noalias !96
; invoke <alloc::collections::btree::map::IntoIter<console::utils::Attribute, alloc::collections::btree::set_val::SetValZST>>::dying_next
  invoke fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10dying_nextB1b_(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i.i.i.i.i)
          to label %.noexc9 unwind label %cleanup.loopexit

.noexc9:                                          ; preds = %bb4.i.i.i.i.i.i.i.i
  %18 = load ptr, ptr %_2.i.i.i.i.i.i.i.i, align 8, !noalias !96, !noundef !7
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %18, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term13ReadWritePairEBK_.exit.i, label %bb4.i.i.i.i.i.i.i.i

terminate.i.i:                                    ; preds = %bb3.i.i, %bb2.i.i4.i.i
  %19 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !101
  unreachable

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term13ReadWritePairEBK_.exit.i: ; preds = %.noexc9, %.noexc
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i.i.i.i.i), !noalias !96
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_x.i.i.i.i.i.i), !noalias !95
  br label %bb8

cleanup.loopexit:                                 ; preds = %bb4.i.i.i.i.i.i.i.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body

cleanup.loopexit.split-lp:                        ; preds = %bb3.i.i.i.i.i.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body

cleanup.body:                                     ; preds = %cleanup.loopexit, %cleanup.loopexit.split-lp, %bb3.i.i
  %eh.lpad-body = phi { ptr, i32 } [ %.pn.i.i, %bb3.i.i ], [ %lpad.loopexit, %cleanup.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.loopexit.split-lp ]
; invoke core::ptr::drop_in_place::<core::option::Option<std::sync::poison::mutex::Mutex<alloc::vec::Vec<u8>>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef align 8 dereferenceable(48) %_1) #32
          to label %bb4 unwind label %terminate

bb8:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term13ReadWritePairEBK_.exit.i, %start
; invoke core::ptr::drop_in_place::<core::option::Option<std::sync::poison::mutex::Mutex<alloc::vec::Vec<u8>>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef align 8 dereferenceable(48) %_1)
          to label %bb7 unwind label %cleanup1

bb4:                                              ; preds = %cleanup.body, %cleanup1
  %.pn = phi { ptr, i32 } [ %23, %cleanup1 ], [ %eh.lpad-body, %cleanup.body ]
  %20 = getelementptr inbounds nuw i8, ptr %_1, i64 128
  %.val = load i64, ptr %20, align 8
  %21 = icmp eq i64 %.val, 0
  br i1 %21, label %bb3, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb4
  %22 = getelementptr inbounds nuw i8, ptr %_1, i64 136
  %.val6 = load ptr, ptr %22, align 8, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val6, i64 noundef %.val, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb3

cleanup1:                                         ; preds = %bb8
  %23 = landingpad { ptr, i32 }
          cleanup
  br label %bb4

bb7:                                              ; preds = %bb8
  %24 = getelementptr inbounds nuw i8, ptr %_1, i64 128
  %.val7 = load i64, ptr %24, align 8
  %25 = icmp eq i64 %.val7, 0
  br i1 %25, label %bb6, label %bb2.i.i.i4.i.i.i.i10

bb2.i.i.i4.i.i.i.i10:                             ; preds = %bb7
  %26 = getelementptr inbounds nuw i8, ptr %_1, i64 136
  %.val8 = load ptr, ptr %26, align 8, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val8, i64 noundef %.val7, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb6

bb3:                                              ; preds = %bb2.i.i.i4.i.i.i.i, %bb4
  %27 = getelementptr inbounds nuw i8, ptr %_1, i64 152
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::Mutex<()>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECsC3JuwEIQwb_7console(ptr noalias noundef align 8 dereferenceable(16) %27) #32
          to label %common.resume unwind label %terminate

bb6:                                              ; preds = %bb2.i.i.i4.i.i.i.i10, %bb7
  %28 = getelementptr inbounds nuw i8, ptr %_1, i64 152
; invoke <std::sys::sync::mutex::pthread::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(16) %28)
          to label %bb4.i.i unwind label %cleanup.i.i12

cleanup.i.i12:                                    ; preds = %bb6
  %29 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %28) #32
          to label %common.resume unwind label %terminate.i.i13

bb4.i.i:                                          ; preds = %bb6
  tail call void @llvm.experimental.noalias.scope.decl(metadata !102)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !105)
  %ptr.i.i.i.i = load ptr, ptr %28, align 8, !alias.scope !108, !noundef !7
  store ptr null, ptr %28, align 8, !alias.scope !108
  %30 = icmp eq ptr %ptr.i.i.i.i, null
  br i1 %30, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECsC3JuwEIQwb_7console.exit, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb4.i.i
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %ptr.i.i.i.i)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i unwind label %bb1.i.i.i.i.i.i.i, !noalias !113

common.resume:                                    ; preds = %bb3, %cleanup.i.i12, %bb1.i.i.i.i.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %31, %bb1.i.i.i.i.i.i.i ], [ %29, %cleanup.i.i12 ], [ %.pn, %bb3 ]
  resume { ptr, i32 } %common.resume.op

bb1.i.i.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i.i
  %31 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef 64, i64 noundef 8) #29, !noalias !113
  br label %common.resume

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i: ; preds = %bb2.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef 64, i64 noundef 8) #29, !noalias !113
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECsC3JuwEIQwb_7console.exit

terminate.i.i13:                                  ; preds = %cleanup.i.i12
  %32 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECsC3JuwEIQwb_7console.exit: ; preds = %bb4.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i
  ret void

terminate:                                        ; preds = %bb3, %cleanup.body
  %33 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; core::ptr::drop_in_place::<console::utils::Style>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console5utils5StyleEBK_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_2.i.i.i.i.i = alloca [24 x i8], align 8
  %_x.i.i.i = alloca [72 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !114)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !117)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !120)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_x.i.i.i), !noalias !123
  %self1.sroa.0.0.copyload.i.i.i = load ptr, ptr %_1, align 8, !alias.scope !123
  %.not.i.i.i = icmp eq ptr %self1.sroa.0.0.copyload.i.i.i, null
  br i1 %.not.i.i.i, label %bb3.i.i.i, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %start
  %self1.sroa.5.0.self.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %self1.sroa.5.0.copyload.i.i.i = load i64, ptr %self1.sroa.5.0.self.sroa_idx.i.i.i, align 8, !alias.scope !123
  %self1.sroa.4.0.self.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self1.sroa.4.0.copyload.i.i.i = load i64, ptr %self1.sroa.4.0.self.sroa_idx.i.i.i, align 8, !alias.scope !123
  %full_range.sroa.4.0._x.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i, i64 8
  store ptr null, ptr %full_range.sroa.4.0._x.sroa_idx.i.i.i, align 8, !noalias !123
  %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i, i64 16
  store ptr %self1.sroa.0.0.copyload.i.i.i, ptr %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i.i, align 8, !noalias !123
  %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i, i64 24
  store i64 %self1.sroa.4.0.copyload.i.i.i, ptr %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i.i, align 8, !noalias !123
  %full_range.sroa.6.0._x.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i, i64 40
  store ptr null, ptr %full_range.sroa.6.0._x.sroa_idx.i.i.i, align 8, !noalias !123
  %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i, i64 48
  store ptr %self1.sroa.0.0.copyload.i.i.i, ptr %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i.i, align 8, !noalias !123
  %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i.i, i64 56
  store i64 %self1.sroa.4.0.copyload.i.i.i, ptr %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i.i, align 8, !noalias !123
  br label %bb3.i.i.i

bb3.i.i.i:                                        ; preds = %bb1.i.i.i, %start
  %.sink9.i.i.i = phi i64 [ 1, %bb1.i.i.i ], [ 0, %start ]
  %self1.sroa.5.0.copyload.sink.i.i.i = phi i64 [ %self1.sroa.5.0.copyload.i.i.i, %bb1.i.i.i ], [ 0, %start ]
  store i64 %.sink9.i.i.i, ptr %_x.i.i.i, align 8, !noalias !123
  %0 = getelementptr inbounds nuw i8, ptr %_x.i.i.i, i64 32
  store i64 %.sink9.i.i.i, ptr %0, align 8, !noalias !123
  %1 = getelementptr inbounds nuw i8, ptr %_x.i.i.i, i64 64
  store i64 %self1.sroa.5.0.copyload.sink.i.i.i, ptr %1, align 8, !noalias !123
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i.i), !noalias !124
; call <alloc::collections::btree::map::IntoIter<console::utils::Attribute, alloc::collections::btree::set_val::SetValZST>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10dying_nextB1b_(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i.i), !noalias !123
  %2 = load ptr, ptr %_2.i.i.i.i.i, align 8, !noalias !124, !noundef !7
  %.not1.i.i.i.i.i = icmp eq ptr %2, null
  br i1 %.not1.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3set8BTreeSetNtNtCsC3JuwEIQwb_7console5utils9AttributeEEB1J_.exit, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb3.i.i.i, %bb4.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i.i), !noalias !124
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i.i), !noalias !124
; call <alloc::collections::btree::map::IntoIter<console::utils::Attribute, alloc::collections::btree::set_val::SetValZST>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10dying_nextB1b_(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i.i), !noalias !123
  %3 = load ptr, ptr %_2.i.i.i.i.i, align 8, !noalias !124, !noundef !7
  %.not.i.i.i.i.i = icmp eq ptr %3, null
  br i1 %.not.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3set8BTreeSetNtNtCsC3JuwEIQwb_7console5utils9AttributeEEB1J_.exit, label %bb4.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3set8BTreeSetNtNtCsC3JuwEIQwb_7console5utils9AttributeEEB1J_.exit: ; preds = %bb4.i.i.i.i.i, %bb3.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i.i), !noalias !124
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_x.i.i.i), !noalias !123
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional, i64 noundef range(i64 1, 5) %elem_layout.0, i64 noundef range(i64 1, 5) %elem_layout.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !129)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !23, !alias.scope !129, !noundef !7
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 range(i64 0, -1) %v16.i)
  %0 = icmp eq i64 %elem_layout.1, 1
  %v1.sroa.0.0.i = select i1 %0, i64 8, i64 4
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 range(i64 0, -1) %v1.sroa.0.0.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !129
  %1 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !129
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsC3JuwEIQwb_7console(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i, i64 noundef range(i64 1, 5) %elem_layout.0, i64 noundef range(i64 1, 5) %elem_layout.1), !noalias !129
  %_35.i = load i64, ptr %self3.i, align 8, !range !6, !noalias !129, !noundef !7
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %3, align 8, !range !132, !noalias !129, !noundef !7
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !129
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !129
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #30
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %3, align 8, !noalias !129, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !129
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !129
  %5 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !129
  ret void
}

; console::unix_term::read_secure::{closure#3}
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RNCNvNtCsC3JuwEIQwb_7console9unix_term11read_secures1_0B5_(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr dead_on_return noalias noundef nonnull align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_12 = load ptr, ptr %0, align 8, !nonnull !7, !noundef !7
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_11 = load i64, ptr %1, align 8, !noundef !7
  %2 = icmp samesign eq i64 %_11, 0
  br i1 %2, label %bb1.i.thread, label %bb17.i.i.i.i.i.preheader.i

bb17.i.i.i.i.i.preheader.i:                       ; preds = %start
  %_7.i.i.i.i = getelementptr inbounds nuw i8, ptr %_12, i64 %_11
  br label %bb17.i.i.i.i.i.i

bb17.i.i.i.i.i.i:                                 ; preds = %bb5.i.i.i, %bb17.i.i.i.i.i.preheader.i
  %_23.i25.i.i.i1213.i.i.i = phi ptr [ %_4.i.i.i.i.i, %bb5.i.i.i ], [ %_7.i.i.i.i, %bb17.i.i.i.i.i.preheader.i ]
  %_23.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i.i, i64 -1
  %w.i.i.i.i.i.i = load i8, ptr %_23.i.i.i.i.i.i.i, align 1, !alias.scope !133, !noalias !136, !noundef !7
  %_6.i.i.i.i.i.i = icmp sgt i8 %w.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb17.i.i.i.i.i.i
  %3 = icmp ne ptr %_12, %_23.i.i.i.i.i.i.i
  tail call void @llvm.assume(i1 %3)
  %_23.i13.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i.i, i64 -2
  %z.i.i.i.i.i.i = load i8, ptr %_23.i13.i.i.i.i.i.i, align 1, !alias.scope !133, !noalias !136, !noundef !7
  %_25.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i, 31
  %4 = zext nneg i8 %_25.i.i.i.i.i.i to i32
  %_12.i.i.i.i.i.i = icmp slt i8 %z.i.i.i.i.i.i, -64
  br i1 %_12.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i, label %bb13.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb17.i.i.i.i.i.i
  %_8.i.i.i.i.i.i = zext nneg i8 %w.i.i.i.i.i.i to i32
  br label %bb1.i.i.i.i.i.i.preheader.i

bb6.i.i.i.i.i.i:                                  ; preds = %bb4.i.i.i.i.i.i
  %5 = icmp ne ptr %_12, %_23.i13.i.i.i.i.i.i
  tail call void @llvm.assume(i1 %5)
  %_23.i19.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i.i, i64 -3
  %y.i.i.i.i.i.i = load i8, ptr %_23.i19.i.i.i.i.i.i, align 1, !alias.scope !133, !noalias !136, !noundef !7
  %_29.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i, 15
  %6 = zext nneg i8 %_29.i.i.i.i.i.i to i32
  %_16.i.i.i.i.i.i = icmp slt i8 %y.i.i.i.i.i.i, -64
  br i1 %_16.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i

bb13.i.i.i.i.i.i:                                 ; preds = %bb11.i.i.i.i.i.i, %bb4.i.i.i.i.i.i
  %_4.i14.i.i.i.i.i = phi ptr [ %_4.i15.i.i.i.i.i, %bb11.i.i.i.i.i.i ], [ %_23.i13.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ]
  %ch.sroa.0.0.i.i.i.i.i.i = phi i32 [ %11, %bb11.i.i.i.i.i.i ], [ %4, %bb4.i.i.i.i.i.i ]
  %_40.i.i.i.i.i.i = shl nuw nsw i32 %ch.sroa.0.0.i.i.i.i.i.i, 6
  %_42.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i, 63
  %_41.i.i.i.i.i.i = zext nneg i8 %_42.i.i.i.i.i.i to i32
  %7 = or disjoint i32 %_40.i.i.i.i.i.i, %_41.i.i.i.i.i.i
  br label %bb1.i.i.i.i.i.i.preheader.i

bb8.i.i.i.i.i.i:                                  ; preds = %bb6.i.i.i.i.i.i
  %8 = icmp ne ptr %_12, %_23.i19.i.i.i.i.i.i
  tail call void @llvm.assume(i1 %8)
  %_23.i25.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i.i, i64 -4
  %x.i.i.i.i.i.i = load i8, ptr %_23.i25.i.i.i.i.i.i, align 1, !alias.scope !133, !noalias !136, !noundef !7
  %_33.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i, 7
  %9 = zext nneg i8 %_33.i.i.i.i.i.i to i32
  %_34.i.i.i.i.i.i = shl nuw nsw i32 %9, 6
  %_36.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i, 63
  %_35.i.i.i.i.i.i = zext nneg i8 %_36.i.i.i.i.i.i to i32
  %10 = or disjoint i32 %_34.i.i.i.i.i.i, %_35.i.i.i.i.i.i
  br label %bb11.i.i.i.i.i.i

bb11.i.i.i.i.i.i:                                 ; preds = %bb8.i.i.i.i.i.i, %bb6.i.i.i.i.i.i
  %_4.i15.i.i.i.i.i = phi ptr [ %_23.i25.i.i.i.i.i.i, %bb8.i.i.i.i.i.i ], [ %_23.i19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ]
  %ch.sroa.0.1.i.i.i.i.i.i = phi i32 [ %10, %bb8.i.i.i.i.i.i ], [ %6, %bb6.i.i.i.i.i.i ]
  %_37.i.i.i.i.i.i = shl nuw nsw i32 %ch.sroa.0.1.i.i.i.i.i.i, 6
  %_39.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i, 63
  %_38.i.i.i.i.i.i = zext nneg i8 %_39.i.i.i.i.i.i to i32
  %11 = or disjoint i32 %_37.i.i.i.i.i.i, %_38.i.i.i.i.i.i
  br label %bb13.i.i.i.i.i.i

bb1.i.i.i.i.i.i.preheader.i:                      ; preds = %bb13.i.i.i.i.i.i, %bb3.i.i.i.i.i.i
  %_4.i.i.i.i.i = phi ptr [ %_23.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ], [ %_4.i14.i.i.i.i.i, %bb13.i.i.i.i.i.i ]
  %_0.sroa.4.1.i.ph.i.i.i.i.i = phi i32 [ %_8.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ], [ %7, %bb13.i.i.i.i.i.i ]
  %12 = icmp samesign ult i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, 1114112
  tail call void @llvm.assume(i1 %12)
  switch i32 %_0.sroa.4.1.i.ph.i.i.i.i.i, label %bb1 [
    i32 13, label %bb5.i.i.i
    i32 10, label %bb5.i.i.i
  ]

bb5.i.i.i:                                        ; preds = %bb1.i.i.i.i.i.i.preheader.i, %bb1.i.i.i.i.i.i.preheader.i
  %13 = icmp eq ptr %_12, %_4.i.i.i.i.i
  br i1 %13, label %bb1.i.thread, label %bb17.i.i.i.i.i.i

cleanup:                                          ; preds = %bb3.i2
  %14 = landingpad { ptr, i32 }
          cleanup
  %_1.val = load i64, ptr %_1, align 8
  %15 = icmp eq i64 %_1.val, 0
  br i1 %15, label %bb4, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_12, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb4

bb1.i.thread:                                     ; preds = %bb5.i.i.i, %start
  %_9.i4 = icmp sgt i64 %_11, -1
  tail call void @llvm.assume(i1 %_9.i4)
  br label %bb14.i

bb1:                                              ; preds = %bb1.i.i.i.i.i.i.preheader.i
  %16 = ptrtoint ptr %_23.i25.i.i.i1213.i.i.i to i64
  %17 = ptrtoint ptr %_12 to i64
  %_15.i.i.i.i = sub i64 %16, %17
  tail call void @llvm.experimental.noalias.scope.decl(metadata !150)
  %_9.i = icmp sgt i64 %_11, -1
  tail call void @llvm.assume(i1 %_9.i)
  %_3.not.i = icmp ugt i64 %_15.i.i.i.i, %_11
  br i1 %_3.not.i, label %bb2, label %bb1.i

bb1.i:                                            ; preds = %bb1
  %18 = icmp ne i64 %_15.i.i.i.i, 0
  %_15.not.i = icmp samesign ult i64 %_15.i.i.i.i, %_11
  %or.cond.i = select i1 %18, i1 %_15.not.i, i1 false
  br i1 %or.cond.i, label %bb11.i, label %bb14.i

bb11.i:                                           ; preds = %bb1.i
  %19 = getelementptr inbounds nuw i8, ptr %_12, i64 %_15.i.i.i.i
  %self1.i = load i8, ptr %19, align 1, !noalias !150, !noundef !7
  %20 = icmp sgt i8 %self1.i, -65
  br i1 %20, label %bb14.i, label %bb3.i2, !prof !30

bb3.i2:                                           ; preds = %bb11.i
; invoke core::panicking::panic
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_84da818df01979596a9e7a52ed4fd1e4, i64 noundef 48, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a5020ff128eafee240ced9e9d62411d1) #34
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb3.i2
  unreachable

bb14.i:                                           ; preds = %bb1.i.thread, %bb11.i, %bb1.i
  %j.sroa.0.0.i610 = phi i64 [ 0, %bb1.i.thread ], [ %_15.i.i.i.i, %bb11.i ], [ %_15.i.i.i.i, %bb1.i ]
  store i64 %j.sroa.0.0.i610, ptr %1, align 8, !alias.scope !150
  br label %bb2

bb2:                                              ; preds = %bb14.i, %bb1
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %_1, i64 24, i1 false)
  ret void

bb4:                                              ; preds = %bb2.i.i.i4.i.i.i, %cleanup
  resume { ptr, i32 } %14
}

; <console::term::Term>::with_inner
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term10with_inner(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_0, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(168) %inner) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_7 = alloca [184 x i8], align 16
  %term = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %term)
  call void @llvm.lifetime.start.p0(i64 184, ptr nonnull %_7)
  store <2 x i64> splat (i64 1), ptr %_7, align 16
  %0 = getelementptr inbounds nuw i8, ptr %_7, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(168) %0, ptr noundef nonnull align 8 dereferenceable(168) %inner, i64 168, i1 false)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !153
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 dereferenceable_or_null(184) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 184, i64 noundef 8) #29, !noalias !153
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb2.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCsC3JuwEIQwb_7console4term9TermInnerEE3newB14_.exit, !prof !5

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 184) #30
          to label %.noexc unwind label %cleanup.i

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup.i:                                        ; preds = %bb2.i
  %3 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<console::term::TermInner>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term9TermInnerEBK_(ptr noalias noundef align 8 dereferenceable(168) %0)
          to label %common.resume unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

common.resume:                                    ; preds = %bb2.i.i.i, %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %3, %cleanup.i ], [ %6, %cleanup ], [ %6, %bb2.i.i.i ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCsC3JuwEIQwb_7console4term9TermInnerEE3newB14_.exit: ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(184) %1, ptr noundef nonnull align 16 dereferenceable(184) %_7, i64 184, i1 false)
  call void @llvm.lifetime.end.p0(i64 184, ptr nonnull %_7)
  store ptr %1, ptr %term, align 8
  %5 = getelementptr inbounds nuw i8, ptr %term, i64 8
  store i16 0, ptr %5, align 8
; invoke <console::term::Term as std::os::fd::raw::AsRawFd>::as_raw_fd
  %_15 = invoke noundef i32 @_RNvXs2_NtCsC3JuwEIQwb_7console4termNtB5_4TermNtNtNtNtCs5sEH5CPMdak_3std2os2fd3raw7AsRawFd9as_raw_fd(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %term)
          to label %bb4 unwind label %cleanup

cleanup:                                          ; preds = %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCsC3JuwEIQwb_7console4term9TermInnerEE3newB14_.exit
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = atomicrmw sub ptr %1, i64 1 release, align 8, !noalias !156
  %8 = icmp eq i64 %7, 1
  br i1 %8, label %bb2.i.i.i, label %common.resume

bb2.i.i.i:                                        ; preds = %cleanup
  fence acquire
; invoke <alloc::sync::Arc<console::term::TermInner>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerE9drop_slowBK_(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %term) #33
          to label %common.resume unwind label %terminate

bb4:                                              ; preds = %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCsC3JuwEIQwb_7console4term9TermInnerEE3newB14_.exit
  %9 = getelementptr inbounds nuw i8, ptr %term, i64 9
  %_14 = tail call noundef i32 @isatty(i32 noundef %_15) #29
  %_4 = icmp ne i32 %_14, 0
  %10 = zext i1 %_4 to i8
  store i8 %10, ptr %9, align 1
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) %term, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %term)
  ret void

terminate:                                        ; preds = %bb2.i.i.i
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; <console::term::Term>::write_line
; Function Attrs: uwtable
define noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term10write_line(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %0, i64 noundef %1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i6 = alloca [16 x i8], align 8
  %e.i = alloca [16 x i8], align 8
  %args = alloca [32 x i8], align 8
  %_23 = alloca [16 x i8], align 8
  %_19 = alloca [24 x i8], align 8
  %prompt = alloca [16 x i8], align 8
  %s = alloca [16 x i8], align 8
  store ptr %0, ptr %s, align 8
  %2 = getelementptr inbounds nuw i8, ptr %s, i64 8
  store i64 %1, ptr %2, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %prompt)
  %_29 = load ptr, ptr %self, align 8, !nonnull !7, !noundef !7
  %_5 = getelementptr inbounds nuw i8, ptr %_29, i64 128
  %3 = load atomic ptr, ptr %_5 monotonic, align 8
  br label %bb1.i

bb1.i:                                            ; preds = %bb3.i, %start
  %prev.sroa.0.0.i = phi ptr [ %3, %start ], [ %_8.sroa.0.0.i.i, %bb3.i ]
  %self1.i.i.i = ptrtoint ptr %prev.sroa.0.0.i to i64
  %_2.i.i.i = and i64 %self1.i.i.i, 2
  %4 = icmp eq i64 %_2.i.i.i, 0
  %_4.i.i.i = icmp ne ptr %prev.sroa.0.0.i, inttoptr (i64 1 to ptr)
  %or.cond.not3.not4.i.i.i = and i1 %_4.i.i.i, %4
  %_11.1.i.i.i = icmp ule ptr %prev.sroa.0.0.i, inttoptr (i64 -17 to ptr)
  %or.cond2.not.i.i.i = and i1 %_11.1.i.i.i, %or.cond.not3.not4.i.i.i
  br i1 %or.cond2.not.i.i.i, label %bb3.i, label %bb24

bb3.i:                                            ; preds = %bb1.i
  %5 = or i64 %self1.i.i.i, 1
  %addr.i.i.i = add i64 %5, 16
  %_5.i.i.i = inttoptr i64 %addr.i.i.i to ptr
  %6 = cmpxchg weak ptr %_5, ptr %prev.sroa.0.0.i, ptr %_5.i.i.i acquire monotonic, align 8
  %_8.sroa.18.0.in.i.i = extractvalue { ptr, i1 } %6, 1
  %_8.sroa.0.0.i.i = extractvalue { ptr, i1 } %6, 0
  br i1 %_8.sroa.18.0.in.i.i, label %bb22, label %bb1.i

bb24:                                             ; preds = %bb1.i
; call <std::sys::sync::rwlock::queue::RwLock>::lock_contended
  tail call void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock14lock_contended(ptr noundef nonnull align 8 %_5, i1 noundef zeroext false)
  br label %bb22

bb22:                                             ; preds = %bb3.i, %bb24
  %_9.i = getelementptr inbounds nuw i8, ptr %_29, i64 136
  %7 = load atomic i8, ptr %_9.i monotonic, align 1, !noalias !163
  %.not68 = icmp eq i8 %7, 0
  %_8.i1.sink.i.i = getelementptr inbounds nuw i8, ptr %_29, i64 144
  br i1 %.not68, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit, label %bb2.i10, !prof !30

bb2.i10:                                          ; preds = %bb22
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i6), !noalias !166
  store ptr %_8.i1.sink.i.i, ptr %e.i6, align 8, !noalias !166
  %8 = getelementptr inbounds nuw i8, ptr %e.i6, i64 8
  store ptr %_5, ptr %8, align 8, !noalias !166
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i6, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_10ae8e1c7422912565b0a725bff29948) #30
          to label %unreachable.i14 unwind label %cleanup.i11, !noalias !166

cleanup.i11:                                      ; preds = %bb2.i10
  %9 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockReadGuard<alloc::string::String>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i6) #32
          to label %common.resume unwind label %terminate.i12, !noalias !166

unreachable.i14:                                  ; preds = %bb2.i10
  unreachable

terminate.i12:                                    ; preds = %cleanup.i11
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !166
  unreachable

common.resume:                                    ; preds = %bb19, %cleanup.i11
  %common.resume.op = phi { ptr, i32 } [ %9, %cleanup.i11 ], [ %.pn, %bb19 ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit: ; preds = %bb22
  store ptr %_8.i1.sink.i.i, ptr %prompt, align 8
  %11 = getelementptr inbounds nuw i8, ptr %prompt, i64 8
  store ptr %_5, ptr %11, align 8
  %12 = getelementptr inbounds nuw i8, ptr %_29, i64 160
  %_37 = load i64, ptr %12, align 8, !noundef !7
  %_38 = icmp sgt i64 %_37, -1
  tail call void @llvm.assume(i1 %_38)
  %13 = icmp eq i64 %_37, 0
  br i1 %13, label %bb5, label %bb3

bb3:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
; invoke <console::term::Term>::write_str
  %14 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_34a907ce632321878f08de7a619b60ed, i64 noundef 5)
          to label %bb26 unwind label %cleanup

bb5:                                              ; preds = %bb26, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
  %15 = getelementptr inbounds nuw i8, ptr %_29, i64 16
  %_9 = load i64, ptr %15, align 8, !range !6, !noundef !7
  %16 = trunc nuw i64 %_9 to i1
  br i1 %16, label %bb7, label %bb6

bb19:                                             ; preds = %cleanup.i, %bb2.i.i.i4.i.i, %cleanup1, %cleanup, %cleanup2
  %.pn = phi { ptr, i32 } [ %48, %cleanup2 ], [ %17, %cleanup ], [ %34, %cleanup1 ], [ %34, %bb2.i.i.i4.i.i ], [ %45, %cleanup.i ]
; invoke core::ptr::drop_in_place::<std::sync::poison::rwlock::RwLockReadGuard<alloc::string::String>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console(ptr noalias noundef align 8 dereferenceable(16) %prompt) #32
          to label %common.resume unwind label %terminate

cleanup:                                          ; preds = %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit.i, %bb6.i.i.i, %bb6, %bb6.i.i, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, %bb7.i.i, %bb3
  %17 = landingpad { ptr, i32 }
          cleanup
  br label %bb19

bb26:                                             ; preds = %bb3
  %.not = icmp eq ptr %14, null
  br i1 %.not, label %bb5, label %bb27

bb27:                                             ; preds = %bb26
  %18 = load atomic ptr, ptr %_5 acquire, align 8, !noalias !169
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb3.i.i.i, %bb27
  %prev.sroa.0.0.i.i.i = phi ptr [ %18, %bb27 ], [ %_8.sroa.0.0.i.i.i.i, %bb3.i.i.i ]
  %_4.i.i.i.i = ptrtoint ptr %prev.sroa.0.0.i.i.i to i64
  %_3.i.i.i.i = and i64 %_4.i.i.i.i, 2
  %19 = icmp eq i64 %_3.i.i.i.i, 0
  br i1 %19, label %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i, label %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i

_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i: ; preds = %bb1.i.i.i
  %count.i.i.i.i = add i64 %_4.i.i.i.i, -17
  %_7.not.i.i.i.i = icmp eq i64 %count.i.i.i.i, 0
  %addr.i.i.i.i = or i64 %count.i.i.i.i, 1
  %20 = inttoptr i64 %addr.i.i.i.i to ptr
  %_6.sroa.0.0.i.i.i.i = select i1 %_7.not.i.i.i.i, ptr null, ptr %20
  br label %bb3.i.i.i

_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i: ; preds = %bb1.i.i.i
  %_9.i.i.i.i = and i64 %_4.i.i.i.i, 8
  %.not.i.i.i.i = icmp eq i64 %_9.i.i.i.i, 0
  %21 = tail call nonnull align 2 ptr @llvm.ptrmask.p0.i64(ptr %prev.sroa.0.0.i.i.i, i64 -10)
  br i1 %.not.i.i.i.i, label %bb3.i.i, label %bb3.i.i.i, !prof !38

bb3.i.i.i:                                        ; preds = %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i
  %_0.sroa.4.0.i7.i.i.i = phi ptr [ %_6.sroa.0.0.i.i.i.i, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i ], [ %21, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i ]
  %22 = cmpxchg weak ptr %_5, ptr %prev.sroa.0.0.i.i.i, ptr %_0.sroa.4.0.i7.i.i.i release acquire, align 8, !noalias !169
  %_8.sroa.18.0.in.i.i.i.i = extractvalue { ptr, i1 } %22, 1
  %_8.sroa.0.0.i.i.i.i = extractvalue { ptr, i1 } %22, 0
  br i1 %_8.sroa.18.0.in.i.i.i.i, label %bb16, label %bb1.i.i.i

bb3.i.i:                                          ; preds = %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i
; call <std::sys::sync::rwlock::queue::RwLock>::read_unlock_contended
  tail call void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock21read_unlock_contended(ptr noundef nonnull align 8 %_5, ptr noundef %prev.sroa.0.0.i.i.i), !noalias !169
  br label %bb16

bb7:                                              ; preds = %bb5
  %mutex = getelementptr inbounds nuw i8, ptr %_29, i64 24
  %23 = load atomic ptr, ptr %mutex acquire, align 8, !noalias !174
  %24 = icmp eq ptr %23, null
  br i1 %24, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, !prof !5

bb7.i.i:                                          ; preds = %bb7
; invoke <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %25 = invoke fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 %mutex)
          to label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i unwind label %cleanup

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i: ; preds = %bb7.i.i, %bb7
  %_0.sroa.0.0.i.i = phi ptr [ %23, %bb7 ], [ %25, %bb7.i.i ]
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::lock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i)
          to label %.noexc23 unwind label %cleanup

.noexc23:                                         ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i
  %26 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !174
  %_6.i.i = and i64 %26, 9223372036854775807
  %27 = icmp eq i64 %_6.i.i, 0
  br i1 %27, label %bb8, label %bb6.i.i, !prof !30

bb6.i.i:                                          ; preds = %.noexc23
; invoke std::panicking::panic_count::is_zero_slow_path
  %28 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
          to label %.noexc24 unwind label %cleanup

.noexc24:                                         ; preds = %bb6.i.i
  %29 = xor i1 %28, true
  br label %bb8

bb6:                                              ; preds = %bb5
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_19)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_23)
  %30 = getelementptr inbounds nuw i8, ptr %_29, i64 152
  %_100 = load ptr, ptr %30, align 8, !nonnull !7, !noundef !7
  %_99 = load i64, ptr %12, align 8, !noundef !7
  store ptr %_100, ptr %_23, align 8
  %31 = getelementptr inbounds nuw i8, ptr %_23, i64 8
  store i64 %_99, ptr %31, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args)
  store ptr %s, ptr %args, align 8
  %_25.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsC3JuwEIQwb_7console, ptr %_25.sroa.4.0..sroa_idx, align 8
  %32 = getelementptr inbounds nuw i8, ptr %args, i64 16
  store ptr %_23, ptr %32, align 8
  %_26.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 24
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsC3JuwEIQwb_7console, ptr %_26.sroa.4.0..sroa_idx, align 8
; invoke alloc::fmt::format::format_inner
  invoke void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_19, ptr noundef nonnull @alloc_71ab024f63e05fa8c544dae9bff3eae8, ptr noundef nonnull %args)
          to label %bb34 unwind label %cleanup

bb34:                                             ; preds = %bb6
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_23)
  %_18.sroa.0.0.copyload = load i64, ptr %_19, align 8
  %_18.sroa.5.0._19.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 8
  %_18.sroa.5.0.copyload = load ptr, ptr %_18.sroa.5.0._19.sroa_idx, align 8, !nonnull !7, !noundef !7
  %_18.sroa.8.0._19.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 16
  %_18.sroa.8.0.copyload = load i64, ptr %_18.sroa.8.0._19.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_19)
; invoke <console::term::Term>::write_through
  %33 = invoke fastcc noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term13write_through(ptr nonnull %_29, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_18.sroa.5.0.copyload, i64 noundef %_18.sroa.8.0.copyload)
          to label %bb11 unwind label %cleanup1

cleanup1:                                         ; preds = %bb34
  %34 = landingpad { ptr, i32 }
          cleanup
  %35 = icmp eq i64 %_18.sroa.0.0.copyload, 0
  br i1 %35, label %bb19, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup1
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_18.sroa.5.0.copyload, i64 noundef %_18.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb19

bb11:                                             ; preds = %bb34
  %36 = icmp eq i64 %_18.sroa.0.0.copyload, 0
  br i1 %36, label %bb13, label %bb2.i.i.i4.i.i26

bb2.i.i.i4.i.i26:                                 ; preds = %bb11
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_18.sroa.5.0.copyload, i64 noundef %_18.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb13

bb13:                                             ; preds = %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit.i, %bb11, %bb2.i.i.i4.i.i26
  %_0.sroa.0.0 = phi ptr [ %33, %bb2.i.i.i4.i.i26 ], [ %33, %bb11 ], [ null, %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit.i ]
  %37 = load atomic ptr, ptr %_5 acquire, align 8, !noalias !177
  br label %bb1.i.i.i29

bb1.i.i.i29:                                      ; preds = %bb3.i.i.i36, %bb13
  %prev.sroa.0.0.i.i.i30 = phi ptr [ %37, %bb13 ], [ %_8.sroa.0.0.i.i.i.i39, %bb3.i.i.i36 ]
  %_4.i.i.i.i31 = ptrtoint ptr %prev.sroa.0.0.i.i.i30 to i64
  %_3.i.i.i.i32 = and i64 %_4.i.i.i.i31, 2
  %38 = icmp eq i64 %_3.i.i.i.i32, 0
  br i1 %38, label %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i41, label %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i33

_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i41: ; preds = %bb1.i.i.i29
  %count.i.i.i.i42 = add i64 %_4.i.i.i.i31, -17
  %_7.not.i.i.i.i43 = icmp eq i64 %count.i.i.i.i42, 0
  %addr.i.i.i.i44 = or i64 %count.i.i.i.i42, 1
  %39 = inttoptr i64 %addr.i.i.i.i44 to ptr
  %_6.sroa.0.0.i.i.i.i45 = select i1 %_7.not.i.i.i.i43, ptr null, ptr %39
  br label %bb3.i.i.i36

_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i33: ; preds = %bb1.i.i.i29
  %_9.i.i.i.i34 = and i64 %_4.i.i.i.i31, 8
  %.not.i.i.i.i35 = icmp eq i64 %_9.i.i.i.i34, 0
  %40 = call nonnull align 2 ptr @llvm.ptrmask.p0.i64(ptr %prev.sroa.0.0.i.i.i30, i64 -10)
  br i1 %.not.i.i.i.i35, label %bb3.i.i40, label %bb3.i.i.i36, !prof !38

bb3.i.i.i36:                                      ; preds = %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i33, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i41
  %_0.sroa.4.0.i7.i.i.i37 = phi ptr [ %_6.sroa.0.0.i.i.i.i45, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.thread.i.i.i41 ], [ %40, %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i33 ]
  %41 = cmpxchg weak ptr %_5, ptr %prev.sroa.0.0.i.i.i30, ptr %_0.sroa.4.0.i7.i.i.i37 release acquire, align 8, !noalias !177
  %_8.sroa.18.0.in.i.i.i.i38 = extractvalue { ptr, i1 } %41, 1
  %_8.sroa.0.0.i.i.i.i39 = extractvalue { ptr, i1 } %41, 0
  br i1 %_8.sroa.18.0.in.i.i.i.i38, label %bb16, label %bb1.i.i.i29

bb3.i.i40:                                        ; preds = %_RNCNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB7_6RwLock11read_unlock0CsC3JuwEIQwb_7console.exit.i.i.i33
; call <std::sys::sync::rwlock::queue::RwLock>::read_unlock_contended
  call void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock21read_unlock_contended(ptr noundef nonnull align 8 %_5, ptr noundef %prev.sroa.0.0.i.i.i30), !noalias !177
  br label %bb16

terminate:                                        ; preds = %cleanup2, %bb19
  %42 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

bb8:                                              ; preds = %.noexc24, %.noexc23
  %_5.sroa.0.0.off0.i.i = phi i1 [ %29, %.noexc24 ], [ false, %.noexc23 ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_29, i64 32
  %43 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !174
  %.not69 = icmp eq i8 %43, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not69, label %bb9, label %bb2.i, !prof !30

bb2.i:                                            ; preds = %bb8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !182
  store ptr %mutex, ptr %e.i, align 8, !noalias !182
  %44 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %44, align 8, !noalias !182
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7954641d60551ca0a31844d45868012f) #30
          to label %unreachable.i unwind label %cleanup.i, !noalias !186

cleanup.i:                                        ; preds = %bb2.i
  %45 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<u8>>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #32
          to label %bb19 unwind label %terminate.i, !noalias !186

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %46 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !186
  unreachable

bb9:                                              ; preds = %bb8
  %_49 = getelementptr inbounds nuw i8, ptr %_29, i64 40
  %_14.0 = load ptr, ptr %s, align 8, !nonnull !7, !align !187, !noundef !7
  %_14.1 = load i64, ptr %2, align 8, !noundef !7
  %47 = getelementptr inbounds nuw i8, ptr %_29, i64 56
  %len.i = load i64, ptr %47, align 8, !alias.scope !188, !noundef !7
  %self2.i = load i64, ptr %_49, align 8, !range !23, !alias.scope !188, !noundef !7
  %_9.i47 = sub i64 %self2.i, %len.i
  %_7.i48 = icmp ugt i64 %_14.1, %_9.i47
  br i1 %_7.i48, label %bb1.i50, label %bb30, !prof !5

bb1.i50:                                          ; preds = %bb9
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %_49, i64 noundef %len.i, i64 noundef %_14.1, i64 noundef 1, i64 noundef 1)
          to label %bb1.i50.bb30_crit_edge unwind label %cleanup2

bb1.i50.bb30_crit_edge:                           ; preds = %bb1.i50
  %_61.pre = load i64, ptr %47, align 8
  br label %bb30

cleanup2:                                         ; preds = %bb1.i60, %bb1.i53, %bb1.i50
  %48 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr nonnull %mutex, i8 %_0.sroa.3.0.i.i) #32
          to label %bb19 unwind label %terminate

bb30:                                             ; preds = %bb1.i50.bb30_crit_edge, %bb9
  %_61 = phi i64 [ %_61.pre, %bb1.i50.bb30_crit_edge ], [ %len.i, %bb9 ]
  %_65 = icmp sgt i64 %_61, -1
  tail call void @llvm.assume(i1 %_65)
  %49 = getelementptr inbounds nuw i8, ptr %_29, i64 48
  %_66 = load ptr, ptr %49, align 8, !nonnull !7, !noundef !7
  %_63 = getelementptr inbounds nuw i8, ptr %_66, i64 %_61
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_63, ptr nonnull align 1 %_14.0, i64 %_14.1, i1 false)
  %50 = load i64, ptr %47, align 8, !noundef !7
  %51 = add i64 %50, %_14.1
  store i64 %51, ptr %47, align 8
  %self1.i = load i64, ptr %_49, align 8, !range !23, !alias.scope !191, !noundef !7
  %_4.i = icmp eq i64 %51, %self1.i
  br i1 %_4.i, label %bb1.i53, label %bb31

bb1.i53:                                          ; preds = %bb30
; invoke <alloc::raw_vec::RawVec<u8>>::grow_one
  invoke void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_(ptr noalias noundef nonnull align 8 dereferenceable(24) %_49)
          to label %bb31 unwind label %cleanup2

bb31:                                             ; preds = %bb30, %bb1.i53
  %_14.i = load ptr, ptr %49, align 8, !alias.scope !191, !nonnull !7, !noundef !7
  %end.i = getelementptr inbounds nuw i8, ptr %_14.i, i64 %51
  store i8 10, ptr %end.i, align 1
  %52 = add i64 %51, 1
  store i64 %52, ptr %47, align 8, !alias.scope !191
  %53 = getelementptr inbounds nuw i8, ptr %_29, i64 152
  %_79 = load ptr, ptr %53, align 8, !nonnull !7, !noundef !7
  %_78 = load i64, ptr %12, align 8, !noundef !7
  %self2.i56 = load i64, ptr %_49, align 8, !range !23, !alias.scope !194, !noundef !7
  %_9.i57 = sub i64 %self2.i56, %52
  %_7.i58 = icmp ugt i64 %_78, %_9.i57
  br i1 %_7.i58, label %bb1.i60, label %bb33, !prof !5

bb1.i60:                                          ; preds = %bb31
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %_49, i64 noundef %52, i64 noundef %_78, i64 noundef 1, i64 noundef 1)
          to label %bb1.i60.bb33_crit_edge unwind label %cleanup2

bb1.i60.bb33_crit_edge:                           ; preds = %bb1.i60
  %_90.pre = load i64, ptr %47, align 8
  br label %bb33

bb33:                                             ; preds = %bb1.i60.bb33_crit_edge, %bb31
  %_90 = phi i64 [ %_90.pre, %bb1.i60.bb33_crit_edge ], [ %52, %bb31 ]
  %_94 = icmp sgt i64 %_90, -1
  tail call void @llvm.assume(i1 %_94)
  %_95 = load ptr, ptr %49, align 8, !nonnull !7, !noundef !7
  %_92 = getelementptr inbounds nuw i8, ptr %_95, i64 %_90
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_92, ptr nonnull align 1 %_79, i64 %_78, i1 false)
  %54 = load i64, ptr %47, align 8, !noundef !7
  %55 = add i64 %54, %_78
  store i64 %55, ptr %47, align 8
  br i1 %_5.sroa.0.0.off0.i.i, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit.i, label %bb1.i.i.i63

bb1.i.i.i63:                                      ; preds = %bb33
  %56 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %56, 9223372036854775807
  %57 = icmp eq i64 %_7.i.i.i, 0
  br i1 %57, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit.i, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i63
; invoke std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i64 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
          to label %_6.i.i.i.noexc unwind label %cleanup

_6.i.i.i.noexc:                                   ; preds = %bb6.i.i.i
  br i1 %_6.i.i.i64, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit.i, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %_6.i.i.i.noexc
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit.i

_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console.exit.i: ; preds = %bb2.i.i.i, %_6.i.i.i.noexc, %bb1.i.i.i63, %bb33
  %58 = load atomic ptr, ptr %mutex monotonic, align 8
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %58)
          to label %bb13 unwind label %cleanup

bb16:                                             ; preds = %bb3.i.i.i, %bb3.i.i.i36, %bb3.i.i40, %bb3.i.i
  %_0.sroa.0.1 = phi ptr [ %14, %bb3.i.i ], [ %_0.sroa.0.0, %bb3.i.i40 ], [ %_0.sroa.0.0, %bb3.i.i.i36 ], [ %14, %bb3.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %prompt)
  ret ptr %_0.sroa.0.1
}

; <console::term::Term>::read_key_raw
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term12read_key_raw(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 9
  %1 = load i8, ptr %0, align 1, !range !29, !noundef !7
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb2, label %bb1

bb1:                                              ; preds = %start
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb3

bb2:                                              ; preds = %start
; call console::unix_term::read_single_key
  tail call fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term15read_single_key(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_0, i1 noundef zeroext true)
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  ret void
}

; <console::term::Term>::write_through
; Function Attrs: uwtable
define internal fastcc noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term13write_through(ptr readonly captures(address_is_null) %self.0.val, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %bytes.0, i64 noundef range(i64 0, -9223372036854775808) %bytes.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [24 x i8], align 8
  %_22.i = alloca [8 x i8], align 8
  %_17.i = alloca [8 x i8], align 8
  %_12.i = alloca [8 x i8], align 8
  %_7.i = alloca [8 x i8], align 8
  %0 = icmp ne ptr %self.0.val, null
  tail call void @llvm.assume(i1 %0)
  %1 = getelementptr inbounds nuw i8, ptr %self.0.val, i64 127
  %2 = load i8, ptr %1, align 1, !range !58, !noalias !197, !noundef !7
  %3 = add nsw i8 %2, -2
  %.inv.i = icmp samesign ult i8 %2, 2
  %narrow.i = select i1 %.inv.i, i8 2, i8 %3
  switch i8 %narrow.i, label %bb1.i [
    i8 0, label %bb4.i
    i8 1, label %bb3.i
    i8 2, label %bb2.i
  ]

bb1.i:                                            ; preds = %start
  unreachable

bb4.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i), !noalias !197
; call std::io::stdio::stdout
  %4 = tail call noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6stdout(), !noalias !197
  store ptr %4, ptr %_7.i, align 8, !noalias !197
; call <std::io::stdio::Stdout as std::io::Write>::write_all
  %5 = call noundef ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write9write_all(ptr noalias noundef nonnull align 8 dereferenceable(8) %_7.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %bytes.0, i64 noundef range(i64 0, -9223372036854775808) %bytes.1)
  %.not9.i = icmp eq ptr %5, null
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i), !noalias !197
  br i1 %.not9.i, label %bb23.i, label %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common.exit

bb3.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_17.i), !noalias !197
  store ptr @_RNvNvNtNtCs5sEH5CPMdak_3std2io5stdio6stderr8INSTANCE, ptr %_17.i, align 8, !noalias !197
; call <std::io::stdio::Stderr as std::io::Write>::write_all
  %6 = call noundef ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write9write_all(ptr noalias noundef nonnull align 8 dereferenceable(8) %_17.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %bytes.0, i64 noundef range(i64 0, -9223372036854775808) %bytes.1)
  %.not7.i = icmp eq ptr %6, null
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_17.i), !noalias !197
  br i1 %.not7.i, label %bb27.i, label %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common.exit

bb2.i:                                            ; preds = %start
  %7 = getelementptr inbounds nuw i8, ptr %self.0.val, i64 80
  %_39.0.i = load ptr, ptr %7, align 8, !noalias !197, !nonnull !7, !noundef !7
  %8 = getelementptr inbounds nuw i8, ptr %self.0.val, i64 88
  %_39.1.i = load ptr, ptr %8, align 8, !noalias !197, !nonnull !7, !align !22, !noundef !7
  %9 = getelementptr inbounds nuw i8, ptr %_39.1.i, i64 16
  %10 = load i64, ptr %9, align 8, !range !24, !invariant.load !7, !noalias !197
  %11 = tail call i64 @llvm.umax.i64(i64 %10, i64 8)
  %12 = add i64 %11, -1
  %13 = and i64 %12, -16
  %14 = getelementptr i8, ptr %_39.0.i, i64 %13
  %_27.0.i = getelementptr i8, ptr %14, i64 16
  %15 = load atomic ptr, ptr %_27.0.i acquire, align 8, !noalias !200
  %16 = icmp eq ptr %15, null
  br i1 %16, label %bb7.i.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i.i, !prof !5

bb7.i.i.i:                                        ; preds = %bb2.i
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %17 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 %_27.0.i), !noalias !200
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i.i: ; preds = %bb7.i.i.i, %bb2.i
  %_0.sroa.0.0.i.i.i = phi ptr [ %17, %bb7.i.i.i ], [ %15, %bb2.i ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i.i), !noalias !200
  %18 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !200
  %_6.i.i.i = and i64 %18, 9223372036854775807
  %19 = icmp eq i64 %_6.i.i.i, 0
  br i1 %19, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit.i, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %20 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33, !noalias !200
  %21 = xor i1 %20, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit.i

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit.i: ; preds = %bb6.i.i.i, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i.i
  %_5.sroa.0.0.off0.i.i.i = phi i1 [ %21, %bb6.i.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i.i ]
  %_7.i.i = getelementptr i8, ptr %14, i64 24
  %22 = load atomic i8, ptr %_7.i.i monotonic, align 1, !noalias !200
  %.not1.i = icmp eq i8 %22, 0
  %_0.sroa.3.0.i.i.i = zext i1 %_5.sroa.0.0.off0.i.i.i to i8
  br i1 %.not1.i, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_.exit.i, label %bb2.i.i, !prof !30

bb2.i.i:                                          ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e.i.i), !noalias !204
  store ptr %_27.0.i, ptr %e.i.i, align 8, !noalias !209
  %_26.sroa.7.8.e.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %e.i.i, i64 8
  store ptr %_39.1.i, ptr %_26.sroa.7.8.e.i.sroa_idx.i, align 8, !noalias !209
  %_26.sroa.9.8.e.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %e.i.i, i64 16
  store i8 %_0.sroa.3.0.i.i.i, ptr %_26.sroa.9.8.e.i.sroa_idx.i, align 8, !noalias !209
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_be34cf22961c134b4f22f36d9b4f226c) #30
          to label %unreachable.i.i unwind label %cleanup.i.i, !noalias !210

cleanup.i.i:                                      ; preds = %bb2.i.i
  %23 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEEB1Z_(ptr noalias noundef nonnull align 8 dereferenceable(24) %e.i.i) #32
          to label %common.resume.i unwind label %terminate.i.i, !noalias !210

unreachable.i.i:                                  ; preds = %bb2.i.i
  unreachable

terminate.i.i:                                    ; preds = %cleanup.i.i
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !210
  unreachable

common.resume.i:                                  ; preds = %cleanup.i, %cleanup.i.i
  %common.resume.op.i = phi { ptr, i32 } [ %23, %cleanup.i.i ], [ %34, %cleanup.i ]
  resume { ptr, i32 } %common.resume.op.i

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_.exit.i: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit.i
  %25 = add i64 %10, -1
  %26 = and i64 %25, -9
  %27 = getelementptr i8, ptr %_27.0.i, i64 %26
  %_58.0.i = getelementptr i8, ptr %27, i64 9
  %28 = getelementptr inbounds nuw i8, ptr %_39.1.i, i64 56
  %29 = load ptr, ptr %28, align 8, !invariant.load !7, !noalias !197, !nonnull !7
  %30 = invoke noundef ptr %29(ptr noundef align 1 %_58.0.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %bytes.0, i64 noundef range(i64 0, -9223372036854775808) %bytes.1)
          to label %bb13.i unwind label %cleanup.i

bb23.i:                                           ; preds = %bb4.i
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_12.i), !noalias !197
; call std::io::stdio::stdout
  %31 = call noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6stdout()
  store ptr %31, ptr %_12.i, align 8, !noalias !197
; call <std::io::stdio::Stdout as std::io::Write>::flush
  %32 = call noundef ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write5flush(ptr noalias noundef nonnull align 8 dereferenceable(8) %_12.i)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_12.i), !noalias !197
  br label %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common.exit

bb27.i:                                           ; preds = %bb3.i
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_22.i), !noalias !197
  store ptr @_RNvNvNtNtCs5sEH5CPMdak_3std2io5stdio6stderr8INSTANCE, ptr %_22.i, align 8, !noalias !197
; call <std::io::stdio::Stderr as std::io::Write>::flush
  %33 = call noundef ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write5flush(ptr noalias noundef nonnull align 8 dereferenceable(8) %_22.i)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_22.i), !noalias !197
  br label %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common.exit

cleanup.i:                                        ; preds = %bb31.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_.exit.i
  %34 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr nonnull %_27.0.i, i8 %_0.sroa.3.0.i.i.i) #32
          to label %common.resume.i unwind label %terminate.i

bb13.i:                                           ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_.exit.i
  %.not.i = icmp eq ptr %30, null
  br i1 %.not.i, label %bb31.i, label %bb17.i

bb31.i:                                           ; preds = %bb13.i
  %35 = getelementptr inbounds nuw i8, ptr %_39.1.i, i64 48
  %36 = load ptr, ptr %35, align 8, !invariant.load !7, !nonnull !7
  %37 = invoke noundef ptr %36(ptr noundef align 1 %_58.0.i)
          to label %bb14.i unwind label %cleanup.i

bb14.i:                                           ; preds = %bb31.i
  %.not6.i = icmp eq ptr %37, null
  br i1 %.not6.i, label %bb33.i, label %bb17.i

bb33.i:                                           ; preds = %bb14.i
  br i1 %_5.sroa.0.0.off0.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit.i, label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb33.i
  %38 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !197
  %_7.i.i.i.i = and i64 %38, 9223372036854775807
  %39 = icmp eq i64 %_7.i.i.i.i, 0
  br i1 %39, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit.i, label %bb6.i.i.i.i, !prof !30

bb6.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit.i, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb6.i.i.i.i
  store atomic i8 1, ptr %_7.i.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit.i: ; preds = %bb2.i.i.i.i, %bb6.i.i.i.i, %bb1.i.i.i.i, %bb33.i
  %40 = load atomic ptr, ptr %_27.0.i monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %40)
  br label %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common.exit

bb17.i:                                           ; preds = %bb14.i, %bb13.i
  %_0.sroa.0.1.i = phi ptr [ %30, %bb13.i ], [ %37, %bb14.i ]
  br i1 %_5.sroa.0.0.off0.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit24.i, label %bb1.i.i.i19.i

bb1.i.i.i19.i:                                    ; preds = %bb17.i
  %41 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !197
  %_7.i.i.i20.i = and i64 %41, 9223372036854775807
  %42 = icmp eq i64 %_7.i.i.i20.i, 0
  br i1 %42, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit24.i, label %bb6.i.i.i21.i, !prof !30

bb6.i.i.i21.i:                                    ; preds = %bb1.i.i.i19.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i22.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i22.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit24.i, label %bb2.i.i.i23.i

bb2.i.i.i23.i:                                    ; preds = %bb6.i.i.i21.i
  store atomic i8 1, ptr %_7.i.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit24.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit24.i: ; preds = %bb2.i.i.i23.i, %bb6.i.i.i21.i, %bb1.i.i.i19.i, %bb17.i
  %43 = load atomic ptr, ptr %_27.0.i monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %43)
  br label %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common.exit

terminate.i:                                      ; preds = %cleanup.i
  %44 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common.exit: ; preds = %bb4.i, %bb3.i, %bb23.i, %bb27.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit24.i
  %_0.sroa.0.0.i = phi ptr [ %_0.sroa.0.1.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit24.i ], [ null, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit.i ], [ %33, %bb27.i ], [ %32, %bb23.i ], [ %5, %bb4.i ], [ %6, %bb3.i ]
  ret ptr %_0.sroa.0.0.i
}

; <console::term::Term>::buffered_stderr
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term15buffered_stderr(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_0) unnamed_addr #1 {
start:
  %_1 = alloca [168 x i8], align 16
  call void @llvm.lifetime.start.p0(i64 168, ptr nonnull %_1)
  %_2.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 111
  store i8 3, ptr %_2.sroa.3.0..sroa_idx, align 1
  store <2 x i64> <i64 1, i64 0>, ptr %_1, align 16
  %_3.sroa.4.sroa.4.0._3.sroa.4.0._1.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 16
  store i8 0, ptr %_3.sroa.4.sroa.4.0._3.sroa.4.0._1.sroa_idx.sroa_idx, align 16
  %_3.sroa.4.sroa.5.sroa.4.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 24
  store i64 0, ptr %_3.sroa.4.sroa.5.sroa.4.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx, align 8
  %_3.sroa.4.sroa.5.sroa.5.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 32
  store ptr inttoptr (i64 1 to ptr), ptr %_3.sroa.4.sroa.5.sroa.5.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx, align 16
  %_3.sroa.4.sroa.5.sroa.6.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 40
  store i64 0, ptr %_3.sroa.4.sroa.5.sroa.6.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 112
  store i64 0, ptr %0, align 16
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 120
  store i8 0, ptr %_6.sroa.4.0..sroa_idx, align 8
  %_6.sroa.5.sroa.3.0._6.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 128
  store i64 0, ptr %_6.sroa.5.sroa.3.0._6.sroa.5.0..sroa_idx.sroa_idx, align 16
  %_6.sroa.5.sroa.4.0._6.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 136
  store ptr inttoptr (i64 1 to ptr), ptr %_6.sroa.5.sroa.4.0._6.sroa.5.0..sroa_idx.sroa_idx, align 8
  %_6.sroa.5.sroa.5.0._6.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 144
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 16 dereferenceable(17) %_6.sroa.5.sroa.5.0._6.sroa.5.0..sroa_idx.sroa_idx, i8 0, i64 17, i1 false)
; call <console::term::Term>::with_inner
  call void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term10with_inner(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_0, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(168) %_1)
  call void @llvm.lifetime.end.p0(i64 168, ptr nonnull %_1)
  ret void
}

; <console::term::Term>::buffered_stdout
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term15buffered_stdout(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_0) unnamed_addr #1 {
start:
  %_1 = alloca [168 x i8], align 16
  call void @llvm.lifetime.start.p0(i64 168, ptr nonnull %_1)
  %_2.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 111
  store i8 2, ptr %_2.sroa.3.0..sroa_idx, align 1
  store <2 x i64> <i64 1, i64 0>, ptr %_1, align 16
  %_3.sroa.4.sroa.4.0._3.sroa.4.0._1.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 16
  store i8 0, ptr %_3.sroa.4.sroa.4.0._3.sroa.4.0._1.sroa_idx.sroa_idx, align 16
  %_3.sroa.4.sroa.5.sroa.4.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 24
  store i64 0, ptr %_3.sroa.4.sroa.5.sroa.4.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx, align 8
  %_3.sroa.4.sroa.5.sroa.5.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 32
  store ptr inttoptr (i64 1 to ptr), ptr %_3.sroa.4.sroa.5.sroa.5.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx, align 16
  %_3.sroa.4.sroa.5.sroa.6.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 40
  store i64 0, ptr %_3.sroa.4.sroa.5.sroa.6.0._3.sroa.4.sroa.5.0._3.sroa.4.0._1.sroa_idx.sroa_idx.sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 112
  store i64 0, ptr %0, align 16
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 120
  store i8 0, ptr %_6.sroa.4.0..sroa_idx, align 8
  %_6.sroa.5.sroa.3.0._6.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 128
  store i64 0, ptr %_6.sroa.5.sroa.3.0._6.sroa.5.0..sroa_idx.sroa_idx, align 16
  %_6.sroa.5.sroa.4.0._6.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 136
  store ptr inttoptr (i64 1 to ptr), ptr %_6.sroa.5.sroa.4.0._6.sroa.5.0..sroa_idx.sroa_idx, align 8
  %_6.sroa.5.sroa.5.0._6.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_1, i64 144
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 16 dereferenceable(17) %_6.sroa.5.sroa.5.0._6.sroa.5.0..sroa_idx.sroa_idx, i8 0, i64 17, i1 false)
; call <console::term::Term>::with_inner
  call void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term10with_inner(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_0, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(168) %_1)
  call void @llvm.lifetime.end.p0(i64 168, ptr nonnull %_1)
  ret void
}

; <console::term::Term>::clear_last_lines
; Function Attrs: uwtable
define noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term16clear_last_lines(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, i64 noundef %n) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args.i26 = alloca [16 x i8], align 8
  %_6.i27 = alloca [24 x i8], align 8
  %n.i28 = alloca [8 x i8], align 8
  %args.i8 = alloca [16 x i8], align 8
  %_6.i9 = alloca [24 x i8], align 8
  %n.i10 = alloca [8 x i8], align 8
  %args.i = alloca [16 x i8], align 8
  %_6.i = alloca [24 x i8], align 8
  %n.i = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %n.i)
  store i64 %n, ptr %n.i, align 8, !noalias !211
  %_3.not.i = icmp eq i64 %n, 0
  br i1 %_3.not.i, label %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit.thread, label %bb1.i

_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit.thread: ; preds = %start
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %n.i)
  br label %bb2.preheader

bb1.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i), !noalias !211
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i), !noalias !211
  store ptr %n.i, ptr %args.i, align 8, !noalias !211
  %_10.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx.i, align 8, !noalias !211
; call alloc::fmt::format::format_inner
  call void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6.i, ptr noundef nonnull @alloc_26f3afb675a2033575e507bda4f7709d, ptr noundef nonnull %args.i), !noalias !211
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i), !noalias !211
  %_5.sroa.0.0.copyload.i = load i64, ptr %_6.i, align 8, !noalias !211
  %_5.sroa.5.0._6.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_6.i, i64 8
  %_5.sroa.5.0.copyload.i = load ptr, ptr %_5.sroa.5.0._6.sroa_idx.i, align 8, !noalias !211, !nonnull !7, !noundef !7
  %_5.sroa.8.0._6.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_6.i, i64 16
  %_5.sroa.8.0.copyload.i = load i64, ptr %_5.sroa.8.0._6.sroa_idx.i, align 8, !noalias !211
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i), !noalias !211
; invoke <console::term::Term>::write_str
  %0 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload.i, i64 noundef %_5.sroa.8.0.copyload.i)
          to label %bb2.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb1.i
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = icmp eq i64 %_5.sroa.0.0.copyload.i, 0
  br i1 %2, label %common.resume, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload.i, i64 noundef %_5.sroa.0.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !211
  br label %common.resume

bb2.i:                                            ; preds = %bb1.i
  %3 = icmp eq i64 %_5.sroa.0.0.copyload.i, 0
  br i1 %3, label %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit, label %bb2.i.i.i4.i.i5.i

bb2.i.i.i4.i.i5.i:                                ; preds = %bb2.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload.i, i64 noundef %_5.sroa.0.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !211
  br label %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit

common.resume:                                    ; preds = %cleanup.i36, %bb2.i.i.i4.i.i.i37, %cleanup.i19, %bb2.i.i.i4.i.i.i20, %cleanup.i, %bb2.i.i.i4.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %1, %bb2.i.i.i4.i.i.i ], [ %1, %cleanup.i ], [ %5, %bb2.i.i.i4.i.i.i20 ], [ %5, %cleanup.i19 ], [ %10, %bb2.i.i.i4.i.i.i37 ], [ %10, %cleanup.i36 ]
  resume { ptr, i32 } %common.resume.op

_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit: ; preds = %bb2.i, %bb2.i.i.i4.i.i5.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %n.i)
  %.not = icmp eq ptr %0, null
  br i1 %.not, label %bb2.preheader, label %bb4

bb2.preheader:                                    ; preds = %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit.thread, %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit
  %_10.sroa.4.0..sroa_idx.i30 = getelementptr inbounds nuw i8, ptr %args.i26, i64 8
  %_5.sroa.5.0._6.sroa_idx.i32 = getelementptr inbounds nuw i8, ptr %_6.i27, i64 8
  %_5.sroa.8.0._6.sroa_idx.i34 = getelementptr inbounds nuw i8, ptr %_6.i27, i64 16
  br label %bb2

bb2:                                              ; preds = %bb2.preheader, %_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down.exit
  %iter.sroa.0.0 = phi i64 [ %_20, %_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down.exit ], [ 0, %bb2.preheader ]
  %exitcond.not = icmp eq i64 %iter.sroa.0.0, %n
  br i1 %exitcond.not, label %bb9, label %bb8

bb9:                                              ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %n.i10)
  store i64 %n, ptr %n.i10, align 8, !noalias !214
  br i1 %_3.not.i, label %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit25, label %bb1.i12

bb1.i12:                                          ; preds = %bb9
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i9), !noalias !214
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i8), !noalias !214
  store ptr %n.i10, ptr %args.i8, align 8, !noalias !214
  %_10.sroa.4.0..sroa_idx.i13 = getelementptr inbounds nuw i8, ptr %args.i8, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx.i13, align 8, !noalias !214
; call alloc::fmt::format::format_inner
  call void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6.i9, ptr noundef nonnull @alloc_26f3afb675a2033575e507bda4f7709d, ptr noundef nonnull %args.i8), !noalias !214
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i8), !noalias !214
  %_5.sroa.0.0.copyload.i14 = load i64, ptr %_6.i9, align 8, !noalias !214
  %_5.sroa.5.0._6.sroa_idx.i15 = getelementptr inbounds nuw i8, ptr %_6.i9, i64 8
  %_5.sroa.5.0.copyload.i16 = load ptr, ptr %_5.sroa.5.0._6.sroa_idx.i15, align 8, !noalias !214, !nonnull !7, !noundef !7
  %_5.sroa.8.0._6.sroa_idx.i17 = getelementptr inbounds nuw i8, ptr %_6.i9, i64 16
  %_5.sroa.8.0.copyload.i18 = load i64, ptr %_5.sroa.8.0._6.sroa_idx.i17, align 8, !noalias !214
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i9), !noalias !214
; invoke <console::term::Term>::write_str
  %4 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload.i16, i64 noundef %_5.sroa.8.0.copyload.i18)
          to label %bb2.i22 unwind label %cleanup.i19

cleanup.i19:                                      ; preds = %bb1.i12
  %5 = landingpad { ptr, i32 }
          cleanup
  %6 = icmp eq i64 %_5.sroa.0.0.copyload.i14, 0
  br i1 %6, label %common.resume, label %bb2.i.i.i4.i.i.i20

bb2.i.i.i4.i.i.i20:                               ; preds = %cleanup.i19
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload.i16, i64 noundef %_5.sroa.0.0.copyload.i14, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !214
  br label %common.resume

bb2.i22:                                          ; preds = %bb1.i12
  %7 = icmp eq i64 %_5.sroa.0.0.copyload.i14, 0
  br i1 %7, label %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit25, label %bb2.i.i.i4.i.i5.i23

bb2.i.i.i4.i.i5.i23:                              ; preds = %bb2.i22
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload.i16, i64 noundef %_5.sroa.0.0.copyload.i14, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !214
  br label %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit25

_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit25: ; preds = %bb9, %bb2.i22, %bb2.i.i.i4.i.i5.i23
  %_0.sroa.0.0.i24 = phi ptr [ null, %bb9 ], [ %4, %bb2.i22 ], [ %4, %bb2.i.i.i4.i.i5.i23 ]
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %n.i10)
  br label %bb4

bb8:                                              ; preds = %bb2
  %_20 = add i64 %iter.sroa.0.0, 1
; call <console::term::Term>::write_str
  %8 = call noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_34a907ce632321878f08de7a619b60ed, i64 noundef 5)
  %.not6 = icmp eq ptr %8, null
  br i1 %.not6, label %bb13, label %bb4

bb4:                                              ; preds = %_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down.exit, %bb8, %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit25, %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit
  %_0.sroa.0.0 = phi ptr [ %0, %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit ], [ %_0.sroa.0.0.i24, %_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up.exit25 ], [ %9, %_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down.exit ], [ %8, %bb8 ]
  ret ptr %_0.sroa.0.0

bb13:                                             ; preds = %bb8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %n.i28)
  store i64 1, ptr %n.i28, align 8, !noalias !217
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i27), !noalias !217
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i26), !noalias !217
  store ptr %n.i28, ptr %args.i26, align 8, !noalias !217
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx.i30, align 8, !noalias !217
; call alloc::fmt::format::format_inner
  call void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6.i27, ptr noundef nonnull @alloc_8041b981a0c9f2271efbceca9d03160b, ptr noundef nonnull %args.i26), !noalias !217
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i26), !noalias !217
  %_5.sroa.0.0.copyload.i31 = load i64, ptr %_6.i27, align 8, !noalias !217
  %_5.sroa.5.0.copyload.i33 = load ptr, ptr %_5.sroa.5.0._6.sroa_idx.i32, align 8, !noalias !217, !nonnull !7, !noundef !7
  %_5.sroa.8.0.copyload.i35 = load i64, ptr %_5.sroa.8.0._6.sroa_idx.i34, align 8, !noalias !217
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i27), !noalias !217
; invoke <console::term::Term>::write_str
  %9 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload.i33, i64 noundef %_5.sroa.8.0.copyload.i35)
          to label %bb2.i39 unwind label %cleanup.i36

cleanup.i36:                                      ; preds = %bb13
  %10 = landingpad { ptr, i32 }
          cleanup
  %11 = icmp eq i64 %_5.sroa.0.0.copyload.i31, 0
  br i1 %11, label %common.resume, label %bb2.i.i.i4.i.i.i37

bb2.i.i.i4.i.i.i37:                               ; preds = %cleanup.i36
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload.i33, i64 noundef %_5.sroa.0.0.copyload.i31, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !217
  br label %common.resume

bb2.i39:                                          ; preds = %bb13
  %12 = icmp eq i64 %_5.sroa.0.0.copyload.i31, 0
  br i1 %12, label %_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down.exit, label %bb2.i.i.i4.i.i5.i40

bb2.i.i.i4.i.i5.i40:                              ; preds = %bb2.i39
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload.i33, i64 noundef %_5.sroa.0.0.copyload.i31, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !217
  br label %_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down.exit

_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down.exit: ; preds = %bb2.i39, %bb2.i.i.i4.i.i5.i40
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %n.i28)
  %.not7 = icmp eq ptr %9, null
  br i1 %.not7, label %bb2, label %bb4
}

; <console::term::Term>::read_secure_line
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term16read_secure_line(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %buf.i.i.i.i.i.i.i.i = alloca [32 x i8], align 8
  %_12.i.i.i.i = alloca [24 x i8], align 8
  %g.i.i.i.i = alloca [16 x i8], align 8
  %_6.i.i.i = alloca [12 x i8], align 4
  %_4.i.i.i = alloca [16 x i8], align 8
  %_1.sroa.7.sroa.0.i.i = alloca [24 x i8], align 8
  %_58.i = alloca [24 x i8], align 8
  %_57.i = alloca [24 x i8], align 8
  %rv.i = alloca [24 x i8], align 8
  %original.i = alloca [72 x i8], align 8
  %termios1.i = alloca [72 x i8], align 8
  %termios.i = alloca [72 x i8], align 8
  %input.i = alloca [48 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 9
  %1 = load i8, ptr %0, align 1, !range !29, !noundef !7
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb2, label %bb14

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !220)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %input.i), !noalias !220
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_1.sroa.7.sroa.0.i.i)
; call std::io::stdio::stdin
  %stdin.i.i.i = tail call noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio5stdin(), !noalias !223
  %_10.i.i.i = tail call noundef i32 @isatty(i32 noundef 0) #29, !noalias !223
  %2 = icmp eq i32 %_10.i.i.i, 0
  br i1 %2, label %bb3.i.i.i, label %bb21.i

bb3.i.i.i:                                        ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4.i.i.i), !noalias !223
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_6.i.i.i), !noalias !223
  store i32 0, ptr %_6.i.i.i, align 4, !noalias !223
  %_11.sroa.4.0._6.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i.i, i64 4
  store i16 438, ptr %_11.sroa.4.0._6.sroa_idx.i.i.i, align 4, !noalias !223
  %_11.sroa.5.0._6.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i.i, i64 6
  %_11.sroa.6.0._6.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i.i, i64 7
  %3 = getelementptr inbounds nuw i8, ptr %_6.i.i.i, i64 8
  store i32 0, ptr %3, align 4, !noalias !223
  store i8 1, ptr %_11.sroa.5.0._6.sroa_idx.i.i.i, align 2, !noalias !223
  store i8 1, ptr %_11.sroa.6.0._6.sroa_idx.i.i.i, align 1, !noalias !223
; call <std::fs::OpenOptions>::_open
  call void @_RNvMsl_NtCs5sEH5CPMdak_3std2fsNtB5_11OpenOptions5__open(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_4.i.i.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %_6.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_890f94c692f51df5fc72c5e0dfe1823b, i64 noundef 8), !noalias !223
  %4 = load i32, ptr %_4.i.i.i, align 8, !range !228, !noalias !223, !noundef !7
  %5 = trunc nuw i32 %4 to i1
  br i1 %5, label %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.thread, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb3.i.i.i
  %6 = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 4
  %_13.i.i.i = load i32, ptr %6, align 4, !range !28, !noalias !223, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4.i.i.i), !noalias !223
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_6.i.i.i), !noalias !223
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !229
; call __rustc::__rust_alloc
  %7 = call noundef dereferenceable_or_null(8192) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 8192, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !229
  %8 = icmp eq ptr %7, null
  br i1 %8, label %bb3.i.i.i.i, label %_RNvMNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB2_9BufReaderNtNtB8_2fs4FileE13with_capacityCsC3JuwEIQwb_7console.exit.i.i

bb3.i.i.i.i:                                      ; preds = %bb3.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 8192) #30
          to label %.noexc.i.i.i unwind label %bb2.i8.i.i, !noalias !234

.noexc.i.i.i:                                     ; preds = %bb3.i.i.i.i
  unreachable

common.resume:                                    ; preds = %cleanup, %bb2.i.i.i4.i.i, %bb2.i8.i.i, %bb14.i
  %common.resume.op = phi { ptr, i32 } [ %9, %bb2.i8.i.i ], [ %.pn12.i, %bb14.i ], [ %102, %bb2.i.i.i4.i.i ], [ %102, %cleanup ]
  resume { ptr, i32 } %common.resume.op

bb2.i8.i.i:                                       ; preds = %bb3.i.i.i.i
  %9 = landingpad { ptr, i32 }
          cleanup
  %_5.i.i.i.i.i.i.i.i = call noundef i32 @close(i32 noundef range(i32 0, -1) %_13.i.i.i) #29, !noalias !234
  br label %common.resume

_RNvMNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB2_9BufReaderNtNtB8_2fs4FileE13with_capacityCsC3JuwEIQwb_7console.exit.i.i: ; preds = %bb3.i.i
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_1.sroa.7.sroa.0.i.i, i8 0, i64 24, i1 false), !noalias !235
  br label %bb21.i

_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.thread: ; preds = %bb3.i.i.i
  %10 = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 8
  %_14.i.i.i = load ptr, ptr %10, align 8, !noalias !223, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4.i.i.i), !noalias !223
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_6.i.i.i), !noalias !223
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_1.sroa.7.sroa.0.i.i)
  %11 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_14.i.i.i, ptr %11, align 8, !alias.scope !220
  store i64 -9223372036854775808, ptr %_0, align 8, !alias.scope !220
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %input.i), !noalias !220
  br label %bb9

bb21.i:                                           ; preds = %_RNvMNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB2_9BufReaderNtNtB8_2fs4FileE13with_capacityCsC3JuwEIQwb_7console.exit.i.i, %bb2
  %_3.sroa.7.0.ph18.i.i = phi i32 [ %_13.i.i.i, %_RNvMNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB2_9BufReaderNtNtB8_2fs4FileE13with_capacityCsC3JuwEIQwb_7console.exit.i.i ], [ undef, %bb2 ]
  %_1.sroa.6.0.i.i = phi ptr [ inttoptr (i64 8192 to ptr), %_RNvMNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB2_9BufReaderNtNtB8_2fs4FileE13with_capacityCsC3JuwEIQwb_7console.exit.i.i ], [ %stdin.i.i.i, %bb2 ]
  %_1.sroa.0.0.i.i = phi ptr [ %7, %_RNvMNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB2_9BufReaderNtNtB8_2fs4FileE13with_capacityCsC3JuwEIQwb_7console.exit.i.i ], [ null, %bb2 ]
  %val.sroa.4.sroa.4.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %input.i, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %val.sroa.4.sroa.4.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(24) %_1.sroa.7.sroa.0.i.i, i64 24, i1 false), !noalias !220
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_1.sroa.7.sroa.0.i.i)
  store ptr %_1.sroa.0.0.i.i, ptr %input.i, align 8, !noalias !220
  %val.sroa.4.0.input.sroa_idx.i = getelementptr inbounds nuw i8, ptr %input.i, i64 8
  store ptr %_1.sroa.6.0.i.i, ptr %val.sroa.4.0.input.sroa_idx.i, align 8, !noalias !220
  %val.sroa.4.sroa.5.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %input.i, i64 40
  store i32 %_3.sroa.7.0.ph18.i.i, ptr %val.sroa.4.sroa.5.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !noalias !220
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %termios.i), !noalias !220
  %.not.i.i.i = icmp eq ptr %_1.sroa.0.0.i.i, null
  %_2.sroa.0.0.i.i.i = select i1 %.not.i.i.i, i32 0, i32 %_3.sroa.7.0.ph18.i.i
  %_0.i.i.i = call noundef i32 @tcgetattr(i32 noundef %_2.sroa.0.0.i.i.i, ptr noundef nonnull align 8 dereferenceable(72) %termios.i) #29, !noalias !220
  %12 = icmp eq i32 %_0.i.i.i, 0
  br i1 %12, label %bb23.i, label %bb22.i

bb14.i:                                           ; preds = %bb2.i.i.i4.i.i56.i, %bb18.i, %cleanup3.i
  %.pn12.i = phi { ptr, i32 } [ %86, %cleanup3.i ], [ %.pn.ph.i, %bb18.i ], [ %.pn.ph.i, %bb2.i.i.i4.i.i56.i ]
; call core::ptr::drop_in_place::<console::unix_term::Input<std::io::buffered::bufreader::BufReader<std::fs::File>>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsC3JuwEIQwb_7console9unix_term5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtB1v_2fs4FileEEEBL_(ptr noalias noundef align 8 dereferenceable(48) %input.i) #32, !noalias !220
  br label %common.resume

bb22.i:                                           ; preds = %bb21.i
  %_5.i.i = tail call noundef ptr @__error() #29
  %_4.i.i = load i32, ptr %_5.i.i, align 4, !noalias !220, !noundef !7
  %_9.i.i = sext i32 %_4.i.i to i64
  %_8.i.i = shl nsw i64 %_9.i.i, 32
  %_7.i.i = or disjoint i64 %_8.i.i, 2
  %_14.i.i = inttoptr i64 %_7.i.i to ptr
  %13 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_14.i.i, ptr %13, align 8, !alias.scope !220
  store i64 -9223372036854775808, ptr %_0, align 8, !alias.scope !220
  br label %bb11.i

bb23.i:                                           ; preds = %bb21.i
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %termios1.i), !noalias !220
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %termios1.i, ptr noundef nonnull align 8 dereferenceable(72) %termios.i, i64 72, i1 false), !noalias !220
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %original.i), !noalias !220
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %original.i, ptr noundef nonnull align 8 dereferenceable(72) %termios.i, i64 72, i1 false), !noalias !220
  %14 = getelementptr inbounds nuw i8, ptr %termios1.i, i64 24
  %15 = load i64, ptr %14, align 8, !noalias !220, !noundef !7
  %16 = and i64 %15, -9
  store i64 %16, ptr %14, align 8, !noalias !220
  %input.val18.i = load ptr, ptr %input.i, align 8, !noalias !220, !noundef !7
  %input.val19.i = load i32, ptr %val.sroa.4.sroa.5.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !noalias !220
  %.not.i.i26.i = icmp eq ptr %input.val18.i, null
  %_2.sroa.0.0.i.i27.i = select i1 %.not.i.i26.i, i32 0, i32 %input.val19.i
  %_0.i.i28.i = call noundef i32 @tcsetattr(i32 noundef %_2.sroa.0.0.i.i27.i, i32 noundef 2, ptr noundef nonnull readonly align 8 dereferenceable(72) %termios1.i) #29, !noalias !220
  %17 = icmp eq i32 %_0.i.i28.i, 0
  br i1 %17, label %bb25.i, label %bb24.i

bb24.i:                                           ; preds = %bb23.i
  %_5.i30.i = tail call noundef ptr @__error() #29
  %_4.i31.i = load i32, ptr %_5.i30.i, align 4, !noalias !220, !noundef !7
  %_9.i32.i = sext i32 %_4.i31.i to i64
  %_8.i33.i = shl nsw i64 %_9.i32.i, 32
  %_7.i34.i = or disjoint i64 %_8.i33.i, 2
  %_14.i35.i = inttoptr i64 %_7.i34.i to ptr
  %18 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_14.i35.i, ptr %18, align 8, !alias.scope !220
  store i64 -9223372036854775808, ptr %_0, align 8, !alias.scope !220
  br label %bb10.i

bb25.i:                                           ; preds = %bb23.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %rv.i), !noalias !220
  store i64 0, ptr %rv.i, align 8, !noalias !220
  %_50.sroa.4.0.rv.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_50.sroa.4.0.rv.sroa_idx.i, align 8, !noalias !220
  %_50.sroa.5.0.rv.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv.i, i64 16
  store i64 0, ptr %_50.sroa.5.0.rv.sroa_idx.i, align 8, !noalias !220
  call void @llvm.experimental.noalias.scope.decl(metadata !236)
  call void @llvm.experimental.noalias.scope.decl(metadata !239)
  %19 = load ptr, ptr %input.i, align 8, !alias.scope !236, !noalias !241, !noundef !7
  %.not.i.i = icmp eq ptr %19, null
  br i1 %.not.i.i, label %bb3.i41.i, label %bb2.i37.i

bb2.i37.i:                                        ; preds = %bb25.i
  call void @llvm.experimental.noalias.scope.decl(metadata !242)
  call void @llvm.experimental.noalias.scope.decl(metadata !245)
  call void @llvm.experimental.noalias.scope.decl(metadata !247)
  call void @llvm.experimental.noalias.scope.decl(metadata !250)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %g.i.i.i.i), !noalias !252
  store ptr %rv.i, ptr %g.i.i.i.i, align 8, !noalias !252
  %20 = getelementptr inbounds nuw i8, ptr %g.i.i.i.i, i64 8
  store i64 0, ptr %20, align 8, !noalias !252
  call void @llvm.experimental.noalias.scope.decl(metadata !253)
  call void @llvm.experimental.noalias.scope.decl(metadata !256)
  call void @llvm.experimental.noalias.scope.decl(metadata !258)
  call void @llvm.experimental.noalias.scope.decl(metadata !261)
  %21 = getelementptr inbounds nuw i8, ptr %input.i, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %buf.i.i.i.i.i.i.i.i, i64 8
  %23 = getelementptr inbounds nuw i8, ptr %buf.i.i.i.i.i.i.i.i, i64 16
  %24 = getelementptr inbounds nuw i8, ptr %buf.i.i.i.i.i.i.i.i, i64 24
  %25 = getelementptr inbounds nuw i8, ptr %input.i, i64 32
  br label %bb1.outer.i.i.i.i.i.i

bb1.outer.i.i.i.i.i.i:                            ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit39.i.i.i.i.i.i, %bb2.i37.i
  %read.sroa.0.0.ph.i.i.i.i.i.i = phi i64 [ %42, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit39.i.i.i.i.i.i ], [ 0, %bb2.i37.i ]
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i.i.backedge, %bb1.outer.i.i.i.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !263)
  call void @llvm.experimental.noalias.scope.decl(metadata !266)
  %_4.i.i.i.i.i.i.i.i = load i64, ptr %val.sroa.4.sroa.4.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !alias.scope !269, !noalias !270, !noundef !7
  %_5.i.i.i.i.i.i.i38.i = load i64, ptr %21, align 8, !alias.scope !269, !noalias !270, !noundef !7
  %_3.not.i.i.i.i.i.i.i.i = icmp ult i64 %_4.i.i.i.i.i.i.i.i, %_5.i.i.i.i.i.i.i38.i
  %_38.0.pre.i.i.i.i.i.i.i.i = load ptr, ptr %input.i, align 8, !alias.scope !269, !noalias !270
  br i1 %_3.not.i.i.i.i.i.i.i.i, label %_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i:                              ; preds = %bb1.i.i.i.i.i.i
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %buf.i.i.i.i.i.i.i.i), !noalias !274
  %_39.1.i.i.i.i.i.i.i.i = load i64, ptr %val.sroa.4.0.input.sroa_idx.i, align 8, !alias.scope !269, !noalias !270, !noundef !7
  store ptr %_38.0.pre.i.i.i.i.i.i.i.i, ptr %buf.i.i.i.i.i.i.i.i, align 8, !noalias !274
  store i64 %_39.1.i.i.i.i.i.i.i.i, ptr %22, align 8, !noalias !274
  store i64 0, ptr %23, align 8, !noalias !274
  %n.i.i.i.i.i.i.i.i = load i64, ptr %25, align 8, !alias.scope !269, !noalias !270, !noundef !7
  store i64 %n.i.i.i.i.i.i.i.i, ptr %24, align 8, !noalias !274
; invoke <std::fs::File as std::io::Read>::read_buf
  %_0.i.i.i.i.i10.i.i.i.i = invoke noundef ptr @_RNvXsa_NtCs5sEH5CPMdak_3std2fsNtB5_4FileNtNtB7_2io4Read8read_buf(ptr noalias noundef nonnull align 4 dereferenceable(4) %val.sroa.4.sroa.5.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, ptr noalias noundef nonnull align 8 dereferenceable(32) %buf.i.i.i.i.i.i.i.i)
          to label %_0.i.i.i.i.i.noexc.i.i.i.i unwind label %cleanup.loopexit.i.i.i.i, !noalias !220

_0.i.i.i.i.i.noexc.i.i.i.i:                       ; preds = %bb1.i.i.i.i.i.i.i.i
  store i64 0, ptr %val.sroa.4.sroa.4.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !alias.scope !269, !noalias !270
  %26 = load <2 x i64>, ptr %23, align 8, !noalias !274
  %_14.i.i.i.i.i.i.i.i = load i64, ptr %23, align 8, !noalias !274, !noundef !7
  store <2 x i64> %26, ptr %21, align 8, !alias.scope !269, !noalias !270
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %_0.i.i.i.i.i10.i.i.i.i, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i.i, label %_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.thread.i.i.i.i.i.i

_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.thread.i.i.i.i.i.i: ; preds = %_0.i.i.i.i.i.noexc.i.i.i.i
  %27 = ptrtoint ptr %_0.i.i.i.i.i10.i.i.i.i to i64
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %buf.i.i.i.i.i.i.i.i), !noalias !274
  br label %bb4.i.i.i.i.i.i

bb11.i.i.i.i.i.i.i.i:                             ; preds = %_0.i.i.i.i.i.noexc.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %buf.i.i.i.i.i.i.i.i), !noalias !274
  br label %_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i

_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i: ; preds = %bb11.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i
  %index1.i.i.i.i.i.i.i.i = phi i64 [ %_5.i.i.i.i.i.i.i38.i, %bb1.i.i.i.i.i.i ], [ %_14.i.i.i.i.i.i.i.i, %bb11.i.i.i.i.i.i.i.i ]
  %index.i.i.i.i.i.i.i.i = phi i64 [ %_4.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i ], [ 0, %bb11.i.i.i.i.i.i.i.i ]
  %new_len.i.i.i.i.i.i.i.i = sub nuw i64 %index1.i.i.i.i.i.i.i.i, %index.i.i.i.i.i.i.i.i
  %28 = icmp eq ptr %_38.0.pre.i.i.i.i.i.i.i.i, null
  br i1 %28, label %bb4.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i, %_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.thread.i.i.i.i.i.i
  %_5.sroa.9.047.i.i.i.i.i.i = phi i64 [ %27, %_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.thread.i.i.i.i.i.i ], [ %new_len.i.i.i.i.i.i.i.i, %_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i ]
  %29 = inttoptr i64 %_5.sroa.9.047.i.i.i.i.i.i to ptr
  %_5.i.i.i.i.i.i.i = and i64 %_5.sroa.9.047.i.i.i.i.i.i, 3
  switch i64 %_5.i.i.i.i.i.i.i, label %default.unreachable [
    i64 2, label %bb20.i.i.i.i.i.i
    i64 3, label %bb4.i.i.i.i.i.i.i
    i64 0, label %bb17.i.i.i.i.i.i
    i64 1, label %bb21.i.i.i.i.i.i
  ], !prof !275

default.unreachable:                              ; preds = %bb4.i.i.i.i.i.i
  unreachable

bb4.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i
  %_10.i.mask.i.i.i.i.i.i = and i64 %_5.sroa.9.047.i.i.i.i.i.i, -4294967296
  %switch.i.i.i.i.i.i = icmp eq i64 %_10.i.mask.i.i.i.i.i.i, 150323855360
  br i1 %switch.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.backedge, label %bb1.i.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i
  %_36.i.i.le.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_38.0.pre.i.i.i.i.i.i.i.i, i64 %index.i.i.i.i.i.i.i.i
  %_3.i20.i.i.i.i.i.i = icmp samesign ult i64 %new_len.i.i.i.i.i.i.i.i, 16
  br i1 %_3.i20.i.i.i.i.i.i, label %bb5.preheader.i.i.i.i.i.i.i, label %bb2.i21.i.i.i.i.i.i

bb5.preheader.i.i.i.i.i.i.i:                      ; preds = %bb5.i.i.i.i.i.i
  %_64.not.i.i.i.i.i.i.i = icmp eq i64 %new_len.i.i.i.i.i.i.i.i, 0
  br i1 %_64.not.i.i.i.i.i.i.i, label %bb4.i23.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i

bb2.i21.i.i.i.i.i.i:                              ; preds = %bb5.i.i.i.i.i.i
; invoke core::slice::memchr::memchr_aligned
  %30 = invoke { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef 10, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_36.i.i.le.i.i.i.i.i.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.i.i.i.i.i.i.i.i)
          to label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i.i.i.i.i.i unwind label %cleanup.loopexit.split-lp.loopexit.i.i.i.i, !noalias !220

bb4.i23.i.i.i.i.i.i:                              ; preds = %bb9.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i, %bb5.preheader.i.i.i.i.i.i.i
  %i.sroa.0.0.lcssa.i.i.i.i.i.i.i = phi i64 [ 0, %bb5.preheader.i.i.i.i.i.i.i ], [ %new_len.i.i.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i ], [ %i.sroa.0.05.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i ]
  %_0.sroa.0.1.i.i.i.i.i.i.i = phi i64 [ 0, %bb5.preheader.i.i.i.i.i.i.i ], [ 0, %bb9.i.i.i.i.i.i.i ], [ 1, %bb7.i.i.i.i.i.i.i ]
  %31 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.i.i.i.i.i.i, 0
  %32 = insertvalue { i64, i64 } %31, i64 %i.sroa.0.0.lcssa.i.i.i.i.i.i.i, 1
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i.i.i.i.i.i

bb7.i.i.i.i.i.i.i:                                ; preds = %bb5.preheader.i.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i
  %i.sroa.0.05.i.i.i.i.i.i.i = phi i64 [ %34, %bb9.i.i.i.i.i.i.i ], [ 0, %bb5.preheader.i.i.i.i.i.i.i ]
  %33 = getelementptr inbounds nuw i8, ptr %_36.i.i.le.i.i.i.i.i.i, i64 %i.sroa.0.05.i.i.i.i.i.i.i
  %_9.i.i.i.i.i.i.i = load i8, ptr %33, align 1, !alias.scope !276, !noalias !279, !noundef !7
  %_8.i.i.i.i.i.i.i = icmp eq i8 %_9.i.i.i.i.i.i.i, 10
  br i1 %_8.i.i.i.i.i.i.i, label %bb4.i23.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i.i

bb9.i.i.i.i.i.i.i:                                ; preds = %bb7.i.i.i.i.i.i.i
  %34 = add nuw nsw i64 %i.sroa.0.05.i.i.i.i.i.i.i, 1
  %exitcond.not.i.i.i.i.i.i.i = icmp eq i64 %34, %new_len.i.i.i.i.i.i.i.i
  br i1 %exitcond.not.i.i.i.i.i.i.i, label %bb4.i23.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i

_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i.i.i.i.i.i: ; preds = %bb4.i23.i.i.i.i.i.i, %bb2.i21.i.i.i.i.i.i
  %.merged.i.i.i.i.i.i.i = phi { i64, i64 } [ %32, %bb4.i23.i.i.i.i.i.i ], [ %30, %bb2.i21.i.i.i.i.i.i ]
  %35 = extractvalue { i64, i64 } %.merged.i.i.i.i.i.i.i, 0
  %36 = trunc nuw i64 %35 to i1
  br i1 %36, label %bb10.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i

bb10.i.i.i.i.i.i:                                 ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i.i.i.i.i.i
  %37 = extractvalue { i64, i64 } %.merged.i.i.i.i.i.i.i, 1
  %_7.i24.i.i.i.i.i.i = icmp ult i64 %37, %new_len.i.i.i.i.i.i.i.i
  br i1 %_7.i24.i.i.i.i.i.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core5slice5indexINtNtNtB9_3ops5range14RangeInclusivejEINtB5_10SliceIndexShE5indexCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, !prof !30

bb6.i.i.i.i.i.i.i:                                ; preds = %bb10.i.i.i.i.i.i
; invoke core::slice::index::slice_index_fail
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %37, i64 noundef range(i64 0, -9223372036854775808) %new_len.i.i.i.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_92437324c5298c66e2abee23d7039388) #34
          to label %.noexc11.i.i.i.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.i.i.i.i, !noalias !220

.noexc11.i.i.i.i:                                 ; preds = %bb6.i.i.i.i.i.i.i
  unreachable

_RNvXs8_NtNtCsjMrxcFdYDNN_4core5slice5indexINtNtNtB9_3ops5range14RangeInclusivejEINtB5_10SliceIndexShE5indexCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i: ; preds = %bb10.i.i.i.i.i.i
  %38 = add nuw i64 %37, 1
  call void @llvm.experimental.noalias.scope.decl(metadata !280)
  %len.i.i.i.i.i.i.i.i = load i64, ptr %_50.sroa.5.0.rv.sroa_idx.i, align 8, !alias.scope !283, !noalias !286, !noundef !7
  %self2.i.i.i.i.i.i.i.i = load i64, ptr %rv.i, align 8, !range !23, !alias.scope !283, !noalias !286, !noundef !7
  %_9.i.i.i.i.i.i.i.i = sub i64 %self2.i.i.i.i.i.i.i.i, %len.i.i.i.i.i.i.i.i
  %_7.i.i.not.i.i.i.i.i.i = icmp ult i64 %37, %_9.i.i.i.i.i.i.i.i
  br i1 %_7.i.i.not.i.i.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i, label %bb1.i.i27.i.i.i.i.i.i, !prof !30

bb1.i.i27.i.i.i.i.i.i:                            ; preds = %_RNvXs8_NtNtCsjMrxcFdYDNN_4core5slice5indexINtNtNtB9_3ops5range14RangeInclusivejEINtB5_10SliceIndexShE5indexCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv.i, i64 noundef %len.i.i.i.i.i.i.i.i, i64 noundef %38, i64 noundef 1, i64 noundef 1)
          to label %.noexc12.i.i.i.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.i.i.i.i, !noalias !220

.noexc12.i.i.i.i:                                 ; preds = %bb1.i.i27.i.i.i.i.i.i
  %len.pre.i.i.i.i.i.i.i = load i64, ptr %_50.sroa.5.0.rv.sroa_idx.i, align 8, !alias.scope !287, !noalias !286
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i: ; preds = %.noexc12.i.i.i.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core5slice5indexINtNtNtB9_3ops5range14RangeInclusivejEINtB5_10SliceIndexShE5indexCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i
  %len.i.i.i.i.i.i.i = phi i64 [ %len.i.i.i.i.i.i.i.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core5slice5indexINtNtNtB9_3ops5range14RangeInclusivejEINtB5_10SliceIndexShE5indexCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i ], [ %len.pre.i.i.i.i.i.i.i, %.noexc12.i.i.i.i ]
  %_9.i25.i.i.i.i.i.i = icmp sgt i64 %len.i.i.i.i.i.i.i, -1
  call void @llvm.assume(i1 %_9.i25.i.i.i.i.i.i)
  %_10.i26.i.i.i.i.i.i = load ptr, ptr %_50.sroa.4.0.rv.sroa_idx.i, align 8, !alias.scope !287, !noalias !286, !nonnull !7, !noundef !7
  %dst.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i26.i.i.i.i.i.i, i64 %len.i.i.i.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %dst.i.i.i.i.i.i.i, ptr noundef nonnull readonly align 1 dereferenceable(1) %_36.i.i.le.i.i.i.i.i.i, i64 %38, i1 false), !noalias !288
  %39 = add i64 %len.i.i.i.i.i.i.i, %38
  store i64 %39, ptr %_50.sroa.5.0.rv.sroa_idx.i, align 8, !alias.scope !287, !noalias !286
  %_6.i.i.i.i.i.i.i = load i64, ptr %val.sroa.4.sroa.4.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !alias.scope !289, !noalias !292, !noundef !7
  %v1.i.i.i.i.i.i.i = add i64 %_6.i.i.i.i.i.i.i, %38
  %v2.i.i.i.i.i.i.i = load i64, ptr %21, align 8, !alias.scope !289, !noalias !292, !noundef !7
  %_0.sroa.0.0.i.i28.i.i.i.i.i.i = call noundef i64 @llvm.umin.i64(i64 %v2.i.i.i.i.i.i.i, i64 %v1.i.i.i.i.i.i.i)
  store i64 %_0.sroa.0.0.i.i28.i.i.i.i.i.i, ptr %val.sroa.4.sroa.4.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !alias.scope !289, !noalias !292
  %40 = add i64 %38, %read.sroa.0.0.ph.i.i.i.i.i.i
  br label %bb11.i.i.i.i.i.i

bb9.i.i.i.i.i.i:                                  ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i.i.i.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !293)
  %len.i.i29.i.i.i.i.i.i = load i64, ptr %_50.sroa.5.0.rv.sroa_idx.i, align 8, !alias.scope !296, !noalias !286, !noundef !7
  %self2.i.i30.i.i.i.i.i.i = load i64, ptr %rv.i, align 8, !range !23, !alias.scope !296, !noalias !286, !noundef !7
  %_9.i.i31.i.i.i.i.i.i = sub i64 %self2.i.i30.i.i.i.i.i.i, %len.i.i29.i.i.i.i.i.i
  %_7.i.i32.i.i.i.i.i.i = icmp ugt i64 %new_len.i.i.i.i.i.i.i.i, %_9.i.i31.i.i.i.i.i.i
  br i1 %_7.i.i32.i.i.i.i.i.i, label %bb1.i.i37.i.i.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit39.i.i.i.i.i.i, !prof !5

bb1.i.i37.i.i.i.i.i.i:                            ; preds = %bb9.i.i.i.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv.i, i64 noundef %len.i.i29.i.i.i.i.i.i, i64 noundef %new_len.i.i.i.i.i.i.i.i, i64 noundef 1, i64 noundef 1)
          to label %.noexc13.i.i.i.i unwind label %cleanup.loopexit.split-lp.loopexit.i.i.i.i, !noalias !220

.noexc13.i.i.i.i:                                 ; preds = %bb1.i.i37.i.i.i.i.i.i
  %len.pre.i38.i.i.i.i.i.i = load i64, ptr %_50.sroa.5.0.rv.sroa_idx.i, align 8, !alias.scope !299, !noalias !286
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit39.i.i.i.i.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit39.i.i.i.i.i.i: ; preds = %.noexc13.i.i.i.i, %bb9.i.i.i.i.i.i
  %len.i33.i.i.i.i.i.i = phi i64 [ %len.i.i29.i.i.i.i.i.i, %bb9.i.i.i.i.i.i ], [ %len.pre.i38.i.i.i.i.i.i, %.noexc13.i.i.i.i ]
  %_9.i34.i.i.i.i.i.i = icmp sgt i64 %len.i33.i.i.i.i.i.i, -1
  call void @llvm.assume(i1 %_9.i34.i.i.i.i.i.i)
  %_10.i35.i.i.i.i.i.i = load ptr, ptr %_50.sroa.4.0.rv.sroa_idx.i, align 8, !alias.scope !299, !noalias !286, !nonnull !7, !noundef !7
  %dst.i36.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i35.i.i.i.i.i.i, i64 %len.i33.i.i.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i36.i.i.i.i.i.i, ptr nonnull readonly align 1 %_36.i.i.le.i.i.i.i.i.i, i64 %new_len.i.i.i.i.i.i.i.i, i1 false), !noalias !300
  %41 = add i64 %len.i33.i.i.i.i.i.i, %new_len.i.i.i.i.i.i.i.i
  store i64 %41, ptr %_50.sroa.5.0.rv.sroa_idx.i, align 8, !alias.scope !299, !noalias !286
  %_6.i40.i.i.i.i.i.i = load i64, ptr %val.sroa.4.sroa.4.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !alias.scope !301, !noalias !292, !noundef !7
  %v1.i41.i.i.i.i.i.i = add i64 %_6.i40.i.i.i.i.i.i, %new_len.i.i.i.i.i.i.i.i
  %v2.i42.i.i.i.i.i.i = load i64, ptr %21, align 8, !alias.scope !301, !noalias !292, !noundef !7
  %_0.sroa.0.0.i.i43.i.i.i.i.i.i = call noundef i64 @llvm.umin.i64(i64 %v2.i42.i.i.i.i.i.i, i64 %v1.i41.i.i.i.i.i.i)
  store i64 %_0.sroa.0.0.i.i43.i.i.i.i.i.i, ptr %val.sroa.4.sroa.4.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !alias.scope !301, !noalias !292
  %42 = add i64 %new_len.i.i.i.i.i.i.i.i, %read.sroa.0.0.ph.i.i.i.i.i.i
  %_18.i.i.i.i.i.i = icmp eq i64 %new_len.i.i.i.i.i.i.i.i, 0
  br i1 %_18.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i, label %bb1.outer.i.i.i.i.i.i

bb11.i.i.i.i.i.i:                                 ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit39.i.i.i.i.i.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i
  %read.sroa.0.1.i.i.i.i.i.i = phi i64 [ %40, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i ], [ %42, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console.exit39.i.i.i.i.i.i ]
  %43 = inttoptr i64 %read.sroa.0.1.i.i.i.i.i.i to ptr
  br label %bb1.i.i.i.i

bb20.i.i.i.i.i.i:                                 ; preds = %bb4.i.i.i.i.i.i
  %_7.i19.mask.i.i.i.i.i.i = and i64 %_5.sroa.9.047.i.i.i.i.i.i, -4294967296
  %44 = icmp eq i64 %_7.i19.mask.i.i.i.i.i.i, 17179869184
  br i1 %44, label %bb1.i.i.i.i.i.i.backedge, label %bb1.i.i.i.i

bb17.i.i.i.i.i.i:                                 ; preds = %bb4.i.i.i.i.i.i
  %45 = icmp ne i64 %_5.sroa.9.047.i.i.i.i.i.i, 0
  call void @llvm.assume(i1 %45)
  %46 = getelementptr inbounds nuw i8, ptr %29, i64 16
  %47 = load i8, ptr %46, align 8, !range !304, !noalias !279, !noundef !7
  %48 = icmp eq i8 %47, 35
  br i1 %48, label %bb1.i.i.i.i.i.i.backedge, label %bb1.i.i.i.i

bb21.i.i.i.i.i.i:                                 ; preds = %bb4.i.i.i.i.i.i
  %49 = getelementptr i8, ptr %29, i64 -1
  %50 = icmp ne ptr %49, null
  call void @llvm.assume(i1 %50)
  %51 = getelementptr i8, ptr %29, i64 15
  %52 = load i8, ptr %51, align 8, !range !304, !noalias !279, !noundef !7
  %53 = icmp eq i8 %52, 35
  br i1 %53, label %bb2.i2.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i

bb2.i2.i.i.i.i.i.i.i.i.i:                         ; preds = %bb21.i.i.i.i.i.i
  %_6.val.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %49, align 8, !noalias !279
  %54 = getelementptr i8, ptr %29, i64 7
  %_6.val1.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %54, align 8, !noalias !279, !nonnull !7, !align !22, !noundef !7
  %55 = load ptr, ptr %_6.val1.i.i.i.i.i.i.i.i.i.i.i, align 8, !invariant.load !7, !noalias !279
  %.not.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %55, null
  br i1 %.not.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i.i.i.i.i.i:            ; preds = %bb2.i2.i.i.i.i.i.i.i.i.i
  %56 = icmp ne ptr %_6.val.i.i.i.i.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %56), !noalias !261
  invoke void %55(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i.i.i.i.i.i, !noalias !279

bb3.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %is_not_null.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i.i.i.i.i.i
  %57 = icmp ne ptr %_6.val.i.i.i.i.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %57), !noalias !261
  %58 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i.i.i.i.i, i64 8
  %59 = load i64, ptr %58, align 8, !range !23, !invariant.load !7, !noalias !279
  %60 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i.i.i.i.i, i64 16
  %61 = load i64, ptr %60, align 8, !range !24, !invariant.load !7, !noalias !279
  %62 = add i64 %61, -1
  %63 = icmp sgt i64 %62, -1
  call void @llvm.assume(i1 %63), !noalias !261
  %64 = icmp eq i64 %59, 0
  br i1 %64, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %59, i64 noundef range(i64 1, -9223372036854775807) %61) #29, !noalias !279
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i.i.i.i.i.i:                ; preds = %is_not_null.i.i.i.i.i.i.i.i.i.i.i.i.i
  %65 = landingpad { ptr, i32 }
          cleanup
  %66 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i.i.i.i.i, i64 8
  %67 = load i64, ptr %66, align 8, !range !23, !invariant.load !7, !noalias !279
  %68 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i.i.i.i.i, i64 16
  %69 = load i64, ptr %68, align 8, !range !24, !invariant.load !7, !noalias !279
  %70 = add i64 %69, -1
  %71 = icmp sgt i64 %70, -1
  call void @llvm.assume(i1 %71), !noalias !261
  %72 = icmp eq i64 %67, 0
  br i1 %72, label %bb1.i.i.i.i.i.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %67, i64 noundef range(i64 1, -9223372036854775807) %69) #29, !noalias !279
  br label %bb1.i.i.i.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i.i:                        ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %49, i64 noundef 24, i64 noundef 8) #29, !noalias !279
  br label %bb7.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %49, i64 noundef 24, i64 noundef 8) #29, !noalias !279
  br label %bb1.i.i.i.i.i.i.backedge

bb1.i.i.i.i.i.i.backedge:                         ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i, %bb17.i.i.i.i.i.i, %bb20.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i
  br label %bb1.i.i.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %cleanup3.i.i.i.i, %cleanup.loopexit.split-lp.loopexit.split-lp.i.i.i.i, %cleanup.loopexit.split-lp.loopexit.i.i.i.i, %cleanup.loopexit.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i.i
  %.pn.i.i.i.i = phi { ptr, i32 } [ %75, %cleanup3.i.i.i.i ], [ %65, %bb1.i.i.i.i.i.i.i.i.i.i.i ], [ %lpad.loopexit.i.i.i.i, %cleanup.loopexit.i.i.i.i ], [ %lpad.loopexit18.i.i.i.i, %cleanup.loopexit.split-lp.loopexit.i.i.i.i ], [ %lpad.loopexit.split-lp19.i.i.i.i, %cleanup.loopexit.split-lp.loopexit.split-lp.i.i.i.i ]
; invoke <std::io::Guard as core::ops::drop::Drop>::drop
  invoke void @_RNvXNtCs5sEH5CPMdak_3std2ioNtB2_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(16) %g.i.i.i.i)
          to label %bb18.i unwind label %terminate.i.i.i.i, !noalias !220

cleanup.loopexit.i.i.i.i:                         ; preds = %bb1.i.i.i.i.i.i.i.i
  %lpad.loopexit.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %bb7.i.i.i.i

cleanup.loopexit.split-lp.loopexit.i.i.i.i:       ; preds = %bb1.i.i37.i.i.i.i.i.i, %bb2.i21.i.i.i.i.i.i
  %lpad.loopexit18.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %bb7.i.i.i.i

cleanup.loopexit.split-lp.loopexit.split-lp.i.i.i.i: ; preds = %bb1.i.i27.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i
  %lpad.loopexit.split-lp19.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %bb7.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb21.i.i.i.i.i.i, %bb17.i.i.i.i.i.i, %bb20.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i, %bb11.i.i.i.i.i.i
  %_0.sroa.3.0.i.i.i.i.i.i = phi ptr [ %43, %bb11.i.i.i.i.i.i ], [ %29, %bb4.i.i.i.i.i.i.i ], [ %29, %bb20.i.i.i.i.i.i ], [ %29, %bb17.i.i.i.i.i.i ], [ %29, %bb21.i.i.i.i.i.i ]
  %_0.sroa.0.0.i.i.i.i.i.i = phi i64 [ 0, %bb11.i.i.i.i.i.i ], [ 1, %bb4.i.i.i.i.i.i.i ], [ 1, %bb20.i.i.i.i.i.i ], [ 1, %bb17.i.i.i.i.i.i ], [ 1, %bb21.i.i.i.i.i.i ]
  %self1.i.i.i.i = load ptr, ptr %g.i.i.i.i, align 8, !noalias !252, !nonnull !7, !align !22, !noundef !7
  %73 = getelementptr inbounds nuw i8, ptr %self1.i.i.i.i, i64 8
  %_21.i.i.i.i = load ptr, ptr %73, align 8, !noalias !220, !nonnull !7, !noundef !7
  %74 = getelementptr inbounds nuw i8, ptr %self1.i.i.i.i, i64 16
  %self2.i.i.i.i = load i64, ptr %74, align 8, !noalias !220, !noundef !7
  %index.i.i.i.i = load i64, ptr %20, align 8, !noalias !252, !noundef !7
  %new_len.i.i.i.i = sub nuw i64 %self2.i.i.i.i, %index.i.i.i.i
  %_27.i.i.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i, i64 %index.i.i.i.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_12.i.i.i.i), !noalias !252
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_12.i.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_27.i.i.i.i, i64 noundef %new_len.i.i.i.i)
          to label %bb2.i.i.i.i unwind label %cleanup3.i.i.i.i, !noalias !220

cleanup3.i.i.i.i:                                 ; preds = %bb1.i.i.i.i
  %75 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<core::result::Result<usize, std::io::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultjNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsC3JuwEIQwb_7console(i64 %_0.sroa.0.0.i.i.i.i.i.i, ptr %_0.sroa.3.0.i.i.i.i.i.i) #32
          to label %bb7.i.i.i.i unwind label %terminate.i.i.i.i, !noalias !220

bb2.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
  %_29.i.i.i.i = load i64, ptr %_12.i.i.i.i, align 8, !range !6, !noalias !252, !noundef !7
  %_28.not.i.i.i.i = icmp eq i64 %_29.i.i.i.i, 0
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_12.i.i.i.i), !noalias !252
  br i1 %_28.not.i.i.i.i, label %bb4.i.i.i.i, label %bb3.i.i.i39.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i.i
  %self4.i.i.i.i = load ptr, ptr %g.i.i.i.i, align 8, !noalias !252, !nonnull !7, !align !22, !noundef !7
  %76 = getelementptr inbounds nuw i8, ptr %self4.i.i.i.i, i64 16
  %_13.i.i.i.i = load i64, ptr %76, align 8, !noalias !220, !noundef !7
  %_33.i.i.i.i = icmp sgt i64 %_13.i.i.i.i, -1
  call void @llvm.assume(i1 %_33.i.i.i.i)
  store i64 %_13.i.i.i.i, ptr %20, align 8, !noalias !252
  br label %_RNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBb_2fs4FileENtB9_7BufRead9read_lineCsC3JuwEIQwb_7console.exit.i.i

bb3.i.i.i39.i:                                    ; preds = %bb2.i.i.i.i
  %77 = trunc nuw i64 %_0.sroa.0.0.i.i.i.i.i.i to i1
  %spec.select.i.i.i.i = select i1 %77, ptr %_0.sroa.3.0.i.i.i.i.i.i, ptr @alloc_a679944945e039118b94625869e7de57
  br label %_RNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBb_2fs4FileENtB9_7BufRead9read_lineCsC3JuwEIQwb_7console.exit.i.i

terminate.i.i.i.i:                                ; preds = %cleanup3.i.i.i.i, %bb7.i.i.i.i
  %78 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !220
  unreachable

_RNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBb_2fs4FileENtB9_7BufRead9read_lineCsC3JuwEIQwb_7console.exit.i.i: ; preds = %bb3.i.i.i39.i, %bb4.i.i.i.i
  %self.sroa.5.0.i.i.i.i = phi ptr [ %_0.sroa.3.0.i.i.i.i.i.i, %bb4.i.i.i.i ], [ %spec.select.i.i.i.i, %bb3.i.i.i39.i ]
  %self.sroa.0.0.i.i.i.i = phi i64 [ %_0.sroa.0.0.i.i.i.i.i.i, %bb4.i.i.i.i ], [ 1, %bb3.i.i.i39.i ]
; invoke <std::io::Guard as core::ops::drop::Drop>::drop
  invoke void @_RNvXNtCs5sEH5CPMdak_3std2ioNtB2_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(16) %g.i.i.i.i)
          to label %.noexc.i unwind label %cleanup2.i, !noalias !220

.noexc.i:                                         ; preds = %_RNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBb_2fs4FileENtB9_7BufRead9read_lineCsC3JuwEIQwb_7console.exit.i.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %g.i.i.i.i), !noalias !252
  %79 = insertvalue { i64, ptr } poison, i64 %self.sroa.0.0.i.i.i.i, 0
  %80 = insertvalue { i64, ptr } %79, ptr %self.sroa.5.0.i.i.i.i, 1
  br label %bb5.i

bb3.i41.i:                                        ; preds = %bb25.i
; invoke <std::io::stdio::Stdin>::read_line
  %81 = invoke { i64, ptr } @_RNvMs1_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_5Stdin9read_line(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %val.sroa.4.0.input.sroa_idx.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %rv.i)
          to label %bb5.i unwind label %cleanup2.i, !noalias !220

cleanup2.i:                                       ; preds = %bb26.i, %bb3.i41.i, %_RNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBb_2fs4FileENtB9_7BufRead9read_lineCsC3JuwEIQwb_7console.exit.i.i
  %82 = landingpad { ptr, i32 }
          cleanup
  br label %bb18.i

bb5.i:                                            ; preds = %bb3.i41.i, %.noexc.i
  %.pn.i.i = phi { i64, ptr } [ %80, %.noexc.i ], [ %81, %bb3.i41.i ]
  %83 = extractvalue { i64, ptr } %.pn.i.i, 0
  %84 = extractvalue { i64, ptr } %.pn.i.i, 1
  %input.val23.i = load ptr, ptr %input.i, align 8, !noalias !220, !noundef !7
  %input.val24.i = load i32, ptr %val.sroa.4.sroa.5.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !noalias !220
  %.not.i.i44.i = icmp eq ptr %input.val23.i, null
  %_2.sroa.0.0.i.i45.i = select i1 %.not.i.i44.i, i32 0, i32 %input.val24.i
  %_0.i.i46.i = call noundef i32 @tcsetattr(i32 noundef %_2.sroa.0.0.i.i45.i, i32 noundef 2, ptr noundef nonnull readonly align 8 dereferenceable(72) %original.i) #29, !noalias !220
  %85 = icmp eq i32 %_0.i.i46.i, 0
  br i1 %85, label %bb27.i, label %bb26.i

cleanup3.i:                                       ; preds = %bb30.i
  %86 = landingpad { ptr, i32 }
          cleanup
  br label %bb14.i

bb26.i:                                           ; preds = %bb5.i
  %_5.i48.i = tail call noundef ptr @__error() #29
  %_4.i49.i = load i32, ptr %_5.i48.i, align 4, !noalias !220, !noundef !7
  %_9.i50.i = sext i32 %_4.i49.i to i64
  %_8.i51.i = shl nsw i64 %_9.i50.i, 32
  %_7.i52.i = or disjoint i64 %_8.i51.i, 2
  %_14.i53.i = inttoptr i64 %_7.i52.i to ptr
  %87 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_14.i53.i, ptr %87, align 8, !alias.scope !220
  store i64 -9223372036854775808, ptr %_0, align 8, !alias.scope !220
; invoke core::ptr::drop_in_place::<core::result::Result<usize, std::io::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultjNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsC3JuwEIQwb_7console(i64 %83, ptr %84)
          to label %bb8.i unwind label %cleanup2.i, !noalias !220

bb27.i:                                           ; preds = %bb5.i
  %_33.sroa.0.0.copyload.i = load i64, ptr %rv.i, align 8, !noalias !220
  %_33.sroa.5.0.copyload.i = load ptr, ptr %_50.sroa.4.0.rv.sroa_idx.i, align 8, !noalias !220
  %88 = trunc nuw i64 %83 to i1
  br i1 %88, label %bb29.i, label %bb30.i

bb29.i:                                           ; preds = %bb27.i
  %89 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %84, ptr %89, align 8, !alias.scope !220
  store i64 -9223372036854775808, ptr %_0, align 8, !alias.scope !220
  %90 = icmp eq i64 %_33.sroa.0.0.copyload.i, 0
  br i1 %90, label %bb28.i, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb29.i
  %91 = icmp ne ptr %_33.sroa.5.0.copyload.i, null
  call void @llvm.assume(i1 %91)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_33.sroa.5.0.copyload.i, i64 noundef %_33.sroa.0.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !220
  br label %bb28.i

bb30.i:                                           ; preds = %bb27.i
  %_33.sroa.6.0.copyload.i = load i64, ptr %_50.sroa.5.0.rv.sroa_idx.i, align 8, !noalias !220
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_57.i), !noalias !220
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_58.i), !noalias !220
  store i64 %_33.sroa.0.0.copyload.i, ptr %_58.i, align 8, !noalias !220
  %_33.sroa.5.0._58.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_58.i, i64 8
  store ptr %_33.sroa.5.0.copyload.i, ptr %_33.sroa.5.0._58.sroa_idx.i, align 8, !noalias !220
  %_33.sroa.6.0._58.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_58.i, i64 16
  store i64 %_33.sroa.6.0.copyload.i, ptr %_33.sroa.6.0._58.sroa_idx.i, align 8, !noalias !220
; invoke console::unix_term::read_secure::{closure#3}
  invoke fastcc void @_RNCNvNtCsC3JuwEIQwb_7console9unix_term11read_secures1_0B5_(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_57.i, ptr noalias noundef align 8 captures(address) dereferenceable(24) %_58.i)
          to label %bb31.i unwind label %cleanup3.i

bb31.i:                                           ; preds = %bb30.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_58.i), !noalias !220
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %_57.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_57.i), !noalias !220
  br label %bb28.i

bb28.i:                                           ; preds = %bb31.i, %bb2.i.i.i4.i.i.i.i, %bb29.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %rv.i), !noalias !220
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %original.i), !noalias !220
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %termios1.i), !noalias !220
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %termios.i), !noalias !220
  call void @llvm.experimental.noalias.scope.decl(metadata !305)
  %92 = load ptr, ptr %input.i, align 8, !alias.scope !305, !noalias !220, !noundef !7
  %93 = icmp eq ptr %92, null
  br i1 %93, label %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit, label %bb2.i55.i

bb2.i55.i:                                        ; preds = %bb28.i
  call void @llvm.experimental.noalias.scope.decl(metadata !308)
  %_1.val2.i.i.i = load i64, ptr %val.sroa.4.0.input.sroa_idx.i, align 8, !alias.scope !311, !noalias !220, !noundef !7
  %94 = icmp eq i64 %_1.val2.i.i.i, 0
  br i1 %94, label %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.sink.split, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i: ; preds = %bb2.i55.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %92, i64 noundef %_1.val2.i.i.i, i64 noundef 1) #29, !noalias !312
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.sink.split

bb8.i:                                            ; preds = %bb26.i
  %rv.val15.i = load i64, ptr %rv.i, align 8, !noalias !220
  %95 = icmp eq i64 %rv.val15.i, 0
  br i1 %95, label %bb9.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb8.i
  %rv.val16.i = load ptr, ptr %_50.sroa.4.0.rv.sroa_idx.i, align 8, !noalias !220, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rv.val16.i, i64 noundef %rv.val15.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !220
  br label %bb9.i

bb9.i:                                            ; preds = %bb2.i.i.i4.i.i.i, %bb8.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %rv.i), !noalias !220
  br label %bb10.i

bb10.i:                                           ; preds = %bb9.i, %bb24.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %original.i), !noalias !220
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %termios1.i), !noalias !220
  br label %bb11.i

bb18.i:                                           ; preds = %cleanup2.i, %bb7.i.i.i.i
  %.pn.ph.i = phi { ptr, i32 } [ %82, %cleanup2.i ], [ %.pn.i.i.i.i, %bb7.i.i.i.i ]
  %rv.val.i = load i64, ptr %rv.i, align 8, !noalias !220
  %96 = icmp eq i64 %rv.val.i, 0
  br i1 %96, label %bb14.i, label %bb2.i.i.i4.i.i56.i

bb2.i.i.i4.i.i56.i:                               ; preds = %bb18.i
  %rv.val14.i = load ptr, ptr %_50.sroa.4.0.rv.sroa_idx.i, align 8, !noalias !220, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rv.val14.i, i64 noundef %rv.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !220
  br label %bb14.i

bb11.i:                                           ; preds = %bb10.i, %bb22.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %termios.i), !noalias !220
  call void @llvm.experimental.noalias.scope.decl(metadata !313)
  %97 = load ptr, ptr %input.i, align 8, !alias.scope !313, !noalias !220, !noundef !7
  %98 = icmp eq ptr %97, null
  br i1 %98, label %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit, label %bb2.i58.i

bb2.i58.i:                                        ; preds = %bb11.i
  call void @llvm.experimental.noalias.scope.decl(metadata !316)
  %_1.val2.i.i59.i = load i64, ptr %val.sroa.4.0.input.sroa_idx.i, align 8, !alias.scope !319, !noalias !220, !noundef !7
  %99 = icmp eq i64 %_1.val2.i.i59.i, 0
  br i1 %99, label %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.sink.split, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i60.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i60.i: ; preds = %bb2.i58.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %97, i64 noundef %_1.val2.i.i59.i, i64 noundef 1) #29, !noalias !320
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.sink.split

_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.sink.split: ; preds = %bb2.i58.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i60.i, %bb2.i55.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i
  %.val1.i.i.i = load i32, ptr %val.sroa.4.sroa.5.0.val.sroa.4.0.input.sroa_idx.sroa_idx.i, align 8, !range !28, !noalias !220, !noundef !7
  %_5.i.i.i.i.i3.i.i.i = call noundef i32 @close(i32 noundef %.val1.i.i.i) #29, !noalias !220
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit

_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit: ; preds = %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.sink.split, %bb28.i, %bb11.i
  %.pr = load i64, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %input.i), !noalias !220
  %100 = icmp eq i64 %.pr, -9223372036854775808
  br i1 %100, label %bb9, label %bb6

bb14:                                             ; preds = %start
  store i64 0, ptr %_0, align 8
  %_3.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_3.sroa.4.0._0.sroa_idx, align 8
  %_3.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 0, ptr %_3.sroa.5.0._0.sroa_idx, align 8
  br label %bb9

bb9:                                              ; preds = %bb7, %bb2.i.i.i4.i.i4, %bb15, %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit.thread, %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit, %bb14
  ret void

bb6:                                              ; preds = %_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure.exit
  %rv.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  %rv.sroa.7.0.copyload = load ptr, ptr %rv.sroa.7.0._0.sroa_idx, align 8
; invoke <console::term::Term>::write_line
  %101 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term10write_line(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) inttoptr (i64 1 to ptr), i64 noundef 0)
          to label %bb7 unwind label %cleanup

cleanup:                                          ; preds = %bb6
  %102 = landingpad { ptr, i32 }
          cleanup
  %103 = icmp eq i64 %.pr, 0
  br i1 %103, label %common.resume, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup
  %104 = icmp ne ptr %rv.sroa.7.0.copyload, null
  call void @llvm.assume(i1 %104)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rv.sroa.7.0.copyload, i64 noundef %.pr, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %common.resume

bb7:                                              ; preds = %bb6
  %.not = icmp eq ptr %101, null
  br i1 %.not, label %bb9, label %bb15

bb15:                                             ; preds = %bb7
  store ptr %101, ptr %rv.sroa.7.0._0.sroa_idx, align 8
  store i64 -9223372036854775808, ptr %_0, align 8
  %105 = icmp eq i64 %.pr, 0
  br i1 %105, label %bb9, label %bb2.i.i.i4.i.i4

bb2.i.i.i4.i.i4:                                  ; preds = %bb15
  %106 = icmp ne ptr %rv.sroa.7.0.copyload, null
  call void @llvm.assume(i1 %106)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rv.sroa.7.0.copyload, i64 noundef %.pr, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb9
}

; <console::term::Term>::read_line_initial_text
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term22read_line_initial_text(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %initial.0, i64 noundef %initial.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args.i.i = alloca [16 x i8], align 8
  %_6.i73.i = alloca [24 x i8], align 8
  %n.i.i = alloca [8 x i8], align 8
  %buf.i.i = alloca [24 x i8], align 8
  %vector.i.i = alloca [24 x i8], align 8
  %args.i = alloca [16 x i8], align 8
  %_41.i = alloca [24 x i8], align 8
  %bytes_char.i = alloca [4 x i8], align 4
  %_8.i = alloca [24 x i8], align 8
  %chars.i = alloca [24 x i8], align 8
  %initial.i = alloca [16 x i8], align 8
  %e.i18 = alloca [16 x i8], align 8
  %e.i9 = alloca [16 x i8], align 8
  %e.i = alloca [16 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 9
  %1 = load i8, ptr %0, align 1, !range !29, !noundef !7
  %_3 = trunc nuw i8 %1 to i1
  br i1 %_3, label %bb2, label %bb28

bb2:                                              ; preds = %start
  %_23.i.i = icmp eq i64 %initial.1, 0
  br i1 %_23.i.i, label %bb31, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb2
  %or.cond.not = icmp sgt i64 %initial.1, 0
  br i1 %or.cond.not, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i, label %bb30, !prof !321

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i: ; preds = %bb6.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !322
; call __rustc::__rust_alloc
  %2 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %initial.1, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !322
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb30, label %bb10.i

bb10.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i
  %4 = ptrtoint ptr %2 to i64
  br label %bb31

bb28:                                             ; preds = %start
  store i64 0, ptr %_0, align 8
  %_4.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_4.sroa.4.0._0.sroa_idx, align 8
  %_4.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 0, ptr %_4.sroa.5.0._0.sroa_idx, align 8
  br label %bb20

bb20:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit103, %bb28
  ret void

bb30:                                             ; preds = %bb6.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i
  %_47.sroa.4.0.ph = phi i64 [ 1, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i ], [ 0, %bb6.i.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_47.sroa.4.0.ph, i64 %initial.1) #30
  unreachable

bb31:                                             ; preds = %bb10.i, %bb2
  %_47.sroa.10.0 = phi i64 [ %4, %bb10.i ], [ 1, %bb2 ]
  %5 = inttoptr i64 %_47.sroa.10.0 to ptr
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %5, ptr nonnull align 1 %initial.0, i64 %initial.1, i1 false)
  %_57 = load ptr, ptr %self, align 8, !nonnull !7, !noundef !7
  %_8 = getelementptr inbounds nuw i8, ptr %_57, i64 128
  %6 = atomicrmw or ptr %_8, i64 1 acquire, align 8, !noalias !325
  %_4.i = and i64 %6, 1
  %7 = icmp eq i64 %_4.i, 0
  br i1 %7, label %bb1.i, label %bb3.i, !prof !30

bb3.i:                                            ; preds = %bb31
; invoke <std::sys::sync::rwlock::queue::RwLock>::lock_contended
  invoke void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock14lock_contended(ptr noundef nonnull align 8 %_8, i1 noundef zeroext true)
          to label %bb1.i unwind label %cleanup

bb1.i:                                            ; preds = %bb3.i, %bb31
  %8 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !325
  %_6.i.i = and i64 %8, 9223372036854775807
  %9 = icmp eq i64 %_6.i.i, 0
  br i1 %9, label %bb3, label %bb6.i.i47, !prof !30

bb6.i.i47:                                        ; preds = %bb1.i
; invoke std::panicking::panic_count::is_zero_slow_path
  %10 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
          to label %.noexc48 unwind label %cleanup

.noexc48:                                         ; preds = %bb6.i.i47
  %11 = xor i1 %10, true
  br label %bb3

cleanup:                                          ; preds = %bb6.i.i47, %bb3.i
  %12 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body

cleanup.body:                                     ; preds = %cleanup.i23, %cleanup
  %eh.lpad-body27 = phi { ptr, i32 } [ %12, %cleanup ], [ %15, %cleanup.i23 ]
  br i1 %_23.i.i, label %common.resume, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup.body
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %5, i64 noundef %initial.1, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %common.resume

bb3:                                              ; preds = %.noexc48, %bb1.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %11, %.noexc48 ], [ false, %bb1.i ]
  %_12.i = getelementptr inbounds nuw i8, ptr %_57, i64 136
  %13 = load atomic i8, ptr %_12.i monotonic, align 1, !noalias !325
  %.not131 = icmp eq i8 %13, 0
  br i1 %.not131, label %bb4, label %bb2.i22, !prof !30

bb2.i22:                                          ; preds = %bb3
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i18), !noalias !328
  store ptr %_8, ptr %e.i18, align 8, !noalias !328
  %14 = getelementptr inbounds nuw i8, ptr %e.i18, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %14, align 8, !noalias !328
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i18, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_cae42f98ba52838b2ed93bf93f07b323) #30
          to label %unreachable.i26 unwind label %cleanup.i23, !noalias !332

cleanup.i23:                                      ; preds = %bb2.i22
  %15 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<alloc::string::String>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i18) #32
          to label %cleanup.body unwind label %terminate.i24, !noalias !332

unreachable.i26:                                  ; preds = %bb2.i22
  unreachable

terminate.i24:                                    ; preds = %cleanup.i23
  %16 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !332
  unreachable

bb4:                                              ; preds = %bb3
  %_60 = getelementptr inbounds nuw i8, ptr %_57, i64 144
  %_60.val = load i64, ptr %_60, align 8
  %17 = getelementptr i8, ptr %_57, i64 152
  %18 = icmp eq i64 %_60.val, 0
  br i1 %18, label %bb5, label %bb2.i.i.i4.i.i49

bb2.i.i.i4.i.i49:                                 ; preds = %bb4
  %_60.val32 = load ptr, ptr %17, align 8, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_60.val32, i64 noundef %_60.val, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb5

bb5:                                              ; preds = %bb2.i.i.i4.i.i49, %bb4
  store i64 %initial.1, ptr %_60, align 8
  store ptr %5, ptr %17, align 8
  %_5.sroa.8.0._60.sroa_idx = getelementptr inbounds nuw i8, ptr %_57, i64 160
  store i64 %initial.1, ptr %_5.sroa.8.0._60.sroa_idx, align 8
  br i1 %_5.sroa.0.0.off0.i.i, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb5
  %19 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %19, 9223372036854775807
  %20 = icmp eq i64 %_7.i.i.i, 0
  br i1 %20, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_12.i monotonic, align 1
  br label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i

_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i: ; preds = %bb2.i.i.i, %bb6.i.i.i, %bb1.i.i.i, %bb5
  %21 = cmpxchg ptr %_8, ptr inttoptr (i64 1 to ptr), ptr null release monotonic, align 8
  %22 = extractvalue { ptr, i1 } %21, 1
  br i1 %22, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit, label %bb3.i.i, !prof !30

bb3.i.i:                                          ; preds = %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i
  %23 = extractvalue { ptr, i1 } %21, 0
; call <std::sys::sync::rwlock::queue::RwLock>::unlock_contended
  tail call void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock16unlock_contended(ptr noundef nonnull align 8 %_8, ptr noundef %23)
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit: ; preds = %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i, %bb3.i.i
  %_11 = getelementptr inbounds nuw i8, ptr %_57, i64 168
  %24 = load atomic ptr, ptr %_11 acquire, align 8, !noalias !333
  %25 = icmp eq ptr %24, null
  br i1 %25, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, !prof !5

bb7.i.i:                                          ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %26 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 %_11), !noalias !333
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i: ; preds = %bb7.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit
  %_0.sroa.0.0.i.i = phi ptr [ %26, %bb7.i.i ], [ %24, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console.exit ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !333
  %27 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !333
  %_6.i.i51 = and i64 %27, 9223372036854775807
  %28 = icmp eq i64 %_6.i.i51, 0
  br i1 %28, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCsC3JuwEIQwb_7console.exit, label %bb6.i.i52, !prof !30

bb6.i.i52:                                        ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %29 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33, !noalias !333
  %30 = xor i1 %29, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCsC3JuwEIQwb_7console.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCsC3JuwEIQwb_7console.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, %bb6.i.i52
  %_5.sroa.0.0.off0.i.i53 = phi i1 [ %30, %bb6.i.i52 ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_57, i64 176
  %31 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !333
  %.not132 = icmp eq i8 %31, 0
  %_0.sroa.3.0.i.i54 = zext i1 %_5.sroa.0.0.off0.i.i53 to i8
  br i1 %.not132, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit, label %bb2.i, !prof !30

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCsC3JuwEIQwb_7console.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !336
  store ptr %_11, ptr %e.i, align 8, !noalias !336
  %32 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i54, ptr %32, align 8, !noalias !336
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1bd3c013d602be6e35682a3072b8f24e) #30
          to label %unreachable.i unwind label %cleanup.i, !noalias !336

cleanup.i:                                        ; preds = %bb2.i
  %33 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<u8>>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #32
          to label %common.resume unwind label %terminate.i, !noalias !336

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %34 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !336
  unreachable

common.resume:                                    ; preds = %bb23, %cleanup.body, %bb2.i.i.i4.i.i, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %33, %cleanup.i ], [ %.pn.pn, %bb23 ], [ %eh.lpad-body27, %cleanup.body ], [ %eh.lpad-body27, %bb2.i.i.i4.i.i ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCsC3JuwEIQwb_7console.exit
; invoke <console::term::Term>::write_str
  %35 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %initial.0, i64 noundef %initial.1)
          to label %bb10 unwind label %cleanup2

bb23:                                             ; preds = %cleanup3.i.i, %bb2.i.i.i4.i.i.i, %bb26.i, %bb2.i.i.i4.i.i65, %cleanup2, %bb22
  %.pn.pn = phi { ptr, i32 } [ %.pn, %bb22 ], [ %36, %cleanup2 ], [ %61, %bb2.i.i.i4.i.i.i ], [ %61, %cleanup3.i.i ], [ %.pn16.i, %bb2.i.i.i4.i.i65 ], [ %.pn16.i, %bb26.i ]
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr nonnull %_11, i8 %_0.sroa.3.0.i.i54) #32
          to label %common.resume unwind label %terminate

cleanup2:                                         ; preds = %bb3.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
  %36 = landingpad { ptr, i32 }
          cleanup
  br label %bb23

bb10:                                             ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
  %.not = icmp eq ptr %35, null
  br i1 %.not, label %bb33, label %bb32

bb32:                                             ; preds = %bb10
  %37 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %35, ptr %37, align 8
  store i64 -9223372036854775808, ptr %_0, align 8
  br i1 %_5.sroa.0.0.off0.i.i53, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit, label %bb1.i.i.i58

bb1.i.i.i58:                                      ; preds = %bb32
  %38 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i59 = and i64 %38, 9223372036854775807
  %39 = icmp eq i64 %_7.i.i.i59, 0
  br i1 %39, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit, label %bb6.i.i.i60, !prof !30

bb6.i.i.i60:                                      ; preds = %bb1.i.i.i58
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i61 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i61, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit, label %bb2.i.i.i62

bb2.i.i.i62:                                      ; preds = %bb6.i.i.i60
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit: ; preds = %bb32, %bb1.i.i.i58, %bb6.i.i.i60, %bb2.i.i.i62
  %40 = load atomic ptr, ptr %_11 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %40)
  br label %bb20

bb33:                                             ; preds = %bb10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !339)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %initial.i)
  store ptr %initial.0, ptr %initial.i, align 8, !noalias !342
  %41 = getelementptr inbounds nuw i8, ptr %initial.i, i64 8
  store i64 %initial.1, ptr %41, align 8, !noalias !342
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %chars.i), !noalias !342
  %_55.i = getelementptr inbounds nuw i8, ptr %initial.0, i64 %initial.1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !345)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i.i), !noalias !348
  br i1 %_23.i.i, label %bb12.i.i, label %bb14.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb33
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %initial.0, i64 1
  %x.i.i.i.i = load i8, ptr %initial.0, align 1, !alias.scope !339, !noalias !349, !noundef !7
  %_6.i.i.i.i = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i, label %bb3.i.i.i.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp samesign ne i64 %initial.1, 1
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %initial.0, i64 2
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !alias.scope !339, !noalias !349, !noundef !7
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %42 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i.i.i.i, label %bb3.i.i63

bb3.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i = zext nneg i8 %x.i.i.i.i to i32
  br label %bb3.i.i63

bb6.i.i.i.i:                                      ; preds = %bb4.i.i.i.i
  %_6.i17.i.i.i.i = icmp samesign ne i64 %initial.1, 2
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %initial.0, i64 3
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !alias.scope !339, !noalias !349, !noundef !7
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %43 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i, label %bb3.i.i63

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i: ; preds = %bb6.i.i.i.i
  %_6.i24.i.i.i.i = icmp samesign ne i64 %initial.1, 3
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %_16.i26.i.i.i.i = getelementptr inbounds nuw i8, ptr %initial.0, i64 4
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !alias.scope !339, !noalias !349, !noundef !7
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %44 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  %.not.i.i = icmp eq i32 %44, 1114112
  br i1 %.not.i.i, label %bb12.i.i, label %bb3.i.i63

bb3.i.i63:                                        ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i, %bb6.i.i.i.i, %bb3.i.i.i.i, %bb4.i.i.i.i
  %spec.select.i15.i.i = phi i32 [ %44, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i ], [ %42, %bb4.i.i.i.i ], [ %43, %bb6.i.i.i.i ], [ %_7.i.i.i.i, %bb3.i.i.i.i ]
  %iterator.sroa.0.014.i.i = phi ptr [ %_16.i26.i.i.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i ], [ %_16.i12.i.i.i.i, %bb4.i.i.i.i ], [ %_16.i19.i.i.i.i, %bb6.i.i.i.i ], [ %_16.i.i.i.i.i, %bb3.i.i.i.i ]
  %45 = ptrtoint ptr %_55.i to i64
  %46 = ptrtoint ptr %iterator.sroa.0.014.i.i to i64
  %47 = sub nuw i64 %45, %46
  %d1.i.i.i = lshr i64 %47, 2
  %r.i.i.i = and i64 %47, 3
  %_8.not.i.i.i = icmp ne i64 %r.i.i.i, 0
  %48 = zext i1 %_8.not.i.i.i to i64
  %_4.sroa.0.0.i.i.i = add nuw nsw i64 %d1.i.i.i, %48
  %49 = tail call i64 @llvm.umax.i64(i64 %_4.sroa.0.0.i.i.i, i64 3)
  %_0.sroa.0.0.i.i.i = add nuw nsw i64 %49, 1
  %_27.0.i.i.i.i.i = shl i64 %49, 2
  %_27.1.i.i.i.i.i = icmp samesign ugt i64 %_4.sroa.0.0.i.i.i, 4611686018427387903
  %_40.i.i.i.i.i = icmp ugt i64 %_27.0.i.i.i.i.i, 9223372036854775803
  %or.cond.i.i.i = or i1 %_27.1.i.i.i.i.i, %_40.i.i.i.i.i
  br i1 %or.cond.i.i.i, label %bb3.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i, !prof !354

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i: ; preds = %bb3.i.i63
  %new_size2.i.i.i.i.i = add nuw nsw i64 %_27.0.i.i.i.i.i, 4
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !355
; call __rustc::__rust_alloc
  %50 = tail call noundef align 4 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #29, !noalias !355
  %51 = icmp eq ptr %50, null
  br i1 %51, label %bb3.i.i.i, label %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCsC3JuwEIQwb_7console.exit.i.i

bb3.i.i.i:                                        ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i, %bb3.i.i63
  %_4.sroa.4.0.ph.i.i.i = phi i64 [ 4, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i ], [ 0, %bb3.i.i63 ]
  %_4.sroa.10.0.ph.i.i.i = phi i64 [ %new_size2.i.i.i.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i ], [ undef, %bb3.i.i63 ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i.i, i64 %_4.sroa.10.0.ph.i.i.i) #30
          to label %.noexc66 unwind label %cleanup2

.noexc66:                                         ; preds = %bb3.i.i.i
  unreachable

_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCsC3JuwEIQwb_7console.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
  store i32 %spec.select.i15.i.i, ptr %50, align 4, !noalias !358
  store i64 %_0.sroa.0.0.i.i.i, ptr %vector.i.i, align 8, !noalias !348
  %vector1.sroa.4.0.vector.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i, i64 8
  store ptr %50, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !noalias !348
  %vector1.sroa.6.0.vector.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i, align 8, !noalias !348
  tail call void @llvm.experimental.noalias.scope.decl(metadata !359)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !362)
  %_6.i.i.not.i16.i.i.i.i = icmp eq ptr %iterator.sroa.0.014.i.i, %_55.i
  br i1 %_6.i.i.not.i16.i.i.i.i, label %bb5.i.i, label %bb14.i.i.i.i.i.i

bb14.i.i.i.i.i.i:                                 ; preds = %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCsC3JuwEIQwb_7console.exit.i.i, %bb8.i.i10.i.i
  %_23.i.i21.i.i = phi ptr [ %_23.i.i.i.i, %bb8.i.i10.i.i ], [ %50, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCsC3JuwEIQwb_7console.exit.i.i ]
  %len.i.i.i.i = phi i64 [ %new_len.i.i.i.i, %bb8.i.i10.i.i ], [ 1, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCsC3JuwEIQwb_7console.exit.i.i ]
  %iterator.sroa.0.017.i.i.i.i = phi ptr [ %iterator.sroa.0.110.i.i.i.i, %bb8.i.i10.i.i ], [ %iterator.sroa.0.014.i.i, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCsC3JuwEIQwb_7console.exit.i.i ]
  %_16.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iterator.sroa.0.017.i.i.i.i, i64 1
  %x.i.i.i.i.i.i = load i8, ptr %iterator.sroa.0.017.i.i.i.i, align 1, !alias.scope !339, !noalias !365, !noundef !7
  %_6.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_30.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i, %_55.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iterator.sroa.0.017.i.i.i.i, i64 2
  %y.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i, align 1, !alias.scope !339, !noalias !365, !noundef !7
  %_33.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i to i32
  %52 = or disjoint i32 %_33.i.i.i.i.i.i, %_34.i.i.i.i.i.i
  %_13.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i, label %bb3.i.i9.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_7.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i to i32
  br label %bb3.i.i9.i.i

bb6.i.i.i.i.i.i:                                  ; preds = %bb4.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i, %_55.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iterator.sroa.0.017.i.i.i.i, i64 3
  %z.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i, align 1, !alias.scope !339, !noalias !365, !noundef !7
  %_38.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i, %_39.i.i.i.i.i.i
  %_20.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 12
  %53 = or disjoint i32 %y_z.i.i.i.i.i.i, %_20.i.i.i.i.i.i
  %_21.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i, label %bb3.i.i9.i.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i: ; preds = %bb6.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i, %_55.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iterator.sroa.0.017.i.i.i.i, i64 4
  %w.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i, align 1, !alias.scope !339, !noalias !365, !noundef !7
  %_26.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i, %_44.i.i.i.i.i.i
  %54 = or disjoint i32 %_27.i.i.i.i.i.i, %_25.i.i.i.i.i.i
  %.not.i.i.i.i = icmp eq i32 %54, 1114112
  br i1 %.not.i.i.i.i, label %bb5.i.i, label %bb3.i.i9.i.i

bb3.i.i9.i.i:                                     ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i, %bb6.i.i.i.i.i.i, %bb3.i.i.i.i.i.i, %bb4.i.i.i.i.i.i
  %spec.select.i11.i.i.i.i = phi i32 [ %54, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i ], [ %52, %bb4.i.i.i.i.i.i ], [ %53, %bb6.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %iterator.sroa.0.110.i.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ], [ %_16.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %_19.i.i.i.i = icmp samesign ult i64 %len.i.i.i.i, 2305843009213693952
  tail call void @llvm.assume(i1 %_19.i.i.i.i)
  %self1.i.i.i.i = load i64, ptr %vector.i.i, align 8, !range !23, !alias.scope !370, !noalias !348, !noundef !7
  %_8.i.i.i.i = icmp eq i64 %len.i.i.i.i, %self1.i.i.i.i
  br i1 %_8.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeccE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i, label %bb8.i.i10.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeccE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i: ; preds = %bb3.i.i9.i.i
  %55 = ptrtoint ptr %iterator.sroa.0.110.i.i.i.i to i64
  %56 = sub nuw i64 %45, %55
  %d1.i.i.i.i.i = lshr i64 %56, 2
  %r.i.i.i.i.i = and i64 %56, 3
  %_8.not.i.i.i.i.i = icmp ne i64 %r.i.i.i.i.i, 0
  %57 = zext i1 %_8.not.i.i.i.i.i to i64
  %_4.sroa.0.0.i.i.i.i.i = add nuw nsw i64 %d1.i.i.i.i.i, 1
  %58 = add nuw nsw i64 %_4.sroa.0.0.i.i.i.i.i, %57
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i.i, i64 noundef %len.i.i.i.i, i64 noundef range(i64 1, 0) %58, i64 noundef 4, i64 noundef 4)
          to label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeccE7reserveCsC3JuwEIQwb_7console.exit.i.i.bb8.i.i10_crit_edge.i.i unwind label %cleanup3.i.i, !noalias !358

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeccE7reserveCsC3JuwEIQwb_7console.exit.i.i.bb8.i.i10_crit_edge.i.i: ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeccE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i
  %_23.i.i.pre.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !alias.scope !370, !noalias !348
  br label %bb8.i.i10.i.i

bb8.i.i10.i.i:                                    ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeccE7reserveCsC3JuwEIQwb_7console.exit.i.i.bb8.i.i10_crit_edge.i.i, %bb3.i.i9.i.i
  %_23.i.i.i.i = phi ptr [ %_23.i.i.pre.i.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeccE7reserveCsC3JuwEIQwb_7console.exit.i.i.bb8.i.i10_crit_edge.i.i ], [ %_23.i.i21.i.i, %bb3.i.i9.i.i ]
  %dst.i.i.i.i = getelementptr inbounds nuw i32, ptr %_23.i.i.i.i, i64 %len.i.i.i.i
  store i32 %spec.select.i11.i.i.i.i, ptr %dst.i.i.i.i, align 4, !noalias !371
  %new_len.i.i.i.i = add nuw nsw i64 %len.i.i.i.i, 1
  store i64 %new_len.i.i.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i, align 8, !alias.scope !370, !noalias !348
  %_6.i.i.not.i.i.i.i.i = icmp eq ptr %iterator.sroa.0.110.i.i.i.i, %_55.i
  br i1 %_6.i.i.not.i.i.i.i.i, label %bb5.i.i, label %bb14.i.i.i.i.i.i

bb12.i.i:                                         ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i, %bb33
  store i64 0, ptr %chars.i, align 8, !alias.scope !345, !noalias !342
  %59 = getelementptr inbounds nuw i8, ptr %chars.i, i64 8
  store ptr inttoptr (i64 4 to ptr), ptr %59, align 8, !alias.scope !345, !noalias !342
  %60 = getelementptr inbounds nuw i8, ptr %chars.i, i64 16
  store i64 0, ptr %60, align 8, !alias.scope !345, !noalias !342
  br label %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VeccEINtB2_18SpecFromIterNestedcNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsE9from_iterCsC3JuwEIQwb_7console.exit.i

cleanup3.i.i:                                     ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeccE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i
  %61 = landingpad { ptr, i32 }
          cleanup
  %vector.val.i.i = load i64, ptr %vector.i.i, align 8, !noalias !348
  %62 = icmp eq i64 %vector.val.i.i, 0
  br i1 %62, label %bb23, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup3.i.i
  %vector.val7.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !noalias !348, !nonnull !7, !noundef !7
  %alloc_size.i.i.i.i5.i.i.i = shl nuw i64 %vector.val.i.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %vector.val7.i.i, i64 noundef %alloc_size.i.i.i.i5.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #29, !noalias !358
  br label %bb23

bb5.i.i:                                          ; preds = %bb8.i.i10.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCsC3JuwEIQwb_7console.exit.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %chars.i, ptr noundef nonnull align 8 dereferenceable(24) %vector.i.i, i64 24, i1 false), !noalias !342
  br label %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VeccEINtB2_18SpecFromIterNestedcNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsE9from_iterCsC3JuwEIQwb_7console.exit.i

_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VeccEINtB2_18SpecFromIterNestedcNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsE9from_iterCsC3JuwEIQwb_7console.exit.i: ; preds = %bb5.i.i, %bb12.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i), !noalias !348
  %_60.sroa.5.0._8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 8
  %63 = getelementptr inbounds nuw i8, ptr %chars.i, i64 16
  %64 = getelementptr inbounds nuw i8, ptr %chars.i, i64 8
  %_23.i.i.i = getelementptr inbounds nuw i8, ptr %bytes_char.i, i64 1
  %_24.i.i.i = getelementptr inbounds nuw i8, ptr %bytes_char.i, i64 2
  %_25.i.i.i = getelementptr inbounds nuw i8, ptr %bytes_char.i, i64 3
  %_10.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 8
  %_5.sroa.5.0._6.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i73.i, i64 8
  %_5.sroa.8.0._6.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i73.i, i64 16
  br label %bb1.us.i

bb1.us.i:                                         ; preds = %bb1.us.i.backedge, %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VeccEINtB2_18SpecFromIterNestedcNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsE9from_iterCsC3JuwEIQwb_7console.exit.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_8.i), !noalias !342
; invoke console::unix_term::read_single_key
  invoke fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term15read_single_key(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_8.i, i1 noundef zeroext false)
          to label %bb2.us.i unwind label %cleanup.split.us.i, !noalias !372

bb2.us.i:                                         ; preds = %bb1.us.i
  %.pr.us.i = load i64, ptr %_8.i, align 8, !noalias !342
  %.pr.fr.us.i = freeze i64 %.pr.us.i
  %65 = icmp eq i64 %.pr.fr.us.i, -9223372036854775787
  %_61.i = load ptr, ptr %_60.sroa.5.0._8.sroa_idx.i, align 8, !noalias !342
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_8.i), !noalias !342
  br i1 %65, label %bb30.i, label %bb32.us.i

bb32.us.i:                                        ; preds = %bb2.us.i
  %66 = icmp ne i64 %.pr.fr.us.i, -9223372036854775807
  call void @llvm.assume(i1 %66)
  %67 = icmp slt i64 %.pr.fr.us.i, 0
  br i1 %67, label %68, label %bb17.us.i

68:                                               ; preds = %bb32.us.i
  %69 = and i64 %.pr.fr.us.i, 9223372036854775807
  switch i64 %69, label %bb1.us.i.backedge [
    i64 6, label %bb4.i
    i64 8, label %bb6.us.i
    i64 19, label %bb5.us.i
  ]

bb5.us.i:                                         ; preds = %68
  %70 = ptrtoint ptr %_61.i to i64
  %_6.sroa.9.8.extract.trunc.us.i = trunc i64 %70 to i32
  call void @llvm.experimental.noalias.scope.decl(metadata !373)
  %len.i.us.i = load i64, ptr %63, align 8, !alias.scope !373, !noalias !342, !noundef !7
  %self1.i.us.i = load i64, ptr %chars.i, align 8, !range !23, !alias.scope !373, !noalias !342, !noundef !7
  %_4.i.us.i = icmp eq i64 %len.i.us.i, %self1.i.us.i
  br i1 %_4.i.us.i, label %bb1.i41.us.i, label %bb33.us.i

bb1.i41.us.i:                                     ; preds = %bb5.us.i
; invoke <alloc::raw_vec::RawVec<char>>::grow_one
  invoke void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVeccE8grow_oneCsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %chars.i)
          to label %bb33.us.i unwind label %cleanup1.loopexit.split.us.i, !noalias !372

bb33.us.i:                                        ; preds = %bb1.i41.us.i, %bb5.us.i
  %_14.i.us.i = load ptr, ptr %64, align 8, !alias.scope !373, !noalias !342, !nonnull !7, !noundef !7
  %end.i.us.i = getelementptr inbounds nuw i32, ptr %_14.i.us.i, i64 %len.i.us.i
  store i32 %_6.sroa.9.8.extract.trunc.us.i, ptr %end.i.us.i, align 4, !noalias !376
  %71 = add i64 %len.i.us.i, 1
  store i64 %71, ptr %63, align 8, !alias.scope !373, !noalias !342
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %bytes_char.i), !noalias !342
  store i32 0, ptr %bytes_char.i, align 4, !noalias !342
  %_11.i.us.i = icmp samesign ult i32 %_6.sroa.9.8.extract.trunc.us.i, 128
  br i1 %_11.i.us.i, label %bb12.i.i110.us.i, label %bb5.i82.us.i

bb5.i82.us.i:                                     ; preds = %bb33.us.i
  %_12.i.us.i = icmp samesign ult i32 %_6.sroa.9.8.extract.trunc.us.i, 2048
  %72 = trunc i64 %70 to i8
  %_5.i.i.us.i = and i8 %72, 63
  %last1.i.i.us.i = or disjoint i8 %_5.i.i.us.i, -128
  %_10.i.i137.us.i = lshr i64 %70, 6
  %73 = trunc i64 %_10.i.i137.us.i to i8
  %_8.i.i.us.i = and i8 %73, 63
  %last2.i.i.us.i = or disjoint i8 %_8.i.i.us.i, -128
  %_14.i.i138.us.i = lshr i64 %70, 12
  %74 = trunc i64 %_14.i.i138.us.i to i8
  %_12.i.i.us.i = and i8 %74, 63
  %last3.i.i.us.i = or disjoint i8 %_12.i.i.us.i, -128
  %_18.i.i139.us.i = lshr i64 %70, 18
  %_16.i.i.us.i = trunc i64 %_18.i.i139.us.i to i8
  %last4.i.i.us.i = or disjoint i8 %_16.i.i.us.i, -16
  br i1 %_12.i.us.i, label %bb1.i.i108.us.i, label %bb2.i.i.us.i

bb2.i.i.us.i:                                     ; preds = %bb5.i82.us.i
  %_13.i.us.i = icmp samesign ult i32 %_6.sroa.9.8.extract.trunc.us.i, 65536
  br i1 %_13.i.us.i, label %bb3.i.i105.us.i, label %bb4.i.i100.us.i

bb4.i.i100.us.i:                                  ; preds = %bb2.i.i.us.i
  store i8 %last4.i.i.us.i, ptr %bytes_char.i, align 4, !alias.scope !377, !noalias !342
  store i8 %last3.i.i.us.i, ptr %_23.i.i.i, align 1, !alias.scope !377, !noalias !342
  store i8 %last2.i.i.us.i, ptr %_24.i.i.i, align 2, !alias.scope !377, !noalias !342
  store i8 %last1.i.i.us.i, ptr %_25.i.i.i, align 1, !alias.scope !377, !noalias !342
  br label %bb42.us.i

bb3.i.i105.us.i:                                  ; preds = %bb2.i.i.us.i
  %75 = or disjoint i8 %74, -32
  store i8 %75, ptr %bytes_char.i, align 4, !alias.scope !377, !noalias !342
  store i8 %last2.i.i.us.i, ptr %_23.i.i.i, align 1, !alias.scope !377, !noalias !342
  store i8 %last1.i.i.us.i, ptr %_24.i.i.i, align 2, !alias.scope !377, !noalias !342
  br label %bb42.us.i

bb1.i.i108.us.i:                                  ; preds = %bb5.i82.us.i
  %76 = or disjoint i8 %73, -64
  store i8 %76, ptr %bytes_char.i, align 4, !alias.scope !377, !noalias !342
  store i8 %last1.i.i.us.i, ptr %_23.i.i.i, align 1, !alias.scope !377, !noalias !342
  br label %bb42.us.i

bb12.i.i110.us.i:                                 ; preds = %bb33.us.i
  %77 = trunc i64 %70 to i8
  store i8 %77, ptr %bytes_char.i, align 4, !alias.scope !377, !noalias !342
  br label %bb42.us.i

bb42.us.i:                                        ; preds = %bb12.i.i110.us.i, %bb1.i.i108.us.i, %bb3.i.i105.us.i, %bb4.i.i100.us.i
  %len.sroa.0.04.i104.us.i = phi i64 [ 1, %bb12.i.i110.us.i ], [ 2, %bb1.i.i108.us.i ], [ 3, %bb3.i.i105.us.i ], [ 4, %bb4.i.i100.us.i ]
; invoke <console::term::Term>::write_str
  %78 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %bytes_char.i, i64 noundef %len.sroa.0.04.i104.us.i)
          to label %bb13.us.i unwind label %cleanup1.loopexit.split.us.i, !noalias !380

bb13.us.i:                                        ; preds = %bb42.us.i
  %.not.us.i = icmp eq ptr %78, null
  br i1 %.not.us.i, label %bb44.us.i, label %bb20.i

bb44.us.i:                                        ; preds = %bb13.us.i
; invoke <console::term::Term>::flush
  %79 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term5flush(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self)
          to label %bb14.us.i unwind label %cleanup1.loopexit.split.us.i, !noalias !380

bb14.us.i:                                        ; preds = %bb44.us.i
  %.not11.us.i = icmp eq ptr %79, null
  br i1 %.not11.us.i, label %bb46.us.i, label %bb20.i

bb46.us.i:                                        ; preds = %bb14.us.i
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %bytes_char.i), !noalias !342
  br label %bb1.us.i.backedge

bb6.us.i:                                         ; preds = %68
  %_13.us.i = load i64, ptr %63, align 8, !noalias !342, !noundef !7
  %_71.us.i = icmp ult i64 %_13.us.i, 2305843009213693952
  call void @llvm.assume(i1 %_71.us.i)
  %_12.us.i = icmp ult i64 %initial.1, %_13.us.i
  br i1 %_12.us.i, label %bb35.us.i, label %bb11.us.i

bb35.us.i:                                        ; preds = %bb6.us.i
  %80 = add nsw i64 %_13.us.i, -1
  store i64 %80, ptr %63, align 8, !noalias !342
  %_80.us.i = load i64, ptr %chars.i, align 8, !range !23, !noalias !342, !noundef !7
  %_73.us.i = icmp samesign ult i64 %80, %_80.us.i
  call void @llvm.assume(i1 %_73.us.i)
  %_81.us.i = load ptr, ptr %64, align 8, !noalias !342, !nonnull !7, !noundef !7
  %_77.us.i = getelementptr inbounds nuw i32, ptr %_81.us.i, i64 %80
  %_76.us.i = load i32, ptr %_77.us.i, align 4, !range !381, !noalias !372, !noundef !7
  %_3.i.us.i = icmp samesign ult i32 %_76.us.i, 127
  br i1 %_3.i.us.i, label %bb1.i70.us.i, label %bb4.i.us.i

bb4.i.us.i:                                       ; preds = %bb35.us.i
  %_5.i.us.i = icmp samesign ugt i32 %_76.us.i, 159
  br i1 %_5.i.us.i, label %bb5.i69.us.i, label %bb8.thread.us.i

bb8.thread.us.i:                                  ; preds = %bb4.i.us.i
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %n.i.i), !noalias !342
  br label %bb36.thread.us.i

bb5.i69.us.i:                                     ; preds = %bb4.i.us.i
; invoke unicode_width::tables::lookup_width
  %81 = invoke fastcc { i8, i16 } @_RNvNtCsDJOD2kcAir_13unicode_width6tables12lookup_width(i32 noundef range(i32 0, 1114112) %_76.us.i) #35
          to label %.noexc72.us.i unwind label %cleanup1.loopexit.split.us.i, !noalias !372

.noexc72.us.i:                                    ; preds = %bb5.i69.us.i
  %_8.0.i.us.i = extractvalue { i8, i16 } %81, 0
  %_6.i.us.i = zext nneg i8 %_8.0.i.us.i to i64
  br label %bb8.us.i

bb1.i70.us.i:                                     ; preds = %bb35.us.i
  %_4.i71.us.i = icmp samesign ugt i32 %_76.us.i, 31
  %spec.select.i.us.i = zext i1 %_4.i71.us.i to i64
  br label %bb8.us.i

bb8.us.i:                                         ; preds = %bb1.i70.us.i, %.noexc72.us.i
  %_0.sroa.0.0.i.us.i = phi i64 [ %_6.i.us.i, %.noexc72.us.i ], [ %spec.select.i.us.i, %bb1.i70.us.i ]
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %n.i.i), !noalias !342
  store i64 %_0.sroa.0.0.i.us.i, ptr %n.i.i, align 8, !noalias !382
  %_3.not.i.us.i = icmp eq i64 %_0.sroa.0.0.i.us.i, 0
  br i1 %_3.not.i.us.i, label %bb36.thread.us.i, label %bb1.i74.us.i

bb1.i74.us.i:                                     ; preds = %bb8.us.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i73.i), !noalias !382
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i.i), !noalias !382
  store ptr %n.i.i, ptr %args.i.i, align 8, !noalias !382
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx.i.i, align 8, !noalias !382
; invoke alloc::fmt::format::format_inner
  invoke void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6.i73.i, ptr noundef nonnull @alloc_80b5bcfc6b71c11e412d8b752ae375dc, ptr noundef nonnull %args.i.i)
          to label %.noexc80.us.i unwind label %cleanup1.loopexit.split.us.i, !noalias !372

.noexc80.us.i:                                    ; preds = %bb1.i74.us.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i.i), !noalias !382
  %_5.sroa.0.0.copyload.i.us.i = load i64, ptr %_6.i73.i, align 8, !noalias !382
  %_5.sroa.5.0.copyload.i.us.i = load ptr, ptr %_5.sroa.5.0._6.sroa_idx.i.i, align 8, !noalias !382, !nonnull !7, !noundef !7
  %_5.sroa.8.0.copyload.i.us.i = load i64, ptr %_5.sroa.8.0._6.sroa_idx.i.i, align 8, !noalias !382
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i73.i), !noalias !382
; invoke <console::term::Term>::write_str
  %82 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload.i.us.i, i64 noundef %_5.sroa.8.0.copyload.i.us.i)
          to label %bb2.i77.us.i unwind label %cleanup.i75.split.us.i, !noalias !380

bb2.i77.us.i:                                     ; preds = %.noexc80.us.i
  %83 = icmp eq i64 %_5.sroa.0.0.copyload.i.us.i, 0
  br i1 %83, label %bb36.us.i, label %bb2.i.i.i4.i.i5.i.us.i

bb2.i.i.i4.i.i5.i.us.i:                           ; preds = %bb2.i77.us.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload.i.us.i, i64 noundef %_5.sroa.0.0.copyload.i.us.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !385
  br label %bb36.us.i

bb36.us.i:                                        ; preds = %bb2.i.i.i4.i.i5.i.us.i, %bb2.i77.us.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %n.i.i), !noalias !342
  %.not12.us.i = icmp eq ptr %82, null
  br i1 %.not12.us.i, label %bb11.us.i, label %bb30.i

bb36.thread.us.i:                                 ; preds = %bb8.us.i, %bb8.thread.us.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %n.i.i), !noalias !342
  br label %bb11.us.i

bb11.us.i:                                        ; preds = %bb36.thread.us.i, %bb36.us.i, %bb6.us.i
; invoke <console::term::Term>::flush
  %84 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term5flush(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self)
          to label %bb12.us.i unwind label %cleanup1.loopexit.split.us.i, !noalias !380

bb12.us.i:                                        ; preds = %bb11.us.i
  %.not13.us.i = icmp eq ptr %84, null
  br i1 %.not13.us.i, label %bb1.us.i.backedge, label %bb30.i

bb17.us.i:                                        ; preds = %bb32.us.i
  %or.cond.i.us.i = icmp eq i64 %.pr.fr.us.i, 0
  br i1 %or.cond.i.us.i, label %bb1.us.i.backedge, label %bb2.i.i.i4.i.i37.us.i

bb2.i.i.i4.i.i37.us.i:                            ; preds = %bb17.us.i
  %alloc_size.i.i.i.i5.i.i38.us.i = shl nuw i64 %.pr.fr.us.i, 2
  %85 = icmp ne ptr %_61.i, null
  call void @llvm.assume(i1 %85)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_61.i, i64 noundef %alloc_size.i.i.i.i5.i.i38.us.i, i64 noundef range(i64 1, -9223372036854775807) 4) #29, !noalias !372
  br label %bb1.us.i.backedge

bb1.us.i.backedge:                                ; preds = %bb2.i.i.i4.i.i37.us.i, %bb17.us.i, %bb12.us.i, %bb46.us.i, %68
  br label %bb1.us.i

cleanup.split.us.i:                               ; preds = %bb1.us.i
  %86 = landingpad { ptr, i32 }
          cleanup
  br label %bb26.i

cleanup1.loopexit.split.us.i:                     ; preds = %bb11.us.i, %bb1.i74.us.i, %bb5.i69.us.i, %bb44.us.i, %bb42.us.i, %bb1.i41.us.i
  %lpad.loopexit.us.i = landingpad { ptr, i32 }
          cleanup
  br label %bb26.i

cleanup.i75.split.us.i:                           ; preds = %.noexc80.us.i
  %87 = landingpad { ptr, i32 }
          cleanup
  %88 = icmp eq i64 %_5.sroa.0.0.copyload.i.us.i, 0
  br i1 %88, label %bb26.i, label %bb2.i.i.i4.i.i.i76.i

bb26.i:                                           ; preds = %bb2.i.i.i4.i.i.i76.i, %bb2.i.i.i4.i.i.i.i, %cleanup.i.i, %bb2.i.i.i4.i.i48.i, %cleanup2.i, %cleanup1.loopexit.split-lp.i, %cleanup.i75.split.us.i, %cleanup1.loopexit.split.us.i, %cleanup.split.us.i
  %.pn16.i = phi { ptr, i32 } [ %86, %cleanup.split.us.i ], [ %lpad.phi.i.i, %bb2.i.i.i4.i.i.i.i ], [ %lpad.phi.i.i, %cleanup.i.i ], [ %91, %cleanup2.i ], [ %91, %bb2.i.i.i4.i.i48.i ], [ %87, %bb2.i.i.i4.i.i.i76.i ], [ %87, %cleanup.i75.split.us.i ], [ %lpad.loopexit.us.i, %cleanup1.loopexit.split.us.i ], [ %lpad.loopexit.split-lp.i, %cleanup1.loopexit.split-lp.i ]
  %chars.val.i = load i64, ptr %chars.i, align 8, !noalias !342
  %89 = icmp eq i64 %chars.val.i, 0
  br i1 %89, label %bb23, label %bb2.i.i.i4.i.i65

bb2.i.i.i4.i.i65:                                 ; preds = %bb26.i
  %chars.val24.i = load ptr, ptr %64, align 8, !noalias !342, !nonnull !7, !noundef !7
  %alloc_size.i.i.i.i5.i.i = shl nuw i64 %chars.val.i, 2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %chars.val24.i, i64 noundef %alloc_size.i.i.i.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #29, !noalias !372
  br label %bb23

bb4.i:                                            ; preds = %68
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_41.i), !noalias !342
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i), !noalias !342
  store ptr %initial.i, ptr %args.i, align 8, !noalias !342
  %_45.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsC3JuwEIQwb_7console, ptr %_45.sroa.4.0..sroa_idx.i, align 8, !noalias !342
; invoke alloc::fmt::format::format_inner
  invoke void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_41.i, ptr noundef nonnull @alloc_afa71497e4832d4e825454760abdae23, ptr noundef nonnull %args.i)
          to label %bb47.i unwind label %cleanup1.loopexit.split-lp.i, !noalias !372

cleanup1.loopexit.split-lp.i:                     ; preds = %bb4.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %bb26.i

bb47.i:                                           ; preds = %bb4.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i), !noalias !342
  %_40.sroa.0.0.copyload.i = load i64, ptr %_41.i, align 8, !noalias !342
  %_40.sroa.7.0._41.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_41.i, i64 8
  %_40.sroa.7.0.copyload.i = load ptr, ptr %_40.sroa.7.0._41.sroa_idx.i, align 8, !noalias !342, !nonnull !7, !noundef !7
  %_40.sroa.11.0._41.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_41.i, i64 16
  %_40.sroa.11.0.copyload.i = load i64, ptr %_40.sroa.11.0._41.sroa_idx.i, align 8, !noalias !342
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_41.i), !noalias !342
; invoke <console::term::Term>::write_through
  %90 = invoke fastcc noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term13write_through(ptr nonnull %_57, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_40.sroa.7.0.copyload.i, i64 noundef %_40.sroa.11.0.copyload.i)
          to label %bb15.i unwind label %cleanup2.i, !noalias !372

cleanup2.i:                                       ; preds = %bb47.i
  %91 = landingpad { ptr, i32 }
          cleanup
  %92 = icmp eq i64 %_40.sroa.0.0.copyload.i, 0
  br i1 %92, label %bb26.i, label %bb2.i.i.i4.i.i48.i

bb2.i.i.i4.i.i48.i:                               ; preds = %cleanup2.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_40.sroa.7.0.copyload.i, i64 noundef %_40.sroa.0.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !372
  br label %bb26.i

bb15.i:                                           ; preds = %bb47.i
  %.not14.i = icmp eq ptr %90, null
  %93 = icmp eq i64 %_40.sroa.0.0.copyload.i, 0
  br i1 %.not14.i, label %bb52.i, label %bb51.i

bb51.i:                                           ; preds = %bb15.i
  br i1 %93, label %bb30.i, label %bb2.i.i.i4.i.i49.i

bb2.i.i.i4.i.i49.i:                               ; preds = %bb51.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_40.sroa.7.0.copyload.i, i64 noundef %_40.sroa.0.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !372
  br label %bb30.i

bb52.i:                                           ; preds = %bb15.i
  br i1 %93, label %bb29.i, label %bb2.i.i.i4.i.i51.i

bb2.i.i.i4.i.i51.i:                               ; preds = %bb52.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_40.sroa.7.0.copyload.i, i64 noundef %_40.sroa.0.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !372
  br label %bb29.i

bb29.i:                                           ; preds = %bb2.i.i.i4.i.i51.i, %bb52.i
  %_133.i = load ptr, ptr %64, align 8, !noalias !342, !nonnull !7, !noundef !7
  %_132.i = load i64, ptr %63, align 8, !noalias !342, !noundef !7
  %_126.i = getelementptr inbounds nuw i32, ptr %_133.i, i64 %_132.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i.i), !noalias !386
  store i64 0, ptr %buf.i.i, align 8, !noalias !386
  %_5.sroa.4.0.buf.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %buf.i.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_5.sroa.4.0.buf.sroa_idx.i.i, align 8, !noalias !386
  %_5.sroa.5.0.buf.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %buf.i.i, i64 16
  store i64 0, ptr %_5.sroa.5.0.buf.sroa_idx.i.i, align 8, !noalias !386
  call void @llvm.experimental.noalias.scope.decl(metadata !390)
  call void @llvm.experimental.noalias.scope.decl(metadata !393)
  %94 = ptrtoint ptr %_126.i to i64
  %_7.i.i.i.not.not.i.i = icmp ugt i64 %_132.i, %initial.1
  br i1 %_7.i.i.i.not.not.i.i, label %bb1.i.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i, !prof !5

bb1.i.i.i.i.i:                                    ; preds = %bb29.i
  %95 = sub nuw nsw i64 %_132.i, %initial.1
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i.i, i64 noundef 0, i64 noundef %95, i64 noundef 1, i64 noundef 1)
          to label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i unwind label %cleanup.loopexit.split-lp.i.i, !noalias !396

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i: ; preds = %bb1.i.i.i.i.i, %bb29.i
  call void @llvm.experimental.noalias.scope.decl(metadata !397)
  call void @llvm.experimental.noalias.scope.decl(metadata !400)
  call void @llvm.experimental.noalias.scope.decl(metadata !403)
  br i1 %_23.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.bb8.i.i.i.i.i.i.i_crit_edge, label %bb1.i.i.i.i.i.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.bb8.i.i.i.i.i.i.i_crit_edge: ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i
  %ret.sroa.16.0.copyload116.pre134.pre = load i64, ptr %_5.sroa.5.0.buf.sroa_idx.i.i, align 8, !noalias !406
  br label %bb8.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i
  %_10.i.i.i.i.i.i.i = add i64 %initial.1, -1
  %_3.not.i.not.i.i.i.i.i.i.i = icmp ult i64 %_10.i.i.i.i.i.i.i, %_132.i
  %_33.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i32, ptr %_133.i, i64 %_10.i.i.i.i.i.i.i
  %_45.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_33.i.i.i.i.i.i.i.i, i64 4
  %ret.sroa.16.0.copyload116.pre134.pre136 = load i64, ptr %_5.sroa.5.0.buf.sroa_idx.i.i, align 8, !noalias !406
  br i1 %_3.not.i.not.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i

bb8.i.i.i.i.i.i.i:                                ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.bb8.i.i.i.i.i.i.i_crit_edge, %bb1.i.i.i.i.i.i.i
  %ret.sroa.16.0.copyload116.pre134 = phi i64 [ %ret.sroa.16.0.copyload116.pre134.pre136, %bb1.i.i.i.i.i.i.i ], [ %ret.sroa.16.0.copyload116.pre134.pre, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.bb8.i.i.i.i.i.i.i_crit_edge ]
  %_12.0.i.i.i.i.i.i.i = phi ptr [ %_45.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i ], [ %_133.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.bb8.i.i.i.i.i.i.i_crit_edge ]
  call void @llvm.experimental.noalias.scope.decl(metadata !407)
  %96 = icmp eq ptr %_12.0.i.i.i.i.i.i.i, %_126.i
  br i1 %96, label %bb53.i, label %bb5.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i:                              ; preds = %bb8.i.i.i.i.i.i.i
  %97 = ptrtoint ptr %_12.0.i.i.i.i.i.i.i to i64
  %98 = sub nuw i64 %94, %97
  %99 = lshr exact i64 %98, 2
  br label %bb9.i.i.i.i.i.i.i.i

bb9.i.i.i.i.i.i.i.i:                              ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i
  %len.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ %ret.sroa.16.0.copyload116.pre134, %bb5.i.i.i.i.i.i.i.i ], [ %new_len.i.i.i.i.i.i.i.i.i.i.i.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i ]
  %i.sroa.0.0.i.i.i.i.i.i.i.i = phi i64 [ 0, %bb5.i.i.i.i.i.i.i.i ], [ %_27.i.i.i.i.i.i.i.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i ]
  %_46.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i32, ptr %_12.0.i.i.i.i.i.i.i, i64 %i.sroa.0.0.i.i.i.i.i.i.i.i
  %_46.val.i.i.i.i.i.i.i.i = load i32, ptr %_46.i.i.i.i.i.i.i.i, align 4, !range !381, !alias.scope !410, !noalias !413, !noundef !7
  call void @llvm.experimental.noalias.scope.decl(metadata !419)
  %_16.i.i.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_46.val.i.i.i.i.i.i.i.i, 128
  br i1 %_16.i.i.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i.i.i.i:                      ; preds = %bb9.i.i.i.i.i.i.i.i
  %_17.i.i.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_46.val.i.i.i.i.i.i.i.i, 2048
  br i1 %_17.i.i.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i.i.i:                      ; preds = %bb3.i.i.i.i.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_46.val.i.i.i.i.i.i.i.i, 65536
  %..i.i.i.i.i.i.i.i.i.i.i.i = select i1 %_18.i.i.i.i.i.i.i.i.i.i.i.i, i64 3, i64 4
  br label %bb2.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i.i.i:                      ; preds = %bb4.i.i.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i.i
  %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ 1, %bb9.i.i.i.i.i.i.i.i ], [ %..i.i.i.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i.i.i ], [ 2, %bb3.i.i.i.i.i.i.i.i.i.i.i.i ]
  %self2.i.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %buf.i.i, align 8, !range !23, !alias.scope !422, !noalias !425, !noundef !7
  %_9.i.i.i.i.i.i.i.i.i.i.i.i.i = sub nsw i64 %self2.i.i.i.i.i.i.i.i.i.i.i.i.i, %len.i.i.i.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp ugt i64 %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i.i, %_9.i.i.i.i.i.i.i.i.i.i.i.i.i
  br i1 %_7.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i.i.i, !prof !5

bb1.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i.i, i64 noundef %len.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef 1, i64 noundef 1)
          to label %.noexc2.i.i unwind label %cleanup.loopexit.i.i, !noalias !396

.noexc2.i.i:                                      ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i
  %count.pre.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %_5.sroa.5.0.buf.sroa_idx.i.i, align 8, !alias.scope !426, !noalias !425
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i.i.i: ; preds = %.noexc2.i.i, %bb2.i.i.i.i.i.i.i.i.i.i.i.i
  %count.i.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ %len.i.i.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i.i.i ], [ %count.pre.i.i.i.i.i.i.i.i.i.i.i.i, %.noexc2.i.i ]
  %_20.i.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i.i, align 8, !alias.scope !426, !noalias !425, !nonnull !7, !noundef !7
  %_21.i.i.i.i.i.i.i.i.i.i.i.i = icmp sgt i64 %count.i.i.i.i.i.i.i.i.i.i.i.i, -1
  call void @llvm.assume(i1 %_21.i.i.i.i.i.i.i.i.i.i.i.i)
  %_8.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_20.i.i.i.i.i.i.i.i.i.i.i.i, i64 %count.i.i.i.i.i.i.i.i.i.i.i.i
  br i1 %_16.i.i.i.i.i.i.i.i.i.i.i.i, label %bb12.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i.i.i.i.i.i

bb7.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i.i.i
  %_27.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_46.val.i.i.i.i.i.i.i.i, 2048
  %100 = trunc i32 %_46.val.i.i.i.i.i.i.i.i to i8
  %_5.i.i.i.i.i.i.i.i.i.i.i.i.i = and i8 %100, 63
  %last1.i.i.i.i.i.i.i.i.i.i.i.i.i = or disjoint i8 %_5.i.i.i.i.i.i.i.i.i.i.i.i.i, -128
  %_10.i.i.i.i.i.i.i.i.i.i.i.i.i = lshr i32 %_46.val.i.i.i.i.i.i.i.i, 6
  %101 = trunc i32 %_10.i.i.i.i.i.i.i.i.i.i.i.i.i to i8
  %_8.i.i.i.i.i.i.i.i.i.i.i.i.i = and i8 %101, 63
  %last2.i.i.i.i.i.i.i.i.i.i.i.i.i = or disjoint i8 %_8.i.i.i.i.i.i.i.i.i.i.i.i.i, -128
  %_14.i.i.i.i.i.i.i.i.i.i.i.i.i = lshr i32 %_46.val.i.i.i.i.i.i.i.i, 12
  %102 = trunc i32 %_14.i.i.i.i.i.i.i.i.i.i.i.i.i to i8
  %_12.i.i.i.i.i.i.i.i.i.i.i.i.i = and i8 %102, 63
  %last3.i.i.i.i.i.i.i.i.i.i.i.i.i = or disjoint i8 %_12.i.i.i.i.i.i.i.i.i.i.i.i.i, -128
  %_18.i.i.i.i.i.i.i.i.i.i.i.i.i = lshr i32 %_46.val.i.i.i.i.i.i.i.i, 18
  %_16.i.i.i.i.i.i.i.i.i.i.i.i.i = trunc nuw nsw i32 %_18.i.i.i.i.i.i.i.i.i.i.i.i.i to i8
  %last4.i.i.i.i.i.i.i.i.i.i.i.i.i = or disjoint i8 %_16.i.i.i.i.i.i.i.i.i.i.i.i.i, -16
  br i1 %_27.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i

bb12.i.i.i.i.i.i.i.i.i.i.i.i.i:                   ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i.i.i.i.i
  %103 = trunc nuw nsw i32 %_46.val.i.i.i.i.i.i.i.i to i8
  store i8 %103, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i

bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i:                   ; preds = %bb7.i.i.i.i.i.i.i.i.i.i.i.i.i
  %104 = or disjoint i8 %101, -64
  store i8 %104, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  %_20.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, i64 1
  store i8 %last1.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_20.i.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb7.i.i.i.i.i.i.i.i.i.i.i.i.i
  %_28.i.i.i.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_46.val.i.i.i.i.i.i.i.i, 65536
  br i1 %_28.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i
  %105 = or disjoint i8 %102, -32
  store i8 %105, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  %_21.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, i64 1
  store i8 %last2.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_21.i.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  %_22.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, i64 2
  store i8 %last1.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_22.i.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i
  store i8 %last4.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  %_23.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, i64 1
  store i8 %last3.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_23.i.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  %_24.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, i64 2
  store i8 %last2.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_24.i.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  %_25.i.i.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i.i.i.i, i64 3
  store i8 %last1.i.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_25.i.i.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !427
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i

_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb1.i2.i.i.i.i.i.i.i.i.i.i.i.i, %bb12.i.i.i.i.i.i.i.i.i.i.i.i.i
  %new_len.i.i.i.i.i.i.i.i.i.i.i.i = add nuw i64 %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i.i.i.i, %len.i.i.i.i.i.i.i.i.i.i.i.i
  store i64 %new_len.i.i.i.i.i.i.i.i.i.i.i.i, ptr %_5.sroa.5.0.buf.sroa_idx.i.i, align 8, !alias.scope !426, !noalias !425
  %_27.i.i.i.i.i.i.i.i = add nuw i64 %i.sroa.0.0.i.i.i.i.i.i.i.i, 1
  %_28.i.i.i.i.i.i.i.i = icmp eq i64 %_27.i.i.i.i.i.i.i.i, %99
  br i1 %_28.i.i.i.i.i.i.i.i, label %bb53.i, label %bb9.i.i.i.i.i.i.i.i

cleanup.loopexit.i.i:                             ; preds = %bb1.i.i.i.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit.i.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i.i

cleanup.loopexit.split-lp.i.i:                    ; preds = %bb1.i.i.i.i.i
  %lpad.loopexit.split-lp.i.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i.i

cleanup.i.i:                                      ; preds = %cleanup.loopexit.split-lp.i.i, %cleanup.loopexit.i.i
  %lpad.phi.i.i = phi { ptr, i32 } [ %lpad.loopexit.i.i, %cleanup.loopexit.i.i ], [ %lpad.loopexit.split-lp.i.i, %cleanup.loopexit.split-lp.i.i ]
  %buf.val.i.i = load i64, ptr %buf.i.i, align 8, !noalias !386
  %106 = icmp eq i64 %buf.val.i.i, 0
  br i1 %106, label %bb26.i, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %cleanup.i.i
  %buf.val1.i.i = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i.i, align 8, !noalias !386, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %buf.val1.i.i, i64 noundef %buf.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !396
  br label %bb26.i

bb53.i:                                           ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i
  %ret.sroa.16.0.copyload116 = phi i64 [ %ret.sroa.16.0.copyload116.pre134, %bb8.i.i.i.i.i.i.i ], [ %ret.sroa.16.0.copyload116.pre134.pre136, %bb1.i.i.i.i.i.i.i ], [ %new_len.i.i.i.i.i.i.i.i.i.i.i.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRccuNvYcNtNtBa_5clone5Clone5cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2u_6StringINtNtB1C_7collect6ExtendcE6extendINtNtB6_6cloned6ClonedINtNtB6_4skip4SkipINtNtNtBa_5slice4iter4ItercEEEE0E0E0CsC3JuwEIQwb_7console.exit.i.i.i.i.i.i.i.i ]
  %ret.sroa.0.0.copyload114 = load i64, ptr %buf.i.i, align 8, !noalias !406
  %ret.sroa.10.0.copyload115 = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i.i, align 8, !noalias !406
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i.i), !noalias !386
  %chars.val27.i = load i64, ptr %chars.i, align 8, !noalias !342
  %107 = icmp eq i64 %chars.val27.i, 0
  br i1 %107, label %bb12, label %bb23.sink.split.i

bb23.sink.split.i:                                ; preds = %bb2.i.i.i4.i113.i, %bb53.i
  %ret.sroa.0.1 = phi i64 [ -9223372036854775808, %bb2.i.i.i4.i113.i ], [ %ret.sroa.0.0.copyload114, %bb53.i ]
  %ret.sroa.10.1 = phi ptr [ %ret.sroa.10.0, %bb2.i.i.i4.i113.i ], [ %ret.sroa.10.0.copyload115, %bb53.i ]
  %ret.sroa.16.0 = phi i64 [ undef, %bb2.i.i.i4.i113.i ], [ %ret.sroa.16.0.copyload116, %bb53.i ]
  %chars.val27.sink.i = phi i64 [ %chars.val25.i, %bb2.i.i.i4.i113.i ], [ %chars.val27.i, %bb53.i ]
  %_133.sink.i = phi ptr [ %chars.val26.i, %bb2.i.i.i4.i113.i ], [ %_133.i, %bb53.i ]
  %alloc_size.i.i.i.i5.i61.i = shl nuw i64 %chars.val27.sink.i, 2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_133.sink.i, i64 noundef %alloc_size.i.i.i.i5.i61.i, i64 noundef range(i64 1, -9223372036854775807) 4) #29, !noalias !372
  br label %bb12

bb2.i.i.i4.i.i.i76.i:                             ; preds = %cleanup.i75.split.us.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload.i.us.i, i64 noundef %_5.sroa.0.0.copyload.i.us.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !385
  br label %bb26.i

bb20.i:                                           ; preds = %bb14.us.i, %bb13.us.i
  %.lcssa.sink.i = phi ptr [ %78, %bb13.us.i ], [ %79, %bb14.us.i ]
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %bytes_char.i), !noalias !342
  br label %bb30.i

bb30.i:                                           ; preds = %bb12.us.i, %bb36.us.i, %bb2.us.i, %bb20.i, %bb2.i.i.i4.i.i49.i, %bb51.i
  %ret.sroa.10.0 = phi ptr [ %90, %bb51.i ], [ %90, %bb2.i.i.i4.i.i49.i ], [ %.lcssa.sink.i, %bb20.i ], [ %84, %bb12.us.i ], [ %82, %bb36.us.i ], [ %_61.i, %bb2.us.i ]
  %chars.val25.i = load i64, ptr %chars.i, align 8, !noalias !342
  %108 = icmp eq i64 %chars.val25.i, 0
  br i1 %108, label %bb12, label %bb2.i.i.i4.i113.i

bb2.i.i.i4.i113.i:                                ; preds = %bb30.i
  %chars.val26.i = load ptr, ptr %64, align 8, !noalias !342, !nonnull !7, !noundef !7
  br label %bb23.sink.split.i

bb12:                                             ; preds = %bb30.i, %bb23.sink.split.i, %bb53.i
  %ret.sroa.0.2 = phi i64 [ -9223372036854775808, %bb30.i ], [ %ret.sroa.0.1, %bb23.sink.split.i ], [ %ret.sroa.0.0.copyload114, %bb53.i ]
  %ret.sroa.10.2 = phi ptr [ %ret.sroa.10.0, %bb30.i ], [ %ret.sroa.10.1, %bb23.sink.split.i ], [ %ret.sroa.10.0.copyload115, %bb53.i ]
  %ret.sroa.16.1 = phi i64 [ undef, %bb30.i ], [ %ret.sroa.16.0, %bb23.sink.split.i ], [ %ret.sroa.16.0.copyload116, %bb53.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %chars.i), !noalias !342
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %initial.i)
  %109 = atomicrmw or ptr %_8, i64 1 acquire, align 8, !noalias !428
  %_4.i68 = and i64 %109, 1
  %110 = icmp eq i64 %_4.i68, 0
  br i1 %110, label %bb1.i70, label %bb3.i69, !prof !30

bb3.i69:                                          ; preds = %bb12
; invoke <std::sys::sync::rwlock::queue::RwLock>::lock_contended
  invoke void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock14lock_contended(ptr noundef nonnull align 8 %_8, i1 noundef zeroext true)
          to label %bb1.i70 unwind label %cleanup3

bb1.i70:                                          ; preds = %bb3.i69, %bb12
  %111 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !428
  %_6.i.i71 = and i64 %111, 9223372036854775807
  %112 = icmp eq i64 %_6.i.i71, 0
  br i1 %112, label %bb13, label %bb6.i.i72, !prof !30

bb6.i.i72:                                        ; preds = %bb1.i70
; invoke std::panicking::panic_count::is_zero_slow_path
  %113 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
          to label %.noexc78 unwind label %cleanup3

.noexc78:                                         ; preds = %bb6.i.i72
  %114 = xor i1 %113, true
  br label %bb13

cleanup3:                                         ; preds = %bb6.i.i72, %bb3.i69
  %115 = landingpad { ptr, i32 }
          cleanup
  br label %bb22

bb13:                                             ; preds = %.noexc78, %bb1.i70
  %_5.sroa.0.0.off0.i.i73 = phi i1 [ %114, %.noexc78 ], [ false, %bb1.i70 ]
  %116 = load atomic i8, ptr %_12.i monotonic, align 1, !noalias !428
  %.not133 = icmp eq i8 %116, 0
  br i1 %.not133, label %bb14, label %bb2.i13, !prof !30

bb2.i13:                                          ; preds = %bb13
  %_0.sroa.3.0.i.i75 = zext i1 %_5.sroa.0.0.off0.i.i73 to i8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i9), !noalias !431
  store ptr %_8, ptr %e.i9, align 8, !noalias !431
  %117 = getelementptr inbounds nuw i8, ptr %e.i9, i64 8
  store i8 %_0.sroa.3.0.i.i75, ptr %117, align 8, !noalias !431
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i9, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d1c5991a811577b5a75f5444621d6d5a) #30
          to label %unreachable.i17 unwind label %cleanup.i14, !noalias !435

cleanup.i14:                                      ; preds = %bb2.i13
  %118 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::rwlock::RwLockWriteGuard<alloc::string::String>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i9) #32
          to label %bb22 unwind label %terminate.i15, !noalias !435

unreachable.i17:                                  ; preds = %bb2.i13
  unreachable

terminate.i15:                                    ; preds = %cleanup.i14
  %119 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !435
  unreachable

bb14:                                             ; preds = %bb13
  %_72.val = load i64, ptr %_60, align 8
  %120 = icmp eq i64 %_72.val, 0
  br i1 %120, label %bb15, label %bb2.i.i.i4.i.i82

bb2.i.i.i4.i.i82:                                 ; preds = %bb14
  %_72.val31 = load ptr, ptr %17, align 8, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_72.val31, i64 noundef %_72.val, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb15

bb15:                                             ; preds = %bb2.i.i.i4.i.i82, %bb14
  store i64 0, ptr %_60, align 8
  store ptr inttoptr (i64 1 to ptr), ptr %17, align 8
  store i64 0, ptr %_5.sroa.8.0._60.sroa_idx, align 8
  br i1 %_5.sroa.0.0.off0.i.i73, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i91, label %bb1.i.i.i86

bb1.i.i.i86:                                      ; preds = %bb15
  %121 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i87 = and i64 %121, 9223372036854775807
  %122 = icmp eq i64 %_7.i.i.i87, 0
  br i1 %122, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i91, label %bb6.i.i.i88, !prof !30

bb6.i.i.i88:                                      ; preds = %bb1.i.i.i86
; invoke std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i8993 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
          to label %_6.i.i.i89.noexc unwind label %cleanup5

_6.i.i.i89.noexc:                                 ; preds = %bb6.i.i.i88
  br i1 %_6.i.i.i8993, label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i91, label %bb2.i.i.i90

bb2.i.i.i90:                                      ; preds = %_6.i.i.i89.noexc
  store atomic i8 1, ptr %_12.i monotonic, align 1
  br label %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i91

_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i91: ; preds = %bb2.i.i.i90, %_6.i.i.i89.noexc, %bb1.i.i.i86, %bb15
  %123 = cmpxchg ptr %_8, ptr inttoptr (i64 1 to ptr), ptr null release monotonic, align 8
  %124 = extractvalue { ptr, i1 } %123, 1
  br i1 %124, label %bb17, label %bb3.i.i92, !prof !30

bb3.i.i92:                                        ; preds = %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i91
  %125 = extractvalue { ptr, i1 } %123, 0
; invoke <std::sys::sync::rwlock::queue::RwLock>::unlock_contended
  invoke void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock16unlock_contended(ptr noundef nonnull align 8 %_8, ptr noundef %125)
          to label %bb17 unwind label %cleanup5

bb22:                                             ; preds = %cleanup.i14, %cleanup3, %cleanup5
  %.pn = phi { ptr, i32 } [ %126, %cleanup5 ], [ %115, %cleanup3 ], [ %118, %cleanup.i14 ]
; invoke core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::io::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsC3JuwEIQwb_7console(i64 %ret.sroa.0.2, ptr %ret.sroa.10.2) #32
          to label %bb23 unwind label %terminate

cleanup5:                                         ; preds = %bb3.i.i92, %bb6.i.i.i88
  %126 = landingpad { ptr, i32 }
          cleanup
  br label %bb22

bb17:                                             ; preds = %_RNvMNtNtCs5sEH5CPMdak_3std4sync6poisonNtB2_4Flag4done.exit.i.i91, %bb3.i.i92
  store i64 %ret.sroa.0.2, ptr %_0, align 8
  %ret.sroa.10.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %ret.sroa.10.2, ptr %ret.sroa.10.0._0.sroa_idx, align 8
  %ret.sroa.16.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %ret.sroa.16.1, ptr %ret.sroa.16.0._0.sroa_idx, align 8
  br i1 %_5.sroa.0.0.off0.i.i53, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit103, label %bb1.i.i.i98

bb1.i.i.i98:                                      ; preds = %bb17
  %127 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i99 = and i64 %127, 9223372036854775807
  %128 = icmp eq i64 %_7.i.i.i99, 0
  br i1 %128, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit103, label %bb6.i.i.i100, !prof !30

bb6.i.i.i100:                                     ; preds = %bb1.i.i.i98
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i101 = call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i101, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit103, label %bb2.i.i.i102

bb2.i.i.i102:                                     ; preds = %bb6.i.i.i100
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit103

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECsC3JuwEIQwb_7console.exit103: ; preds = %bb17, %bb1.i.i.i98, %bb6.i.i.i100, %bb2.i.i.i102
  %129 = load atomic ptr, ptr %_11 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %129)
  br label %bb20

terminate:                                        ; preds = %bb22, %bb23
  %130 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; <console::term::Term>::flush
; Function Attrs: uwtable
define noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term5flush(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [16 x i8], align 8
  %_10 = load ptr, ptr %self, align 8, !nonnull !7, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %_10, i64 16
  %_2 = load i64, ptr %0, align 8, !range !6, !noundef !7
  %1 = trunc nuw i64 %_2 to i1
  br i1 %1, label %bb1, label %bb12

bb1:                                              ; preds = %start
  %buffer1 = getelementptr inbounds nuw i8, ptr %_10, i64 24
  %2 = load atomic ptr, ptr %buffer1 acquire, align 8, !noalias !436
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, !prof !5

bb7.i.i:                                          ; preds = %bb1
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %4 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 %buffer1), !noalias !436
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i: ; preds = %bb7.i.i, %bb1
  %_0.sroa.0.0.i.i = phi ptr [ %4, %bb7.i.i ], [ %2, %bb1 ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !436
  %5 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !436
  %_6.i.i = and i64 %5, 9223372036854775807
  %6 = icmp eq i64 %_6.i.i, 0
  br i1 %6, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit, label %bb6.i.i, !prof !30

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %7 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33, !noalias !436
  %8 = xor i1 %7, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %8, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_10, i64 32
  %9 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !436
  %.not18 = icmp eq i8 %9, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not18, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit, label %bb2.i, !prof !30

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !439
  store ptr %buffer1, ptr %e.i, align 8, !noalias !439
  %10 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %10, align 8, !noalias !439
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e6817151e95acf2e0d9d4b0416a3fb0b) #30
          to label %unreachable.i unwind label %cleanup.i, !noalias !443

cleanup.i:                                        ; preds = %bb2.i
  %11 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<u8>>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #32
          to label %common.resume unwind label %terminate.i, !noalias !443

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !443
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %11, %cleanup.i ], [ %19, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit
  %13 = getelementptr inbounds nuw i8, ptr %_10, i64 56
  %_14 = load i64, ptr %13, align 8, !noundef !7
  %_15 = icmp sgt i64 %_14, -1
  tail call void @llvm.assume(i1 %_15)
  %14 = icmp eq i64 %_14, 0
  br i1 %14, label %bb8, label %bb5

bb5:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
  %15 = getelementptr inbounds nuw i8, ptr %_10, i64 48
  %_23 = load ptr, ptr %15, align 8, !nonnull !7, !noundef !7
; invoke <console::term::Term>::write_through
  %16 = invoke fastcc noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term13write_through(ptr nonnull %_10, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_23, i64 noundef %_14)
          to label %bb6 unwind label %cleanup

bb8:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit, %bb16
  br i1 %_5.sroa.0.0.off0.i.i, label %bb12.sink.split, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb8
  %17 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %17, 9223372036854775807
  %18 = icmp eq i64 %_7.i.i.i, 0
  br i1 %18, label %bb12.sink.split, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %bb12.sink.split, label %bb12.sink.split.sink.split

cleanup:                                          ; preds = %bb5
  %19 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr nonnull %buffer1, i8 %_0.sroa.3.0.i.i) #32
          to label %common.resume unwind label %terminate

bb6:                                              ; preds = %bb5
  %.not = icmp eq ptr %16, null
  br i1 %.not, label %bb16, label %bb15

bb15:                                             ; preds = %bb6
  br i1 %_5.sroa.0.0.off0.i.i, label %bb12.sink.split, label %bb1.i.i.i9

bb1.i.i.i9:                                       ; preds = %bb15
  %20 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i10 = and i64 %20, 9223372036854775807
  %21 = icmp eq i64 %_7.i.i.i10, 0
  br i1 %21, label %bb12.sink.split, label %bb6.i.i.i11, !prof !30

bb6.i.i.i11:                                      ; preds = %bb1.i.i.i9
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i12 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i12, label %bb12.sink.split, label %bb12.sink.split.sink.split

bb16:                                             ; preds = %bb6
  store i64 0, ptr %13, align 8
  br label %bb8

bb12.sink.split.sink.split:                       ; preds = %bb6.i.i.i11, %bb6.i.i.i
  %_0.sroa.0.0.ph.ph = phi ptr [ null, %bb6.i.i.i ], [ %16, %bb6.i.i.i11 ]
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %bb12.sink.split

bb12.sink.split:                                  ; preds = %bb12.sink.split.sink.split, %bb6.i.i.i11, %bb1.i.i.i9, %bb15, %bb6.i.i.i, %bb1.i.i.i, %bb8
  %_0.sroa.0.0.ph = phi ptr [ null, %bb8 ], [ null, %bb1.i.i.i ], [ null, %bb6.i.i.i ], [ %16, %bb15 ], [ %16, %bb1.i.i.i9 ], [ %16, %bb6.i.i.i11 ], [ %_0.sroa.0.0.ph.ph, %bb12.sink.split.sink.split ]
  %22 = load atomic ptr, ptr %buffer1 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %22)
  br label %bb12

bb12:                                             ; preds = %bb12.sink.split, %start
  %_0.sroa.0.0 = phi ptr [ null, %start ], [ %_0.sroa.0.0.ph, %bb12.sink.split ]
  ret ptr %_0.sroa.0.0

terminate:                                        ; preds = %cleanup
  %23 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; <console::term::Term>::read_key
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term8read_key(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 9
  %1 = load i8, ptr %0, align 1, !range !29, !noundef !7
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb2, label %bb1

bb1:                                              ; preds = %start
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb3

bb2:                                              ; preds = %start
; call console::unix_term::read_single_key
  tail call fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term15read_single_key(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_0, i1 noundef zeroext false)
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  ret void
}

; <console::term::Term>::read_char
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9read_char(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_6 = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 9
  %1 = load i8, ptr %0, align 1, !range !29, !noundef !7
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term8read_key.exit.critedge, label %bb1

bb1:                                              ; preds = %start
; call <std::io::error::Error>::new::<&str>
  %_3 = tail call noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newReEBa_(i8 noundef 7, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a42bcd32b1b51ab8fb3130fb27019cbe, i64 noundef 14) #33
  %2 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_3, ptr %2, align 8
  br label %bb11

bb11:                                             ; preds = %bb10, %bb14, %bb1
  %.sink = phi i32 [ 0, %bb10 ], [ 1, %bb14 ], [ 1, %bb1 ]
  store i32 %.sink, ptr %_0, align 8
  ret void

_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term8read_key.exit.critedge: ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
; call console::unix_term::read_single_key
  call fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term15read_single_key(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_6, i1 noundef zeroext false), !noalias !444
  %.pr26 = load i64, ptr %_6, align 8
  %.pr.fr27 = freeze i64 %.pr26
  %3 = icmp eq i64 %.pr.fr27, -9223372036854775787
  br i1 %3, label %bb14, label %bb15.lr.ph

bb15.lr.ph:                                       ; preds = %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term8read_key.exit.critedge
  %_12.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  br label %bb15

bb14:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console2kb3KeyEBK_.exit, %_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term8read_key.exit.critedge
  %4 = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %_13 = load ptr, ptr %4, align 8, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_13, ptr %5, align 8
  br label %bb11

bb15:                                             ; preds = %bb15.lr.ph, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console2kb3KeyEBK_.exit
  %.pr.fr28 = phi i64 [ %.pr.fr27, %bb15.lr.ph ], [ %.pr.fr, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console2kb3KeyEBK_.exit ]
  %_12.sroa.5.0.copyload = load ptr, ptr %_12.sroa.5.0._6.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  %6 = icmp ne i64 %.pr.fr28, -9223372036854775807
  tail call void @llvm.assume(i1 %6)
  %7 = icmp slt i64 %.pr.fr28, 0
  br i1 %7, label %8, label %bb7

8:                                                ; preds = %bb15
  %9 = and i64 %.pr.fr28, 9223372036854775807
  switch i64 %9, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console2kb3KeyEBK_.exit [
    i64 6, label %bb10
    i64 19, label %bb9
  ]

bb7:                                              ; preds = %bb15
  %or.cond.i = icmp eq i64 %.pr.fr28, 0
  br i1 %or.cond.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console2kb3KeyEBK_.exit, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb7
  %alloc_size.i.i.i.i5.i.i = shl nuw i64 %.pr.fr28, 2
  %10 = icmp ne ptr %_12.sroa.5.0.copyload, null
  tail call void @llvm.assume(i1 %10)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_12.sroa.5.0.copyload, i64 noundef %alloc_size.i.i.i.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #29
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console2kb3KeyEBK_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console2kb3KeyEBK_.exit: ; preds = %8, %bb7, %bb2.i.i.i4.i.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
; call console::unix_term::read_single_key
  call fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term15read_single_key(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_6, i1 noundef zeroext false), !noalias !444
  %.pr = load i64, ptr %_6, align 8
  %.pr.fr = freeze i64 %.pr
  %11 = icmp eq i64 %.pr.fr, -9223372036854775787
  br i1 %11, label %bb14, label %bb15

bb9:                                              ; preds = %8
  %12 = ptrtoint ptr %_12.sroa.5.0.copyload to i64
  %_4.sroa.6.8.extract.trunc = trunc i64 %12 to i32
  br label %bb10

bb10:                                             ; preds = %8, %bb9
  %_4.sroa.6.8.extract.trunc.sink = phi i32 [ %_4.sroa.6.8.extract.trunc, %bb9 ], [ 10, %8 ]
  %13 = getelementptr inbounds nuw i8, ptr %_0, i64 4
  store i32 %_4.sroa.6.8.extract.trunc.sink, ptr %13, align 4
  br label %bb11
}

; <console::term::Term>::read_line
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9read_line(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 {
start:
; call <console::term::Term>::read_line_initial_text
  tail call void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term22read_line_initial_text(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) inttoptr (i64 1 to ptr), i64 noundef 0)
  ret void
}

; <console::term::Term>::write_str
; Function Attrs: uwtable
define noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [16 x i8], align 8
  %_11 = load ptr, ptr %self, align 8, !nonnull !7, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %_11, i64 16
  %_3 = load i64, ptr %0, align 8, !range !6, !noundef !7
  %1 = trunc nuw i64 %_3 to i1
  br i1 %1, label %bb3, label %bb2

bb3:                                              ; preds = %start
  %buffer = getelementptr inbounds nuw i8, ptr %_11, i64 24
  %2 = load atomic ptr, ptr %buffer acquire, align 8, !noalias !447
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, !prof !5

bb7.i.i:                                          ; preds = %bb3
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %4 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 %buffer), !noalias !447
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i: ; preds = %bb7.i.i, %bb3
  %_0.sroa.0.0.i.i = phi ptr [ %4, %bb7.i.i ], [ %2, %bb3 ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !447
  %5 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !447
  %_6.i.i = and i64 %5, 9223372036854775807
  %6 = icmp eq i64 %_6.i.i, 0
  br i1 %6, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit, label %bb6.i.i, !prof !30

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %7 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33, !noalias !447
  %8 = xor i1 %7, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %8, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_11, i64 32
  %9 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !447
  %.not = icmp eq i8 %9, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit, label %bb2.i, !prof !30

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !450
  store ptr %buffer, ptr %e.i, align 8, !noalias !450
  %10 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %10, align 8, !noalias !450
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_118a6c87c06c70363f5ebdfdf2788916) #30
          to label %unreachable.i unwind label %cleanup.i, !noalias !454

cleanup.i:                                        ; preds = %bb2.i
  %11 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<u8>>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #32
          to label %common.resume unwind label %terminate.i, !noalias !454

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !454
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %11, %cleanup.i ], [ %15, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit
  %_14 = getelementptr inbounds nuw i8, ptr %_11, i64 40
  %13 = getelementptr inbounds nuw i8, ptr %_11, i64 56
  %len.i = load i64, ptr %13, align 8, !alias.scope !455, !noundef !7
  %self2.i = load i64, ptr %_14, align 8, !range !23, !alias.scope !455, !noundef !7
  %_9.i = sub i64 %self2.i, %len.i
  %_7.i5 = icmp ugt i64 %s.1, %_9.i
  br i1 %_7.i5, label %bb1.i, label %bb11, !prof !5

bb1.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %_14, i64 noundef %len.i, i64 noundef %s.1, i64 noundef 1, i64 noundef 1)
          to label %bb1.i.bb11_crit_edge unwind label %cleanup

bb1.i.bb11_crit_edge:                             ; preds = %bb1.i
  %_26.pre = load i64, ptr %13, align 8
  br label %bb11

bb2:                                              ; preds = %start
; call <console::term::Term>::write_through
  %14 = tail call fastcc noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term13write_through(ptr nonnull %_11, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1)
  br label %bb7

bb7:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, %bb2
  %_0.sroa.0.0 = phi ptr [ null, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit ], [ %14, %bb2 ]
  ret ptr %_0.sroa.0.0

cleanup:                                          ; preds = %bb1.i
  %15 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr nonnull %buffer, i8 %_0.sroa.3.0.i.i) #32
          to label %common.resume unwind label %terminate

bb11:                                             ; preds = %bb1.i.bb11_crit_edge, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
  %_26 = phi i64 [ %_26.pre, %bb1.i.bb11_crit_edge ], [ %len.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit ]
  %_30 = icmp sgt i64 %_26, -1
  tail call void @llvm.assume(i1 %_30)
  %16 = getelementptr inbounds nuw i8, ptr %_11, i64 48
  %_31 = load ptr, ptr %16, align 8, !nonnull !7, !noundef !7
  %_28 = getelementptr inbounds nuw i8, ptr %_31, i64 %_26
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_28, ptr nonnull align 1 %s.0, i64 %s.1, i1 false)
  %17 = load i64, ptr %13, align 8, !noundef !7
  %18 = add i64 %17, %s.1
  store i64 %18, ptr %13, align 8
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb11
  %19 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %19, 9223372036854775807
  %20 = icmp eq i64 %_7.i.i.i, 0
  br i1 %20, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit: ; preds = %bb11, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %21 = load atomic ptr, ptr %buffer monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %21)
  br label %bb7

terminate:                                        ; preds = %cleanup
  %22 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; <console::utils::Style>::from_dotted_str
; Function Attrs: uwtable
define void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style15from_dotted_str(ptr dead_on_unwind noalias noundef writable writeonly sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #1 personality ptr @rust_eh_personality {
bb68:
  %_51 = alloca [32 x i8], align 8
  %_41 = alloca [32 x i8], align 8
  %_40 = alloca [32 x i8], align 8
  %_39 = alloca [32 x i8], align 8
  %_38 = alloca [32 x i8], align 8
  %_37 = alloca [32 x i8], align 8
  %_36 = alloca [32 x i8], align 8
  %_35 = alloca [32 x i8], align 8
  %_34 = alloca [32 x i8], align 8
  %rv = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %rv)
  %0 = getelementptr inbounds nuw i8, ptr %rv, i64 24
  store i8 9, ptr %0, align 8
  %1 = getelementptr inbounds nuw i8, ptr %rv, i64 26
  store i8 9, ptr %1, align 2
  store ptr null, ptr %rv, align 8
  %_53.sroa.5.0.rv.sroa_idx = getelementptr inbounds nuw i8, ptr %rv, i64 16
  store i64 0, ptr %_53.sroa.5.0.rv.sroa_idx, align 8
  %2 = getelementptr inbounds nuw i8, ptr %rv, i64 28
  store <4 x i8> <i8 2, i8 0, i8 0, i8 0>, ptr %2, align 4
  %3 = getelementptr inbounds nuw i8, ptr %_51, i64 24
  %4 = getelementptr inbounds nuw i8, ptr %_51, i64 29
  %5 = getelementptr inbounds nuw i8, ptr %_51, i64 26
  %6 = getelementptr inbounds nuw i8, ptr %_51, i64 30
  %7 = getelementptr inbounds nuw i8, ptr %_51, i64 25
  %8 = getelementptr inbounds nuw i8, ptr %_51, i64 27
  br label %bb2.i

bb2.i:                                            ; preds = %bb68, %bb1.backedge
  %9 = phi i64 [ 0, %bb68 ], [ %.ph120, %bb1.backedge ]
  %_3.sroa.3.sroa.3.0.copyload116490 = phi i64 [ 0, %bb68 ], [ %_3.sroa.3.sroa.3.0.copyload115.ph, %bb1.backedge ]
  %_4325.i.i = icmp ult i64 %s.1, %_3.sroa.3.sroa.3.0.copyload116490
  br i1 %_4325.i.i, label %bb3, label %bb12.us.i.i

bb12.us.i.i:                                      ; preds = %bb2.i, %bb9.us.i.i
  %10 = phi i64 [ %19, %bb9.us.i.i ], [ %_3.sroa.3.sroa.3.0.copyload116490, %bb2.i ]
  %new_len.us.i.i = sub nuw i64 %s.1, %10
  %_46.us.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 %10
  %_3.i.us.i.i = icmp samesign ult i64 %new_len.us.i.i, 16
  br i1 %_3.i.us.i.i, label %bb5.preheader.i.us.i.i, label %bb2.i.us.i.i

bb2.i.us.i.i:                                     ; preds = %bb12.us.i.i
; invoke core::slice::memchr::memchr_aligned
  %11 = invoke { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef 46, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us.i.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i.i)
          to label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i unwind label %bb66.loopexit

bb5.preheader.i.us.i.i:                           ; preds = %bb12.us.i.i
  %_64.not.i.us.i.i = icmp eq i64 %new_len.us.i.i, 0
  br i1 %_64.not.i.us.i.i, label %bb4.i.us.i.i, label %bb7.i.us.i.i

bb7.i.us.i.i:                                     ; preds = %bb5.preheader.i.us.i.i, %bb9.i.us.i.i
  %i.sroa.0.05.i.us.i.i = phi i64 [ %13, %bb9.i.us.i.i ], [ 0, %bb5.preheader.i.us.i.i ]
  %12 = getelementptr inbounds nuw i8, ptr %_46.us.i.i, i64 %i.sroa.0.05.i.us.i.i
  %_9.i.us.i.i = load i8, ptr %12, align 1, !alias.scope !458, !noalias !461, !noundef !7
  %_8.i.us.i.i = icmp eq i8 %_9.i.us.i.i, 46
  br i1 %_8.i.us.i.i, label %bb4.i.us.i.i, label %bb9.i.us.i.i

bb9.i.us.i.i:                                     ; preds = %bb7.i.us.i.i
  %13 = add nuw nsw i64 %i.sroa.0.05.i.us.i.i, 1
  %exitcond.not.i.us.i.i = icmp eq i64 %13, %new_len.us.i.i
  br i1 %exitcond.not.i.us.i.i, label %bb4.i.us.i.i, label %bb7.i.us.i.i

bb4.i.us.i.i:                                     ; preds = %bb9.i.us.i.i, %bb7.i.us.i.i, %bb5.preheader.i.us.i.i
  %i.sroa.0.0.lcssa.i.us.i.i = phi i64 [ 0, %bb5.preheader.i.us.i.i ], [ %new_len.us.i.i, %bb9.i.us.i.i ], [ %i.sroa.0.05.i.us.i.i, %bb7.i.us.i.i ]
  %_0.sroa.0.1.i.us.i.i = phi i64 [ 0, %bb5.preheader.i.us.i.i ], [ 0, %bb9.i.us.i.i ], [ 1, %bb7.i.us.i.i ]
  %14 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.us.i.i, 0
  %15 = insertvalue { i64, i64 } %14, i64 %i.sroa.0.0.lcssa.i.us.i.i, 1
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i

_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i: ; preds = %bb2.i.us.i.i, %bb4.i.us.i.i
  %.merged.i.us.i.i = phi { i64, i64 } [ %15, %bb4.i.us.i.i ], [ %11, %bb2.i.us.i.i ]
  %16 = extractvalue { i64, i64 } %.merged.i.us.i.i, 0
  %17 = trunc nuw i64 %16 to i1
  br i1 %17, label %bb4.us.i.i, label %bb3

bb4.us.i.i:                                       ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i
  %18 = extractvalue { i64, i64 } %.merged.i.us.i.i, 1
  %_16.us.i.i = add i64 %10, 1
  %19 = add i64 %_16.us.i.i, %18
  %_54.not.us.i.i = icmp ugt i64 %19, %s.1
  %20 = add i64 %18, %10
  %or.cond.i.i.not = icmp ult i64 %20, %s.1
  br i1 %or.cond.i.i.not, label %bb19.us.i.i, label %bb9.us.i.i

bb19.us.i.i:                                      ; preds = %bb4.us.i.i
  %_62.us.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 %20
  %lhsc = load i8, ptr %_62.us.i.i, align 1
  %21 = icmp eq i8 %lhsc, 46
  br i1 %21, label %bb3, label %bb9.us.i.i

bb9.us.i.i:                                       ; preds = %bb19.us.i.i, %bb4.us.i.i
  br i1 %_54.not.us.i.i, label %bb3, label %bb12.us.i.i

bb3:                                              ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i, %bb9.us.i.i, %bb19.us.i.i, %bb2.i
  %_3.sroa.3.sroa.3.0.copyload115.ph = phi i64 [ %_3.sroa.3.sroa.3.0.copyload116490, %bb2.i ], [ %19, %bb19.us.i.i ], [ %s.1, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i ], [ %19, %bb9.us.i.i ]
  %.ph.off0 = phi i1 [ true, %bb2.i ], [ false, %bb19.us.i.i ], [ true, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i ], [ true, %bb9.us.i.i ]
  %.ph120 = phi i64 [ %9, %bb2.i ], [ %19, %bb19.us.i.i ], [ %9, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i ], [ %9, %bb9.us.i.i ]
  %s.1.pn = phi i64 [ %s.1, %bb2.i ], [ %20, %bb19.us.i.i ], [ %s.1, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i ], [ %s.1, %bb9.us.i.i ]
  %_0.sroa.0.1.i.ph = getelementptr inbounds nuw i8, ptr %s.0, i64 %9
  %_0.sroa.4.1.i.ph = sub nuw i64 %s.1.pn, %9
  %_3.not.i = icmp eq i64 %_0.sroa.4.1.i.ph, 5
  br i1 %_3.not.i, label %bb70, label %bb7

bb4:                                              ; preds = %bb1.backedge
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %rv)
  ret void

bb70:                                             ; preds = %bb3
  %22 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(5) @alloc_f46351429e0b545e52edab17c91e045b, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !467
  %23 = icmp eq i32 %22, 0
  br i1 %23, label %bb6, label %bb73

bb7:                                              ; preds = %bb3
  %_3.not.i10 = icmp eq i64 %_0.sroa.4.1.i.ph, 3
  br i1 %_3.not.i10, label %bb72, label %bb11

bb6:                                              ; preds = %bb70
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 0, ptr %3, align 8
  br label %bb61

bb72:                                             ; preds = %bb7
  %24 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(3) @alloc_940496d3cd18caebe90765385b962abe, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !471
  %25 = icmp eq i32 %24, 0
  br i1 %25, label %bb8, label %bb91

bb8:                                              ; preds = %bb72
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 1, ptr %3, align 8
  br label %bb61

bb73:                                             ; preds = %bb70
  %26 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(5) @alloc_b8ebeef8965b067fea87e4e40f7a2696, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !475
  %27 = icmp eq i32 %26, 0
  br i1 %27, label %bb10, label %bb78

bb11:                                             ; preds = %bb7
  switch i64 %_0.sroa.4.1.i.ph, label %bb5 [
    i64 6, label %bb74
    i64 4, label %bb75
    i64 7, label %bb76
    i64 8, label %bb80
    i64 9, label %bb83
    i64 10, label %bb85
    i64 13, label %bb103
  ]

bb10:                                             ; preds = %bb73
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 2, ptr %3, align 8
  br label %bb61

bb74:                                             ; preds = %bb11
  %28 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(6) @alloc_761ec3f22b4eea6465689eb413e86232, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !479
  %29 = icmp eq i32 %28, 0
  br i1 %29, label %bb12, label %bb19.thread233

bb12:                                             ; preds = %bb74
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 3, ptr %3, align 8
  br label %bb61

bb75:                                             ; preds = %bb11
  %30 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(4) @alloc_d11e62a55d3bc6b15beecd4f0707fbb1, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !483
  %31 = icmp eq i32 %30, 0
  br i1 %31, label %bb14, label %bb77

bb14:                                             ; preds = %bb75
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 4, ptr %3, align 8
  br label %bb61

bb76:                                             ; preds = %bb11
  %32 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(7) @alloc_3d9fd8ba1c29d9f6085e0294be30b396, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !487
  %33 = icmp eq i32 %32, 0
  br i1 %33, label %bb16, label %bb84

bb16:                                             ; preds = %bb76
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 5, ptr %3, align 8
  br label %bb61

bb77:                                             ; preds = %bb75
  %34 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(4) @alloc_6fc1c32c52b5575ce6b85d0118457658, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !491
  %35 = icmp eq i32 %34, 0
  br i1 %35, label %bb18, label %bb89

bb19.thread233:                                   ; preds = %bb74
  %36 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(6) @alloc_0e40d41a90d9449ef3c0a2d4ebecf25e, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !495
  %37 = icmp eq i32 %36, 0
  br i1 %37, label %bb22, label %bb81

bb18:                                             ; preds = %bb77
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 6, ptr %3, align 8
  br label %bb61

bb78:                                             ; preds = %bb73
  %38 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(5) @alloc_16c9842326e8d04943b3c4e15a061707, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !499
  %39 = icmp eq i32 %38, 0
  br i1 %39, label %bb20, label %bb95

bb20:                                             ; preds = %bb78
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 7, ptr %3, align 8
  br label %bb61

bb22:                                             ; preds = %bb19.thread233
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 1, ptr %4, align 1
  br label %bb61

bb80:                                             ; preds = %bb11
  %40 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(8) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(8) @alloc_988dbb775df289ae5d387217d1326fbb, i64 range(i64 0, -9223372036854775808) 8), !alias.scope !503
  %41 = icmp eq i32 %40, 0
  br i1 %41, label %bb24, label %bb82

bb24:                                             ; preds = %bb80
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 0, ptr %5, align 2
  br label %bb61

bb81:                                             ; preds = %bb19.thread233
  %42 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_00afc89fad6ce0b69c03ae44a4c57e63, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !507
  %43 = icmp eq i32 %42, 0
  br i1 %43, label %bb26, label %bb101

bb26:                                             ; preds = %bb81
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 1, ptr %5, align 2
  br label %bb61

bb82:                                             ; preds = %bb80
  %44 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(8) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(8) @alloc_d3f73fbdad6723a6f7e70241cc5ffd7b, i64 range(i64 0, -9223372036854775808) 8), !alias.scope !511
  %45 = icmp eq i32 %44, 0
  br i1 %45, label %bb28, label %bb87

bb28:                                             ; preds = %bb82
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 2, ptr %5, align 2
  br label %bb61

bb83:                                             ; preds = %bb11
  %46 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(9) @alloc_b2d76a187fec1d5904bbbaf8cd5a45f9, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !515
  %47 = icmp eq i32 %46, 0
  br i1 %47, label %bb30, label %bb88

bb30:                                             ; preds = %bb83
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 3, ptr %5, align 2
  br label %bb61

bb84:                                             ; preds = %bb76
  %48 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_0ea3ed158ba78f67fb80c06310d3d976, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !519
  %49 = icmp eq i32 %48, 0
  br i1 %49, label %bb32, label %bb86

bb32:                                             ; preds = %bb84
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 4, ptr %5, align 2
  br label %bb61

bb85:                                             ; preds = %bb11
  %50 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(10) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(10) @alloc_93fbedf531e254d87a372581b818455c, i64 range(i64 0, -9223372036854775808) 10), !alias.scope !523
  %51 = icmp eq i32 %50, 0
  br i1 %51, label %bb34, label %bb93

bb34:                                             ; preds = %bb85
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 5, ptr %5, align 2
  br label %bb61

bb86:                                             ; preds = %bb84
  %52 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_70cd957082e55e3219d604e7774b3a2a, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !527
  %53 = icmp eq i32 %52, 0
  br i1 %53, label %bb36, label %bb99

bb36:                                             ; preds = %bb86
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 6, ptr %5, align 2
  br label %bb61

bb87:                                             ; preds = %bb82
  %54 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_28c707092aa47c9e62a5453e9825cf31, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !531
  %55 = icmp eq i32 %54, 0
  br i1 %55, label %bb38, label %bb55

bb38:                                             ; preds = %bb87
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 7, ptr %5, align 2
  br label %bb61

bb88:                                             ; preds = %bb83
  %56 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_d0d4965f28f12d18e1eb5434656a1634, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !535
  %57 = icmp eq i32 %56, 0
  br i1 %57, label %bb40, label %bb55

bb40:                                             ; preds = %bb88
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 1, ptr %6, align 2
  br label %bb61

bb89:                                             ; preds = %bb77
  %58 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(4) @alloc_8004f499ef1e85ed18420cba06aae287, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !539
  %59 = icmp eq i32 %58, 0
  br i1 %59, label %bb42, label %bb71

bb42:                                             ; preds = %bb89
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_34)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_34, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
; call <console::utils::Style>::attr
  call fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_51, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_34, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_34)
  br label %bb61

bb91:                                             ; preds = %bb72
  %60 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_bc56aad9843e15c95b8ae131ad5c2648, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !543
  %61 = icmp eq i32 %60, 0
  br i1 %61, label %bb44, label %bb55

bb44:                                             ; preds = %bb91
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_35)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_35, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
; call <console::utils::Style>::attr
  call fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_51, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_35, i8 noundef 1)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_35)
  br label %bb61

bb93:                                             ; preds = %bb85
  %62 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_35da8c1274638488887234a29631ac8a, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !547
  %63 = icmp eq i32 %62, 0
  br i1 %63, label %bb46, label %bb97

bb46:                                             ; preds = %bb93
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_36)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_36, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
; call <console::utils::Style>::attr
  call fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_51, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_36, i8 noundef 3)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_36)
  br label %bb61

bb95:                                             ; preds = %bb78
  %64 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(5) @alloc_343238722d8106d5e592de156f114b52, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !551
  %65 = icmp eq i32 %64, 0
  br i1 %65, label %bb48, label %bb71

bb48:                                             ; preds = %bb95
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_37)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_37, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
; call <console::utils::Style>::attr
  call fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_51, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_37, i8 noundef 4)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_37)
  br label %bb61

bb97:                                             ; preds = %bb93
  %66 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_94ec8616ca31a77937e20e985e657532, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !555
  %67 = icmp eq i32 %66, 0
  br i1 %67, label %bb50, label %bb55

bb50:                                             ; preds = %bb97
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_38)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_38, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
; call <console::utils::Style>::attr
  call fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_51, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_38, i8 noundef 5)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_38)
  br label %bb61

bb99:                                             ; preds = %bb86
  %68 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_6f7f7c851d095ea6be0620cebf08a8c7, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !559
  %69 = icmp eq i32 %68, 0
  br i1 %69, label %bb52, label %bb55

bb52:                                             ; preds = %bb99
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_39)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_39, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
; call <console::utils::Style>::attr
  call fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_51, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_39, i8 noundef 6)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_39)
  br label %bb61

bb101:                                            ; preds = %bb81
  %70 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.1.i.ph, ptr nonnull @alloc_a49a2be336e4029d0e1a46cd0ad31a8b, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.1.i.ph), !alias.scope !563
  %71 = icmp eq i32 %70, 0
  br i1 %71, label %bb54, label %bb55

bb55:                                             ; preds = %bb99, %bb97, %bb87, %bb88, %bb91, %bb101
  %_3.not.i106 = icmp eq i64 %_0.sroa.4.1.i.ph, 13
  br i1 %_3.not.i106, label %bb103, label %bb5

bb54:                                             ; preds = %bb101
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_40)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_40, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
; call <console::utils::Style>::attr
  call fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_51, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_40, i8 noundef 7)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_40)
  br label %bb61

bb103:                                            ; preds = %bb11, %bb55
  %72 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(13) %_0.sroa.0.1.i.ph, ptr noundef nonnull dereferenceable(13) @alloc_4a97ccfb756c0a73db16da629adb6fac, i64 range(i64 0, -9223372036854775808) 13), !alias.scope !567
  %73 = icmp eq i32 %72, 0
  br i1 %73, label %bb56, label %bb71.thread295

bb5:                                              ; preds = %bb11, %bb55
  %_4.i110 = icmp samesign ugt i64 %_0.sroa.4.1.i.ph, 2
  br i1 %_4.i110, label %bb71, label %bb58

bb56:                                             ; preds = %bb103
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_41)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_41, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
; call <console::utils::Style>::attr
  call fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_51, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_41, i8 noundef 8)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_41)
  br label %bb61

bb71:                                             ; preds = %bb95, %bb89, %bb5
  %_3.not.i10130133135141144153157170175192197224229253254270309332334373388417424459467601606620625628631 = phi i1 [ %_3.not.i10, %bb5 ], [ false, %bb89 ], [ false, %bb95 ]
  %74 = tail call i32 @memcmp(ptr noundef nonnull dereferenceable(3) @alloc_607574f98762a7642d8175dbddfc81da, ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.1.i.ph, i64 3), !alias.scope !571
  %75 = icmp eq i32 %74, 0
  br i1 %75, label %bb57, label %bb58

bb71.thread295:                                   ; preds = %bb103
  %76 = tail call i32 @memcmp(ptr noundef nonnull dereferenceable(3) @alloc_607574f98762a7642d8175dbddfc81da, ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.1.i.ph, i64 3), !alias.scope !571
  %77 = icmp eq i32 %76, 0
  br i1 %77, label %bb9.i, label %bb58

bb58:                                             ; preds = %bb5, %bb71.thread295, %bb71
; call <u8>::from_ascii_radix
  %78 = tail call fastcc { i1, i8 } @_RNvMsx_NtCsjMrxcFdYDNN_4core3numh16from_ascii_radix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph, i64 noundef %_0.sroa.4.1.i.ph)
  %79 = extractvalue { i1, i8 } %78, 0
  br i1 %79, label %bb1.backedge, label %bb60

bb57:                                             ; preds = %bb71
  %_8.not.i.not = icmp eq i64 %_0.sroa.4.1.i.ph, 3
  br i1 %_8.not.i.not, label %bb6.i, label %bb9.i

bb6.i:                                            ; preds = %bb57
  br i1 %_3.not.i10130133135141144153157170175192197224229253254270309332334373388417424459467601606620625628631, label %bb107, label %bb106

bb9.i:                                            ; preds = %bb71.thread295, %bb57
  %80 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph, i64 3
  %self1.i = load i8, ptr %80, align 1, !alias.scope !575, !noundef !7
  %81 = icmp sgt i8 %self1.i, -65
  br i1 %81, label %bb107, label %bb106

bb60:                                             ; preds = %bb58
  %82 = extractvalue { i1, i8 } %78, 1
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 8, ptr %3, align 8
  store i8 %82, ptr %7, align 1
  br label %bb61

bb61:                                             ; preds = %bb6, %bb8, %bb10, %bb12, %bb14, %bb16, %bb18, %bb20, %bb22, %bb24, %bb26, %bb28, %bb30, %bb32, %bb34, %bb36, %bb38, %bb40, %bb42, %bb44, %bb46, %bb48, %bb50, %bb52, %bb54, %bb56, %bb59, %bb60
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %rv, ptr noundef nonnull align 8 dereferenceable(32) %_51, i64 32, i1 false)
  br label %bb1.backedge

bb1.backedge:                                     ; preds = %bb58, %bb107, %bb61
  br i1 %.ph.off0, label %bb4, label %bb2.i

bb107:                                            ; preds = %bb9.i, %bb6.i
  %new_len.i112 = add i64 %_0.sroa.4.1.i.ph, -3
  %data.i113 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph, i64 3
; call <u8>::from_ascii_radix
  %83 = tail call fastcc { i1, i8 } @_RNvMsx_NtCsjMrxcFdYDNN_4core3numh16from_ascii_radix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i113, i64 noundef %new_len.i112)
  %84 = extractvalue { i1, i8 } %83, 0
  br i1 %84, label %bb1.backedge, label %bb59

bb106:                                            ; preds = %bb9.i, %bb6.i
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph, i64 noundef %_0.sroa.4.1.i.ph, i64 noundef 3, i64 noundef %_0.sroa.4.1.i.ph, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9749b5a9842556100920529e544f7dc1) #30
          to label %unreachable unwind label %bb66.loopexit.split-lp

unreachable:                                      ; preds = %bb106
  unreachable

bb59:                                             ; preds = %bb107
  %85 = extractvalue { i1, i8 } %83, 1
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_51, ptr noundef nonnull align 8 dereferenceable(32) %rv, i64 32, i1 false)
  store i8 8, ptr %5, align 2
  store i8 %85, ptr %8, align 1
  br label %bb61

bb65:                                             ; preds = %bb66
  resume { ptr, i32 } %lpad.phi

bb66.loopexit:                                    ; preds = %bb2.i.us.i.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %bb66

bb66.loopexit.split-lp:                           ; preds = %bb106
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %bb66

bb66:                                             ; preds = %bb66.loopexit.split-lp, %bb66.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %bb66.loopexit ], [ %lpad.loopexit.split-lp, %bb66.loopexit.split-lp ]
; invoke core::ptr::drop_in_place::<console::utils::Style>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console5utils5StyleEBK_(ptr noalias noundef align 8 dereferenceable(32) %rv) #32
          to label %bb65 unwind label %terminate

terminate:                                        ; preds = %bb66
  %86 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; <console::utils::Style>::attr
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RNvMs1_NtCsC3JuwEIQwb_7console5utilsNtB5_5Style4attr(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(32) %_0, ptr dead_on_return noalias noundef nonnull align 8 captures(none) dereferenceable(32) %self, i8 noundef range(i8 0, 9) %attr) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !578)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !581)
  %0 = load ptr, ptr %self, align 8, !alias.scope !584, !noalias !585, !noundef !7
  %.not.i.i = icmp eq ptr %0, null
  br i1 %.not.i.i, label %bb3.i11.i, label %bb2.i.i

bb2.i.i:                                          ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_33.i.i = load i64, ptr %1, align 8, !alias.scope !584, !noalias !585, !noundef !7
  %__self_discr.i.i.i.i.i = zext nneg i8 %attr to i64
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb8.i.i.i, %bb2.i.i
  %self.sroa.3.0.i.i.i = phi i64 [ %_33.i.i, %bb2.i.i ], [ %_27.i.i.i, %bb8.i.i.i ]
  %self.sroa.0.0.i.i.i = phi ptr [ %0, %bb2.i.i ], [ %node.i.i.i, %bb8.i.i.i ]
  %_22.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.0.i.i.i, i64 12
  %2 = getelementptr inbounds nuw i8, ptr %self.sroa.0.0.i.i.i, i64 10
  %small.i.i.i.i = load i16, ptr %2, align 2, !noalias !587, !noundef !7
  %index.i.i.i.i = zext i16 %small.i.i.i.i to i64
  %_39.i.i.i.i = getelementptr inbounds nuw i8, ptr %_22.i.i.i.i, i64 %index.i.i.i.i
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb4.i.i.i.i, %bb1.i.i.i
  %iter.sroa.8.0.i.i.i.i = phi i64 [ 0, %bb1.i.i.i ], [ %_8.0.i.i.i.i.i, %bb4.i.i.i.i ]
  %iter.sroa.0.0.i.i.i.i = phi ptr [ %_22.i.i.i.i, %bb1.i.i.i ], [ %_16.i.i.i.i.i.i, %bb4.i.i.i.i ]
  %_6.i.i.i.i.i.i = icmp eq ptr %iter.sroa.0.0.i.i.i.i, %_39.i.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb5.i.i.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0.i.i.i.i, i64 1
  %_8.0.i.i.i.i.i = add nuw nsw i64 %iter.sroa.8.0.i.i.i.i, 1
  %_15.val.i.i.i.i = load i8, ptr %iter.sroa.0.0.i.i.i.i, align 1, !range !590, !noalias !587, !noundef !7
  %__arg1_discr.i.i.i.i.i = zext nneg i8 %_15.val.i.i.i.i to i64
  %_0.i.i.i.i.i = tail call noundef range(i8 -1, 2) i8 @llvm.scmp.i8.i64(i64 %__self_discr.i.i.i.i.i, i64 %__arg1_discr.i.i.i.i.i)
  switch i8 %_0.i.i.i.i.i, label %bb3.i.i.i.i [
    i8 -1, label %bb5.i.i.i
    i8 0, label %bb3
    i8 1, label %bb1.i.i.i.i
  ]

bb3.i.i.i.i:                                      ; preds = %bb4.i.i.i.i
  unreachable

bb5.i.i.i:                                        ; preds = %bb4.i.i.i.i, %bb1.i.i.i.i
  %_0.sroa.4.0.i.ph.i.i.i = phi i64 [ %index.i.i.i.i, %bb1.i.i.i.i ], [ %iter.sroa.8.0.i.i.i.i, %bb4.i.i.i.i ]
  %3 = icmp eq i64 %self.sroa.3.0.i.i.i, 0
  br i1 %3, label %bb2.i6.i, label %bb8.i.i.i

bb8.i.i.i:                                        ; preds = %bb5.i.i.i
  %_26.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.0.i.i.i, i64 24
  %_30.i.i.i = icmp samesign ult i64 %_0.sroa.4.0.i.ph.i.i.i, 12
  tail call void @llvm.assume(i1 %_30.i.i.i)
  %self10.i.i.i = getelementptr inbounds nuw ptr, ptr %_26.i.i.i, i64 %_0.sroa.4.0.i.ph.i.i.i
  %node.i.i.i = load ptr, ptr %self10.i.i.i, align 8, !noalias !587, !nonnull !7, !noundef !7
  %_27.i.i.i = add i64 %self.sroa.3.0.i.i.i, -1
  br label %bb1.i.i.i

bb2.i6.i:                                         ; preds = %bb5.i.i.i
  %4 = getelementptr inbounds nuw i8, ptr %self.sroa.0.0.i.i.i, i64 10
  %_5.i.i.i.i = icmp ult i16 %small.i.i.i.i, 11
  br i1 %_5.i.i.i.i, label %bb1.i.i.i9.i, label %bb3.i.i.i7.i

bb3.i.i.i7.i:                                     ; preds = %bb2.i6.i
  %_45.i.i.i.i = icmp samesign ult i64 %_0.sroa.4.0.i.ph.i.i.i, 5
  br i1 %_45.i.i.i.i, label %bb19.i.i.i.i, label %bb22.i.i.i.i

bb1.i.i.i9.i:                                     ; preds = %bb2.i6.i
  %_6.i.not.i.i.i.i.not.i = icmp samesign ult i64 %_0.sroa.4.0.i.ph.i.i.i, %index.i.i.i.i
  br i1 %_6.i.not.i.i.i.i.not.i, label %bb1.i.i.i.i.i.i, label %bb5.i.i10.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i9.i
  %_10.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_22.i.i.i.i, i64 %_0.sroa.4.0.i.ph.i.i.i
  %dst.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i.i.i.i, i64 1
  %count1.i.i.i.i.i.i = sub nuw nsw i64 %index.i.i.i.i, %_0.sroa.4.0.i.ph.i.i.i
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 1 %dst.i.i.i.i.i.i, ptr nonnull align 1 %_10.i.i.i.i.i.i, i64 %count1.i.i.i.i.i.i, i1 false), !alias.scope !591, !noalias !594
  br label %bb5.i.i10.i

bb22.i.i.i.i:                                     ; preds = %bb3.i.i.i7.i
  switch i64 %_0.sroa.4.0.i.ph.i.i.i, label %bb20.i.i.i.i [
    i64 5, label %bb19.i.i.i.i
    i64 6, label %bb24.i.i.i.i
  ]

bb20.i.i.i.i:                                     ; preds = %bb22.i.i.i.i
  %_47.i.i.i.i = add nsw i64 %_0.sroa.4.0.i.ph.i.i.i, -7
  br label %bb19.i.i.i.i

bb24.i.i.i.i:                                     ; preds = %bb22.i.i.i.i
  br label %bb19.i.i.i.i

bb19.i.i.i.i:                                     ; preds = %bb24.i.i.i.i, %bb20.i.i.i.i, %bb22.i.i.i.i, %bb3.i.i.i7.i
  %insertion.sroa.0.0.i.i.i.i = phi i1 [ true, %bb20.i.i.i.i ], [ true, %bb24.i.i.i.i ], [ false, %bb3.i.i.i7.i ], [ false, %bb22.i.i.i.i ]
  %insertion.sroa.5.0.i.i.i.i = phi i64 [ %_47.i.i.i.i, %bb20.i.i.i.i ], [ 0, %bb24.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i, %bb3.i.i.i7.i ], [ %_0.sroa.4.0.i.ph.i.i.i, %bb22.i.i.i.i ]
  %middle_kv_idx.sroa.0.0.i.i.i.i = phi i64 [ 6, %bb20.i.i.i.i ], [ 5, %bb24.i.i.i.i ], [ 4, %bb3.i.i.i7.i ], [ %_0.sroa.4.0.i.ph.i.i.i, %bb22.i.i.i.i ]
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !607
; call __rustc::__rust_alloc
  %5 = tail call noalias noundef align 8 dereferenceable_or_null(24) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 24, i64 noundef range(i64 1, -9223372036854775807) 8) #29, !noalias !607
  %6 = icmp eq ptr %5, null
  br i1 %6, label %bb3.i.i11.i.i.invoke, label %_RINvMNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB3_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB5_7set_val9SetValZSTE3newNtNtB9_5alloc6GlobalEB1a_.exit.i.i.i.i.i, !prof !5

_RINvMNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB3_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB5_7set_val9SetValZSTE3newNtNtB9_5alloc6GlobalEB1a_.exit.i.i.i.i.i: ; preds = %bb19.i.i.i.i
  store ptr null, ptr %5, align 8, !noalias !607
  %7 = getelementptr inbounds nuw i8, ptr %5, i64 10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !611)
  %_33.i.i.i.i.i.i = load i16, ptr %4, align 2, !noalias !614, !noundef !7
  %old_len.i.i.i.i.i.i = zext i16 %_33.i.i.i.i.i.i to i64
  %8 = xor i64 %middle_kv_idx.sroa.0.0.i.i.i.i, -1
  %new_len.i.i.i.i.i.i = add nsw i64 %old_len.i.i.i.i.i.i, %8
  %9 = trunc i64 %new_len.i.i.i.i.i.i to i16
  store i16 %9, ptr %7, align 2, !alias.scope !611, !noalias !607
  %_67.i.i.i.i.i.i = icmp ult i64 %new_len.i.i.i.i.i.i, 12
  br i1 %_67.i.i.i.i.i.i, label %_RINvMsV_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_.exit.i.i.i.i, label %bb15.i.i.i.i.i.i, !prof !615

bb15.i.i.i.i.i.i:                                 ; preds = %_RINvMNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB3_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB5_7set_val9SetValZSTE3newNtNtB9_5alloc6GlobalEB1a_.exit.i.i.i.i.i
; invoke core::slice::index::slice_index_fail
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %new_len.i.i.i.i.i.i, i64 noundef 11, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b0d6f4768d6dbcaaba9bf9a6c0de2b5f) #30
          to label %.noexc.i.i.i.i.i unwind label %bb6.i.i.i.i.i, !noalias !607

.noexc.i.i.i.i.i:                                 ; preds = %bb15.i.i.i.i.i.i
  unreachable

bb6.i.i.i.i.i:                                    ; preds = %bb15.i.i.i.i.i.i
  %10 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %5, i64 noundef 24, i64 noundef 8) #29, !noalias !607
  br label %cleanup.body

_RINvMsV_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_.exit.i.i.i.i: ; preds = %_RINvMNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB3_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB5_7set_val9SetValZSTE3newNtNtB9_5alloc6GlobalEB1a_.exit.i.i.i.i.i
  %self2.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_22.i.i.i.i, i64 %middle_kv_idx.sroa.0.0.i.i.i.i
  %k.i.i.i.i.i.i = load i8, ptr %self2.i.i.i.i.i.i, align 1, !range !590, !noalias !614, !noundef !7
  %_65.i.i.i.i.i.i = getelementptr i8, ptr %self2.i.i.i.i.i.i, i64 1
  %self9.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %5, i64 12
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %self9.i.i.i.i.i.i, ptr nonnull readonly align 1 %_65.i.i.i.i.i.i, i64 range(i64 0, 65536) %new_len.i.i.i.i.i.i, i1 false), !alias.scope !616, !noalias !607
  %11 = trunc nuw nsw i64 %middle_kv_idx.sroa.0.0.i.i.i.i to i16
  store i16 %11, ptr %4, align 2, !noalias !614
  %spec.select.i.i.i.i = select i1 %insertion.sroa.0.0.i.i.i.i, ptr %5, ptr %self.sroa.0.0.i.i.i
  %12 = getelementptr inbounds nuw i8, ptr %spec.select.i.i.i.i, i64 10
  %_19.i8.i.i.i.i = load i16, ptr %12, align 2, !noalias !620, !noundef !7
  %_6.i9.i.i.i.i = zext i16 %_19.i8.i.i.i.i to i64
  %self1.i10.i.i.i.i = getelementptr inbounds nuw i8, ptr %spec.select.i.i.i.i, i64 12
  %_6.i.not.i13.not.i.i.i.i = icmp ult i64 %insertion.sroa.5.0.i.i.i.i, %_6.i9.i.i.i.i
  br i1 %_6.i.not.i13.not.i.i.i.i, label %bb1.i.i14.i.i.i.i, label %bb4.i.i.i

bb1.i.i14.i.i.i.i:                                ; preds = %_RINvMsV_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_.exit.i.i.i.i
  %_10.i.i15.i.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i10.i.i.i.i, i64 %insertion.sroa.5.0.i.i.i.i
  %dst.i.i16.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i15.i.i.i.i, i64 1
  %count1.i.i17.i.i.i.i = sub nuw nsw i64 %_6.i9.i.i.i.i, %insertion.sroa.5.0.i.i.i.i
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 1 %dst.i.i16.i.i.i.i, ptr nonnull align 1 %_10.i.i15.i.i.i.i, i64 %count1.i.i17.i.i.i.i, i1 false), !alias.scope !624, !noalias !620
  br label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb1.i.i14.i.i.i.i, %_RINvMsV_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_.exit.i.i.i.i
  %new_len.i18.i.i.i.i = add i16 %_19.i8.i.i.i.i, 1
  %self.i.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i10.i.i.i.i, i64 %insertion.sroa.5.0.i.i.i.i
  store i8 %attr, ptr %self.i.i19.i.i.i.i, align 1, !alias.scope !624, !noalias !620
  store i16 %new_len.i18.i.i.i.i, ptr %12, align 2, !noalias !620
  %13 = load ptr, ptr %self.sroa.0.0.i.i.i, align 8, !noalias !627, !noundef !7
  %.not.i74.i.i.i = icmp eq ptr %13, null
  br i1 %.not.i74.i.i.i, label %bb10.i.i.i.i, label %bb9.i.i.i

bb5.i.i10.i:                                      ; preds = %bb1.i.i.i.i.i.i, %bb1.i.i.i9.i
  %new_len.i.i.i.i.i = add nuw nsw i16 %small.i.i.i.i, 1
  %self.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_22.i.i.i.i, i64 %_0.sroa.4.0.i.ph.i.i.i
  store i8 %attr, ptr %self.i.i.i.i.i.i, align 1, !alias.scope !591, !noalias !594
  store i16 %new_len.i.i.i.i.i, ptr %4, align 2, !noalias !594
  br label %_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_.exit.i

bb10.i.i.i.i:                                     ; preds = %bb12.i.i.i, %bb4.i.i.i
  %split.sroa.11.0.lcssa.i.i.i = phi i8 [ %k.i.i.i.i.i.i, %bb4.i.i.i ], [ %k.i.i.i49.i.i.i, %bb12.i.i.i ]
  %split.sroa.9.0.lcssa.i.i.i = phi i64 [ 0, %bb4.i.i.i ], [ %_18.i.i.i.i, %bb12.i.i.i ]
  %split.sroa.7.0.lcssa.i.i.i = phi ptr [ %5, %bb4.i.i.i ], [ %30, %bb12.i.i.i ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !630)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !633
; call __rustc::__rust_alloc
  %14 = tail call noalias noundef align 8 dereferenceable_or_null(120) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 120, i64 noundef range(i64 1, -9223372036854775807) 8) #29, !noalias !633
  %15 = icmp eq ptr %14, null
  br i1 %15, label %bb3.i.i.i.i.i.i.i.i.i.i, label %_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_12InternalNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE3newNtNtBb_5alloc6GlobalEB1h_.exit.i.i.i.i.i.i.i.i, !prof !5

bb3.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb10.i.i.i.i
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 120) #30
          to label %.noexc.i.i28.i.i.i unwind label %cleanup.i.i.i.i.i, !noalias !633

.noexc.i.i28.i.i.i:                               ; preds = %bb3.i.i.i.i.i.i.i.i.i.i
  unreachable

_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_12InternalNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE3newNtNtBb_5alloc6GlobalEB1h_.exit.i.i.i.i.i.i.i.i: ; preds = %bb10.i.i.i.i
  store ptr null, ptr %14, align 8, !noalias !633
  %16 = getelementptr inbounds nuw i8, ptr %14, i64 10
  store i16 0, ptr %16, align 2, !noalias !633
  %17 = getelementptr inbounds nuw i8, ptr %14, i64 24
  store ptr %0, ptr %17, align 8, !noalias !633
  %n.i.i.i.i.i.i.i.i = add i64 %_33.i.i, 1
  %.not.i.i.i.i.i.i.i.i = icmp eq i64 %n.i.i.i.i.i.i.i.i, 0
  br i1 %.not.i.i.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i.i.i, label %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node7NodeRefNtNtB10_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1k_14LeafOrInternalEuNCINvB2_8take_mutBX_NCINvMss_B10_BX_19push_internal_levelNtNtB8_5alloc6GlobalE0E0EB1H_.exit.i.i.i.i, !prof !5

bb9.i.i.i.i.i.i.i.i:                              ; preds = %_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_12InternalNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE3newNtNtBb_5alloc6GlobalEB1h_.exit.i.i.i.i.i.i.i.i
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_36e9d3034d1040a1b97b71beadbdf183) #30
          to label %unreachable.i.i.i.i.i.i.i.i unwind label %bb3.i.i.i.i.i.i.i.i, !noalias !633

unreachable.i.i.i.i.i.i.i.i:                      ; preds = %bb9.i.i.i.i.i.i.i.i
  unreachable

bb3.i.i.i.i.i.i.i.i:                              ; preds = %bb9.i.i.i.i.i.i.i.i
  %18 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %14, i64 noundef 120, i64 noundef 8) #29, !noalias !633
  br label %cleanup.body.i.i.i.i.i

cleanup.i.i.i.i.i:                                ; preds = %bb3.i.i.i.i.i.i.i.i.i.i
  %19 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i.i.i.i.i

cleanup.body.i.i.i.i.i:                           ; preds = %cleanup.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i
  tail call void @llvm.trap()
  unreachable

_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node7NodeRefNtNtB10_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1k_14LeafOrInternalEuNCINvB2_8take_mutBX_NCINvMss_B10_BX_19push_internal_levelNtNtB8_5alloc6GlobalE0E0EB1H_.exit.i.i.i.i: ; preds = %_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_12InternalNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE3newNtNtBb_5alloc6GlobalEB1h_.exit.i.i.i.i.i.i.i.i
  store ptr %14, ptr %0, align 8, !noalias !636
  %20 = getelementptr inbounds nuw i8, ptr %0, i64 8
  store i16 0, ptr %20, align 8, !noalias !641
  store ptr %14, ptr %self, align 8, !alias.scope !642, !noalias !643
  store i64 %n.i.i.i.i.i.i.i.i, ptr %1, align 8, !alias.scope !642, !noalias !643
  %_5.i.i.i.i.i = icmp eq i64 %split.sroa.9.0.lcssa.i.i.i, %_33.i.i
  br i1 %_5.i.i.i.i.i, label %_RNCNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB7_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBb_7set_val9SetValZSTE12insert_entry0B1p_.exit.i.i.i, label %bb2.i.i.i.i.i, !prof !30

bb2.i.i.i.i.i:                                    ; preds = %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node7NodeRefNtNtB10_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1k_14LeafOrInternalEuNCINvB2_8take_mutBX_NCINvMss_B10_BX_19push_internal_levelNtNtB8_5alloc6GlobalE0E0EB1H_.exit.i.i.i.i
; invoke core::panicking::panic
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_50679f783fca94dab7457816f752eff0, i64 noundef 48, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6f083584960555ed8732225b7b53e5b1) #30
          to label %.noexc1 unwind label %cleanup

.noexc1:                                          ; preds = %bb2.i.i.i.i.i
  unreachable

_RNCNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB7_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBb_7set_val9SetValZSTE12insert_entry0B1p_.exit.i.i.i: ; preds = %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node7NodeRefNtNtB10_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1k_14LeafOrInternalEuNCINvB2_8take_mutBX_NCINvMss_B10_BX_19push_internal_levelNtNtB8_5alloc6GlobalE0E0EB1H_.exit.i.i.i.i
  store i16 1, ptr %16, align 2, !noalias !644
  %self1.i.i27.i.i.i = getelementptr inbounds nuw i8, ptr %14, i64 12
  store i8 %split.sroa.11.0.lcssa.i.i.i, ptr %self1.i.i27.i.i.i, align 4, !noalias !644
  %self6.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %14, i64 32
  store ptr %split.sroa.7.0.lcssa.i.i.i, ptr %self6.i.i.i.i.i, align 8, !noalias !644
  store ptr %14, ptr %split.sroa.7.0.lcssa.i.i.i, align 8, !noalias !644
  %21 = getelementptr inbounds nuw i8, ptr %split.sroa.7.0.lcssa.i.i.i, i64 8
  store i16 1, ptr %21, align 8, !noalias !644
  br label %_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_.exit.i

bb9.i.i.i:                                        ; preds = %bb4.i.i.i, %bb12.i.i.i
  %22 = phi ptr [ %47, %bb12.i.i.i ], [ %13, %bb4.i.i.i ]
  %split.sroa.0.079.i.i.i = phi ptr [ %22, %bb12.i.i.i ], [ %self.sroa.0.0.i.i.i, %bb4.i.i.i ]
  %split.sroa.6.078.i.i.i = phi i64 [ %_18.i.i.i.i, %bb12.i.i.i ], [ 0, %bb4.i.i.i ]
  %split.sroa.7.077.i.i.i = phi ptr [ %30, %bb12.i.i.i ], [ %5, %bb4.i.i.i ]
  %split.sroa.11.075.i.i.i = phi i8 [ %k.i.i.i49.i.i.i, %bb12.i.i.i ], [ %k.i.i.i.i.i.i, %bb4.i.i.i ]
  %_18.i.i.i.i = add i64 %split.sroa.6.078.i.i.i, 1
  %23 = getelementptr inbounds nuw i8, ptr %split.sroa.0.079.i.i.i, i64 8
  %24 = load i16, ptr %23, align 8, !noalias !627
  %_20.i.i.i.i = zext i16 %24 to i64
  %25 = getelementptr inbounds nuw i8, ptr %22, i64 10
  %_39.i32.i.i.i = load i16, ptr %25, align 2, !noalias !645, !noundef !7
  %_11.i.i.i.i = icmp ult i16 %_39.i32.i.i.i, 11
  br i1 %_11.i.i.i.i, label %bb3.i57.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb9.i.i.i
  %_43.i.i.i.i = icmp ult i16 %24, 5
  br i1 %_43.i.i.i.i, label %bb21.i.i.i.i, label %bb24.i34.i.i.i

bb3.i57.i.i.i:                                    ; preds = %bb9.i.i.i
  %_7.i.i.i.i.i = zext nneg i16 %_39.i32.i.i.i to i64
  %new_len.i.i58.i.i.i = add nuw nsw i16 %_39.i32.i.i.i, 1
  %self1.i.i59.i.i.i = getelementptr inbounds nuw i8, ptr %22, i64 12
  %count.i.i.i60.i.i.i = add nuw nsw i64 %_20.i.i.i.i, 1
  %_6.i.not.i.i61.not.i.i.i = icmp ult i16 %24, %_39.i32.i.i.i
  %_10.i.i.i63.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i.i59.i.i.i, i64 %_20.i.i.i.i
  br i1 %_6.i.not.i.i61.not.i.i.i, label %bb1.i8.i.i.i.i.i, label %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i.i.i.i.i

bb1.i8.i.i.i.i.i:                                 ; preds = %bb3.i57.i.i.i
  %dst.i.i.i64.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i.i59.i.i.i, i64 %count.i.i.i60.i.i.i
  %count1.i.i.i65.i.i.i = sub nsw i64 %_7.i.i.i.i.i, %_20.i.i.i.i
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 1 %dst.i.i.i64.i.i.i, ptr nonnull align 1 %_10.i.i.i63.i.i.i, i64 %count1.i.i.i65.i.i.i, i1 false), !alias.scope !649, !noalias !652
  %self3.i.i144.i.i.i = getelementptr inbounds nuw i8, ptr %22, i64 24
  %_10.i9.i.i.i.i.i = getelementptr inbounds nuw ptr, ptr %self3.i.i144.i.i.i, i64 %count.i.i.i60.i.i.i
  %26 = getelementptr inbounds nuw ptr, ptr %self3.i.i144.i.i.i, i64 %_20.i.i.i.i
  %dst.i10.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %26, i64 16
  %27 = shl nsw i64 %count1.i.i.i65.i.i.i, 3
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 8 %dst.i10.i.i.i.i.i, ptr nonnull align 8 %_10.i9.i.i.i.i.i, i64 %27, i1 false), !alias.scope !655, !noalias !652
  br label %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i.i.i.i.i

_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i.i.i.i.i: ; preds = %bb1.i8.i.i.i.i.i, %bb3.i57.i.i.i
  store i8 %split.sroa.11.075.i.i.i, ptr %_10.i.i.i63.i.i.i, align 1, !alias.scope !649, !noalias !652
  %28 = getelementptr inbounds nuw i8, ptr %22, i64 24
  %index.i.i145.i.i.i = add nuw nsw i64 %_7.i.i.i.i.i, 2
  %self.i7.i.i.i.i.i = getelementptr inbounds nuw ptr, ptr %28, i64 %count.i.i.i60.i.i.i
  store ptr %split.sroa.7.077.i.i.i, ptr %self.i7.i.i.i.i.i, align 8, !alias.scope !655, !noalias !652
  store i16 %new_len.i.i58.i.i.i, ptr %25, align 2, !noalias !652
  %_0.i.i.i4.i.i.i.i.i.i = icmp samesign ult i64 %count.i.i.i60.i.i.i, %index.i.i145.i.i.i
  br i1 %_0.i.i.i4.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i, label %_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_.exit.i

bb4.i.i.i.i.i.i:                                  ; preds = %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i.i.i.i.i, %bb4.i.i.i.i.i.i
  %iter.sroa.0.05.i.i.i.i.i.i = phi i64 [ %_0.i1.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %count.i.i.i60.i.i.i, %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i.i.i.i.i ]
  %_0.i1.i.i.i.i.i.i.i.i = add nuw nsw i64 %iter.sroa.0.05.i.i.i.i.i.i, 1
  %_19.i.i.i.i.i.i.i = icmp samesign ult i64 %iter.sroa.0.05.i.i.i.i.i.i, 12
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i.i.i)
  %self3.i.i.i.i.i.i.i = getelementptr inbounds nuw ptr, ptr %28, i64 %iter.sroa.0.05.i.i.i.i.i.i
  %node.i.i.i.i.i.i.i = load ptr, ptr %self3.i.i.i.i.i.i.i, align 8, !noalias !652, !nonnull !7, !noundef !7
  store ptr %22, ptr %node.i.i.i.i.i.i.i, align 8, !noalias !652
  %_23.i.i.i.i.i.i.i = trunc nuw nsw i64 %iter.sroa.0.05.i.i.i.i.i.i to i16
  %29 = getelementptr inbounds nuw i8, ptr %node.i.i.i.i.i.i.i, i64 8
  store i16 %_23.i.i.i.i.i.i.i, ptr %29, align 8, !noalias !652
  %exitcond.not.i.i.i.i.i.i = icmp eq i64 %_0.i1.i.i.i.i.i.i.i.i, %index.i.i145.i.i.i
  br i1 %exitcond.not.i.i.i.i.i.i, label %_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_.exit.i, label %bb4.i.i.i.i.i.i

bb24.i34.i.i.i:                                   ; preds = %bb5.i.i.i.i
  switch i16 %24, label %bb22.i55.i.i.i [
    i16 5, label %bb21.i.i.i.i
    i16 6, label %bb26.i.i.i.i
  ]

bb22.i55.i.i.i:                                   ; preds = %bb24.i34.i.i.i
  %_45.i56.i.i.i = add nsw i64 %_20.i.i.i.i, -7
  br label %bb21.i.i.i.i

bb26.i.i.i.i:                                     ; preds = %bb24.i34.i.i.i
  br label %bb21.i.i.i.i

bb21.i.i.i.i:                                     ; preds = %bb26.i.i.i.i, %bb22.i55.i.i.i, %bb24.i34.i.i.i, %bb5.i.i.i.i
  %middle_kv_idx.sroa.0.0.i35.i.i.i = phi i64 [ 6, %bb22.i55.i.i.i ], [ 5, %bb26.i.i.i.i ], [ 4, %bb5.i.i.i.i ], [ 5, %bb24.i34.i.i.i ]
  %insertion.sroa.5.0.i36.i.i.i = phi i64 [ %_45.i56.i.i.i, %bb22.i55.i.i.i ], [ 0, %bb26.i.i.i.i ], [ %_20.i.i.i.i, %bb5.i.i.i.i ], [ 5, %bb24.i34.i.i.i ]
  %insertion.sroa.0.0.i37.i.i.i = phi i1 [ true, %bb22.i55.i.i.i ], [ true, %bb26.i.i.i.i ], [ false, %bb5.i.i.i.i ], [ false, %bb24.i34.i.i.i ]
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !658
; call __rustc::__rust_alloc
  %30 = tail call noalias noundef align 8 dereferenceable_or_null(120) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 120, i64 noundef range(i64 1, -9223372036854775807) 8) #29, !noalias !658
  %31 = icmp eq ptr %30, null
  br i1 %31, label %bb3.i.i11.i.i.invoke, label %_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_12InternalNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE3newNtNtBb_5alloc6GlobalEB1h_.exit.i.i.i.i.i, !prof !5

_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_12InternalNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE3newNtNtBb_5alloc6GlobalEB1h_.exit.i.i.i.i.i: ; preds = %bb21.i.i.i.i
  store ptr null, ptr %30, align 8, !noalias !658
  %32 = getelementptr inbounds nuw i8, ptr %30, i64 10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !662)
  %_33.i.i.i38.i.i.i = load i16, ptr %25, align 2, !noalias !665, !noundef !7
  %old_len.i.i.i39.i.i.i = zext i16 %_33.i.i.i38.i.i.i to i64
  %33 = xor i64 %middle_kv_idx.sroa.0.0.i35.i.i.i, -1
  %new_len.i.i.i40.i.i.i = add nsw i64 %old_len.i.i.i39.i.i.i, %33
  %34 = trunc i64 %new_len.i.i.i40.i.i.i to i16
  store i16 %34, ptr %32, align 2, !alias.scope !662, !noalias !658
  %_67.i.i.i41.i.i.i = icmp ult i64 %new_len.i.i.i40.i.i.i, 12
  br i1 %_67.i.i.i41.i.i.i, label %bb2.i.i46.i.i.i, label %bb15.i.i.i42.i.i.i, !prof !615

bb15.i.i.i42.i.i.i:                               ; preds = %_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_12InternalNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE3newNtNtBb_5alloc6GlobalEB1h_.exit.i.i.i.i.i
; invoke core::slice::index::slice_index_fail
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %new_len.i.i.i40.i.i.i, i64 noundef 11, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b0d6f4768d6dbcaaba9bf9a6c0de2b5f) #30
          to label %.noexc.i.i45.i.i.i unwind label %cleanup.i.i43.i.i.i, !noalias !658

.noexc.i.i45.i.i.i:                               ; preds = %bb15.i.i.i42.i.i.i
  unreachable

cleanup.i.i43.i.i.i:                              ; preds = %bb15.i.i.i42.i.i.i
  %35 = landingpad { ptr, i32 }
          cleanup
  br label %bb6.i.i44.i.i.i

bb2.i.i46.i.i.i:                                  ; preds = %_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_12InternalNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE3newNtNtBb_5alloc6GlobalEB1h_.exit.i.i.i.i.i
  %self1.i.i.i47.i.i.i = getelementptr inbounds nuw i8, ptr %22, i64 12
  %self2.i.i.i48.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i.i.i47.i.i.i, i64 %middle_kv_idx.sroa.0.0.i35.i.i.i
  %k.i.i.i49.i.i.i = load i8, ptr %self2.i.i.i48.i.i.i, align 1, !range !590, !noalias !665, !noundef !7
  %_65.i.i.i50.i.i.i = getelementptr i8, ptr %self2.i.i.i48.i.i.i, i64 1
  %self9.i.i.i51.i.i.i = getelementptr inbounds nuw i8, ptr %30, i64 12
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 4 %self9.i.i.i51.i.i.i, ptr nonnull readonly align 1 %_65.i.i.i50.i.i.i, i64 range(i64 0, 65536) %new_len.i.i.i40.i.i.i, i1 false), !alias.scope !666, !noalias !658
  %36 = trunc nuw nsw i64 %middle_kv_idx.sroa.0.0.i35.i.i.i to i16
  store i16 %36, ptr %25, align 2, !noalias !665
  %index5.i.i.i.i.i = sub nsw i64 %old_len.i.i.i39.i.i.i, %middle_kv_idx.sroa.0.0.i35.i.i.i
  %_44.i.i.i.i.i = icmp ult i16 %34, 12
  br i1 %_44.i.i.i.i.i, label %bb11.i.i.i.i.i, label %bb12.i.i.i.i.i, !prof !615

bb12.i.i.i.i.i:                                   ; preds = %bb2.i.i46.i.i.i
; invoke core::slice::index::slice_index_fail
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %index5.i.i.i.i.i, i64 noundef 12, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5c50b0573939a5b3b419995fc3e13e19) #30
          to label %unreachable.i.i.i.i.i unwind label %cleanup6.i.i.i.i.i, !noalias !658

bb11.i.i.i.i.i:                                   ; preds = %bb2.i.i46.i.i.i
  %self4.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %30, i64 24
  %_3.i.i.i.i.i.i = icmp eq i16 %_39.i32.i.i.i, %_33.i.i.i38.i.i.i
  br i1 %_3.i.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb2.i.i.i.i.i.i, !prof !30

bb2.i.i.i.i.i.i:                                  ; preds = %bb11.i.i.i.i.i
; invoke core::panicking::panic
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_930f3883875d82d5043e839301ab22e6, i64 noundef 40, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3d1b356cbfc916b26f15e548d269cb74) #34
          to label %.noexc10.i.i.i.i.i unwind label %cleanup6.i.i.i.i.i, !noalias !658

.noexc10.i.i.i.i.i:                               ; preds = %bb2.i.i.i.i.i.i
  unreachable

cleanup6.i.i.i.i.i:                               ; preds = %bb2.i.i.i.i.i.i, %bb12.i.i.i.i.i
  %37 = landingpad { ptr, i32 }
          cleanup
  br label %bb6.i.i44.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb11.i.i.i.i.i
  %38 = getelementptr i8, ptr %22, i64 32
  %_42.i.i.i.i.i = getelementptr ptr, ptr %38, i64 %middle_kv_idx.sroa.0.0.i35.i.i.i
  %39 = shl nuw nsw i64 %index5.i.i.i.i.i, 3
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %self4.i.i.i.i.i, ptr nonnull readonly align 8 %_42.i.i.i.i.i, i64 %39, i1 false), !alias.scope !670, !noalias !658
  %40 = icmp ne i64 %_18.i.i.i.i, 0
  tail call void @llvm.assume(i1 %40)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !674)
  br label %bb2.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i:                            ; preds = %bb2.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i
  %iter.sroa.0.012.i.i.i.i.i.i.i = phi i64 [ 0, %bb3.i.i.i.i.i ], [ %spec.select8.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i ]
  %_0.i3.i.i.i.i.i.i.i.i.i = icmp samesign uge i64 %iter.sroa.0.012.i.i.i.i.i.i.i, %new_len.i.i.i40.i.i.i
  %not._0.i3.i.i.i.i.i.i.i.i.i = xor i1 %_0.i3.i.i.i.i.i.i.i.i.i, true
  %_0.i4.i.i.i.i.i.i.i.i.i = zext i1 %not._0.i3.i.i.i.i.i.i.i.i.i to i64
  %spec.select8.i.i.i.i.i.i.i = add nuw nsw i64 %iter.sroa.0.012.i.i.i.i.i.i.i, %_0.i4.i.i.i.i.i.i.i.i.i
  %self3.i.i.i.i.i.i.i.i = getelementptr inbounds nuw ptr, ptr %self4.i.i.i.i.i, i64 %iter.sroa.0.012.i.i.i.i.i.i.i
  %node.i.i.i.i.i.i.i.i = load ptr, ptr %self3.i.i.i.i.i.i.i.i, align 8, !alias.scope !674, !noalias !677, !nonnull !7, !noundef !7
  store ptr %30, ptr %node.i.i.i.i.i.i.i.i, align 8, !noalias !680
  %_23.i.i.i.i.i.i.i.i = trunc nuw nsw i64 %iter.sroa.0.012.i.i.i.i.i.i.i to i16
  %41 = getelementptr inbounds nuw i8, ptr %node.i.i.i.i.i.i.i.i, i64 8
  store i16 %_23.i.i.i.i.i.i.i.i, ptr %41, align 8, !noalias !677
  %_0.i.not.i.i.i.i.i.i.i.i.i = icmp samesign ugt i64 %spec.select8.i.i.i.i.i.i.i, %new_len.i.i.i40.i.i.i
  %or.cond.i.i.i.i.i.i.i = select i1 %_0.i3.i.i.i.i.i.i.i.i.i, i1 true, i1 %_0.i.not.i.i.i.i.i.i.i.i.i
  br i1 %or.cond.i.i.i.i.i.i.i, label %_RINvMsW_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_.exit.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i

unreachable.i.i.i.i.i:                            ; preds = %bb12.i.i.i.i.i
  unreachable

bb6.i.i44.i.i.i:                                  ; preds = %cleanup6.i.i.i.i.i, %cleanup.i.i43.i.i.i
  %.pn.i.i.i.i.i = phi { ptr, i32 } [ %37, %cleanup6.i.i.i.i.i ], [ %35, %cleanup.i.i43.i.i.i ]
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %30, i64 noundef 120, i64 noundef 8) #29, !noalias !658
  br label %cleanup.body

_RINvMsW_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_.exit.i.i.i.i: ; preds = %bb2.i.i.i.i.i.i.i.i.i
  %spec.select.i52.i.i.i = select i1 %insertion.sroa.0.0.i37.i.i.i, ptr %30, ptr %22
  %42 = getelementptr inbounds nuw i8, ptr %spec.select.i52.i.i.i, i64 10
  %_32.i8.i.i.i.i = load i16, ptr %42, align 2, !noalias !681, !noundef !7
  %_7.i9.i.i.i.i = zext i16 %_32.i8.i.i.i.i to i64
  %new_len.i10.i.i.i.i = add i16 %_32.i8.i.i.i.i, 1
  %self1.i11.i.i.i.i = getelementptr inbounds nuw i8, ptr %spec.select.i52.i.i.i, i64 12
  %count.i.i13.i.i.i.i = add nuw nsw i64 %insertion.sroa.5.0.i36.i.i.i, 1
  %_6.i.not.i14.not.i.i.i.i = icmp samesign ult i64 %insertion.sroa.5.0.i36.i.i.i, %_7.i9.i.i.i.i
  %_10.i.i16.i.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i11.i.i.i.i, i64 %insertion.sroa.5.0.i36.i.i.i
  br i1 %_6.i.not.i14.not.i.i.i.i, label %bb1.i8.i36.i.i.i.i, label %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i25.i.i.i.i

bb1.i8.i36.i.i.i.i:                               ; preds = %_RINvMsW_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_.exit.i.i.i.i
  %dst.i.i17.i.i.i.i = getelementptr inbounds nuw i8, ptr %self1.i11.i.i.i.i, i64 %count.i.i13.i.i.i.i
  %count1.i.i18.i.i.i.i = sub nuw nsw i64 %_7.i9.i.i.i.i, %insertion.sroa.5.0.i36.i.i.i
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 1 %dst.i.i17.i.i.i.i, ptr nonnull align 1 %_10.i.i16.i.i.i.i, i64 %count1.i.i18.i.i.i.i, i1 false), !alias.scope !684, !noalias !681
  %self3.i2242.i.i.i.i = getelementptr inbounds nuw i8, ptr %spec.select.i52.i.i.i, i64 24
  %_10.i9.i37.i.i.i.i = getelementptr inbounds nuw ptr, ptr %self3.i2242.i.i.i.i, i64 %count.i.i13.i.i.i.i
  %43 = getelementptr inbounds nuw ptr, ptr %self3.i2242.i.i.i.i, i64 %insertion.sroa.5.0.i36.i.i.i
  %dst.i10.i38.i.i.i.i = getelementptr inbounds nuw i8, ptr %43, i64 16
  %44 = shl nuw nsw i64 %count1.i.i18.i.i.i.i, 3
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 8 %dst.i10.i38.i.i.i.i, ptr nonnull align 8 %_10.i9.i37.i.i.i.i, i64 %44, i1 false), !alias.scope !687, !noalias !681
  br label %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i25.i.i.i.i

_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i25.i.i.i.i: ; preds = %bb1.i8.i36.i.i.i.i, %_RINvMsW_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_.exit.i.i.i.i
  store i8 %split.sroa.11.075.i.i.i, ptr %_10.i.i16.i.i.i.i, align 1, !alias.scope !684, !noalias !681
  %45 = getelementptr inbounds nuw i8, ptr %spec.select.i52.i.i.i, i64 24
  %index.i2143.i.i.i.i = add nuw nsw i64 %_7.i9.i.i.i.i, 2
  %self.i7.i26.i.i.i.i = getelementptr inbounds nuw ptr, ptr %45, i64 %count.i.i13.i.i.i.i
  store ptr %split.sroa.7.077.i.i.i, ptr %self.i7.i26.i.i.i.i, align 8, !alias.scope !687, !noalias !681
  store i16 %new_len.i10.i.i.i.i, ptr %42, align 2, !noalias !681
  %_0.i.i.i4.i.i27.i.i.i.i = icmp samesign ult i64 %count.i.i13.i.i.i.i, %index.i2143.i.i.i.i
  br i1 %_0.i.i.i4.i.i27.i.i.i.i, label %bb4.i.i28.i.i.i.i, label %bb12.i.i.i

bb4.i.i28.i.i.i.i:                                ; preds = %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i25.i.i.i.i, %bb4.i.i28.i.i.i.i
  %iter.sroa.0.05.i.i29.i.i.i.i = phi i64 [ %_0.i1.i.i.i.i30.i.i.i.i, %bb4.i.i28.i.i.i.i ], [ %count.i.i13.i.i.i.i, %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i25.i.i.i.i ]
  %_0.i1.i.i.i.i30.i.i.i.i = add nuw nsw i64 %iter.sroa.0.05.i.i29.i.i.i.i, 1
  %_19.i.i.i31.i.i.i.i = icmp samesign ult i64 %iter.sroa.0.05.i.i29.i.i.i.i, 12
  tail call void @llvm.assume(i1 %_19.i.i.i31.i.i.i.i)
  %self3.i.i.i32.i.i.i.i = getelementptr inbounds nuw ptr, ptr %45, i64 %iter.sroa.0.05.i.i29.i.i.i.i
  %node.i.i.i33.i.i.i.i = load ptr, ptr %self3.i.i.i32.i.i.i.i, align 8, !noalias !681, !nonnull !7, !noundef !7
  store ptr %spec.select.i52.i.i.i, ptr %node.i.i.i33.i.i.i.i, align 8, !noalias !681
  %_23.i.i.i34.i.i.i.i = trunc nuw nsw i64 %iter.sroa.0.05.i.i29.i.i.i.i to i16
  %46 = getelementptr inbounds nuw i8, ptr %node.i.i.i33.i.i.i.i, i64 8
  store i16 %_23.i.i.i34.i.i.i.i, ptr %46, align 8, !noalias !681
  %exitcond.not.i.i35.i.i.i.i = icmp eq i64 %_0.i1.i.i.i.i30.i.i.i.i, %index.i2143.i.i.i.i
  br i1 %exitcond.not.i.i35.i.i.i.i, label %bb12.i.i.i, label %bb4.i.i28.i.i.i.i

bb12.i.i.i:                                       ; preds = %bb4.i.i28.i.i.i.i, %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i25.i.i.i.i
  %47 = load ptr, ptr %22, align 8, !noalias !627, !noundef !7
  %.not.i.i.i.i = icmp eq ptr %47, null
  br i1 %.not.i.i.i.i, label %bb10.i.i.i.i, label %bb9.i.i.i

bb3.i11.i:                                        ; preds = %start
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !690
; call __rustc::__rust_alloc
  %48 = tail call noalias noundef align 8 dereferenceable_or_null(24) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 24, i64 noundef range(i64 1, -9223372036854775807) 8) #29, !noalias !690
  %49 = icmp eq ptr %48, null
  br i1 %49, label %bb3.i.i11.i.i.invoke, label %_RINvMNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB3_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB5_7set_val9SetValZSTE3newNtNtB9_5alloc6GlobalEB1a_.exit.i.i, !prof !5

bb3.i.i11.i.i.invoke:                             ; preds = %bb21.i.i.i.i, %bb3.i11.i, %bb19.i.i.i.i
  %50 = phi i64 [ 24, %bb19.i.i.i.i ], [ 24, %bb3.i11.i ], [ 120, %bb21.i.i.i.i ]
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef %50) #30
          to label %bb3.i.i11.i.i.cont unwind label %cleanup

bb3.i.i11.i.i.cont:                               ; preds = %bb3.i.i11.i.i.invoke
  unreachable

_RINvMNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB3_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB5_7set_val9SetValZSTE3newNtNtB9_5alloc6GlobalEB1a_.exit.i.i: ; preds = %bb3.i11.i
  store ptr null, ptr %48, align 8, !noalias !690
  %51 = getelementptr inbounds nuw i8, ptr %48, i64 10
  store ptr %48, ptr %self, align 8, !alias.scope !578, !noalias !691
  %52 = getelementptr inbounds nuw i8, ptr %self, i64 8
  store i64 0, ptr %52, align 8, !alias.scope !578, !noalias !691
  store i16 1, ptr %51, align 2, !noalias !692
  %self1.i.i.i = getelementptr inbounds nuw i8, ptr %48, i64 12
  store i8 %attr, ptr %self1.i.i.i, align 4, !noalias !692
  br label %_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_.exit.i

_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_.exit.i: ; preds = %bb4.i.i.i.i.i.i, %_RINvMNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB3_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB5_7set_val9SetValZSTE3newNtNtB9_5alloc6GlobalEB1a_.exit.i.i, %_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_.exit.i.i.i.i.i, %_RNCNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB7_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBb_7set_val9SetValZSTE12insert_entry0B1p_.exit.i.i.i, %bb5.i.i10.i
  %53 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %54 = load i64, ptr %53, align 8, !alias.scope !578, !noalias !691, !noundef !7
  %55 = add i64 %54, 1
  store i64 %55, ptr %53, align 8, !alias.scope !578, !noalias !691
  br label %bb3

cleanup:                                          ; preds = %bb3.i.i11.i.i.invoke, %bb2.i.i.i.i.i
  %56 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body

cleanup.body:                                     ; preds = %bb6.i.i.i.i.i, %bb6.i.i44.i.i.i, %cleanup
  %eh.lpad-body = phi { ptr, i32 } [ %56, %cleanup ], [ %10, %bb6.i.i.i.i.i ], [ %.pn.i.i.i.i.i, %bb6.i.i44.i.i.i ]
; invoke core::ptr::drop_in_place::<console::utils::Style>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console5utils5StyleEBK_(ptr noalias noundef align 8 dereferenceable(32) %self) #32
          to label %bb2 unwind label %terminate

bb3:                                              ; preds = %bb4.i.i.i.i, %_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_.exit.i
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %self, i64 32, i1 false)
  ret void

terminate:                                        ; preds = %cleanup.body
  %57 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

bb2:                                              ; preds = %cleanup.body
  resume { ptr, i32 } %eh.lpad-body
}

; <alloc::raw_vec::RawVec<char>>::grow_one
; Function Attrs: cold noinline uwtable
define void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVeccE8grow_oneCsC3JuwEIQwb_7console(ptr noalias noundef align 8 captures(none) dereferenceable(16) %self) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  %self1 = load i64, ptr %self, align 8, !range !23, !noundef !7
  tail call void @llvm.experimental.noalias.scope.decl(metadata !695)
  %v16.i = shl nuw i64 %self1, 1
  %0 = tail call i64 @llvm.umax.i64(i64 %v16.i, i64 range(i64 0, -1) 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !695
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !695
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsC3JuwEIQwb_7console(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self1, ptr %self.val15.i, i64 noundef %0, i64 noundef 4, i64 noundef 4), !noalias !695
  %_35.i = load i64, ptr %self3.i, align 8, !range !6, !noalias !695, !noundef !7
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %start
  %e.0.i = load i64, ptr %3, align 8, !range !132, !noalias !695, !noundef !7
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !695
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !695
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %e.0.i, i64 %e.1.i) #30
  unreachable

bb3:                                              ; preds = %start
  %v.0.i = load ptr, ptr %3, align 8, !noalias !695, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !695
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !695
  %5 = icmp sgt i64 %0, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %0, ptr %self, align 8, !alias.scope !695
  ret void
}

; <console::ansi::AnsiCodeIterator>::rest_slice
; Function Attrs: uwtable
define { ptr, i64 } @_RNvMs4_NtCsC3JuwEIQwb_7console4ansiNtB5_16AnsiCodeIterator10rest_slice(ptr noalias noundef readonly align 8 captures(none) dereferenceable(112) %self) unnamed_addr #1 {
start:
  %_4.0 = load ptr, ptr %self, align 8, !nonnull !7, !align !187, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_4.1 = load i64, ptr %0, align 8, !noundef !7
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_3 = load i64, ptr %1, align 8, !noundef !7
  %2 = icmp eq i64 %_3, 0
  br i1 %2, label %bb4, label %bb5.i

bb5.i:                                            ; preds = %start
  %_8.not.i = icmp ult i64 %_3, %_4.1
  br i1 %_8.not.i, label %bb9.i, label %bb6.i

bb6.i:                                            ; preds = %bb5.i
  %3 = icmp eq i64 %_3, %_4.1
  br i1 %3, label %bb4, label %bb3

bb9.i:                                            ; preds = %bb5.i
  %4 = getelementptr inbounds nuw i8, ptr %_4.0, i64 %_3
  %self1.i = load i8, ptr %4, align 1, !alias.scope !698, !noundef !7
  %5 = icmp sgt i8 %self1.i, -65
  br i1 %5, label %bb4, label %bb3

bb4:                                              ; preds = %bb9.i, %bb6.i, %start
  %new_len.i = sub nuw i64 %_4.1, %_3
  %data.i = getelementptr inbounds nuw i8, ptr %_4.0, i64 %_3
  %6 = insertvalue { ptr, i64 } poison, ptr %data.i, 0
  %7 = insertvalue { ptr, i64 } %6, i64 %new_len.i, 1
  ret { ptr, i64 } %7

bb3:                                              ; preds = %bb9.i, %bb6.i
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_4.0, i64 noundef %_4.1, i64 noundef %_3, i64 noundef %_4.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_01955e5c48a2e8a6df036a754cf3097b) #34
  unreachable
}

; <console::ansi::AnsiCodeIterator>::current_slice
; Function Attrs: uwtable
define { ptr, i64 } @_RNvMs4_NtCsC3JuwEIQwb_7console4ansiNtB5_16AnsiCodeIterator13current_slice(ptr noalias noundef readonly align 8 captures(none) dereferenceable(112) %self) unnamed_addr #1 {
start:
  %_4.0 = load ptr, ptr %self, align 8, !nonnull !7, !align !187, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_4.1 = load i64, ptr %0, align 8, !noundef !7
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_3 = load i64, ptr %1, align 8, !noundef !7
  %2 = icmp eq i64 %_3, 0
  br i1 %2, label %bb4, label %bb5.i

bb5.i:                                            ; preds = %start
  %_8.not.i = icmp ult i64 %_3, %_4.1
  br i1 %_8.not.i, label %bb9.i, label %bb6.i

bb6.i:                                            ; preds = %bb5.i
  %3 = icmp eq i64 %_3, %_4.1
  br i1 %3, label %bb4, label %bb3

bb9.i:                                            ; preds = %bb5.i
  %4 = getelementptr inbounds nuw i8, ptr %_4.0, i64 %_3
  %self1.i = load i8, ptr %4, align 1, !alias.scope !701, !noundef !7
  %5 = icmp sgt i8 %self1.i, -65
  br i1 %5, label %bb4, label %bb3

bb4:                                              ; preds = %bb9.i, %bb6.i, %start
  %6 = insertvalue { ptr, i64 } poison, ptr %_4.0, 0
  %7 = insertvalue { ptr, i64 } %6, i64 %_3, 1
  ret { ptr, i64 } %7

bb3:                                              ; preds = %bb9.i, %bb6.i
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_4.0, i64 noundef %_4.1, i64 noundef 0, i64 noundef %_3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8c1ea329d6664f53eac78246d452a6c) #34
  unreachable
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsC3JuwEIQwb_7console(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap, i64 noundef range(i64 1, 5) %elem_layout.0, i64 noundef range(i64 1, 5) %elem_layout.1) unnamed_addr #5 {
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
  br i1 %or.cond.i, label %bb11, label %bb11.i, !prof !704

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
  %raw_ptr.i.i = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %alloc_size.i23, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %_27.sroa.7.01321) #29
  br label %bb7

bb7.thread:                                       ; preds = %bb14.thread
  %_16.i.i = inttoptr i64 %elem_layout.0 to ptr
  br label %bb9

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29
; call __rustc::__rust_alloc
  %4 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0) #29
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

; <std::io::error::Error>::kind
; Function Attrs: inlinehint mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read, inaccessiblemem: write) uwtable
define internal fastcc noundef range(i8 0, 42) i8 @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4kind(ptr %self.0.val) unnamed_addr #6 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %self.0.val, null
  tail call void @llvm.assume(i1 %0)
  %bits.i = ptrtoint ptr %self.0.val to i64
  %_5.i = and i64 %bits.i, 3
  switch i64 %_5.i, label %default.unreachable [
    i64 2, label %bb5
    i64 3, label %bb4.i
    i64 0, label %bb2
    i64 1, label %bb4
  ], !prof !275

default.unreachable:                              ; preds = %start
  unreachable

bb4.i:                                            ; preds = %start
  %_10.i = lshr i64 %bits.i, 32
  %switch.idx.cast = trunc i64 %_10.i to i8
  br label %bb6

bb5:                                              ; preds = %start
  %_7.i = lshr i64 %bits.i, 32
  %code.i = trunc nuw i64 %_7.i to i32
  switch i32 %code.i, label %bb43.i [
    i32 7, label %bb6
    i32 48, label %bb37.i
    i32 49, label %bb36.i
    i32 16, label %bb35.i
    i32 53, label %bb34.i
    i32 61, label %bb33.i
    i32 54, label %bb32.i
    i32 11, label %bb31.i
    i32 69, label %bb30.i
    i32 17, label %bb29.i
    i32 27, label %bb28.i
    i32 65, label %bb27.i
    i32 4, label %bb26.i
    i32 22, label %bb25.i
    i32 21, label %bb24.i
    i32 62, label %bb23.i
    i32 2, label %bb22.i
    i32 12, label %bb21.i
    i32 28, label %bb20.i
    i32 78, label %bb19.i
    i32 31, label %bb18.i
    i32 63, label %bb17.i
    i32 50, label %bb16.i
    i32 51, label %bb15.i
    i32 57, label %bb14.i
    i32 20, label %bb13.i
    i32 66, label %bb12.i
    i32 32, label %bb11.i
    i32 30, label %bb10.i
    i32 29, label %bb9.i
    i32 70, label %bb8.i
    i32 60, label %bb7.i
    i32 26, label %bb6.i
    i32 18, label %bb5.i3
    i32 36, label %bb4.i2
    i32 102, label %bb19.i
    i32 13, label %bb2.i1
    i32 1, label %bb2.i1
    i32 35, label %bb42.i
  ]

bb37.i:                                           ; preds = %bb5
  br label %bb6

bb36.i:                                           ; preds = %bb5
  br label %bb6

bb35.i:                                           ; preds = %bb5
  br label %bb6

bb34.i:                                           ; preds = %bb5
  br label %bb6

bb33.i:                                           ; preds = %bb5
  br label %bb6

bb32.i:                                           ; preds = %bb5
  br label %bb6

bb31.i:                                           ; preds = %bb5
  br label %bb6

bb30.i:                                           ; preds = %bb5
  br label %bb6

bb29.i:                                           ; preds = %bb5
  br label %bb6

bb28.i:                                           ; preds = %bb5
  br label %bb6

bb27.i:                                           ; preds = %bb5
  br label %bb6

bb26.i:                                           ; preds = %bb5
  br label %bb6

bb25.i:                                           ; preds = %bb5
  br label %bb6

bb24.i:                                           ; preds = %bb5
  br label %bb6

bb23.i:                                           ; preds = %bb5
  br label %bb6

bb22.i:                                           ; preds = %bb5
  br label %bb6

bb21.i:                                           ; preds = %bb5
  br label %bb6

bb20.i:                                           ; preds = %bb5
  br label %bb6

bb19.i:                                           ; preds = %bb5, %bb5
  br label %bb6

bb18.i:                                           ; preds = %bb5
  br label %bb6

bb17.i:                                           ; preds = %bb5
  br label %bb6

bb16.i:                                           ; preds = %bb5
  br label %bb6

bb15.i:                                           ; preds = %bb5
  br label %bb6

bb14.i:                                           ; preds = %bb5
  br label %bb6

bb13.i:                                           ; preds = %bb5
  br label %bb6

bb12.i:                                           ; preds = %bb5
  br label %bb6

bb11.i:                                           ; preds = %bb5
  br label %bb6

bb10.i:                                           ; preds = %bb5
  br label %bb6

bb9.i:                                            ; preds = %bb5
  br label %bb6

bb8.i:                                            ; preds = %bb5
  br label %bb6

bb7.i:                                            ; preds = %bb5
  br label %bb6

bb6.i:                                            ; preds = %bb5
  br label %bb6

bb5.i3:                                           ; preds = %bb5
  br label %bb6

bb4.i2:                                           ; preds = %bb5
  br label %bb6

bb2.i1:                                           ; preds = %bb5, %bb5
  br label %bb6

bb43.i:                                           ; preds = %bb5
  br label %bb6

bb42.i:                                           ; preds = %bb5
  br label %bb6

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self.0.val, i64 16
  %2 = load i8, ptr %1, align 8, !range !304, !noundef !7
  br label %bb6

bb4:                                              ; preds = %start
  %3 = getelementptr i8, ptr %self.0.val, i64 -1
  %4 = icmp ne ptr %3, null
  tail call void @llvm.assume(i1 %4)
  %5 = getelementptr i8, ptr %self.0.val, i64 15
  %6 = load i8, ptr %5, align 8, !range !304, !noundef !7
  br label %bb6

bb6:                                              ; preds = %bb4.i, %bb42.i, %bb43.i, %bb2.i1, %bb4.i2, %bb5.i3, %bb6.i, %bb7.i, %bb8.i, %bb9.i, %bb10.i, %bb11.i, %bb12.i, %bb13.i, %bb14.i, %bb15.i, %bb16.i, %bb17.i, %bb18.i, %bb19.i, %bb20.i, %bb21.i, %bb22.i, %bb23.i, %bb24.i, %bb25.i, %bb26.i, %bb27.i, %bb28.i, %bb29.i, %bb30.i, %bb31.i, %bb32.i, %bb33.i, %bb34.i, %bb35.i, %bb36.i, %bb37.i, %bb5, %bb4, %bb2
  %kind.sroa.0.0 = phi i8 [ %2, %bb2 ], [ %6, %bb4 ], [ 41, %bb43.i ], [ 8, %bb37.i ], [ 9, %bb36.i ], [ 28, %bb35.i ], [ 6, %bb34.i ], [ 2, %bb33.i ], [ 3, %bb32.i ], [ 30, %bb31.i ], [ 26, %bb30.i ], [ 12, %bb29.i ], [ 27, %bb28.i ], [ 4, %bb27.i ], [ 35, %bb26.i ], [ 20, %bb25.i ], [ 15, %bb24.i ], [ 18, %bb23.i ], [ 0, %bb22.i ], [ 38, %bb21.i ], [ 24, %bb20.i ], [ 36, %bb19.i ], [ 32, %bb18.i ], [ 33, %bb17.i ], [ 10, %bb16.i ], [ 5, %bb15.i ], [ 7, %bb14.i ], [ 14, %bb13.i ], [ 16, %bb12.i ], [ 11, %bb11.i ], [ 17, %bb10.i ], [ 25, %bb9.i ], [ 19, %bb8.i ], [ 22, %bb7.i ], [ 29, %bb6.i ], [ 31, %bb5.i3 ], [ 39, %bb4.i2 ], [ 1, %bb2.i1 ], [ 13, %bb42.i ], [ 34, %bb5 ], [ %switch.idx.cast, %bb4.i ]
  ret i8 %kind.sroa.0.0
}

; <alloc::sync::Arc<std::sync::poison::mutex::Mutex<dyn console::term::TermRead>>>::drop_slow
; Function Attrs: noinline uwtable
define void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EE9drop_slowB1A_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #7 personality ptr @rust_eh_personality {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !7, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load ptr, ptr %0, align 8, !nonnull !7, !align !22, !noundef !7
  %1 = getelementptr inbounds nuw i8, ptr %_3.1, i64 16
  %2 = load i64, ptr %1, align 8, !invariant.load !7
  %3 = tail call i64 @llvm.umax.i64(i64 %2, i64 8)
  %4 = add i64 %3, -1
  %5 = and i64 %4, -16
  %6 = getelementptr i8, ptr %_3.0, i64 %5
  %_6.0 = getelementptr i8, ptr %6, i64 16
  %self.1.val = load ptr, ptr %_3.1, align 8
; invoke <std::sys::sync::mutex::pthread::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(8) %_6.0)
          to label %bb4.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %start
  %7 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(8) %_6.0) #32
          to label %cleanup.body.i unwind label %terminate.i.i

bb4.i.i:                                          ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !705)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !708)
  %ptr.i.i.i.i = load ptr, ptr %_6.0, align 8, !alias.scope !711, !noundef !7
  store ptr null, ptr %_6.0, align 8, !alias.scope !711
  %8 = icmp eq ptr %ptr.i.i.i.i, null
  br i1 %8, label %bb4.i, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb4.i.i
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %ptr.i.i.i.i)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i unwind label %bb1.i.i.i.i.i.i.i, !noalias !714

bb1.i.i.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i.i
  %9 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef 64, i64 noundef 8) #29, !noalias !714
  br label %cleanup.body.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i: ; preds = %bb2.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef 64, i64 noundef 8) #29, !noalias !714
  br label %bb4.i

terminate.i.i:                                    ; preds = %cleanup.i.i
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

cleanup.body.i:                                   ; preds = %bb1.i.i.i.i.i.i.i, %cleanup.i.i
  %eh.lpad-body.i = phi { ptr, i32 } [ %9, %bb1.i.i.i.i.i.i.i ], [ %7, %cleanup.i.i ]
  %.not.i.i = icmp eq ptr %self.1.val, null
  br i1 %.not.i.i, label %cleanup.body, label %is_not_null.i.i

is_not_null.i.i:                                  ; preds = %cleanup.body.i
  %11 = add i64 %2, -1
  %12 = and i64 %11, -9
  %13 = getelementptr i8, ptr %_6.0, i64 %12
  %14 = getelementptr i8, ptr %13, i64 9
  invoke void %self.1.val(ptr noundef nonnull align 1 %14) #35
          to label %cleanup.body unwind label %terminate.i

bb4.i:                                            ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECsC3JuwEIQwb_7console.exit.i.i.i.i.i, %bb4.i.i
  %.not.i2.i = icmp eq ptr %self.1.val, null
  br i1 %.not.i2.i, label %bb1, label %is_not_null.i3.i

is_not_null.i3.i:                                 ; preds = %bb4.i
  %15 = add i64 %2, -1
  %16 = and i64 %15, -9
  %17 = getelementptr i8, ptr %_6.0, i64 %16
  %18 = getelementptr i8, ptr %17, i64 9
  invoke void %self.1.val(ptr noundef nonnull align 1 %18) #35
          to label %bb1 unwind label %cleanup

terminate.i:                                      ; preds = %is_not_null.i.i
  %19 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

cleanup:                                          ; preds = %is_not_null.i3.i
  %20 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body

cleanup.body:                                     ; preds = %cleanup.body.i, %is_not_null.i.i, %cleanup
  %eh.lpad-body = phi { ptr, i32 } [ %20, %cleanup ], [ %eh.lpad-body.i, %is_not_null.i.i ], [ %eh.lpad-body.i, %cleanup.body.i ]
; call core::ptr::drop_in_place::<alloc::sync::Weak<std::sync::poison::mutex::Mutex<dyn console::term::TermRead>, &alloc::alloc::Global>>
  tail call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtBL_5alloc6GlobalEEB29_(ptr nonnull %_3.0, ptr nonnull %_3.1) #32
  resume { ptr, i32 } %eh.lpad-body

bb1:                                              ; preds = %bb4.i, %is_not_null.i3.i
  %_16.i.i = icmp eq ptr %_3.0, inttoptr (i64 -1 to ptr)
  br i1 %_16.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtBL_5alloc6GlobalEEB29_.exit, label %bb8.i.i

bb8.i.i:                                          ; preds = %bb1
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_3.0, i64 8
  %21 = atomicrmw sub ptr %_20.i.i, i64 1 release, align 8
  %22 = icmp eq i64 %21, 1
  br i1 %22, label %bb1.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtBL_5alloc6GlobalEEB29_.exit

bb1.i.i:                                          ; preds = %bb8.i.i
  fence acquire
  %23 = getelementptr inbounds nuw i8, ptr %_3.1, i64 8
  %24 = load i64, ptr %23, align 8, !range !23, !invariant.load !7
  %25 = add i64 %2, -1
  %26 = add i64 %25, %24
  %27 = sub i64 0, %2
  %28 = and i64 %26, %27
  %29 = add i64 %3, 8
  %30 = add i64 %29, %28
  %31 = sub i64 0, %3
  %32 = and i64 %30, %31
  %33 = add i64 %3, 15
  %34 = add i64 %33, %32
  %35 = and i64 %34, %31
  %36 = icmp eq i64 %35, 0
  br i1 %36, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtBL_5alloc6GlobalEEB29_.exit, label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb1.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.0, i64 noundef %35, i64 noundef range(i64 1, -9223372036854775807) %3) #29
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtBL_5alloc6GlobalEEB29_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_ERNtNtBL_5alloc6GlobalEEB29_.exit: ; preds = %bb1, %bb8.i.i, %bb1.i.i, %bb1.i.i.i.i
  ret void
}

; <alloc::sync::Arc<console::term::TermInner>>::drop_slow
; Function Attrs: noinline uwtable
define void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerE9drop_slowBK_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #7 personality ptr @rust_eh_personality {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !7, !noundef !7
  %_6 = getelementptr inbounds nuw i8, ptr %_3, i64 16
; invoke core::ptr::drop_in_place::<console::term::TermInner>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term9TermInnerEBK_(ptr noalias noundef align 8 dereferenceable(168) %_6)
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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3, i64 noundef 184, i64 noundef range(i64 1, -9223372036854775807) 8) #29
  br label %bb4

bb1:                                              ; preds = %start
  %_16.i.i3 = icmp eq ptr %_3, inttoptr (i64 -1 to ptr)
  br i1 %_16.i.i3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtCsC3JuwEIQwb_7console4term9TermInnerRNtNtBL_5alloc6GlobalEEB1j_.exit7, label %bb8.i.i4

bb8.i.i4:                                         ; preds = %bb1
  %_20.i.i5 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %3 = atomicrmw sub ptr %_20.i.i5, i64 1 release, align 8
  %4 = icmp eq i64 %3, 1
  br i1 %4, label %bb1.i.i6, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtCsC3JuwEIQwb_7console4term9TermInnerRNtNtBL_5alloc6GlobalEEB1j_.exit7

bb1.i.i6:                                         ; preds = %bb8.i.i4
  fence acquire
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3, i64 noundef 184, i64 noundef range(i64 1, -9223372036854775807) 8) #29
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtCsC3JuwEIQwb_7console4term9TermInnerRNtNtBL_5alloc6GlobalEEB1j_.exit7

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtCsC3JuwEIQwb_7console4term9TermInnerRNtNtBL_5alloc6GlobalEEB1j_.exit7: ; preds = %bb1, %bb8.i.i4, %bb1.i.i6
  ret void

bb4:                                              ; preds = %bb1.i.i, %bb8.i.i, %cleanup
  resume { ptr, i32 } %0
}

; <u8>::from_ascii_radix
; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(argmem: read) uwtable
define internal fastcc { i1, i8 } @_RNvMsx_NtCsjMrxcFdYDNN_4core3numh16from_ascii_radix(ptr noalias noundef nonnull readonly align 1 captures(none) %0, i64 noundef range(i64 0, -9223372036854775808) %1) unnamed_addr #8 {
start:
  switch i64 %1, label %bb9thread-pre-split [
    i64 0, label %bb28
    i64 1, label %bb7
  ]

bb28:                                             ; preds = %bb33, %bb40, %bb22, %bb20, %bb16, %bb15.preheader, %bb31, %bb7, %bb7, %start
  %_0.sroa.8.0 = phi i8 [ 0, %start ], [ 1, %bb7 ], [ 1, %bb7 ], [ %spec.select, %bb31 ], [ 0, %bb15.preheader ], [ %14, %bb20 ], [ 1, %bb16 ], [ 2, %bb40 ], [ 1, %bb33 ], [ %result.sroa.0.0, %bb22 ]
  %_0.sroa.0.0.off0 = phi i1 [ true, %start ], [ true, %bb7 ], [ true, %bb7 ], [ true, %bb31 ], [ false, %bb15.preheader ], [ %_14.i46, %bb16 ], [ %_14.i46, %bb20 ], [ %_30.not.not.not, %bb22 ], [ %_30.not.not.not, %bb40 ], [ %_30.not.not.not, %bb33 ]
  %2 = insertvalue { i1, i8 } poison, i1 %_0.sroa.0.0.off0, 0
  %3 = insertvalue { i1, i8 } %2, i8 %_0.sroa.8.0, 1
  ret { i1, i8 } %3

bb7:                                              ; preds = %start
  %4 = load i8, ptr %0, align 1, !noundef !7
  switch i8 %4, label %bb9 [
    i8 43, label %bb28
    i8 45, label %bb28
  ]

bb9thread-pre-split:                              ; preds = %start
  %.pr = load i8, ptr %0, align 1
  br label %bb9

bb9:                                              ; preds = %bb9thread-pre-split, %bb7
  %5 = phi i8 [ %.pr, %bb9thread-pre-split ], [ %4, %bb7 ]
  %cond = icmp eq i8 %5, 43
  %rest.1 = sext i1 %cond to i64
  %src.sroa.15.0 = add nsw i64 %1, %rest.1
  %src.sroa.0.0.idx = zext i1 %cond to i64
  %src.sroa.0.0 = getelementptr inbounds nuw i8, ptr %0, i64 %src.sroa.0.0.idx
  %_10 = icmp samesign ult i64 %src.sroa.15.0, 3
  br i1 %_10, label %bb15.preheader, label %bb22

bb15.preheader:                                   ; preds = %bb9
  %_13.not52 = icmp eq i64 %src.sroa.15.0, 0
  br i1 %_13.not52, label %bb28, label %bb16

bb22:                                             ; preds = %bb9, %bb40
  %result.sroa.0.0 = phi i8 [ %_63.0, %bb40 ], [ 0, %bb9 ]
  %src.sroa.15.1 = phi i64 [ %rest.12, %bb40 ], [ %src.sroa.15.0, %bb9 ]
  %src.sroa.0.1 = phi ptr [ %rest.01, %bb40 ], [ %src.sroa.0.0, %bb9 ]
  %_30.not.not.not = icmp ne i64 %src.sroa.15.1, 0
  br i1 %_30.not.not.not, label %bb23, label %bb28

bb23:                                             ; preds = %bb22
  %rest.01 = getelementptr inbounds nuw i8, ptr %src.sroa.0.1, i64 1
  %rest.12 = add nsw i64 %src.sroa.15.1, -1
  %6 = tail call { i8, i1 } @llvm.umul.with.overflow.i8(i8 %result.sroa.0.0, i8 10)
  %_60.0 = extractvalue { i8, i1 } %6, 0
  %_60.1 = extractvalue { i8, i1 } %6, 1
  %7 = load i8, ptr %src.sroa.0.1, align 1, !noundef !7
  br i1 %_60.1, label %bb31, label %bb33, !prof !5

bb33:                                             ; preds = %bb23
  %8 = zext i8 %7 to i32
  %9 = add nsw i32 %8, -48
  %_14.i = icmp ult i32 %9, 10
  br i1 %_14.i, label %bb40, label %bb28

bb31:                                             ; preds = %bb23
  %10 = add i8 %7, -48
  %_14.i44 = icmp ult i8 %10, 10
  %spec.select = select i1 %_14.i44, i8 2, i8 1
  br label %bb28

bb40:                                             ; preds = %bb33
  %11 = trunc nuw nsw i32 %9 to i8
  %_63.0 = add i8 %_60.0, %11
  %_63.1 = icmp ult i8 %_63.0, %_60.0
  br i1 %_63.1, label %bb28, label %bb22, !prof !5

bb16:                                             ; preds = %bb15.preheader, %bb20
  %src.sroa.0.255 = phi ptr [ %rest.05, %bb20 ], [ %src.sroa.0.0, %bb15.preheader ]
  %src.sroa.15.254 = phi i64 [ %rest.16, %bb20 ], [ %src.sroa.15.0, %bb15.preheader ]
  %result.sroa.0.253 = phi i8 [ %14, %bb20 ], [ 0, %bb15.preheader ]
  %_20 = load i8, ptr %src.sroa.0.255, align 1, !noundef !7
  %_19 = zext i8 %_20 to i32
  %12 = add nsw i32 %_19, -48
  %_14.i46 = icmp ugt i32 %12, 9
  br i1 %_14.i46, label %bb28, label %bb20

bb20:                                             ; preds = %bb16
  %13 = mul i8 %result.sroa.0.253, 10
  %rest.16 = add nsw i64 %src.sroa.15.254, -1
  %rest.05 = getelementptr inbounds nuw i8, ptr %src.sroa.0.255, i64 1
  %_24 = trunc nuw nsw i32 %12 to i8
  %14 = add i8 %13, %_24
  %_13.not = icmp eq i64 %rest.16, 0
  br i1 %_13.not, label %bb28, label %bb16
}

; <alloc::collections::btree::map::IntoIter<console::utils::Attribute, alloc::collections::btree::set_val::SetValZST>>::dying_next
; Function Attrs: uwtable
define internal fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10dying_nextB1b_(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_2 = load i64, ptr %0, align 8, !noundef !7
  %1 = icmp eq i64 %_2, 0
  br i1 %1, label %bb1, label %bb4

bb1:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !715)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !718)
  %self1.sroa.0.0.copyload.i.i = load i64, ptr %self, align 8, !alias.scope !721, !noalias !722
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self1.sroa.5.sroa.0.0.copyload.i.i = load ptr, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !721, !noalias !722
  %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self1.sroa.5.sroa.5.0.copyload.i.i = load ptr, ptr %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !721, !noalias !722
  %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.sroa.5.sroa.6.0.copyload.i.i = load i64, ptr %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !721, !noalias !722
  store i64 0, ptr %self, align 8, !alias.scope !721, !noalias !722
  %2 = trunc nuw i64 %self1.sroa.0.0.copyload.i.i to i1
  br i1 %2, label %bb7.i.i, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE16deallocating_endNtNtBc_5alloc6GlobalEB1O_.exit

bb7.i.i:                                          ; preds = %bb1
  %.not.i.i = icmp eq ptr %self1.sroa.5.sroa.0.0.copyload.i.i, null
  br i1 %.not.i.i, label %bb3.i.i, label %bb2.i

bb3.i.i:                                          ; preds = %bb7.i.i
  %3 = icmp ne ptr %self1.sroa.5.sroa.5.0.copyload.i.i, null
  tail call void @llvm.assume(i1 %3)
  %4 = icmp eq i64 %self1.sroa.5.sroa.6.0.copyload.i.i, 0
  br i1 %4, label %bb2.i, label %bb10.i.i

bb10.i.i:                                         ; preds = %bb3.i.i, %bb10.i.i
  %root2.sroa.0.011.i.i = phi ptr [ %5, %bb10.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ]
  %root.sroa.0.010.i.i = phi i64 [ %6, %bb10.i.i ], [ %self1.sroa.5.sroa.6.0.copyload.i.i, %bb3.i.i ]
  %_19.i.i = getelementptr inbounds nuw i8, ptr %root2.sroa.0.011.i.i, i64 24
  %5 = load ptr, ptr %_19.i.i, align 8, !noalias !724, !nonnull !7, !noundef !7
  %6 = add i64 %root.sroa.0.010.i.i, -1
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %bb2.i, label %bb10.i.i

bb2.i:                                            ; preds = %bb10.i.i, %bb3.i.i, %bb7.i.i
  %_3.sroa.8.0.ph.i = phi ptr [ null, %bb3.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb7.i.i ], [ null, %bb10.i.i ]
  %_3.sroa.0.0.ph.i = phi ptr [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ], [ %self1.sroa.5.sroa.0.0.copyload.i.i, %bb7.i.i ], [ %5, %bb10.i.i ]
  %8 = ptrtoint ptr %_3.sroa.8.0.ph.i to i64
  %9 = load ptr, ptr %_3.sroa.0.0.ph.i, align 8, !noalias !725, !noundef !7
  %.not.i.i4.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i4.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalEB1W_.exit.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i, %bb4.i.i
  %10 = phi ptr [ %11, %bb4.i.i ], [ %9, %bb2.i ]
  %edge.sroa.0.06.i.i = phi ptr [ %10, %bb4.i.i ], [ %_3.sroa.0.0.ph.i, %bb2.i ]
  %edge.sroa.3.05.i.i = phi i64 [ %_18.i.i.i.i, %bb4.i.i ], [ %8, %bb2.i ]
  %_18.i.i.i.i = add i64 %edge.sroa.3.05.i.i, 1
  %_10.not.i.i.i = icmp eq i64 %edge.sroa.3.05.i.i, 0
  %..i.i.i = select i1 %_10.not.i.i.i, i64 24, i64 120
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.06.i.i, i64 noundef %..i.i.i, i64 noundef 8) #29, !noalias !730
  %11 = load ptr, ptr %10, align 8, !noalias !725, !noundef !7
  %.not.i.i.i.i = icmp eq ptr %11, null
  br i1 %.not.i.i.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalEB1W_.exit.i, label %bb4.i.i

_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalEB1W_.exit.i: ; preds = %bb4.i.i, %bb2.i
  %edge.sroa.3.0.lcssa.i.i = phi i64 [ %8, %bb2.i ], [ %_18.i.i.i.i, %bb4.i.i ]
  %edge.sroa.0.0.lcssa.i.i = phi ptr [ %_3.sroa.0.0.ph.i, %bb2.i ], [ %10, %bb4.i.i ]
  %_10.not.i2.i.i = icmp eq i64 %edge.sroa.3.0.lcssa.i.i, 0
  %..i3.i.i = select i1 %_10.not.i2.i.i, i64 24, i64 120
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.0.lcssa.i.i, i64 noundef %..i3.i.i, i64 noundef 8) #29, !noalias !730
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE16deallocating_endNtNtBc_5alloc6GlobalEB1O_.exit

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE16deallocating_endNtNtBc_5alloc6GlobalEB1O_.exit: ; preds = %bb1, %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalEB1W_.exit.i
  store ptr null, ptr %_0, align 8
  br label %bb7

bb4:                                              ; preds = %start
  %12 = add i64 %_2, -1
  store i64 %12, ptr %0, align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !731)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !734)
  %_3.i.i = load i64, ptr %self, align 8, !range !6, !alias.scope !737, !noalias !738, !noundef !7
  %13 = trunc nuw i64 %_3.i.i to i1
  br i1 %13, label %bb1.i.i, label %bb6.i

bb1.i.i:                                          ; preds = %bb4
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %15 = load ptr, ptr %14, align 8, !alias.scope !737, !noalias !738, !noundef !7
  %.not.i.i1 = icmp eq ptr %15, null
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 16
  br i1 %.not.i.i1, label %bb2.i.i, label %bb1.i.i.bb7.i_crit_edge

bb1.i.i.bb7.i_crit_edge:                          ; preds = %bb1.i.i
  %value.sroa.2.0.copyload.i.i.pre = load i64, ptr %16, align 8, !alias.scope !740, !noalias !743
  %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 24
  %value.sroa.3.0.copyload.i.i.pre = load i64, ptr %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert, align 8, !alias.scope !740, !noalias !743
  br label %bb7.i

bb2.i.i:                                          ; preds = %bb1.i.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %18 = load i64, ptr %17, align 8, !alias.scope !737, !noalias !738, !noundef !7
  %self2.sroa.0.07.i.i = load ptr, ptr %16, align 8, !alias.scope !737, !noalias !738, !nonnull !7, !noundef !7
  %19 = icmp eq i64 %18, 0
  br i1 %19, label %bb11.i.i, label %bb12.i.i

bb11.i.i:                                         ; preds = %bb12.i.i, %bb2.i.i
  %self2.sroa.0.0.lcssa.i.i = phi ptr [ %self2.sroa.0.07.i.i, %bb2.i.i ], [ %self2.sroa.0.0.i.i, %bb12.i.i ]
  store i64 1, ptr %self, align 8, !alias.scope !737, !noalias !738
  br label %bb7.i

bb12.i.i:                                         ; preds = %bb2.i.i, %bb12.i.i
  %self2.sroa.0.09.i.i = phi ptr [ %self2.sroa.0.0.i.i, %bb12.i.i ], [ %self2.sroa.0.07.i.i, %bb2.i.i ]
  %self1.sroa.0.08.i.i = phi i64 [ %20, %bb12.i.i ], [ %18, %bb2.i.i ]
  %_19.i.i2 = getelementptr inbounds nuw i8, ptr %self2.sroa.0.09.i.i, i64 24
  %20 = add i64 %self1.sroa.0.08.i.i, -1
  %self2.sroa.0.0.i.i = load ptr, ptr %_19.i.i2, align 8, !noalias !745, !nonnull !7, !noundef !7
  %21 = icmp eq i64 %20, 0
  br i1 %21, label %bb11.i.i, label %bb12.i.i

bb7.i:                                            ; preds = %bb1.i.i.bb7.i_crit_edge, %bb11.i.i
  %value.sroa.3.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.3.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.2.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.2.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.0.0.copyload.i.i = phi ptr [ %self2.sroa.0.0.lcssa.i.i, %bb11.i.i ], [ %15, %bb1.i.i.bb7.i_crit_edge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !746)
  %value.sroa.2.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %value.sroa.3.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %value.sroa.0.0.copyload.i.i, i64 10
  %_2219.i.i.i.i = load i16, ptr %22, align 2, !noalias !747, !noundef !7
  %_1820.i.i.i.i = zext i16 %_2219.i.i.i.i to i64
  %_1621.i.i.i.i = icmp ult i64 %value.sroa.3.0.copyload.i.i, %_1820.i.i.i.i
  br i1 %_1621.i.i.i.i, label %bb12.i.i.i.i, label %bb13.i.i.i.i

bb13.i.i.i.i:                                     ; preds = %bb7.i, %bb7.i.i.i.i
  %edge.sroa.0.023.i.i.i.i = phi ptr [ %23, %bb7.i.i.i.i ], [ %value.sroa.0.0.copyload.i.i, %bb7.i ]
  %edge.sroa.5.022.i.i.i.i = phi i64 [ %_18.i.i.i.i.i.i, %bb7.i.i.i.i ], [ %value.sroa.2.0.copyload.i.i, %bb7.i ]
  %23 = load ptr, ptr %edge.sroa.0.023.i.i.i.i, align 8, !noalias !754, !noundef !7
  %.not.i.i.i.i.i.i = icmp eq ptr %23, null
  br i1 %.not.i.i.i.i.i.i, label %bb3.i.i.i, label %bb7.i.i.i.i

bb12.loopexit.i.i.i.i:                            ; preds = %bb7.i.i.i.i
  %_20.i.i.i.i.i.i = zext i16 %28 to i64
  br label %bb12.i.i.i.i

bb12.i.i.i.i:                                     ; preds = %bb12.loopexit.i.i.i.i, %bb7.i
  %edge.sroa.8.0.lcssa.i.i.i.i = phi i64 [ %value.sroa.3.0.copyload.i.i, %bb7.i ], [ %_20.i.i.i.i.i.i, %bb12.loopexit.i.i.i.i ]
  %edge.sroa.5.0.lcssa.i.i.i.i = phi i64 [ %value.sroa.2.0.copyload.i.i, %bb7.i ], [ %_18.i.i.i.i.i.i, %bb12.loopexit.i.i.i.i ]
  %edge.sroa.0.0.lcssa.i.i.i.i = phi ptr [ %value.sroa.0.0.copyload.i.i, %bb7.i ], [ %23, %bb12.loopexit.i.i.i.i ]
  %24 = icmp eq i64 %edge.sroa.5.0.lcssa.i.i.i.i, 0
  br i1 %24, label %bb2.i.i.i.i.i, label %bb3.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %_11.i.i.i.i.i = add nuw nsw i64 %edge.sroa.8.0.lcssa.i.i.i.i, 1
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalEB1O_.exit

bb3.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %25 = getelementptr i8, ptr %edge.sroa.0.0.lcssa.i.i.i.i, i64 32
  %self9.i.i.i.i.i = getelementptr ptr, ptr %25, i64 %edge.sroa.8.0.lcssa.i.i.i.i
  br label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i, %bb3.i.i.i.i.i
  %node.sroa.0.0.in.i.i.i.i.i = phi ptr [ %self9.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_29.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.in.i.i.i.i.i = phi i64 [ %edge.sroa.5.0.lcssa.i.i.i.i, %bb3.i.i.i.i.i ], [ %self1.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.i.i.i.i.i = add i64 %self1.sroa.0.0.in.i.i.i.i.i, -1
  %node.sroa.0.0.i.i.i.i.i = load ptr, ptr %node.sroa.0.0.in.i.i.i.i.i, align 8, !noalias !759, !nonnull !7, !noundef !7
  %26 = icmp eq i64 %self1.sroa.0.0.i.i.i.i.i, 0
  %_29.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %node.sroa.0.0.i.i.i.i.i, i64 24
  br i1 %26, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalEB1O_.exit, label %bb6.i.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb13.i.i.i.i
  %_18.i.i.i.i.i.i = add i64 %edge.sroa.5.022.i.i.i.i, 1
  %27 = getelementptr inbounds nuw i8, ptr %edge.sroa.0.023.i.i.i.i, i64 8
  %28 = load i16, ptr %27, align 8, !noalias !754
  %_10.not.i.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i.i.i.i.i = select i1 %_10.not.i.i.i.i.i, i64 24, i64 120
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i.i.i.i.i, i64 noundef 8) #29, !noalias !763
  %29 = getelementptr inbounds nuw i8, ptr %23, i64 10
  %_22.i.i.i.i = load i16, ptr %29, align 2, !noalias !747, !noundef !7
  %_16.i.i.i.i = icmp ult i16 %28, %_22.i.i.i.i
  br i1 %_16.i.i.i.i, label %bb12.loopexit.i.i.i.i, label %bb13.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb13.i.i.i.i
  %_10.not.i14.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i15.i.i.i.i = select i1 %_10.not.i14.i.i.i.i, i64 24, i64 120
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i15.i.i.i.i, i64 noundef 8) #29, !noalias !763
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_93816f04728d387347072ad30618ff9c) #34
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !764

.noexc.i.i:                                       ; preds = %bb3.i.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb3.i.i.i
  %30 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.trap()
  unreachable

bb6.i:                                            ; preds = %bb4
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1df1e5171bffdf21494df69d159bd444) #30, !noalias !765
  unreachable

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalEB1O_.exit: ; preds = %bb6.i.i.i.i.i, %bb2.i.i.i.i.i
  %self.sroa.7.0.ph.i.i.i = phi i64 [ %_11.i.i.i.i.i, %bb2.i.i.i.i.i ], [ 0, %bb6.i.i.i.i.i ]
  %self.sroa.0.0.ph.i.i.i = phi ptr [ %edge.sroa.0.0.lcssa.i.i.i.i, %bb2.i.i.i.i.i ], [ %node.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  store ptr %self.sroa.0.0.ph.i.i.i, ptr %14, align 8, !alias.scope !740, !noalias !743
  store i64 0, ptr %value.sroa.2.0.v.sroa_idx.i.i, align 8, !alias.scope !740, !noalias !743
  store i64 %self.sroa.7.0.ph.i.i.i, ptr %value.sroa.3.0.v.sroa_idx.i.i, align 8, !alias.scope !740, !noalias !743
  store ptr %edge.sroa.0.0.lcssa.i.i.i.i, ptr %_0, align 8
  %_7.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %edge.sroa.5.0.lcssa.i.i.i.i, ptr %_7.sroa.4.0._0.sroa_idx, align 8
  %_7.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %edge.sroa.8.0.lcssa.i.i.i.i, ptr %_7.sroa.5.0._0.sroa_idx, align 8
  br label %bb7

bb7:                                              ; preds = %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalEB1O_.exit, %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE16deallocating_endNtNtBc_5alloc6GlobalEB1O_.exit
  ret void
}

; console::common_term::clear_chars
; Function Attrs: uwtable
define noundef ptr @_RNvNtCsC3JuwEIQwb_7console11common_term11clear_chars(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %out, i64 noundef %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args = alloca [16 x i8], align 8
  %_6 = alloca [24 x i8], align 8
  %n = alloca [8 x i8], align 8
  store i64 %0, ptr %n, align 8
  %_3.not = icmp eq i64 %0, 0
  br i1 %_3.not, label %bb5, label %bb1

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %n, ptr %args, align 8
  %_10.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx, align 8
; call alloc::fmt::format::format_inner
  call void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6, ptr noundef nonnull @alloc_80b5bcfc6b71c11e412d8b752ae375dc, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  %_5.sroa.0.0.copyload = load i64, ptr %_6, align 8
  %_5.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %_5.sroa.5.0.copyload = load ptr, ptr %_5.sroa.5.0._6.sroa_idx, align 8, !nonnull !7, !noundef !7
  %_5.sroa.8.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 16
  %_5.sroa.8.0.copyload = load i64, ptr %_5.sroa.8.0._6.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
; invoke <console::term::Term>::write_str
  %1 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.8.0.copyload)
          to label %bb2 unwind label %cleanup

bb5:                                              ; preds = %bb2.i.i.i4.i.i5, %bb2, %start
  %_0.sroa.0.0 = phi ptr [ null, %start ], [ %1, %bb2 ], [ %1, %bb2.i.i.i4.i.i5 ]
  ret ptr %_0.sroa.0.0

cleanup:                                          ; preds = %bb1
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %3, label %bb7, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb7

bb2:                                              ; preds = %bb1
  %4 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %4, label %bb5, label %bb2.i.i.i4.i.i5

bb2.i.i.i4.i.i5:                                  ; preds = %bb2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb5

bb7:                                              ; preds = %bb2.i.i.i4.i.i, %cleanup
  resume { ptr, i32 } %2
}

; console::common_term::move_cursor_up
; Function Attrs: uwtable
define noundef ptr @_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %out, i64 noundef %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args = alloca [16 x i8], align 8
  %_6 = alloca [24 x i8], align 8
  %n = alloca [8 x i8], align 8
  store i64 %0, ptr %n, align 8
  %_3.not = icmp eq i64 %0, 0
  br i1 %_3.not, label %bb5, label %bb1

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %n, ptr %args, align 8
  %_10.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx, align 8
; call alloc::fmt::format::format_inner
  call void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6, ptr noundef nonnull @alloc_26f3afb675a2033575e507bda4f7709d, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  %_5.sroa.0.0.copyload = load i64, ptr %_6, align 8
  %_5.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %_5.sroa.5.0.copyload = load ptr, ptr %_5.sroa.5.0._6.sroa_idx, align 8, !nonnull !7, !noundef !7
  %_5.sroa.8.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 16
  %_5.sroa.8.0.copyload = load i64, ptr %_5.sroa.8.0._6.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
; invoke <console::term::Term>::write_str
  %1 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.8.0.copyload)
          to label %bb2 unwind label %cleanup

bb5:                                              ; preds = %bb2.i.i.i4.i.i5, %bb2, %start
  %_0.sroa.0.0 = phi ptr [ null, %start ], [ %1, %bb2 ], [ %1, %bb2.i.i.i4.i.i5 ]
  ret ptr %_0.sroa.0.0

cleanup:                                          ; preds = %bb1
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %3, label %bb7, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb7

bb2:                                              ; preds = %bb1
  %4 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %4, label %bb5, label %bb2.i.i.i4.i.i5

bb2.i.i.i4.i.i5:                                  ; preds = %bb2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb5

bb7:                                              ; preds = %bb2.i.i.i4.i.i, %cleanup
  resume { ptr, i32 } %2
}

; console::common_term::move_cursor_down
; Function Attrs: uwtable
define noundef ptr @_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %out, i64 noundef %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args = alloca [16 x i8], align 8
  %_6 = alloca [24 x i8], align 8
  %n = alloca [8 x i8], align 8
  store i64 %0, ptr %n, align 8
  %_3.not = icmp eq i64 %0, 0
  br i1 %_3.not, label %bb5, label %bb1

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %n, ptr %args, align 8
  %_10.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx, align 8
; call alloc::fmt::format::format_inner
  call void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6, ptr noundef nonnull @alloc_8041b981a0c9f2271efbceca9d03160b, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  %_5.sroa.0.0.copyload = load i64, ptr %_6, align 8
  %_5.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %_5.sroa.5.0.copyload = load ptr, ptr %_5.sroa.5.0._6.sroa_idx, align 8, !nonnull !7, !noundef !7
  %_5.sroa.8.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 16
  %_5.sroa.8.0.copyload = load i64, ptr %_5.sroa.8.0._6.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
; invoke <console::term::Term>::write_str
  %1 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.8.0.copyload)
          to label %bb2 unwind label %cleanup

bb5:                                              ; preds = %bb2.i.i.i4.i.i5, %bb2, %start
  %_0.sroa.0.0 = phi ptr [ null, %start ], [ %1, %bb2 ], [ %1, %bb2.i.i.i4.i.i5 ]
  ret ptr %_0.sroa.0.0

cleanup:                                          ; preds = %bb1
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %3, label %bb7, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb7

bb2:                                              ; preds = %bb1
  %4 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %4, label %bb5, label %bb2.i.i.i4.i.i5

bb2.i.i.i4.i.i5:                                  ; preds = %bb2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb5

bb7:                                              ; preds = %bb2.i.i.i4.i.i, %cleanup
  resume { ptr, i32 } %2
}

; console::common_term::move_cursor_left
; Function Attrs: uwtable
define noundef ptr @_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_left(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %out, i64 noundef %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args = alloca [16 x i8], align 8
  %_6 = alloca [24 x i8], align 8
  %n = alloca [8 x i8], align 8
  store i64 %0, ptr %n, align 8
  %_3.not = icmp eq i64 %0, 0
  br i1 %_3.not, label %bb5, label %bb1

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %n, ptr %args, align 8
  %_10.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx, align 8
; call alloc::fmt::format::format_inner
  call void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6, ptr noundef nonnull @alloc_91eb3b305a07e4e7da5a5c0c180f5643, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  %_5.sroa.0.0.copyload = load i64, ptr %_6, align 8
  %_5.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %_5.sroa.5.0.copyload = load ptr, ptr %_5.sroa.5.0._6.sroa_idx, align 8, !nonnull !7, !noundef !7
  %_5.sroa.8.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 16
  %_5.sroa.8.0.copyload = load i64, ptr %_5.sroa.8.0._6.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
; invoke <console::term::Term>::write_str
  %1 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.8.0.copyload)
          to label %bb2 unwind label %cleanup

bb5:                                              ; preds = %bb2.i.i.i4.i.i5, %bb2, %start
  %_0.sroa.0.0 = phi ptr [ null, %start ], [ %1, %bb2 ], [ %1, %bb2.i.i.i4.i.i5 ]
  ret ptr %_0.sroa.0.0

cleanup:                                          ; preds = %bb1
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %3, label %bb7, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb7

bb2:                                              ; preds = %bb1
  %4 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %4, label %bb5, label %bb2.i.i.i4.i.i5

bb2.i.i.i4.i.i5:                                  ; preds = %bb2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb5

bb7:                                              ; preds = %bb2.i.i.i4.i.i, %cleanup
  resume { ptr, i32 } %2
}

; console::common_term::move_cursor_right
; Function Attrs: uwtable
define noundef ptr @_RNvNtCsC3JuwEIQwb_7console11common_term17move_cursor_right(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %out, i64 noundef %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args = alloca [16 x i8], align 8
  %_6 = alloca [24 x i8], align 8
  %n = alloca [8 x i8], align 8
  store i64 %0, ptr %n, align 8
  %_3.not = icmp eq i64 %0, 0
  br i1 %_3.not, label %bb5, label %bb1

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %n, ptr %args, align 8
  %_10.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_10.sroa.4.0..sroa_idx, align 8
; call alloc::fmt::format::format_inner
  call void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6, ptr noundef nonnull @alloc_e721d4fe15bdeb41f351b3369d7e026f, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  %_5.sroa.0.0.copyload = load i64, ptr %_6, align 8
  %_5.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %_5.sroa.5.0.copyload = load ptr, ptr %_5.sroa.5.0._6.sroa_idx, align 8, !nonnull !7, !noundef !7
  %_5.sroa.8.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 16
  %_5.sroa.8.0.copyload = load i64, ptr %_5.sroa.8.0._6.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
; invoke <console::term::Term>::write_str
  %1 = invoke noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term9write_str(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.8.0.copyload)
          to label %bb2 unwind label %cleanup

bb5:                                              ; preds = %bb2.i.i.i4.i.i5, %bb2, %start
  %_0.sroa.0.0 = phi ptr [ null, %start ], [ %1, %bb2 ], [ %1, %bb2.i.i.i4.i.i5 ]
  ret ptr %_0.sroa.0.0

cleanup:                                          ; preds = %bb1
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %3, label %bb7, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb7

bb2:                                              ; preds = %bb1
  %4 = icmp eq i64 %_5.sroa.0.0.copyload, 0
  br i1 %4, label %bb5, label %bb2.i.i.i4.i.i5

bb2.i.i.i4.i.i5:                                  ; preds = %bb2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.5.0.copyload, i64 noundef %_5.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb5

bb7:                                              ; preds = %bb2.i.i.i4.i.i, %cleanup
  resume { ptr, i32 } %2
}

; console::ansi::strip_ansi_codes
; Function Attrs: uwtable
define void @_RNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_5.i.i.i.i.i = alloca [24 x i8], align 8
  %_4.i.i.i.i = alloca [112 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  %char_it = alloca [40 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 40, ptr nonnull %char_it)
  %_16 = getelementptr inbounds nuw i8, ptr %s.0, i64 %s.1
  store ptr %s.0, ptr %char_it, align 8
  %_3.sroa.4.0.char_it.sroa_idx = getelementptr inbounds nuw i8, ptr %char_it, i64 8
  store ptr %_16, ptr %_3.sroa.4.0.char_it.sroa_idx, align 8
  %_3.sroa.5.0.char_it.sroa_idx = getelementptr inbounds nuw i8, ptr %char_it, i64 16
  store i64 0, ptr %_3.sroa.5.0.char_it.sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %char_it, i64 32
  store i32 1114113, ptr %0, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
; call console::ansi::find_ansi_code_exclusive
  call fastcc void @_RNvNtCsC3JuwEIQwb_7console4ansi24find_ansi_code_exclusive(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_4, ptr noalias noundef align 8 dereferenceable(40) %char_it)
  %_6 = load i64, ptr %_4, align 8, !range !6, !noundef !7
  %1 = trunc nuw i64 %_6 to i1
  br i1 %1, label %bb4, label %bb3

bb4:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !766
  store i64 0, ptr %buf.i, align 8, !noalias !766
  %_5.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_5.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !766
  %_5.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_5.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !766
  tail call void @llvm.experimental.noalias.scope.decl(metadata !770)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !773)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !776)
  call void @llvm.lifetime.start.p0(i64 112, ptr nonnull %_4.i.i.i.i), !noalias !779
  store ptr %s.0, ptr %_4.i.i.i.i, align 8, !noalias !783
  %_8.sroa.4.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 8
  store i64 %s.1, ptr %_8.sroa.4.0._4.i.i.i.i.sroa_idx, align 8, !noalias !783
  %_8.sroa.5.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 16
  %_8.sroa.71.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 48
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_8.sroa.5.0._4.i.i.i.i.sroa_idx, i8 0, i64 16, i1 false)
  store i8 2, ptr %_8.sroa.71.0._4.i.i.i.i.sroa_idx, align 8, !noalias !783
  %_8.sroa.82.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 56
  store ptr %s.0, ptr %_8.sroa.82.0._4.i.i.i.i.sroa_idx, align 8, !noalias !783
  %_8.sroa.9.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 64
  store i64 %s.1, ptr %_8.sroa.9.0._4.i.i.i.i.sroa_idx, align 8, !noalias !783
  %_8.sroa.10.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 72
  store ptr %s.0, ptr %_8.sroa.10.0._4.i.i.i.i.sroa_idx, align 8, !noalias !783
  %_8.sroa.11.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 80
  store ptr %_16, ptr %_8.sroa.11.0._4.i.i.i.i.sroa_idx, align 8, !noalias !783
  %_8.sroa.12.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 88
  store i64 0, ptr %_8.sroa.12.0._4.i.i.i.i.sroa_idx, align 8, !noalias !783
  %_8.sroa.133.0._4.i.i.i.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 104
  store i32 1114113, ptr %_8.sroa.133.0._4.i.i.i.i.sroa_idx, align 8, !noalias !783
  tail call void @llvm.experimental.noalias.scope.decl(metadata !784)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !787
; call <console::ansi::AnsiCodeIterator as core::iter::traits::iterator::Iterator>::next
  call void @_RNvXs5_NtCsC3JuwEIQwb_7console4ansiNtB5_16AnsiCodeIteratorNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5.i.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(112) %_4.i.i.i.i), !noalias !766
  %2 = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i.i, i64 16
  %3 = load i8, ptr %2, align 8, !range !789, !noalias !787, !noundef !7
  %.not1.i.i.i.i.i = icmp eq i8 %3, 2
  br i1 %.not1.i.i.i.i.i, label %_RINvXs7_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorReE9from_iterINtNtNtBS_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2F_16strip_ansi_codes0EEB2H_.exit, label %bb3.lr.ph.i.i.i.i.i

bb3.lr.ph.i.i.i.i.i:                              ; preds = %bb4
  %x.sroa.2.0._5.sroa_idx.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i.i, i64 8
  %extract.t.i.i.i.i.i = trunc nuw i8 %3 to i1
  br label %bb3.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %.noexc3.i, %bb3.lr.ph.i.i.i.i.i
  %_10.i.i.i.i.i.i.i.i6.i = phi ptr [ inttoptr (i64 1 to ptr), %bb3.lr.ph.i.i.i.i.i ], [ %_10.i.i.i.i.i.i.i.i7.i, %.noexc3.i ]
  %len.i.i.i.i.i.i.i.i.i.i = phi i64 [ 0, %bb3.lr.ph.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i5.i, %.noexc3.i ]
  %.off0.i.i.i.i.i = phi i1 [ %extract.t.i.i.i.i.i, %bb3.lr.ph.i.i.i.i.i ], [ %extract.t2.i.i.i.i.i, %.noexc3.i ]
  %x.sroa.0.0.copyload.i.i.i.i.i = load ptr, ptr %_5.i.i.i.i.i, align 8, !noalias !787
  %x.sroa.2.0.copyload.i.i.i.i.i = load i64, ptr %x.sroa.2.0._5.sroa_idx.i.i.i.i.i, align 8, !noalias !787
  %.not1.i.i.i.i.i.i = icmp eq ptr %x.sroa.0.0.copyload.i.i.i.i.i, null
  %.not.i.i.i.i.i.i = select i1 %.off0.i.i.i.i.i, i1 true, i1 %.not1.i.i.i.i.i.i
  br i1 %.not.i.i.i.i.i.i, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map15filter_map_foldTRebEB1b_uNCNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1b_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB4_9FilterMapNtB1o_16AnsiCodeIteratorB1k_ENtB3e_13SpecExtendStr16spec_extend_into0E0E0B1q_.exit.i.i.i.i.i, label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !790)
  %self2.i.i.i.i.i.i.i.i.i.i = load i64, ptr %buf.i, align 8, !range !23, !alias.scope !793, !noalias !796, !noundef !7
  %_9.i.i.i.i.i.i.i.i.i.i = sub i64 %self2.i.i.i.i.i.i.i.i.i.i, %len.i.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i = icmp ugt i64 %x.sroa.2.0.copyload.i.i.i.i.i, %_9.i.i.i.i.i.i.i.i.i.i
  br i1 %_7.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtBc_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2z_16strip_ansi_codes0ENtB1p_13SpecExtendStr16spec_extend_into0E0B2B_.exit.i.i.i.i.i.i, !prof !5

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb4.i.i.i.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i.i.i.i.i.i.i, i64 noundef %x.sroa.2.0.copyload.i.i.i.i.i, i64 noundef 1, i64 noundef 1)
          to label %.noexc2.i unwind label %cleanup.i, !noalias !766

.noexc2.i:                                        ; preds = %bb1.i.i.i.i.i.i.i.i.i.i
  %len.pre.i.i.i.i.i.i.i.i.i = load i64, ptr %_5.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !803, !noalias !796
  %_10.i.i.i.i.i.i.i.i.pre.i = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !803, !noalias !796
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtBc_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2z_16strip_ansi_codes0ENtB1p_13SpecExtendStr16spec_extend_into0E0B2B_.exit.i.i.i.i.i.i

_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtBc_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2z_16strip_ansi_codes0ENtB1p_13SpecExtendStr16spec_extend_into0E0B2B_.exit.i.i.i.i.i.i: ; preds = %.noexc2.i, %bb4.i.i.i.i.i.i
  %_10.i.i.i.i.i.i.i.i.i = phi ptr [ %_10.i.i.i.i.i.i.i.i6.i, %bb4.i.i.i.i.i.i ], [ %_10.i.i.i.i.i.i.i.i.pre.i, %.noexc2.i ]
  %len.i.i.i.i.i.i.i.i.i = phi i64 [ %len.i.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %len.pre.i.i.i.i.i.i.i.i.i, %.noexc2.i ]
  %_9.i.i.i.i.i.i.i.i.i = icmp sgt i64 %len.i.i.i.i.i.i.i.i.i, -1
  tail call void @llvm.assume(i1 %_9.i.i.i.i.i.i.i.i.i)
  %dst.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i.i.i.i.i.i.i, i64 %len.i.i.i.i.i.i.i.i.i
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i.i.i.i.i.i.i.i.i, ptr nonnull readonly align 1 %x.sroa.0.0.copyload.i.i.i.i.i, i64 %x.sroa.2.0.copyload.i.i.i.i.i, i1 false), !noalias !804
  %4 = add i64 %len.i.i.i.i.i.i.i.i.i, %x.sroa.2.0.copyload.i.i.i.i.i
  store i64 %4, ptr %_5.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !803, !noalias !796
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map15filter_map_foldTRebEB1b_uNCNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1b_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB4_9FilterMapNtB1o_16AnsiCodeIteratorB1k_ENtB3e_13SpecExtendStr16spec_extend_into0E0E0B1q_.exit.i.i.i.i.i

_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map15filter_map_foldTRebEB1b_uNCNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1b_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB4_9FilterMapNtB1o_16AnsiCodeIteratorB1k_ENtB3e_13SpecExtendStr16spec_extend_into0E0E0B1q_.exit.i.i.i.i.i: ; preds = %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtBc_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2z_16strip_ansi_codes0ENtB1p_13SpecExtendStr16spec_extend_into0E0B2B_.exit.i.i.i.i.i.i, %bb3.i.i.i.i.i
  %_10.i.i.i.i.i.i.i.i7.i = phi ptr [ %_10.i.i.i.i.i.i.i.i.i, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtBc_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2z_16strip_ansi_codes0ENtB1p_13SpecExtendStr16spec_extend_into0E0B2B_.exit.i.i.i.i.i.i ], [ %_10.i.i.i.i.i.i.i.i6.i, %bb3.i.i.i.i.i ]
  %len.i.i.i.i.i.i.i.i.i5.i = phi i64 [ %4, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtBc_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2z_16strip_ansi_codes0ENtB1p_13SpecExtendStr16spec_extend_into0E0B2B_.exit.i.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !787
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !787
; invoke <console::ansi::AnsiCodeIterator as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXs5_NtCsC3JuwEIQwb_7console4ansiNtB5_16AnsiCodeIteratorNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5.i.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(112) %_4.i.i.i.i)
          to label %.noexc3.i unwind label %cleanup.i, !noalias !766

.noexc3.i:                                        ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map15filter_map_foldTRebEB1b_uNCNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1b_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB4_9FilterMapNtB1o_16AnsiCodeIteratorB1k_ENtB3e_13SpecExtendStr16spec_extend_into0E0E0B1q_.exit.i.i.i.i.i
  %5 = load i8, ptr %2, align 8, !range !789, !noalias !787, !noundef !7
  %.not.i.i.i.i.i = icmp eq i8 %5, 2
  %extract.t2.i.i.i.i.i = trunc nuw i8 %5 to i1
  br i1 %.not.i.i.i.i.i, label %_RINvXs7_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorReE9from_iterINtNtNtBS_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2F_16strip_ansi_codes0EEB2H_.exit, label %bb3.i.i.i.i.i

cleanup.i:                                        ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map15filter_map_foldTRebEB1b_uNCNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1b_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB4_9FilterMapNtB1o_16AnsiCodeIteratorB1k_ENtB3e_13SpecExtendStr16spec_extend_into0E0E0B1q_.exit.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit.i = landingpad { ptr, i32 }
          cleanup
  %buf.val.pre.i = load i64, ptr %buf.i, align 8, !noalias !766
  %6 = icmp eq i64 %buf.val.pre.i, 0
  br i1 %6, label %bb2.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i
  %buf.val1.i = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !766, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %buf.val1.i, i64 noundef %buf.val.pre.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !766
  br label %bb2.i

bb2.i:                                            ; preds = %bb2.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.loopexit.i

_RINvXs7_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorReE9from_iterINtNtNtBS_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2F_16strip_ansi_codes0EEB2H_.exit: ; preds = %.noexc3.i, %bb4
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !787
  call void @llvm.lifetime.end.p0(i64 112, ptr nonnull %_4.i.i.i.i), !noalias !779
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !766
  br label %bb5

bb3:                                              ; preds = %start
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %s.0, ptr %7, align 8
  %8 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %s.1, ptr %8, align 8
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb5

bb5:                                              ; preds = %_RINvXs7_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorReE9from_iterINtNtNtBS_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2F_16strip_ansi_codes0EEB2H_.exit, %bb3
  call void @llvm.lifetime.end.p0(i64 40, ptr nonnull %char_it)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; console::ansi::find_ansi_code_exclusive
; Function Attrs: nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable
define internal fastcc void @_RNvNtCsC3JuwEIQwb_7console4ansi24find_ansi_code_exclusive(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(40) %it) unnamed_addr #9 personality ptr @rust_eh_personality {
start:
  %_29 = getelementptr inbounds nuw i8, ptr %it, i64 24
  %0 = getelementptr inbounds nuw i8, ptr %it, i64 32
  %it.promoted139 = load ptr, ptr %it, align 8
  %_29.promoted149 = load i64, ptr %_29, align 8
  %1 = getelementptr inbounds nuw i8, ptr %it, i64 8
  %_4.i3.i.i.i = load ptr, ptr %1, align 8, !nonnull !7
  %2 = getelementptr inbounds nuw i8, ptr %it, i64 16
  %.promoted155 = load i32, ptr %0, align 8
  %.promoted = load i64, ptr %2, align 8
  br label %bb1

bb1:                                              ; preds = %bb1.backedge, %start
  %3 = phi i64 [ %.promoted, %start ], [ %.be, %bb1.backedge ]
  %.promoted156 = phi i32 [ %.promoted155, %start ], [ %.promoted156.be, %bb1.backedge ]
  %_29.promoted150 = phi i64 [ %_29.promoted149, %start ], [ %_29.promoted150.be, %bb1.backedge ]
  %it.promoted141 = phi ptr [ %it.promoted139, %start ], [ %it.promoted141.be, %bb1.backedge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !805)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !808)
  %.not.i = icmp eq i32 %.promoted156, 1114113
  br i1 %.not.i, label %bb1.i, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit

bb1.i:                                            ; preds = %bb1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !810)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !813)
  %4 = ptrtoint ptr %it.promoted141 to i64
  tail call void @llvm.experimental.noalias.scope.decl(metadata !816)
  %_6.i.i.i.i.i = icmp eq ptr %it.promoted141, %_4.i3.i.i.i
  br i1 %_6.i.i.i.i.i, label %_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console.exit.i, label %bb14.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb1.i
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %it.promoted141, i64 1
  store ptr %_16.i.i.i.i.i, ptr %it, align 8, !alias.scope !819, !noalias !805
  %x.i.i.i.i = load i8, ptr %it.promoted141, align 1, !noalias !822, !noundef !7
  %_6.i.i.i.i = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i, label %bb3.i.i.i.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %it.promoted141, i64 2
  store ptr %_16.i12.i.i.i.i, ptr %it, align 8, !alias.scope !823, !noalias !805
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !noalias !822, !noundef !7
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %5 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i.i.i.i, label %bb7.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i = zext nneg i8 %x.i.i.i.i to i32
  br label %bb7.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb4.i.i.i.i
  %_6.i17.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %it.promoted141, i64 3
  store ptr %_16.i19.i.i.i.i, ptr %it, align 8, !alias.scope !826, !noalias !805
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !noalias !822, !noundef !7
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %6 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %bb8.i.i.i.i, label %bb7.i.i.i

bb8.i.i.i.i:                                      ; preds = %bb6.i.i.i.i
  %_6.i24.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %_16.i26.i.i.i.i = getelementptr inbounds nuw i8, ptr %it.promoted141, i64 4
  store ptr %_16.i26.i.i.i.i, ptr %it, align 8, !alias.scope !829, !noalias !805
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !noalias !822, !noundef !7
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %7 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  br label %bb7.i.i.i

bb7.i.i.i:                                        ; preds = %bb8.i.i.i.i, %bb6.i.i.i.i, %bb3.i.i.i.i, %bb4.i.i.i.i
  %subtracted.i.i.i.i = phi ptr [ %_16.i12.i.i.i.i, %bb4.i.i.i.i ], [ %_16.i19.i.i.i.i, %bb6.i.i.i.i ], [ %_16.i26.i.i.i.i, %bb8.i.i.i.i ], [ %_16.i.i.i.i.i, %bb3.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i = phi i32 [ %5, %bb4.i.i.i.i ], [ %6, %bb6.i.i.i.i ], [ %7, %bb8.i.i.i.i ], [ %_7.i.i.i.i, %bb3.i.i.i.i ]
  %8 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i, 1114112
  tail call void @llvm.assume(i1 %8)
  %9 = ptrtoint ptr %subtracted.i.i.i.i to i64
  %_10.i.i.i = sub i64 %9, %4
  %10 = add i64 %_10.i.i.i, %3
  store i64 %10, ptr %2, align 8, !alias.scope !832, !noalias !805
  br label %_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console.exit.i

_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console.exit.i: ; preds = %bb7.i.i.i, %bb1.i
  %11 = phi i64 [ %10, %bb7.i.i.i ], [ %3, %bb1.i ]
  %it.promoted148 = phi ptr [ %subtracted.i.i.i.i, %bb7.i.i.i ], [ %it.promoted141, %bb1.i ]
  %_0.sroa.3.0.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i, %bb7.i.i.i ], [ 1114112, %bb1.i ]
  %_0.sroa.0.0.i.i.i = phi i64 [ %3, %bb7.i.i.i ], [ undef, %bb1.i ]
  store i64 %_0.sroa.0.0.i.i.i, ptr %_29, align 8, !alias.scope !805, !noalias !808
  store i32 %_0.sroa.3.0.i.i.i, ptr %0, align 8, !alias.scope !805, !noalias !808
  br label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit: ; preds = %bb1, %_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console.exit.i
  %12 = phi i64 [ %3, %bb1 ], [ %11, %_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console.exit.i ]
  %.promoted160 = phi i32 [ %.promoted156, %bb1 ], [ %_0.sroa.3.0.i.i.i, %_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console.exit.i ]
  %_29.promoted154 = phi i64 [ %_29.promoted150, %bb1 ], [ %_0.sroa.0.0.i.i.i, %_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console.exit.i ]
  %it.promoted147 = phi ptr [ %it.promoted141, %bb1 ], [ %it.promoted148, %_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console.exit.i ]
  switch i32 %.promoted160, label %bb3 [
    i32 1114112, label %bb16
    i32 27, label %bb30.peel
    i32 155, label %bb30.peel
  ]

bb16:                                             ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit, %bb13
  %storemerge = phi i64 [ 1, %bb13 ], [ 0, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit ]
  store i64 %storemerge, ptr %_0, align 8
  ret void

bb3:                                              ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit
  store i32 1114113, ptr %0, align 8
  br label %bb1.backedge

bb1.backedge:                                     ; preds = %bb3, %bb12
  %.be = phi i64 [ %28, %bb12 ], [ %12, %bb3 ]
  %.promoted156.be = phi i32 [ %.promoted158.lcssa.ph, %bb12 ], [ 1114113, %bb3 ]
  %_29.promoted150.be = phi i64 [ %_0.sroa.0.0.i.i.i40168, %bb12 ], [ %_29.promoted154, %bb3 ]
  %it.promoted141.be = phi ptr [ %it.promoted144167, %bb12 ], [ %it.promoted147, %bb3 ]
  br label %bb1

bb30.peel:                                        ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit
  store i32 1114113, ptr %0, align 8
  %_6.i.i.i.i.i18202 = icmp eq ptr %it.promoted147, %_4.i3.i.i.i
  br i1 %_6.i.i.i.i.i18202, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread, label %bb14.i.i.i.i19

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread.loopexit: ; preds = %bb30
  store ptr %subtracted.i.i.i.i34, ptr %it, align 8, !alias.scope !833, !noalias !842
  store i64 %20, ptr %2, align 8, !alias.scope !844, !noalias !842
  store i64 %13, ptr %_29, align 8, !alias.scope !842, !noalias !845
  br label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread: ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread.loopexit, %bb30.peel
  %.lcssa = phi i64 [ %12, %bb30.peel ], [ %20, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread.loopexit ]
  %it.promoted145.lcssa = phi ptr [ %it.promoted147, %bb30.peel ], [ %subtracted.i.i.i.i34, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread.loopexit ]
  %maybe_end.sroa.3.0.lcssa = phi i64 [ undef, %bb30.peel ], [ %maybe_end.sroa.3.1, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread.loopexit ]
  %maybe_end.sroa.0.0.lcssa = phi i1 [ false, %bb30.peel ], [ %maybe_end.sroa.0.1, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread.loopexit ]
  store i32 1114112, ptr %0, align 8, !alias.scope !842, !noalias !845
  br label %bb12

bb14.i.i.i.i19:                                   ; preds = %bb30.peel, %bb30
  %maybe_end.sroa.0.0206 = phi i1 [ %maybe_end.sroa.0.1, %bb30 ], [ false, %bb30.peel ]
  %maybe_end.sroa.3.0205 = phi i64 [ %maybe_end.sroa.3.1, %bb30 ], [ undef, %bb30.peel ]
  %state.sroa.0.0204 = phi i8 [ %state.sroa.0.2, %bb30 ], [ 1, %bb30.peel ]
  %it.promoted145203 = phi ptr [ %subtracted.i.i.i.i34, %bb30 ], [ %it.promoted147, %bb30.peel ]
  %13 = phi i64 [ %20, %bb30 ], [ %12, %bb30.peel ]
  %14 = ptrtoint ptr %it.promoted145203 to i64
  tail call void @llvm.experimental.noalias.scope.decl(metadata !842)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !845)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !846)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !847)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !848)
  %_16.i.i.i.i.i20 = getelementptr inbounds nuw i8, ptr %it.promoted145203, i64 1
  %x.i.i.i.i21 = load i8, ptr %it.promoted145203, align 1, !noalias !849, !noundef !7
  %_6.i.i.i.i22 = icmp sgt i8 %x.i.i.i.i21, -1
  br i1 %_6.i.i.i.i22, label %bb3.i.i.i.i61, label %bb4.i.i.i.i23

bb4.i.i.i.i23:                                    ; preds = %bb14.i.i.i.i19
  %_30.i.i.i.i24 = and i8 %x.i.i.i.i21, 31
  %init.i.i.i.i25 = zext nneg i8 %_30.i.i.i.i24 to i32
  %_6.i10.i.i.i.i26 = icmp ne ptr %_16.i.i.i.i.i20, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i26)
  %_16.i12.i.i.i.i27 = getelementptr inbounds nuw i8, ptr %it.promoted145203, i64 2
  %y.i.i.i.i28 = load i8, ptr %_16.i.i.i.i.i20, align 1, !noalias !849, !noundef !7
  %_33.i.i.i.i29 = shl nuw nsw i32 %init.i.i.i.i25, 6
  %_35.i.i.i.i30 = and i8 %y.i.i.i.i28, 63
  %_34.i.i.i.i31 = zext nneg i8 %_35.i.i.i.i30 to i32
  %15 = or disjoint i32 %_33.i.i.i.i29, %_34.i.i.i.i31
  %_13.i.i.i.i32 = icmp samesign ugt i8 %x.i.i.i.i21, -33
  br i1 %_13.i.i.i.i32, label %bb6.i.i.i.i41, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63

bb3.i.i.i.i61:                                    ; preds = %bb14.i.i.i.i19
  %_7.i.i.i.i62 = zext nneg i8 %x.i.i.i.i21 to i32
  br label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63

bb6.i.i.i.i41:                                    ; preds = %bb4.i.i.i.i23
  %_6.i17.i.i.i.i42 = icmp ne ptr %_16.i12.i.i.i.i27, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i42)
  %_16.i19.i.i.i.i43 = getelementptr inbounds nuw i8, ptr %it.promoted145203, i64 3
  %z.i.i.i.i44 = load i8, ptr %_16.i12.i.i.i.i27, align 1, !noalias !849, !noundef !7
  %_38.i.i.i.i45 = shl nuw nsw i32 %_34.i.i.i.i31, 6
  %_40.i.i.i.i46 = and i8 %z.i.i.i.i44, 63
  %_39.i.i.i.i47 = zext nneg i8 %_40.i.i.i.i46 to i32
  %y_z.i.i.i.i48 = or disjoint i32 %_38.i.i.i.i45, %_39.i.i.i.i47
  %_20.i.i.i.i49 = shl nuw nsw i32 %init.i.i.i.i25, 12
  %16 = or disjoint i32 %y_z.i.i.i.i48, %_20.i.i.i.i49
  %_21.i.i.i.i50 = icmp samesign ugt i8 %x.i.i.i.i21, -17
  br i1 %_21.i.i.i.i50, label %bb8.i.i.i.i51, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63

bb8.i.i.i.i51:                                    ; preds = %bb6.i.i.i.i41
  %_6.i24.i.i.i.i52 = icmp ne ptr %_16.i19.i.i.i.i43, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i52)
  %_16.i26.i.i.i.i53 = getelementptr inbounds nuw i8, ptr %it.promoted145203, i64 4
  %w.i.i.i.i54 = load i8, ptr %_16.i19.i.i.i.i43, align 1, !noalias !849, !noundef !7
  %_26.i.i.i.i55 = shl nuw nsw i32 %init.i.i.i.i25, 18
  %_25.i.i.i.i56 = and i32 %_26.i.i.i.i55, 1835008
  %_43.i.i.i.i57 = shl nuw nsw i32 %y_z.i.i.i.i48, 6
  %_45.i.i.i.i58 = and i8 %w.i.i.i.i54, 63
  %_44.i.i.i.i59 = zext nneg i8 %_45.i.i.i.i58 to i32
  %_27.i.i.i.i60 = or disjoint i32 %_43.i.i.i.i57, %_44.i.i.i.i59
  %17 = or disjoint i32 %_27.i.i.i.i60, %_25.i.i.i.i56
  br label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63: ; preds = %bb4.i.i.i.i23, %bb3.i.i.i.i61, %bb6.i.i.i.i41, %bb8.i.i.i.i51
  %subtracted.i.i.i.i34 = phi ptr [ %_16.i12.i.i.i.i27, %bb4.i.i.i.i23 ], [ %_16.i19.i.i.i.i43, %bb6.i.i.i.i41 ], [ %_16.i26.i.i.i.i53, %bb8.i.i.i.i51 ], [ %_16.i.i.i.i.i20, %bb3.i.i.i.i61 ]
  %_0.sroa.4.0.i.ph.i.i.i35 = phi i32 [ %15, %bb4.i.i.i.i23 ], [ %16, %bb6.i.i.i.i41 ], [ %17, %bb8.i.i.i.i51 ], [ %_7.i.i.i.i62, %bb3.i.i.i.i61 ]
  %18 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i35, 1114112
  tail call void @llvm.assume(i1 %18)
  %19 = ptrtoint ptr %subtracted.i.i.i.i34 to i64
  %_10.i.i.i37 = sub i64 %19, %14
  %20 = add i64 %_10.i.i.i37, %13
  store i32 %_0.sroa.4.0.i.ph.i.i.i35, ptr %0, align 8, !alias.scope !842, !noalias !845
  switch i32 %_0.sroa.4.0.i.ph.i.i.i35, label %bb2.i [
    i32 60, label %bb12.i
    i32 27, label %bb12.loopexit
    i32 155, label %bb12.loopexit
    i32 40, label %bb15.i
    i32 41, label %bb15.i
    i32 59, label %bb14.i
    i32 91, label %bb13.i
    i32 35, label %bb13.i
    i32 63, label %bb13.i
    i32 82, label %bb12.i
    i32 90, label %bb12.i
    i32 99, label %bb12.i
    i32 113, label %bb12.i
    i32 114, label %bb12.i
    i32 121, label %bb12.i
    i32 61, label %bb12.i
    i32 62, label %bb12.i
  ]

bb2.i:                                            ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63
  %21 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i35, -48
  %or.cond.i = icmp ult i32 %21, 3
  br i1 %or.cond.i, label %bb3.i, label %bb4.i

bb15.i:                                           ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63
  switch i8 %state.sroa.0.0204, label %bb12.loopexit [
    i8 1, label %bb30
    i8 2, label %bb20.i
    i8 4, label %bb20.i
  ]

bb14.i:                                           ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63
  switch i8 %state.sroa.0.0204, label %bb12.loopexit [
    i8 1, label %bb30
    i8 2, label %bb30
    i8 4, label %bb30
    i8 5, label %bb23.i
    i8 6, label %bb23.i
    i8 7, label %bb23.i
    i8 8, label %bb23.i
    i8 10, label %bb23.i
  ]

bb13.i:                                           ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63
  switch i8 %state.sroa.0.0204, label %bb12.loopexit [
    i8 1, label %bb30
    i8 2, label %bb30
    i8 4, label %bb30
  ]

bb12.i:                                           ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %bb6.i
  switch i8 %state.sroa.0.0204, label %bb12.loopexit [
    i8 1, label %bb30
    i8 2, label %bb30
    i8 4, label %bb30
    i8 5, label %bb30
    i8 6, label %bb30
    i8 7, label %bb30
    i8 8, label %bb30
    i8 10, label %bb30
  ]

bb20.i:                                           ; preds = %bb15.i, %bb15.i
  br label %bb30

bb23.i:                                           ; preds = %bb14.i, %bb14.i, %bb14.i, %bb14.i, %bb14.i
  br label %bb30

bb4.i:                                            ; preds = %bb2.i
  %22 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i35, -51
  %or.cond1.i = icmp ult i32 %22, 7
  br i1 %or.cond1.i, label %bb5.i, label %bb6.i

bb3.i:                                            ; preds = %bb2.i
  switch i8 %state.sroa.0.0204, label %bb12.loopexit [
    i8 1, label %bb30
    i8 2, label %bb28.i
    i8 4, label %bb30
    i8 5, label %bb29.i
    i8 6, label %bb30.i
    i8 7, label %bb31.i
    i8 8, label %bb32.i
    i8 10, label %bb30
  ]

bb6.i:                                            ; preds = %bb4.i
  %23 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i35, -65
  %or.cond2.i = icmp ult i32 %23, 16
  %24 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i35, -102
  %or.cond3.i = icmp ult i32 %24, 9
  %or.cond4.i = select i1 %or.cond2.i, i1 true, i1 %or.cond3.i
  br i1 %or.cond4.i, label %bb12.i, label %bb12.loopexit

bb5.i:                                            ; preds = %bb4.i
  switch i8 %state.sroa.0.0204, label %bb12.loopexit [
    i8 1, label %bb30
    i8 2, label %bb30
    i8 4, label %bb30
    i8 5, label %bb37.i
    i8 6, label %bb38.i
    i8 7, label %bb39.i
    i8 8, label %bb40.i
    i8 10, label %bb30
  ]

bb37.i:                                           ; preds = %bb5.i
  br label %bb30

bb38.i:                                           ; preds = %bb5.i
  br label %bb30

bb39.i:                                           ; preds = %bb5.i
  br label %bb30

bb40.i:                                           ; preds = %bb5.i
  br label %bb30

bb28.i:                                           ; preds = %bb3.i
  br label %bb30

bb29.i:                                           ; preds = %bb3.i
  br label %bb30

bb30.i:                                           ; preds = %bb3.i
  br label %bb30

bb31.i:                                           ; preds = %bb3.i
  br label %bb30

bb32.i:                                           ; preds = %bb3.i
  br label %bb30

bb13:                                             ; preds = %bb12
  %25 = add i64 %maybe_end.sroa.3.0197, 1
  %26 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_29.promoted154, ptr %26, align 8
  %27 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %25, ptr %27, align 8
  br label %bb16

bb12.loopexit:                                    ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63, %bb3.i, %bb5.i, %bb12.i, %bb6.i, %bb13.i, %bb14.i, %bb15.i
  %.promoted158.lcssa.ph.ph = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i35, %bb15.i ], [ 59, %bb14.i ], [ %_0.sroa.4.0.i.ph.i.i.i35, %bb13.i ], [ %_0.sroa.4.0.i.ph.i.i.i35, %bb6.i ], [ %_0.sroa.4.0.i.ph.i.i.i35, %bb12.i ], [ %_0.sroa.4.0.i.ph.i.i.i35, %bb5.i ], [ %_0.sroa.4.0.i.ph.i.i.i35, %bb3.i ], [ %_0.sroa.4.0.i.ph.i.i.i35, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63 ], [ %_0.sroa.4.0.i.ph.i.i.i35, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63 ]
  store ptr %subtracted.i.i.i.i34, ptr %it, align 8, !alias.scope !833, !noalias !842
  store i64 %20, ptr %2, align 8, !alias.scope !844, !noalias !842
  store i64 %13, ptr %_29, align 8, !alias.scope !842, !noalias !845
  br label %bb12

bb12:                                             ; preds = %bb12.loopexit, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread
  %maybe_end.sroa.3.0197 = phi i64 [ %maybe_end.sroa.3.0.lcssa, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread ], [ %maybe_end.sroa.3.0205, %bb12.loopexit ]
  %maybe_end.sroa.0.0195 = phi i1 [ %maybe_end.sroa.0.0.lcssa, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread ], [ %maybe_end.sroa.0.0206, %bb12.loopexit ]
  %_0.sroa.0.0.i.i.i40168 = phi i64 [ undef, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread ], [ %13, %bb12.loopexit ]
  %it.promoted144167 = phi ptr [ %it.promoted145.lcssa, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread ], [ %subtracted.i.i.i.i34, %bb12.loopexit ]
  %28 = phi i64 [ %.lcssa, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread ], [ %20, %bb12.loopexit ]
  %.promoted158.lcssa.ph = phi i32 [ 1114112, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread ], [ %.promoted158.lcssa.ph.ph, %bb12.loopexit ]
  br i1 %maybe_end.sroa.0.0195, label %bb13, label %bb1.backedge

bb30:                                             ; preds = %bb13.i, %bb13.i, %bb13.i, %bb14.i, %bb14.i, %bb14.i, %bb15.i, %bb20.i, %bb23.i, %bb3.i, %bb3.i, %bb3.i, %bb5.i, %bb5.i, %bb5.i, %bb5.i, %bb12.i, %bb12.i, %bb12.i, %bb12.i, %bb12.i, %bb12.i, %bb12.i, %bb12.i, %bb32.i, %bb31.i, %bb30.i, %bb29.i, %bb28.i, %bb40.i, %bb39.i, %bb38.i, %bb37.i
  %state.sroa.0.2 = phi i8 [ 10, %bb23.i ], [ 4, %bb20.i ], [ 2, %bb15.i ], [ 4, %bb14.i ], [ 4, %bb14.i ], [ 4, %bb14.i ], [ 4, %bb13.i ], [ 4, %bb13.i ], [ 4, %bb13.i ], [ 3, %bb28.i ], [ 6, %bb29.i ], [ 7, %bb30.i ], [ 8, %bb31.i ], [ 9, %bb32.i ], [ 6, %bb37.i ], [ 7, %bb38.i ], [ 8, %bb39.i ], [ 9, %bb40.i ], [ 11, %bb12.i ], [ 11, %bb12.i ], [ 11, %bb12.i ], [ 11, %bb12.i ], [ 11, %bb12.i ], [ 11, %bb12.i ], [ 11, %bb12.i ], [ 11, %bb12.i ], [ 5, %bb5.i ], [ 5, %bb5.i ], [ 5, %bb5.i ], [ 5, %bb5.i ], [ 5, %bb3.i ], [ 5, %bb3.i ], [ 5, %bb3.i ]
  %maybe_end.sroa.3.1 = phi i64 [ %maybe_end.sroa.3.0205, %bb23.i ], [ %maybe_end.sroa.3.0205, %bb20.i ], [ %maybe_end.sroa.3.0205, %bb15.i ], [ %maybe_end.sroa.3.0205, %bb14.i ], [ %maybe_end.sroa.3.0205, %bb14.i ], [ %maybe_end.sroa.3.0205, %bb14.i ], [ %maybe_end.sroa.3.0205, %bb13.i ], [ %maybe_end.sroa.3.0205, %bb13.i ], [ %maybe_end.sroa.3.0205, %bb13.i ], [ %13, %bb28.i ], [ %13, %bb29.i ], [ %13, %bb30.i ], [ %13, %bb31.i ], [ %13, %bb32.i ], [ %13, %bb37.i ], [ %13, %bb38.i ], [ %13, %bb39.i ], [ %13, %bb40.i ], [ %13, %bb12.i ], [ %13, %bb12.i ], [ %13, %bb12.i ], [ %13, %bb12.i ], [ %13, %bb12.i ], [ %13, %bb12.i ], [ %13, %bb12.i ], [ %13, %bb12.i ], [ %13, %bb5.i ], [ %13, %bb5.i ], [ %13, %bb5.i ], [ %13, %bb5.i ], [ %13, %bb3.i ], [ %13, %bb3.i ], [ %13, %bb3.i ]
  %maybe_end.sroa.0.1 = phi i1 [ %maybe_end.sroa.0.0206, %bb23.i ], [ %maybe_end.sroa.0.0206, %bb20.i ], [ %maybe_end.sroa.0.0206, %bb15.i ], [ %maybe_end.sroa.0.0206, %bb14.i ], [ %maybe_end.sroa.0.0206, %bb14.i ], [ %maybe_end.sroa.0.0206, %bb14.i ], [ %maybe_end.sroa.0.0206, %bb13.i ], [ %maybe_end.sroa.0.0206, %bb13.i ], [ %maybe_end.sroa.0.0206, %bb13.i ], [ true, %bb28.i ], [ true, %bb29.i ], [ true, %bb30.i ], [ true, %bb31.i ], [ true, %bb32.i ], [ true, %bb37.i ], [ true, %bb38.i ], [ true, %bb39.i ], [ true, %bb40.i ], [ true, %bb12.i ], [ true, %bb12.i ], [ true, %bb12.i ], [ true, %bb12.i ], [ true, %bb12.i ], [ true, %bb12.i ], [ true, %bb12.i ], [ true, %bb12.i ], [ true, %bb5.i ], [ true, %bb5.i ], [ true, %bb5.i ], [ true, %bb5.i ], [ true, %bb3.i ], [ true, %bb3.i ], [ true, %bb3.i ]
  store i32 1114113, ptr %0, align 8
  %_6.i.i.i.i.i18 = icmp eq ptr %subtracted.i.i.i.i34, %_4.i3.i.i.i
  br i1 %_6.i.i.i.i.i18, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console.exit63.thread.loopexit, label %bb14.i.i.i.i19, !llvm.loop !850
}

; console::utils::pad_str_with
; Function Attrs: uwtable
define void @_RNvNtCsC3JuwEIQwb_7console5utils12pad_str_with(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i64 noundef %width, i8 noundef range(i8 0, 3) %align, ptr noalias noundef readonly align 1 captures(address) %0, i64 %1, i32 noundef range(i32 0, 1114112) %pad) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_3.i = alloca [24 x i8], align 8
  %rv = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3.i), !noalias !852
; call console::ansi::strip_ansi_codes
  call void @_RNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_3.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1)
  %2 = load i64, ptr %_3.i, align 8, !range !132, !noalias !852, !noundef !7
  %3 = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  %_9.i = load ptr, ptr %3, align 8, !noalias !852
  %4 = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  %_8.i = load i64, ptr %4, align 8, !noalias !852
; invoke console::utils::str_width
  %_0.i = invoke fastcc noundef i64 @_RNvNtCsC3JuwEIQwb_7console5utils9str_width(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_9.i, i64 noundef %_8.i)
          to label %bb2.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %start
  %5 = landingpad { ptr, i32 }
          cleanup
  switch i64 %2, label %bb2.i.i.i4.i.i.i.i [
    i64 -9223372036854775808, label %common.resume
    i64 0, label %common.resume
  ]

bb2.i.i.i4.i.i.i.i:                               ; preds = %cleanup.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_9.i, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %common.resume

bb2.i:                                            ; preds = %start
  switch i64 %2, label %bb2.i.i.i4.i.i.i4.i [
    i64 -9223372036854775808, label %_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width.exit
    i64 0, label %_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width.exit
  ]

bb2.i.i.i4.i.i.i4.i:                              ; preds = %bb2.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_9.i, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width.exit

common.resume:                                    ; preds = %cleanup, %bb2.i.i.i4.i.i, %cleanup.i, %cleanup.i, %bb2.i.i.i4.i.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %5, %bb2.i.i.i4.i.i.i.i ], [ %5, %cleanup.i ], [ %5, %cleanup.i ], [ %lpad.phi, %bb2.i.i.i4.i.i ], [ %lpad.phi, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width.exit: ; preds = %bb2.i, %bb2.i, %bb2.i.i.i4.i.i.i4.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i), !noalias !852
  %_7.not = icmp ult i64 %_0.i, %width
  br i1 %_7.not, label %bb6, label %bb2

bb6:                                              ; preds = %_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width.exit
  %6 = sub nuw i64 %width, %_0.i
  switch i8 %align, label %default.unreachable171 [
    i8 0, label %bb10.thread
    i8 1, label %bb9
    i8 2, label %bb10
  ]

bb10.thread:                                      ; preds = %bb6
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %rv)
  store i64 0, ptr %rv, align 8
  %_20.sroa.4.0.rv.sroa_idx174 = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_20.sroa.4.0.rv.sroa_idx174, align 8
  %_20.sroa.5.0.rv.sroa_idx175 = getelementptr inbounds nuw i8, ptr %rv, i64 16
  store i64 0, ptr %_20.sroa.5.0.rv.sroa_idx175, align 8
  br label %bb20

bb2:                                              ; preds = %_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width.exit
  %.not = icmp eq ptr %0, null
  br i1 %.not, label %bb5, label %bb4

default.unreachable171:                           ; preds = %bb6
  unreachable

bb9:                                              ; preds = %bb6
  %7 = lshr i64 %6, 1
  %_13 = sub i64 %6, %7
  br label %bb10

bb10:                                             ; preds = %bb6, %bb9
  %left_pad.sroa.0.0 = phi i64 [ %7, %bb9 ], [ %6, %bb6 ]
  %diff.sroa.0.0 = phi i64 [ %_13, %bb9 ], [ 0, %bb6 ]
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %rv)
  store i64 0, ptr %rv, align 8
  %_20.sroa.4.0.rv.sroa_idx = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_20.sroa.4.0.rv.sroa_idx, align 8
  %_20.sroa.5.0.rv.sroa_idx = getelementptr inbounds nuw i8, ptr %rv, i64 16
  store i64 0, ptr %_20.sroa.5.0.rv.sroa_idx, align 8
  %_2169.not = icmp eq i64 %left_pad.sroa.0.0, 0
  br i1 %_2169.not, label %bb20, label %bb19.lr.ph

bb19.lr.ph:                                       ; preds = %bb10
  %_16.i = icmp samesign ult i32 %pad, 128
  %_18.i = icmp samesign ult i32 %pad, 65536
  %..i = select i1 %_18.i, i64 3, i64 4
  %8 = trunc i32 %pad to i8
  %_5.i.i = and i8 %8, 63
  %last1.i.i = or disjoint i8 %_5.i.i, -128
  %_10.i.i = lshr i32 %pad, 6
  %9 = trunc i32 %_10.i.i to i8
  %_8.i.i = and i8 %9, 63
  %last2.i.i = or disjoint i8 %_8.i.i, -128
  %_14.i.i = lshr i32 %pad, 12
  %10 = trunc i32 %_14.i.i to i8
  %_12.i.i = and i8 %10, 63
  %last3.i.i = or disjoint i8 %_12.i.i, -128
  %_18.i.i = lshr i32 %pad, 18
  %_16.i.i = trunc nuw nsw i32 %_18.i.i to i8
  %last4.i.i = or disjoint i8 %_16.i.i, -16
  %11 = or disjoint i8 %10, -32
  %12 = or disjoint i8 %9, -64
  br i1 %_16.i, label %bb19.us, label %bb19.lr.ph.split

bb19.us:                                          ; preds = %bb19.lr.ph, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us
  %_20.i.us164 = phi ptr [ %_20.i.us, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us ], [ inttoptr (i64 1 to ptr), %bb19.lr.ph ]
  %len.i13.us = phi i64 [ %new_len.i.us, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us ], [ 0, %bb19.lr.ph ]
  %iter.sroa.0.070.us = phi i64 [ %_22.us, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us ], [ 0, %bb19.lr.ph ]
  %_22.us = add nuw i64 %iter.sroa.0.070.us, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !855)
  %_14.i.us = icmp sgt i64 %len.i13.us, -1
  tail call void @llvm.assume(i1 %_14.i.us)
  %self2.i.i.us = load i64, ptr %rv, align 8, !range !23, !alias.scope !858, !noundef !7
  %_7.i.i.us = icmp eq i64 %self2.i.i.us, %len.i13.us
  br i1 %_7.i.i.us, label %bb1.i.i.us, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us, !prof !5

bb1.i.i.us:                                       ; preds = %bb19.us
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv, i64 noundef %len.i13.us, i64 noundef 1, i64 noundef 1, i64 noundef 1)
          to label %.noexc16.us unwind label %cleanup.loopexit.split-lp.loopexit.split.us

.noexc16.us:                                      ; preds = %bb1.i.i.us
  %count.pre.i.us = load i64, ptr %_20.sroa.5.0.rv.sroa_idx, align 8, !alias.scope !855
  %_20.i.us.pre = load ptr, ptr %_20.sroa.4.0.rv.sroa_idx, align 8, !alias.scope !855
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us: ; preds = %.noexc16.us, %bb19.us
  %_20.i.us = phi ptr [ %_20.i.us164, %bb19.us ], [ %_20.i.us.pre, %.noexc16.us ]
  %count.i.us = phi i64 [ %len.i13.us, %bb19.us ], [ %count.pre.i.us, %.noexc16.us ]
  %_21.i.us = icmp sgt i64 %count.i.us, -1
  tail call void @llvm.assume(i1 %_21.i.us)
  %_8.i15.us = getelementptr inbounds nuw i8, ptr %_20.i.us, i64 %count.i.us
  store i8 %8, ptr %_8.i15.us, align 1, !noalias !855
  %new_len.i.us = add nuw i64 %len.i13.us, 1
  store i64 %new_len.i.us, ptr %_20.sroa.5.0.rv.sroa_idx, align 8
  %exitcond152.not = icmp eq i64 %_22.us, %left_pad.sroa.0.0
  br i1 %exitcond152.not, label %bb20, label %bb19.us

cleanup.loopexit.split-lp.loopexit.split.us:      ; preds = %bb1.i.i.us
  %lpad.loopexit66.us = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

bb19.lr.ph.split:                                 ; preds = %bb19.lr.ph
  %_17.i = icmp samesign ult i32 %pad, 2048
  br i1 %_17.i, label %bb19.us71, label %bb19.lr.ph.split.split

bb19.us71:                                        ; preds = %bb19.lr.ph.split, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85
  %_20.i.us87161 = phi ptr [ %_20.i.us87, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85 ], [ inttoptr (i64 1 to ptr), %bb19.lr.ph.split ]
  %len.i13.us74 = phi i64 [ %new_len.i.us91, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85 ], [ 0, %bb19.lr.ph.split ]
  %iter.sroa.0.070.us72 = phi i64 [ %_22.us73, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85 ], [ 0, %bb19.lr.ph.split ]
  %_22.us73 = add nuw i64 %iter.sroa.0.070.us72, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !855)
  %_14.i.us75 = icmp sgt i64 %len.i13.us74, -1
  tail call void @llvm.assume(i1 %_14.i.us75)
  %self2.i.i.us79 = load i64, ptr %rv, align 8, !range !23, !alias.scope !858, !noundef !7
  %_9.i.i.us80 = sub nsw i64 %self2.i.i.us79, %len.i13.us74
  %_7.i.i.us81 = icmp ult i64 %_9.i.i.us80, 2
  br i1 %_7.i.i.us81, label %bb1.i.i.us82, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85, !prof !5

bb1.i.i.us82:                                     ; preds = %bb19.us71
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv, i64 noundef %len.i13.us74, i64 noundef 2, i64 noundef 1, i64 noundef 1)
          to label %.noexc16.us83 unwind label %cleanup.loopexit.split-lp.loopexit.split.split.us

.noexc16.us83:                                    ; preds = %bb1.i.i.us82
  %count.pre.i.us84 = load i64, ptr %_20.sroa.5.0.rv.sroa_idx, align 8, !alias.scope !855
  %_20.i.us87.pre = load ptr, ptr %_20.sroa.4.0.rv.sroa_idx, align 8, !alias.scope !855
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85: ; preds = %.noexc16.us83, %bb19.us71
  %_20.i.us87 = phi ptr [ %_20.i.us87161, %bb19.us71 ], [ %_20.i.us87.pre, %.noexc16.us83 ]
  %count.i.us86 = phi i64 [ %len.i13.us74, %bb19.us71 ], [ %count.pre.i.us84, %.noexc16.us83 ]
  %_21.i.us88 = icmp sgt i64 %count.i.us86, -1
  tail call void @llvm.assume(i1 %_21.i.us88)
  %_8.i15.us89 = getelementptr inbounds nuw i8, ptr %_20.i.us87, i64 %count.i.us86
  store i8 %12, ptr %_8.i15.us89, align 1, !noalias !855
  %_20.i.i.us = getelementptr inbounds nuw i8, ptr %_8.i15.us89, i64 1
  store i8 %last1.i.i, ptr %_20.i.i.us, align 1, !noalias !855
  %new_len.i.us91 = add nuw i64 %len.i13.us74, 2
  store i64 %new_len.i.us91, ptr %_20.sroa.5.0.rv.sroa_idx, align 8
  %exitcond151.not = icmp eq i64 %_22.us73, %left_pad.sroa.0.0
  br i1 %exitcond151.not, label %bb20, label %bb19.us71

cleanup.loopexit.split-lp.loopexit.split.split.us: ; preds = %bb1.i.i.us82
  %lpad.loopexit66.us93 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

bb19.lr.ph.split.split:                           ; preds = %bb19.lr.ph.split
  br i1 %_18.i, label %bb19.us95, label %bb19

bb19.us95:                                        ; preds = %bb19.lr.ph.split.split, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109
  %_20.i.us111158 = phi ptr [ %_20.i.us111, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109 ], [ inttoptr (i64 1 to ptr), %bb19.lr.ph.split.split ]
  %len.i13.us98 = phi i64 [ %new_len.i.us115, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109 ], [ 0, %bb19.lr.ph.split.split ]
  %iter.sroa.0.070.us96 = phi i64 [ %_22.us97, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109 ], [ 0, %bb19.lr.ph.split.split ]
  %_22.us97 = add nuw i64 %iter.sroa.0.070.us96, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !855)
  %_14.i.us99 = icmp sgt i64 %len.i13.us98, -1
  tail call void @llvm.assume(i1 %_14.i.us99)
  %self2.i.i.us103 = load i64, ptr %rv, align 8, !range !23, !alias.scope !858, !noundef !7
  %_9.i.i.us104 = sub nsw i64 %self2.i.i.us103, %len.i13.us98
  %_7.i.i.us105 = icmp ugt i64 %..i, %_9.i.i.us104
  br i1 %_7.i.i.us105, label %bb1.i.i.us106, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109, !prof !5

bb1.i.i.us106:                                    ; preds = %bb19.us95
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv, i64 noundef %len.i13.us98, i64 noundef %..i, i64 noundef 1, i64 noundef 1)
          to label %.noexc16.us107 unwind label %cleanup.loopexit.split-lp.loopexit.split.split.split.us

.noexc16.us107:                                   ; preds = %bb1.i.i.us106
  %count.pre.i.us108 = load i64, ptr %_20.sroa.5.0.rv.sroa_idx, align 8, !alias.scope !855
  %_20.i.us111.pre = load ptr, ptr %_20.sroa.4.0.rv.sroa_idx, align 8, !alias.scope !855
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109: ; preds = %.noexc16.us107, %bb19.us95
  %_20.i.us111 = phi ptr [ %_20.i.us111158, %bb19.us95 ], [ %_20.i.us111.pre, %.noexc16.us107 ]
  %count.i.us110 = phi i64 [ %len.i13.us98, %bb19.us95 ], [ %count.pre.i.us108, %.noexc16.us107 ]
  %_21.i.us112 = icmp sgt i64 %count.i.us110, -1
  tail call void @llvm.assume(i1 %_21.i.us112)
  %_8.i15.us113 = getelementptr inbounds nuw i8, ptr %_20.i.us111, i64 %count.i.us110
  store i8 %11, ptr %_8.i15.us113, align 1, !noalias !855
  %_21.i.i.us = getelementptr inbounds nuw i8, ptr %_8.i15.us113, i64 1
  store i8 %last2.i.i, ptr %_21.i.i.us, align 1, !noalias !855
  %_22.i.i.us = getelementptr inbounds nuw i8, ptr %_8.i15.us113, i64 2
  store i8 %last1.i.i, ptr %_22.i.i.us, align 1, !noalias !855
  %new_len.i.us115 = add nuw i64 %..i, %len.i13.us98
  store i64 %new_len.i.us115, ptr %_20.sroa.5.0.rv.sroa_idx, align 8
  %exitcond150.not = icmp eq i64 %_22.us97, %left_pad.sroa.0.0
  br i1 %exitcond150.not, label %bb20, label %bb19.us95

cleanup.loopexit.split-lp.loopexit.split.split.split.us: ; preds = %bb1.i.i.us106
  %lpad.loopexit66.us117 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

bb20:                                             ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us, %bb10.thread, %bb10
  %_20.sroa.5.0.rv.sroa_idx180 = phi ptr [ %_20.sroa.5.0.rv.sroa_idx, %bb10 ], [ %_20.sroa.5.0.rv.sroa_idx175, %bb10.thread ], [ %_20.sroa.5.0.rv.sroa_idx, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us ], [ %_20.sroa.5.0.rv.sroa_idx, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85 ], [ %_20.sroa.5.0.rv.sroa_idx, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109 ], [ %_20.sroa.5.0.rv.sroa_idx, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i ]
  %_20.sroa.4.0.rv.sroa_idx178 = phi ptr [ %_20.sroa.4.0.rv.sroa_idx, %bb10 ], [ %_20.sroa.4.0.rv.sroa_idx174, %bb10.thread ], [ %_20.sroa.4.0.rv.sroa_idx, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us ], [ %_20.sroa.4.0.rv.sroa_idx, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85 ], [ %_20.sroa.4.0.rv.sroa_idx, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109 ], [ %_20.sroa.4.0.rv.sroa_idx, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i ]
  %diff.sroa.0.0177 = phi i64 [ %diff.sroa.0.0, %bb10 ], [ %6, %bb10.thread ], [ %diff.sroa.0.0, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us ], [ %diff.sroa.0.0, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85 ], [ %diff.sroa.0.0, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109 ], [ %diff.sroa.0.0, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i ]
  %len.i = phi i64 [ 0, %bb10 ], [ 0, %bb10.thread ], [ %new_len.i.us, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us ], [ %new_len.i.us91, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us85 ], [ %new_len.i.us115, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i.us109 ], [ %new_len.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i ]
  %self2.i = load i64, ptr %rv, align 8, !range !23, !alias.scope !861, !noundef !7
  %_9.i11 = sub i64 %self2.i, %len.i
  %_7.i = icmp ugt i64 %s.1, %_9.i11
  br i1 %_7.i, label %bb1.i, label %bb22, !prof !5

bb1.i:                                            ; preds = %bb20
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv, i64 noundef %len.i, i64 noundef %s.1, i64 noundef 1, i64 noundef 1)
          to label %bb1.i.bb22_crit_edge unwind label %cleanup.loopexit.split-lp.loopexit.split-lp

bb1.i.bb22_crit_edge:                             ; preds = %bb1.i
  %_37.pre = load i64, ptr %_20.sroa.5.0.rv.sroa_idx180, align 8
  br label %bb22

bb19:                                             ; preds = %bb19.lr.ph.split.split, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i
  %_20.i155 = phi ptr [ %_20.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i ], [ inttoptr (i64 1 to ptr), %bb19.lr.ph.split.split ]
  %len.i13 = phi i64 [ %new_len.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i ], [ 0, %bb19.lr.ph.split.split ]
  %iter.sroa.0.070 = phi i64 [ %_22, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i ], [ 0, %bb19.lr.ph.split.split ]
  %_22 = add nuw i64 %iter.sroa.0.070, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !855)
  %_14.i = icmp sgt i64 %len.i13, -1
  tail call void @llvm.assume(i1 %_14.i)
  %self2.i.i = load i64, ptr %rv, align 8, !range !23, !alias.scope !858, !noundef !7
  %_9.i.i = sub nsw i64 %self2.i.i, %len.i13
  %_7.i.i = icmp ugt i64 %..i, %_9.i.i
  br i1 %_7.i.i, label %bb1.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i, !prof !5

bb1.i.i:                                          ; preds = %bb19
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv, i64 noundef %len.i13, i64 noundef %..i, i64 noundef 1, i64 noundef 1)
          to label %.noexc16 unwind label %cleanup.loopexit.split-lp.loopexit.split.split.split.split

.noexc16:                                         ; preds = %bb1.i.i
  %count.pre.i = load i64, ptr %_20.sroa.5.0.rv.sroa_idx, align 8, !alias.scope !855
  %_20.i.pre = load ptr, ptr %_20.sroa.4.0.rv.sroa_idx, align 8, !alias.scope !855
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i: ; preds = %.noexc16, %bb19
  %_20.i = phi ptr [ %_20.i155, %bb19 ], [ %_20.i.pre, %.noexc16 ]
  %count.i = phi i64 [ %len.i13, %bb19 ], [ %count.pre.i, %.noexc16 ]
  %_21.i = icmp sgt i64 %count.i, -1
  tail call void @llvm.assume(i1 %_21.i)
  %_8.i15 = getelementptr inbounds nuw i8, ptr %_20.i, i64 %count.i
  store i8 %last4.i.i, ptr %_8.i15, align 1, !noalias !855
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_8.i15, i64 1
  store i8 %last3.i.i, ptr %_23.i.i, align 1, !noalias !855
  %_24.i.i = getelementptr inbounds nuw i8, ptr %_8.i15, i64 2
  store i8 %last2.i.i, ptr %_24.i.i, align 1, !noalias !855
  %_25.i.i = getelementptr inbounds nuw i8, ptr %_8.i15, i64 3
  store i8 %last1.i.i, ptr %_25.i.i, align 1, !noalias !855
  %new_len.i = add nuw i64 %..i, %len.i13
  store i64 %new_len.i, ptr %_20.sroa.5.0.rv.sroa_idx, align 8
  %exitcond.not = icmp eq i64 %_22, %left_pad.sroa.0.0
  br i1 %exitcond.not, label %bb20, label %bb19

cleanup.loopexit:                                 ; preds = %bb1.i.i61
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

cleanup.loopexit.split-lp.loopexit.split.split.split.split: ; preds = %bb1.i.i
  %lpad.loopexit66 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

cleanup.loopexit.split-lp.loopexit.split-lp:      ; preds = %bb1.i
  %lpad.loopexit.split-lp67 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

cleanup:                                          ; preds = %cleanup.loopexit.split-lp.loopexit.split-lp, %cleanup.loopexit.split-lp.loopexit.split.split.us, %cleanup.loopexit.split-lp.loopexit.split.split.split.split, %cleanup.loopexit.split-lp.loopexit.split.split.split.us, %cleanup.loopexit.split-lp.loopexit.split.us, %cleanup.loopexit
  %_20.sroa.4.0.rv.sroa_idx179 = phi ptr [ %_20.sroa.4.0.rv.sroa_idx178, %cleanup.loopexit ], [ %_20.sroa.4.0.rv.sroa_idx178, %cleanup.loopexit.split-lp.loopexit.split-lp ], [ %_20.sroa.4.0.rv.sroa_idx, %cleanup.loopexit.split-lp.loopexit.split.us ], [ %_20.sroa.4.0.rv.sroa_idx, %cleanup.loopexit.split-lp.loopexit.split.split.us ], [ %_20.sroa.4.0.rv.sroa_idx, %cleanup.loopexit.split-lp.loopexit.split.split.split.us ], [ %_20.sroa.4.0.rv.sroa_idx, %cleanup.loopexit.split-lp.loopexit.split.split.split.split ]
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup.loopexit ], [ %lpad.loopexit.split-lp67, %cleanup.loopexit.split-lp.loopexit.split-lp ], [ %lpad.loopexit66.us, %cleanup.loopexit.split-lp.loopexit.split.us ], [ %lpad.loopexit66.us93, %cleanup.loopexit.split-lp.loopexit.split.split.us ], [ %lpad.loopexit66.us117, %cleanup.loopexit.split-lp.loopexit.split.split.split.us ], [ %lpad.loopexit66, %cleanup.loopexit.split-lp.loopexit.split.split.split.split ]
  %rv.val = load i64, ptr %rv, align 8
  %13 = icmp eq i64 %rv.val, 0
  br i1 %13, label %common.resume, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup
  %rv.val10 = load ptr, ptr %_20.sroa.4.0.rv.sroa_idx179, align 8, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rv.val10, i64 noundef %rv.val, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %common.resume

bb22:                                             ; preds = %bb1.i.bb22_crit_edge, %bb20
  %_37 = phi i64 [ %_37.pre, %bb1.i.bb22_crit_edge ], [ %len.i, %bb20 ]
  %_41 = icmp sgt i64 %_37, -1
  tail call void @llvm.assume(i1 %_41)
  %_42 = load ptr, ptr %_20.sroa.4.0.rv.sroa_idx178, align 8, !nonnull !7, !noundef !7
  %_39 = getelementptr inbounds nuw i8, ptr %_42, i64 %_37
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_39, ptr nonnull align 1 %s.0, i64 %s.1, i1 false)
  %14 = add i64 %_37, %s.1
  store i64 %14, ptr %_20.sroa.5.0.rv.sroa_idx180, align 8
  %_43141.not = icmp eq i64 %diff.sroa.0.0177, 0
  br i1 %_43141.not, label %bb24, label %bb23.lr.ph

bb23.lr.ph:                                       ; preds = %bb22
  %_16.i19 = icmp samesign ult i32 %pad, 128
  %_17.i21 = icmp samesign ult i32 %pad, 2048
  %_18.i23 = icmp samesign ult i32 %pad, 65536
  %..i24 = select i1 %_18.i23, i64 3, i64 4
  %15 = trunc i32 %pad to i8
  %_5.i.i37 = and i8 %15, 63
  %last1.i.i38 = or disjoint i8 %_5.i.i37, -128
  %_10.i.i39 = lshr i32 %pad, 6
  %16 = trunc i32 %_10.i.i39 to i8
  %_8.i.i40 = and i8 %16, 63
  %last2.i.i41 = or disjoint i8 %_8.i.i40, -128
  %_14.i.i42 = lshr i32 %pad, 12
  %17 = trunc i32 %_14.i.i42 to i8
  %_12.i.i43 = and i8 %17, 63
  %last3.i.i44 = or disjoint i8 %_12.i.i43, -128
  %_18.i.i45 = lshr i32 %pad, 18
  %_16.i.i46 = trunc nuw nsw i32 %_18.i.i45 to i8
  %last4.i.i47 = or disjoint i8 %_16.i.i46, -16
  %18 = or disjoint i8 %17, -32
  %19 = or disjoint i8 %16, -64
  %spec.select144 = select i1 %_17.i21, i64 2, i64 %..i24
  %ch_len.sroa.0.0.i26 = select i1 %_16.i19, i64 1, i64 %spec.select144
  br label %bb23

bb24:                                             ; preds = %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64, %bb22
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %rv, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %rv)
  br label %bb16

bb23:                                             ; preds = %bb23.lr.ph, %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64
  %_20.i32169 = phi ptr [ %_42, %bb23.lr.ph ], [ %_20.i32, %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64 ]
  %len.i17 = phi i64 [ %14, %bb23.lr.ph ], [ %new_len.i54, %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64 ]
  %iter1.sroa.0.0142 = phi i64 [ 0, %bb23.lr.ph ], [ %_44, %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64 ]
  %_44 = add nuw i64 %iter1.sroa.0.0142, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !864)
  %_14.i18 = icmp sgt i64 %len.i17, -1
  tail call void @llvm.assume(i1 %_14.i18)
  %self2.i.i27 = load i64, ptr %rv, align 8, !range !23, !alias.scope !867, !noundef !7
  %_9.i.i28 = sub nsw i64 %self2.i.i27, %len.i17
  %_7.i.i29 = icmp ugt i64 %ch_len.sroa.0.0.i26, %_9.i.i28
  br i1 %_7.i.i29, label %bb1.i.i61, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i30, !prof !5

bb1.i.i61:                                        ; preds = %bb23
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv, i64 noundef %len.i17, i64 noundef %ch_len.sroa.0.0.i26, i64 noundef 1, i64 noundef 1)
          to label %.noexc63 unwind label %cleanup.loopexit

.noexc63:                                         ; preds = %bb1.i.i61
  %count.pre.i62 = load i64, ptr %_20.sroa.5.0.rv.sroa_idx180, align 8, !alias.scope !864
  %_20.i32.pre = load ptr, ptr %_20.sroa.4.0.rv.sroa_idx178, align 8, !alias.scope !864
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i30

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i30: ; preds = %.noexc63, %bb23
  %_20.i32 = phi ptr [ %_20.i32169, %bb23 ], [ %_20.i32.pre, %.noexc63 ]
  %count.i31 = phi i64 [ %len.i17, %bb23 ], [ %count.pre.i62, %.noexc63 ]
  %_21.i33 = icmp sgt i64 %count.i31, -1
  tail call void @llvm.assume(i1 %_21.i33)
  %_8.i34 = getelementptr inbounds nuw i8, ptr %_20.i32, i64 %count.i31
  br i1 %_16.i19, label %bb12.i.i60, label %bb7.i.i35

bb7.i.i35:                                        ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i30
  br i1 %_17.i21, label %bb1.i2.i58, label %bb2.i.i48

bb12.i.i60:                                       ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console.exit.i30
  store i8 %15, ptr %_8.i34, align 1, !noalias !864
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64

bb1.i2.i58:                                       ; preds = %bb7.i.i35
  store i8 %19, ptr %_8.i34, align 1, !noalias !864
  %_20.i.i59 = getelementptr inbounds nuw i8, ptr %_8.i34, i64 1
  store i8 %last1.i.i38, ptr %_20.i.i59, align 1, !noalias !864
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64

bb2.i.i48:                                        ; preds = %bb7.i.i35
  %_21.i.i56 = getelementptr inbounds nuw i8, ptr %_8.i34, i64 1
  %_22.i.i57 = getelementptr inbounds nuw i8, ptr %_8.i34, i64 2
  br i1 %_18.i23, label %bb3.i.i55, label %bb4.i.i50

bb3.i.i55:                                        ; preds = %bb2.i.i48
  store i8 %18, ptr %_8.i34, align 1, !noalias !864
  store i8 %last2.i.i41, ptr %_21.i.i56, align 1, !noalias !864
  store i8 %last1.i.i38, ptr %_22.i.i57, align 1, !noalias !864
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64

bb4.i.i50:                                        ; preds = %bb2.i.i48
  store i8 %last4.i.i47, ptr %_8.i34, align 1, !noalias !864
  store i8 %last3.i.i44, ptr %_21.i.i56, align 1, !noalias !864
  store i8 %last2.i.i41, ptr %_22.i.i57, align 1, !noalias !864
  %_25.i.i53 = getelementptr inbounds nuw i8, ptr %_8.i34, i64 3
  store i8 %last1.i.i38, ptr %_25.i.i53, align 1, !noalias !864
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit64: ; preds = %bb12.i.i60, %bb1.i2.i58, %bb3.i.i55, %bb4.i.i50
  %new_len.i54 = add nuw i64 %ch_len.sroa.0.0.i26, %len.i17
  store i64 %new_len.i54, ptr %_20.sroa.5.0.rv.sroa_idx180, align 8
  %exitcond153.not = icmp eq i64 %_44, %diff.sroa.0.0177
  br i1 %exitcond153.not, label %bb24, label %bb23

bb16:                                             ; preds = %bb5, %bb4, %bb24
  ret void

bb4:                                              ; preds = %bb2
; call console::utils::truncate_str
  tail call void @_RNvNtCsC3JuwEIQwb_7console5utils12truncate_str(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i64 noundef %width, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %0, i64 noundef %1)
  br label %bb16

bb5:                                              ; preds = %bb2
  %20 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %s.0, ptr %20, align 8
  %21 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %s.1, ptr %21, align 8
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb16
}

; console::utils::truncate_str
; Function Attrs: uwtable
define void @_RNvNtCsC3JuwEIQwb_7console5utils12truncate_str(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i64 noundef %width, ptr noalias noundef nonnull readonly align 1 captures(address) %tail.0, i64 noundef %tail.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %buf = alloca [24 x i8], align 8
  %_7 = alloca [24 x i8], align 8
  %rv = alloca [24 x i8], align 8
  %iter = alloca [112 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 112, ptr nonnull %iter)
  %_61 = getelementptr inbounds nuw i8, ptr %s.0, i64 %s.1
  store ptr %s.0, ptr %iter, align 8
  %0 = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 %s.1, ptr %0, align 8
  %_51.sroa.3.0..sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 48
  store i8 2, ptr %_51.sroa.3.0..sroa_idx, align 8
  %1 = getelementptr inbounds nuw i8, ptr %iter, i64 16
  %2 = getelementptr inbounds nuw i8, ptr %iter, i64 24
  %3 = getelementptr inbounds nuw i8, ptr %iter, i64 56
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %1, i8 0, i64 16, i1 false)
  store ptr %s.0, ptr %3, align 8
  %_52.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 64
  store i64 %s.1, ptr %_52.sroa.4.0..sroa_idx, align 8
  %_52.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 72
  store ptr %s.0, ptr %_52.sroa.5.0..sroa_idx, align 8
  %_52.sroa.5.sroa.0.sroa.4.0._52.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 80
  store ptr %_61, ptr %_52.sroa.5.sroa.0.sroa.4.0._52.sroa.5.0..sroa_idx.sroa_idx, align 8
  %_52.sroa.5.sroa.0.sroa.5.0._52.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 88
  store i64 0, ptr %_52.sroa.5.sroa.0.sroa.5.0._52.sroa.5.0..sroa_idx.sroa_idx, align 8
  %_52.sroa.5.sroa.5.0._52.sroa.5.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 104
  store i32 1114113, ptr %_52.sroa.5.sroa.5.0._52.sroa.5.0..sroa_idx.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %rv)
  store i64 -9223372036854775808, ptr %rv, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_7, i64 16
  %5 = getelementptr inbounds nuw i8, ptr %_7, i64 8
  %_92.sroa.4.0.buf.sroa_idx = getelementptr inbounds nuw i8, ptr %buf, i64 8
  %_92.sroa.6.0.buf.sroa_idx = getelementptr inbounds nuw i8, ptr %buf, i64 16
  %_7.i45.not = icmp eq i64 %tail.1, 0
  %6 = getelementptr inbounds nuw i8, ptr %rv, i64 16
  %7 = getelementptr inbounds nuw i8, ptr %rv, i64 8
  br label %bb1

bb1:                                              ; preds = %bb26, %start
  %_101.sroa.10.0 = phi i64 [ undef, %start ], [ %_101.sroa.10.2, %bb26 ]
  %length.sroa.0.0 = phi i64 [ 0, %start ], [ %length.sroa.0.1, %bb26 ]
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_7)
; invoke <console::ansi::AnsiCodeIterator as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXs5_NtCsC3JuwEIQwb_7console4ansiNtB5_16AnsiCodeIteratorNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_7, ptr noalias noundef nonnull align 8 dereferenceable(112) %iter)
          to label %bb2 unwind label %bb39

bb39:                                             ; preds = %bb1, %bb6, %bb7, %bb10, %bb21, %bb1.i56
  %lpad.loopexit101 = landingpad { ptr, i32 }
          cleanup
  %.pre = load i64, ptr %rv, align 8, !range !132
  switch i64 %.pre, label %bb2.i.i.i4.i.i59 [
    i64 -9223372036854775808, label %bb30
    i64 0, label %bb30
  ]

bb2:                                              ; preds = %bb1
  %8 = load i8, ptr %4, align 8, !range !789, !noundef !7
  %.not = icmp eq i8 %8, 2
  br i1 %.not, label %bb27, label %bb3

bb3:                                              ; preds = %bb2
  %item.0 = load ptr, ptr %_7, align 8, !nonnull !7, !align !187, !noundef !7
  %item.1 = load i64, ptr %5, align 8, !noundef !7
  %item = trunc nuw i8 %8 to i1
  %9 = load i64, ptr %rv, align 8, !range !132, !noundef !7
  %.not23 = icmp eq i64 %9, -9223372036854775808
  br i1 %item, label %bb4, label %bb5

bb27:                                             ; preds = %bb2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_7)
  %10 = load i64, ptr %rv, align 8, !range !132, !noundef !7
  %.not18 = icmp eq i64 %10, -9223372036854775808
  br i1 %.not18, label %bb28, label %bb29

bb5:                                              ; preds = %bb3
  br i1 %.not23, label %bb6, label %bb26

bb4:                                              ; preds = %bb3
  br i1 %.not23, label %bb26, label %bb24

bb6:                                              ; preds = %bb5
; invoke console::utils::str_width
  %_13 = invoke fastcc noundef i64 @_RNvNtCsC3JuwEIQwb_7console5utils9str_width(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %item.0, i64 noundef %item.1)
          to label %bb7 unwind label %bb39

bb7:                                              ; preds = %bb6
; invoke console::utils::str_width
  %_16 = invoke fastcc noundef i64 @_RNvNtCsC3JuwEIQwb_7console5utils9str_width(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %tail.0, i64 noundef %tail.1)
          to label %bb8 unwind label %bb39

bb8:                                              ; preds = %bb7
  %_12 = add i64 %_13, %length.sroa.0.0
  %_15 = sub i64 %width, %_16
  %_11 = icmp ugt i64 %_12, %_15
  br i1 %_11, label %bb9, label %bb21

bb9:                                              ; preds = %bb8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !870)
  %_4.0.i = load ptr, ptr %iter, align 8, !alias.scope !870, !nonnull !7, !align !187, !noundef !7
  %_4.1.i = load i64, ptr %0, align 8, !alias.scope !870, !noundef !7
  %_3.i = load i64, ptr %2, align 8, !alias.scope !870, !noundef !7
  %11 = icmp eq i64 %_3.i, 0
  br i1 %11, label %bb10, label %bb5.i.i

bb5.i.i:                                          ; preds = %bb9
  %_8.not.i.i = icmp ult i64 %_3.i, %_4.1.i
  br i1 %_8.not.i.i, label %bb9.i.i, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb5.i.i
  %12 = icmp eq i64 %_3.i, %_4.1.i
  br i1 %12, label %bb10, label %bb3.i

bb9.i.i:                                          ; preds = %bb5.i.i
  %13 = getelementptr inbounds nuw i8, ptr %_4.0.i, i64 %_3.i
  %self1.i.i = load i8, ptr %13, align 1, !alias.scope !873, !noalias !870, !noundef !7
  %14 = icmp sgt i8 %self1.i.i, -65
  br i1 %14, label %bb10, label %bb3.i

bb3.i:                                            ; preds = %bb9.i.i, %bb6.i.i
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_4.0.i, i64 noundef %_4.1.i, i64 noundef 0, i64 noundef %_3.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a8c1ea329d6664f53eac78246d452a6c) #34
  unreachable

bb21:                                             ; preds = %bb8, %bb31
  %_101.sroa.10.1 = phi i64 [ %_101.sroa.10.3, %bb31 ], [ %_101.sroa.10.0, %bb8 ]
; invoke console::utils::str_width
  %_42 = invoke fastcc noundef i64 @_RNvNtCsC3JuwEIQwb_7console5utils9str_width(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %item.0, i64 noundef %item.1)
          to label %bb22 unwind label %bb39

bb10:                                             ; preds = %bb9.i.i, %bb6.i.i, %bb9
; invoke console::utils::str_width
  %_23 = invoke fastcc noundef i64 @_RNvNtCsC3JuwEIQwb_7console5utils9str_width(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %tail.0, i64 noundef %tail.1)
          to label %bb11 unwind label %bb39

bb11:                                             ; preds = %bb10
  %15 = add i64 %length.sroa.0.0, %_23
  %rest_width = sub i64 %width, %15
  %_72 = getelementptr inbounds nuw i8, ptr %item.0, i64 %item.1
  br label %bb12

bb12:                                             ; preds = %bb14, %bb11
  %iter1.sroa.0.0 = phi ptr [ %item.0, %bb11 ], [ %iter1.sroa.0.1.ph7082, %bb14 ]
  %s_width.sroa.0.0 = phi i64 [ 0, %bb11 ], [ %30, %bb14 ]
  %s_byte.sroa.0.0 = phi i64 [ 0, %bb11 ], [ %29, %bb14 ]
  %_6.i.i = icmp eq ptr %iter1.sroa.0.0, %_72
  br i1 %_6.i.i, label %bb17, label %bb14.i

bb14.i:                                           ; preds = %bb12
  %_16.i.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0, i64 1
  %x.i = load i8, ptr %iter1.sroa.0.0, align 1, !noalias !876, !noundef !7
  %_6.i = icmp sgt i8 %x.i, -1
  br i1 %_6.i, label %bb42.thread, label %bb4.i

bb4.i:                                            ; preds = %bb14.i
  %_30.i = and i8 %x.i, 31
  %init.i = zext nneg i8 %_30.i to i32
  %_6.i10.i = icmp ne ptr %_16.i.i, %_72
  tail call void @llvm.assume(i1 %_6.i10.i)
  %_16.i12.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0, i64 2
  %y.i = load i8, ptr %_16.i.i, align 1, !noalias !876, !noundef !7
  %_33.i = shl nuw nsw i32 %init.i, 6
  %_35.i = and i8 %y.i, 63
  %_34.i = zext nneg i8 %_35.i to i32
  %16 = or disjoint i32 %_33.i, %_34.i
  %_13.i = icmp samesign ugt i8 %x.i, -33
  br i1 %_13.i, label %bb6.i, label %bb42

bb42.thread:                                      ; preds = %bb14.i
  %_7.i = zext nneg i8 %x.i to i32
  br label %bb43

bb6.i:                                            ; preds = %bb4.i
  %_6.i17.i = icmp ne ptr %_16.i12.i, %_72
  tail call void @llvm.assume(i1 %_6.i17.i)
  %_16.i19.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0, i64 3
  %z.i = load i8, ptr %_16.i12.i, align 1, !noalias !876, !noundef !7
  %_38.i = shl nuw nsw i32 %_34.i, 6
  %_40.i = and i8 %z.i, 63
  %_39.i = zext nneg i8 %_40.i to i32
  %y_z.i = or disjoint i32 %_38.i, %_39.i
  %_20.i = shl nuw nsw i32 %init.i, 12
  %17 = or disjoint i32 %y_z.i, %_20.i
  %_21.i = icmp samesign ugt i8 %x.i, -17
  br i1 %_21.i, label %bb8.i, label %bb42

bb8.i:                                            ; preds = %bb6.i
  %_6.i24.i = icmp ne ptr %_16.i19.i, %_72
  tail call void @llvm.assume(i1 %_6.i24.i)
  %_16.i26.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0, i64 4
  %w.i = load i8, ptr %_16.i19.i, align 1, !noalias !876, !noundef !7
  %_26.i = shl nuw nsw i32 %init.i, 18
  %_25.i = and i32 %_26.i, 1835008
  %_43.i = shl nuw nsw i32 %y_z.i, 6
  %_45.i = and i8 %w.i, 63
  %_44.i = zext nneg i8 %_45.i to i32
  %_27.i = or disjoint i32 %_43.i, %_44.i
  %18 = or disjoint i32 %_27.i, %_25.i
  br label %bb42

bb42:                                             ; preds = %bb8.i, %bb6.i, %bb4.i
  %iter1.sroa.0.1.ph = phi ptr [ %_16.i12.i, %bb4.i ], [ %_16.i19.i, %bb6.i ], [ %_16.i26.i, %bb8.i ]
  %_0.sroa.4.0.i.ph = phi i32 [ %16, %bb4.i ], [ %17, %bb6.i ], [ %18, %bb8.i ]
  %19 = icmp samesign ult i32 %_0.sroa.4.0.i.ph, 1114112
  tail call void @llvm.assume(i1 %19)
  %_82 = icmp samesign ult i32 %_0.sroa.4.0.i.ph, 128
  br i1 %_82, label %bb43, label %bb44

bb17:                                             ; preds = %bb12, %bb14, %bb50
  %s_byte.sroa.0.1 = phi i64 [ %31, %bb50 ], [ %s_byte.sroa.0.0, %bb12 ], [ %29, %bb14 ]
  %_35 = sub i64 %_3.i, %item.1
  %idx = add i64 %s_byte.sroa.0.1, %_35
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf)
  %20 = icmp eq i64 %idx, 0
  br i1 %20, label %bb62, label %bb5.i

bb5.i:                                            ; preds = %bb17
  %_8.not.i = icmp ult i64 %idx, %_3.i
  br i1 %_8.not.i, label %bb9.i, label %bb6.i33

bb6.i33:                                          ; preds = %bb5.i
  %21 = icmp eq i64 %idx, %_3.i
  br i1 %21, label %bb6.i.i43, label %bb58

bb9.i:                                            ; preds = %bb5.i
  %22 = getelementptr inbounds nuw i8, ptr %_4.0.i, i64 %idx
  %self1.i = load i8, ptr %22, align 1, !alias.scope !879, !noundef !7
  %23 = icmp sgt i8 %self1.i, -65
  br i1 %23, label %bb6.i.i43, label %bb58

bb44:                                             ; preds = %bb42
  %_83 = icmp samesign ult i32 %_0.sroa.4.0.i.ph, 2048
  br i1 %_83, label %bb4.i37, label %bb4.i37.thread88

bb4.i37.thread88:                                 ; preds = %bb44
  %_84 = icmp samesign ult i32 %_0.sroa.4.0.i.ph, 65536
  %. = select i1 %_84, i64 3, i64 4
  %24 = add i64 %., %s_byte.sroa.0.0
  br label %bb5.i40

bb43:                                             ; preds = %bb42.thread, %bb42
  %_0.sroa.4.0.i.ph71 = phi i32 [ %_0.sroa.4.0.i.ph, %bb42 ], [ %_7.i, %bb42.thread ]
  %iter1.sroa.0.1.ph70 = phi ptr [ %iter1.sroa.0.1.ph, %bb42 ], [ %_16.i.i, %bb42.thread ]
  %25 = add i64 %s_byte.sroa.0.0, 1
  %_3.i36.not = icmp eq i32 %_0.sroa.4.0.i.ph71, 127
  br i1 %_3.i36.not, label %bb14, label %bb1.i

bb4.i37:                                          ; preds = %bb44
  %26 = add i64 %s_byte.sroa.0.0, 2
  %_5.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph, 159
  br i1 %_5.i, label %bb5.i40, label %bb14

bb1.i:                                            ; preds = %bb43
  %_4.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph71, 31
  %spec.select.i = zext i1 %_4.i to i64
  br label %bb14

bb5.i40:                                          ; preds = %bb4.i37.thread88, %bb4.i37
  %27 = phi i64 [ %24, %bb4.i37.thread88 ], [ %26, %bb4.i37 ]
; call unicode_width::tables::lookup_width
  %28 = tail call fastcc { i8, i16 } @_RNvNtCsDJOD2kcAir_13unicode_width6tables12lookup_width(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph) #35
  %_8.0.i = extractvalue { i8, i16 } %28, 0
  %_6.i41 = zext nneg i8 %_8.0.i to i64
  br label %bb14

bb14:                                             ; preds = %bb43, %bb5.i40, %bb1.i, %bb4.i37
  %29 = phi i64 [ %26, %bb4.i37 ], [ %27, %bb5.i40 ], [ %25, %bb1.i ], [ %25, %bb43 ]
  %iter1.sroa.0.1.ph7082 = phi ptr [ %iter1.sroa.0.1.ph, %bb4.i37 ], [ %iter1.sroa.0.1.ph, %bb5.i40 ], [ %iter1.sroa.0.1.ph70, %bb1.i ], [ %iter1.sroa.0.1.ph70, %bb43 ]
  %_0.sroa.4.0.i.ph7180 = phi i32 [ %_0.sroa.4.0.i.ph, %bb4.i37 ], [ %_0.sroa.4.0.i.ph, %bb5.i40 ], [ %_0.sroa.4.0.i.ph71, %bb1.i ], [ 127, %bb43 ]
  %_827278 = phi i1 [ false, %bb4.i37 ], [ false, %bb5.i40 ], [ true, %bb1.i ], [ true, %bb43 ]
  %_0.sroa.0.0.i39 = phi i64 [ 0, %bb4.i37 ], [ %_6.i41, %bb5.i40 ], [ %spec.select.i, %bb1.i ], [ 0, %bb43 ]
  %30 = add i64 %_0.sroa.0.0.i39, %s_width.sroa.0.0
  %_31 = tail call i8 @llvm.ucmp.i8.i64(i64 %30, i64 %rest_width)
  switch i8 %_31, label %bb13 [
    i8 -1, label %bb12
    i8 0, label %bb17
    i8 1, label %bb15
  ]

bb13:                                             ; preds = %bb14
  unreachable

bb15:                                             ; preds = %bb14
  br i1 %_827278, label %bb50, label %bb51

bb51:                                             ; preds = %bb15
  %_86 = icmp samesign ult i32 %_0.sroa.4.0.i.ph7180, 2048
  br i1 %_86, label %bb50, label %bb52

bb52:                                             ; preds = %bb51
  %_87 = icmp samesign ult i32 %_0.sroa.4.0.i.ph7180, 65536
  %.27 = select i1 %_87, i64 -3, i64 -4
  br label %bb50

bb50:                                             ; preds = %bb51, %bb52, %bb15
  %_33.sroa.0.0.neg = phi i64 [ -1, %bb15 ], [ %.27, %bb52 ], [ -2, %bb51 ]
  %31 = add i64 %_33.sroa.0.0.neg, %29
  br label %bb17

bb6.i.i43:                                        ; preds = %bb9.i, %bb6.i33
  %or.cond.not = icmp sgt i64 %idx, 0
  br i1 %or.cond.not, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i, label %bb61, !prof !321

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i: ; preds = %bb6.i.i43
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !882
; call __rustc::__rust_alloc
  %32 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %idx, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !882
  %33 = icmp eq ptr %32, null
  br i1 %33, label %bb61, label %bb10.i

bb10.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i
  %34 = ptrtoint ptr %32 to i64
  br label %bb62

bb58:                                             ; preds = %bb9.i, %bb6.i33
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_4.0.i, i64 noundef %_3.i, i64 noundef 0, i64 noundef %idx, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7ba7d7c5f08be500d53e5ec9430a471e) #30
  unreachable

bb61:                                             ; preds = %bb6.i.i43, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i
  %_101.sroa.4.0.ph = phi i64 [ 1, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i ], [ 0, %bb6.i.i43 ]
  %_101.sroa.10.3.ph = phi i64 [ %idx, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i ], [ %_101.sroa.10.0, %bb6.i.i43 ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_101.sroa.4.0.ph, i64 %_101.sroa.10.3.ph) #30
  unreachable

bb62:                                             ; preds = %bb17, %bb10.i
  %_101.sroa.10.3 = phi i64 [ %34, %bb10.i ], [ 1, %bb17 ]
  %35 = inttoptr i64 %_101.sroa.10.3 to ptr
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %35, ptr nonnull align 1 %_4.0.i, i64 %idx, i1 false)
  store i64 %idx, ptr %buf, align 8
  store ptr %35, ptr %_92.sroa.4.0.buf.sroa_idx, align 8
  store i64 %idx, ptr %_92.sroa.6.0.buf.sroa_idx, align 8
  br i1 %_7.i45.not, label %bb31, label %bb1.i47, !prof !30

bb1.i47:                                          ; preds = %bb62
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf, i64 noundef %idx, i64 noundef %tail.1, i64 noundef 1, i64 noundef 1)
          to label %bb1.i47.bb31_crit_edge unwind label %bb35

bb1.i47.bb31_crit_edge:                           ; preds = %bb1.i47
  %_123.pre = load i64, ptr %_92.sroa.6.0.buf.sroa_idx, align 8
  %_128.pre = load ptr, ptr %_92.sroa.4.0.buf.sroa_idx, align 8
  br label %bb31

bb31:                                             ; preds = %bb1.i47.bb31_crit_edge, %bb62
  %_128 = phi ptr [ %_128.pre, %bb1.i47.bb31_crit_edge ], [ %35, %bb62 ]
  %_123 = phi i64 [ %_123.pre, %bb1.i47.bb31_crit_edge ], [ %idx, %bb62 ]
  %_127 = icmp sgt i64 %_123, -1
  tail call void @llvm.assume(i1 %_127)
  %_125 = getelementptr inbounds nuw i8, ptr %_128, i64 %_123
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_125, ptr nonnull align 1 %tail.0, i64 %tail.1, i1 false)
  %36 = add i64 %_123, %tail.1
  store i64 %36, ptr %_92.sroa.6.0.buf.sroa_idx, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %rv, ptr noundef nonnull align 8 dereferenceable(24) %buf, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf)
  br label %bb21

bb22:                                             ; preds = %bb21
  %37 = add i64 %_42, %length.sroa.0.0
  br label %bb26

bb26:                                             ; preds = %bb4, %bb66, %bb5, %bb22
  %_101.sroa.10.2 = phi i64 [ %_101.sroa.10.0, %bb4 ], [ %_101.sroa.10.0, %bb66 ], [ %_101.sroa.10.1, %bb22 ], [ %_101.sroa.10.0, %bb5 ]
  %length.sroa.0.1 = phi i64 [ %length.sroa.0.0, %bb4 ], [ %length.sroa.0.0, %bb66 ], [ %37, %bb22 ], [ %length.sroa.0.0, %bb5 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_7)
  br label %bb1

bb35:                                             ; preds = %bb1.i47
  %38 = landingpad { ptr, i32 }
          cleanup
  %buf.val = load i64, ptr %buf, align 8
  %39 = icmp eq i64 %buf.val, 0
  br i1 %39, label %bb30, label %bb2.i.i.i4.i.i49

bb2.i.i.i4.i.i49:                                 ; preds = %bb35
  %buf.val29 = load ptr, ptr %_92.sroa.4.0.buf.sroa_idx, align 8, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %buf.val29, i64 noundef %buf.val, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb30

bb24:                                             ; preds = %bb4
  %len.i51 = load i64, ptr %6, align 8, !alias.scope !885, !noundef !7
  %_9.i53 = sub i64 %9, %len.i51
  %_7.i54 = icmp ugt i64 %item.1, %_9.i53
  br i1 %_7.i54, label %bb1.i56, label %bb66, !prof !5

bb1.i56:                                          ; preds = %bb24
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %rv, i64 noundef %len.i51, i64 noundef %item.1, i64 noundef 1, i64 noundef 1)
          to label %bb1.i56.bb66_crit_edge unwind label %bb39

bb1.i56.bb66_crit_edge:                           ; preds = %bb1.i56
  %_142.pre = load i64, ptr %6, align 8
  br label %bb66

bb66:                                             ; preds = %bb1.i56.bb66_crit_edge, %bb24
  %_142 = phi i64 [ %_142.pre, %bb1.i56.bb66_crit_edge ], [ %len.i51, %bb24 ]
  %_146 = icmp sgt i64 %_142, -1
  tail call void @llvm.assume(i1 %_146)
  %_147 = load ptr, ptr %7, align 8, !nonnull !7, !noundef !7
  %_144 = getelementptr inbounds nuw i8, ptr %_147, i64 %_142
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_144, ptr nonnull align 1 %item.0, i64 %item.1, i1 false)
  %40 = add i64 %_142, %item.1
  store i64 %40, ptr %6, align 8
  br label %bb26

bb29:                                             ; preds = %bb27
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %rv, i64 24, i1 false)
  br label %bb34

bb28:                                             ; preds = %bb27
  %41 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %s.0, ptr %41, align 8
  %42 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %s.1, ptr %42, align 8
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb34

bb34:                                             ; preds = %bb28, %bb29
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %rv)
  call void @llvm.lifetime.end.p0(i64 112, ptr nonnull %iter)
  ret void

bb30:                                             ; preds = %bb2.i.i.i4.i.i49, %bb35, %bb39, %bb39, %bb2.i.i.i4.i.i59
  %.pn24148 = phi { ptr, i32 } [ %lpad.loopexit101, %bb39 ], [ %lpad.loopexit101, %bb39 ], [ %lpad.loopexit101, %bb2.i.i.i4.i.i59 ], [ %38, %bb2.i.i.i4.i.i49 ], [ %38, %bb35 ]
  resume { ptr, i32 } %.pn24148

bb2.i.i.i4.i.i59:                                 ; preds = %bb39
  %rv.val28 = load ptr, ptr %7, align 8, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rv.val28, i64 noundef %.pre, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb30
}

; console::utils::measure_text_width
; Function Attrs: uwtable
define noundef i64 @_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_3 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3)
; call console::ansi::strip_ansi_codes
  call void @_RNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1)
  %0 = load i64, ptr %_3, align 8, !range !132, !noundef !7
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_9 = load ptr, ptr %1, align 8
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %_8 = load i64, ptr %2, align 8
; invoke console::utils::str_width
  %_0 = invoke fastcc noundef i64 @_RNvNtCsC3JuwEIQwb_7console5utils9str_width(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_9, i64 noundef %_8)
          to label %bb2 unwind label %cleanup

cleanup:                                          ; preds = %start
  %3 = landingpad { ptr, i32 }
          cleanup
  switch i64 %0, label %bb2.i.i.i4.i.i.i [
    i64 -9223372036854775808, label %bb5
    i64 0, label %bb5
  ]

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_9, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb5

bb2:                                              ; preds = %start
  switch i64 %0, label %bb2.i.i.i4.i.i.i4 [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc6borrow3CoweEECsC3JuwEIQwb_7console.exit5
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc6borrow3CoweEECsC3JuwEIQwb_7console.exit5
  ]

bb2.i.i.i4.i.i.i4:                                ; preds = %bb2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_9, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc6borrow3CoweEECsC3JuwEIQwb_7console.exit5

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc6borrow3CoweEECsC3JuwEIQwb_7console.exit5: ; preds = %bb2, %bb2, %bb2.i.i.i4.i.i.i4
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  ret i64 %_0

bb5:                                              ; preds = %bb2.i.i.i4.i.i.i, %cleanup, %cleanup
  resume { ptr, i32 } %3
}

; console::utils::default_colors_enabled
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvNtCsC3JuwEIQwb_7console5utils22default_colors_enabled(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %out) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_7 = alloca [32 x i8], align 8
  %_5 = alloca [32 x i8], align 8
; call console::unix_term::is_a_color_terminal
  %_2 = tail call noundef zeroext i1 @_RNvNtCsC3JuwEIQwb_7console9unix_term19is_a_color_terminal(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out)
  br i1 %_2, label %bb1, label %bb7

bb7:                                              ; preds = %bb2.i.i.i4.i.i13, %bb5, %start
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_7)
; call std::env::_var
  call void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_7, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_787344c135fb1ca228327e24d16f41dc, i64 noundef 14)
  %_23 = load i64, ptr %_7, align 8, !range !6, !noundef !7
  %0 = trunc nuw i64 %_23 to i1
  %1 = getelementptr inbounds nuw i8, ptr %_7, i64 8
  %_24.sroa.0.0.copyload = load i64, ptr %1, align 8
  %_24.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_7, i64 16
  %_24.sroa.4.0.copyload = load ptr, ptr %_24.sroa.4.0..sroa_idx, align 8
  br i1 %0, label %bb22, label %bb21

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5)
; call std::env::_var
  call void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_5c8c5330792b76d76bc036d6b9372d18, i64 noundef 8)
  %_10 = load i64, ptr %_5, align 8, !range !6, !noundef !7
  %2 = trunc nuw i64 %_10 to i1
  %3 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %_11.sroa.0.0.copyload = load i64, ptr %3, align 8
  %_11.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 16
  %_11.sroa.4.0.copyload = load ptr, ptr %_11.sroa.4.0..sroa_idx, align 8
  br i1 %2, label %bb17, label %bb15

bb17:                                             ; preds = %bb1
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !888
; call __rustc::__rust_alloc
  %4 = tail call noundef dereferenceable_or_null(1) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 1, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !888
  %5 = icmp eq ptr %4, null
  br i1 %5, label %bb6.i, label %bb7.i

cleanup.i:                                        ; preds = %bb6.i
  %6 = landingpad { ptr, i32 }
          cleanup
  switch i64 %_11.sroa.0.0.copyload, label %bb2.i.i.i4.i.i.i.i.i [
    i64 -9223372036854775808, label %common.resume
    i64 0, label %common.resume
  ]

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup.i
  %7 = icmp ne ptr %_11.sroa.4.0.copyload, null
  tail call void @llvm.assume(i1 %7)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_11.sroa.4.0.copyload, i64 noundef %_11.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !893
  br label %common.resume

bb6.i:                                            ; preds = %bb17
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 1) #30
          to label %unreachable.i unwind label %cleanup.i, !noalias !893

bb7.i:                                            ; preds = %bb17
  store i8 49, ptr %4, align 1, !noalias !893
  switch i64 %_11.sroa.0.0.copyload, label %bb20.thread48 [
    i64 -9223372036854775808, label %bb20.thread48.thread
    i64 0, label %bb20.thread48.thread
  ]

unreachable.i:                                    ; preds = %bb6.i
  unreachable

common.resume:                                    ; preds = %cleanup.i22, %cleanup.i22, %bb2.i.i.i4.i.i.i.i.i24, %cleanup.i, %cleanup.i, %bb2.i.i.i4.i.i.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %6, %bb2.i.i.i4.i.i.i.i.i ], [ %6, %cleanup.i ], [ %6, %cleanup.i ], [ %16, %bb2.i.i.i4.i.i.i.i.i24 ], [ %16, %cleanup.i22 ], [ %16, %cleanup.i22 ]
  resume { ptr, i32 } %common.resume.op

bb15:                                             ; preds = %bb1
  %_4.sroa.13.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 24
  %_4.sroa.13.0.copyload = load i64, ptr %_4.sroa.13.0..sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5)
  %_3.not.i = icmp eq i64 %_4.sroa.13.0.copyload, 1
  br i1 %_3.not.i, label %bb20, label %bb3

bb20:                                             ; preds = %bb15
  %lhsc74 = load i8, ptr %_11.sroa.4.0.copyload, align 1
  %8 = icmp eq i8 %lhsc74, 48
  br i1 %8, label %bb5, label %bb3

bb20.thread48.thread:                             ; preds = %bb7.i, %bb7.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5)
  br label %bb2.i.i.i4.i.i15

bb20.thread48:                                    ; preds = %bb7.i
  %9 = icmp ne ptr %_11.sroa.4.0.copyload, null
  tail call void @llvm.assume(i1 %9)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_11.sroa.4.0.copyload, i64 noundef %_11.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !893
  %lhsc.pre = load i8, ptr %4, align 1
  %10 = icmp eq i8 %lhsc.pre, 48
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5)
  br i1 %10, label %bb2.i.i.i4.i.i13, label %bb2.i.i.i4.i.i15

bb5:                                              ; preds = %bb20
  %11 = icmp eq i64 %_11.sroa.0.0.copyload, 0
  br i1 %11, label %bb7, label %bb2.i.i.i4.i.i13

bb2.i.i.i4.i.i13:                                 ; preds = %bb20.thread48, %bb5
  %_4.sroa.0.0425257 = phi i64 [ %_11.sroa.0.0.copyload, %bb5 ], [ 1, %bb20.thread48 ]
  %_4.sroa.8.0395356 = phi ptr [ %_11.sroa.4.0.copyload, %bb5 ], [ %4, %bb20.thread48 ]
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.8.0395356, i64 noundef %_4.sroa.0.0425257, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb7

bb3:                                              ; preds = %bb15, %bb20
  %12 = icmp eq i64 %_11.sroa.0.0.copyload, 0
  br i1 %12, label %bb10, label %bb2.i.i.i4.i.i15

bb2.i.i.i4.i.i15:                                 ; preds = %bb20.thread48.thread, %bb20.thread48, %bb3
  %_4.sroa.0.0414661 = phi i64 [ %_11.sroa.0.0.copyload, %bb3 ], [ 1, %bb20.thread48 ], [ 1, %bb20.thread48.thread ]
  %_4.sroa.8.0404760 = phi ptr [ %_11.sroa.4.0.copyload, %bb3 ], [ %4, %bb20.thread48 ], [ %4, %bb20.thread48.thread ]
  %13 = icmp ne ptr %_4.sroa.8.0404760, null
  tail call void @llvm.assume(i1 %13)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.8.0404760, i64 noundef %_4.sroa.0.0414661, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb10

bb22:                                             ; preds = %bb7
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !894
; call __rustc::__rust_alloc
  %14 = tail call noundef dereferenceable_or_null(1) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 1, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !894
  %15 = icmp eq ptr %14, null
  br i1 %15, label %bb6.i21, label %bb7.i17

cleanup.i22:                                      ; preds = %bb6.i21
  %16 = landingpad { ptr, i32 }
          cleanup
  switch i64 %_24.sroa.0.0.copyload, label %bb2.i.i.i4.i.i.i.i.i24 [
    i64 -9223372036854775808, label %common.resume
    i64 0, label %common.resume
  ]

bb2.i.i.i4.i.i.i.i.i24:                           ; preds = %cleanup.i22
  %17 = icmp ne ptr %_24.sroa.4.0.copyload, null
  tail call void @llvm.assume(i1 %17)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_24.sroa.4.0.copyload, i64 noundef %_24.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !899
  br label %common.resume

bb6.i21:                                          ; preds = %bb22
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 1) #30
          to label %unreachable.i25 unwind label %cleanup.i22, !noalias !899

bb7.i17:                                          ; preds = %bb22
  store i8 48, ptr %14, align 1, !noalias !899
  switch i64 %_24.sroa.0.0.copyload, label %bb2.i.i.i4.i.i.i.i4.i20 [
    i64 -9223372036854775808, label %bb25.thread
    i64 0, label %bb25.thread
  ]

bb2.i.i.i4.i.i.i.i4.i20:                          ; preds = %bb7.i17
  %18 = icmp ne ptr %_24.sroa.4.0.copyload, null
  tail call void @llvm.assume(i1 %18)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_24.sroa.4.0.copyload, i64 noundef %_24.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !899
  br label %bb25.thread

unreachable.i25:                                  ; preds = %bb6.i21
  unreachable

bb25.thread:                                      ; preds = %bb2.i.i.i4.i.i.i.i4.i20, %bb7.i17, %bb7.i17
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_7)
  br label %bb2.i.i.i4.i.i32

bb21:                                             ; preds = %bb7
  %_6.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_7, i64 24
  %_6.sroa.10.0.copyload = load i64, ptr %_6.sroa.10.0..sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_7)
  %_3.not.i26 = icmp eq i64 %_6.sroa.10.0.copyload, 1
  br i1 %_3.not.i26, label %bb21.bb2.i28_crit_edge, label %bb25

bb21.bb2.i28_crit_edge:                           ; preds = %bb21
  %lhsc71.pre = load i8, ptr %_24.sroa.4.0.copyload, align 1
  %19 = icmp ne i8 %lhsc71.pre, 48
  br label %bb25

bb25:                                             ; preds = %bb21.bb2.i28_crit_edge, %bb21
  %_0.sroa.0.0.off0.i27 = phi i1 [ true, %bb21 ], [ %19, %bb21.bb2.i28_crit_edge ]
  %20 = icmp eq i64 %_24.sroa.0.0.copyload, 0
  br i1 %20, label %bb10, label %bb2.i.i.i4.i.i32

bb2.i.i.i4.i.i32:                                 ; preds = %bb25.thread, %bb25
  %_0.sroa.0.0.off0.i2781 = phi i1 [ false, %bb25.thread ], [ %_0.sroa.0.0.off0.i27, %bb25 ]
  %_6.sroa.6.06880 = phi ptr [ %14, %bb25.thread ], [ %_24.sroa.4.0.copyload, %bb25 ]
  %_6.sroa.0.06979 = phi i64 [ 1, %bb25.thread ], [ %_24.sroa.0.0.copyload, %bb25 ]
  %21 = icmp ne ptr %_6.sroa.6.06880, null
  tail call void @llvm.assume(i1 %21)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.sroa.6.06880, i64 noundef %_6.sroa.0.06979, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb10

bb10:                                             ; preds = %bb2.i.i.i4.i.i32, %bb25, %bb2.i.i.i4.i.i15, %bb3
  %_0.sroa.0.0.off0 = phi i1 [ true, %bb3 ], [ true, %bb2.i.i.i4.i.i15 ], [ %_0.sroa.0.0.off0.i27, %bb25 ], [ %_0.sroa.0.0.off0.i2781, %bb2.i.i.i4.i.i32 ]
  ret i1 %_0.sroa.0.0.off0
}

; console::utils::pad_str
; Function Attrs: uwtable
define void @_RNvNtCsC3JuwEIQwb_7console5utils7pad_str(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i64 noundef %width, i8 noundef range(i8 0, 3) %align, ptr noalias noundef readonly align 1 captures(address) %truncate.0, i64 %truncate.1) unnamed_addr #1 {
start:
; call console::utils::pad_str_with
  tail call void @_RNvNtCsC3JuwEIQwb_7console5utils12pad_str_with(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i64 noundef %width, i8 noundef %align, ptr noalias noundef readonly align 1 captures(address, read_provenance) %truncate.0, i64 %truncate.1, i32 noundef 32)
  ret void
}

; console::utils::str_width
; Function Attrs: uwtable
define internal fastcc noundef i64 @_RNvNtCsC3JuwEIQwb_7console5utils9str_width(ptr noalias noundef nonnull readonly align 1 captures(address) %s.0, i64 noundef %s.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %.not.i23.i = icmp samesign eq i64 %s.1, 0
  br i1 %.not.i23.i, label %_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator5rfoldTjNtNtCsDJOD2kcAir_13unicode_width6tables9WidthInfoENCNvB1N_9str_width0ECsC3JuwEIQwb_7console.exit, label %bb17.i.i.i.preheader

bb17.i.i.i.preheader:                             ; preds = %start
  %_9 = getelementptr inbounds nuw i8, ptr %s.0, i64 %s.1
  br label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %bb17.i.i.i.preheader, %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
  %accum.sroa.0.026.i = phi i64 [ %_9.i.i, %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i ], [ 0, %bb17.i.i.i.preheader ]
  %accum.sroa.6.025.i = phi i16 [ %_0.sroa.51.0.i.i.i, %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i ], [ 0, %bb17.i.i.i.preheader ]
  %self.sroa.2.024.i = phi ptr [ %self.sroa.2.320.i, %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i ], [ %_9, %bb17.i.i.i.preheader ]
  %_23.i.i.i.i = getelementptr inbounds i8, ptr %self.sroa.2.024.i, i64 -1
  %w.i.i.i = load i8, ptr %_23.i.i.i.i, align 1, !noalias !900, !noundef !7
  %_6.i.i.i = icmp sgt i8 %w.i.i.i, -1
  br i1 %_6.i.i.i, label %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.thread16.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb17.i.i.i
  %0 = icmp ne ptr %s.0, %_23.i.i.i.i
  tail call void @llvm.assume(i1 %0)
  %_23.i13.i.i.i = getelementptr inbounds i8, ptr %self.sroa.2.024.i, i64 -2
  %z.i.i.i = load i8, ptr %_23.i13.i.i.i, align 1, !noalias !900, !noundef !7
  %_25.i.i.i = and i8 %z.i.i.i, 31
  %1 = zext nneg i8 %_25.i.i.i to i32
  %_12.i.i.i = icmp slt i8 %z.i.i.i, -64
  br i1 %_12.i.i.i, label %bb6.i.i.i, label %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.i

_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.thread16.i: ; preds = %bb17.i.i.i
  %_8.i.i.i = zext nneg i8 %w.i.i.i to i32
  br label %bb3.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %2 = icmp ne ptr %s.0, %_23.i13.i.i.i
  tail call void @llvm.assume(i1 %2)
  %_23.i19.i.i.i = getelementptr inbounds i8, ptr %self.sroa.2.024.i, i64 -3
  %y.i.i.i = load i8, ptr %_23.i19.i.i.i, align 1, !noalias !900, !noundef !7
  %_29.i.i.i = and i8 %y.i.i.i, 15
  %3 = zext nneg i8 %_29.i.i.i to i32
  %_16.i.i.i = icmp slt i8 %y.i.i.i, -64
  br i1 %_16.i.i.i, label %bb8.i.i.i, label %bb11.i.i.i

bb8.i.i.i:                                        ; preds = %bb6.i.i.i
  %4 = icmp ne ptr %s.0, %_23.i19.i.i.i
  tail call void @llvm.assume(i1 %4)
  %_23.i25.i.i.i = getelementptr inbounds i8, ptr %self.sroa.2.024.i, i64 -4
  %x.i.i.i = load i8, ptr %_23.i25.i.i.i, align 1, !noalias !900, !noundef !7
  %_33.i.i.i = and i8 %x.i.i.i, 7
  %5 = zext nneg i8 %_33.i.i.i to i32
  %_34.i.i.i = shl nuw nsw i32 %5, 6
  %_36.i.i.i = and i8 %y.i.i.i, 63
  %_35.i.i.i = zext nneg i8 %_36.i.i.i to i32
  %6 = or disjoint i32 %_34.i.i.i, %_35.i.i.i
  br label %bb11.i.i.i

bb11.i.i.i:                                       ; preds = %bb8.i.i.i, %bb6.i.i.i
  %self.sroa.2.2.i = phi ptr [ %_23.i25.i.i.i, %bb8.i.i.i ], [ %_23.i19.i.i.i, %bb6.i.i.i ]
  %ch.sroa.0.1.i.i.i = phi i32 [ %6, %bb8.i.i.i ], [ %3, %bb6.i.i.i ]
  %_37.i.i.i = shl nuw nsw i32 %ch.sroa.0.1.i.i.i, 6
  %_39.i.i.i = and i8 %z.i.i.i, 63
  %_38.i.i.i = zext nneg i8 %_39.i.i.i to i32
  %7 = or disjoint i32 %_37.i.i.i, %_38.i.i.i
  br label %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.i

_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.i: ; preds = %bb11.i.i.i, %bb4.i.i.i
  %self.sroa.2.1.i = phi ptr [ %self.sroa.2.2.i, %bb11.i.i.i ], [ %_23.i13.i.i.i, %bb4.i.i.i ]
  %ch.sroa.0.0.i.i.i = phi i32 [ %7, %bb11.i.i.i ], [ %1, %bb4.i.i.i ]
  %_40.i.i.i = shl nuw nsw i32 %ch.sroa.0.0.i.i.i, 6
  %_42.i.i.i = and i8 %w.i.i.i, 63
  %_41.i.i.i = zext nneg i8 %_42.i.i.i to i32
  %8 = or disjoint i32 %_40.i.i.i, %_41.i.i.i
  %.not.i = icmp eq i32 %8, 1114112
  br i1 %.not.i, label %_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator5rfoldTjNtNtCsDJOD2kcAir_13unicode_width6tables9WidthInfoENCNvB1N_9str_width0ECsC3JuwEIQwb_7console.exit, label %bb3.i

bb3.i:                                            ; preds = %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.i, %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.thread16.i
  %spec.select.i21.i = phi i32 [ %_8.i.i.i, %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.thread16.i ], [ %8, %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.i ]
  %self.sroa.2.320.i = phi ptr [ %_23.i.i.i.i, %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.thread16.i ], [ %self.sroa.2.1.i, %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.i ]
  %.not.i.i.i = icmp sgt i16 %accum.sroa.6.025.i, -1
  br i1 %.not.i.i.i, label %bb9.i.i.i, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb3.i
  %top_bits.i.i.i.i = lshr i32 %spec.select.i21.i, 10
  switch i32 %top_bits.i.i.i.i, label %bb7.i.i.i [
    i32 0, label %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i
    i32 8, label %bb7.i.i.i.i
    i32 9, label %bb6.i.i.i.i
    i32 10, label %bb5.i.i.i.i
    i32 12, label %bb4.i.i.i.i
    i32 124, label %bb3.i.i.i.i
    i32 125, label %bb2.i.i.i.i
  ]

bb7.i.i.i.i:                                      ; preds = %bb1.i.i.i
  br label %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb1.i.i.i
  br label %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb1.i.i.i
  br label %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb1.i.i.i
  br label %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb1.i.i.i
  br label %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb1.i.i.i
  br label %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i

_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i: ; preds = %bb2.i.i.i.i, %bb3.i.i.i.i, %bb4.i.i.i.i, %bb5.i.i.i.i, %bb6.i.i.i.i, %bb7.i.i.i.i, %bb1.i.i.i
  %idx_of_leaf.sroa.0.0.i.i.i.i = phi i64 [ 1, %bb7.i.i.i.i ], [ 2, %bb6.i.i.i.i ], [ 3, %bb5.i.i.i.i ], [ 4, %bb4.i.i.i.i ], [ 5, %bb3.i.i.i.i ], [ 6, %bb2.i.i.i.i ], [ 0, %bb1.i.i.i ]
  %_8.i.i.i.i = lshr i32 %spec.select.i21.i, 3
  %_7.i.i.i.i = and i32 %_8.i.i.i.i, 127
  %_16.i.i.i.i = zext nneg i32 %_7.i.i.i.i to i64
  %9 = getelementptr inbounds nuw [128 x i8], ptr @_RNvNtCsDJOD2kcAir_13unicode_width6tables25EMOJI_PRESENTATION_LEAVES, i64 %idx_of_leaf.sroa.0.0.i.i.i.i
  %10 = getelementptr inbounds nuw i8, ptr %9, i64 %_16.i.i.i.i
  %leaf_byte.i.i.i.i = load i8, ptr %10, align 1, !noundef !7
  %11 = trunc i32 %spec.select.i21.i to i8
  %12 = and i8 %11, 7
  %13 = lshr i8 %leaf_byte.i.i.i.i, %12
  %extract.t.i.i.i.i = trunc i8 %13 to i1
  br i1 %extract.t.i.i.i.i, label %bb3.i.i8.i, label %bb7.i.i.i

bb7.i.i.i:                                        ; preds = %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i, %bb1.i.i.i
  %_55.i.i.i = and i16 %accum.sroa.6.025.i, 8192
  %_54.not.i.i.i = icmp eq i16 %_55.i.i.i, 0
  br i1 %_54.not.i.i.i, label %bb159.i.i.i, label %bb158.i.i.i

bb3.i.i8.i:                                       ; preds = %_RNvNtCsDJOD2kcAir_13unicode_width6tables29starts_emoji_presentation_seq.exit.i.i.i
  %_53.i.i.i = and i16 %accum.sroa.6.025.i, -20480
  %14 = icmp eq i16 %_53.i.i.i, -28672
  %..i.i.i = select i1 %14, i8 0, i8 2
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb159.i.i.i:                                      ; preds = %bb7.i.i.i
  %15 = icmp samesign ult i32 %spec.select.i21.i, 161
  br i1 %15, label %bb198.i.i.i, label %bb151.i.i.i

bb158.i.i.i:                                      ; preds = %bb7.i.i.i
  %_56.i.i.i = and i16 %accum.sroa.6.025.i, 32767
  br label %bb9.i.i.i

bb198.i.i.i:                                      ; preds = %bb159.i.i.i
  %cond15.i.i.i = icmp eq i32 %spec.select.i21.i, 10
  br i1 %cond15.i.i.i, label %bb13.i.i7.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb151.i.i.i:                                      ; preds = %bb109.i.i.i, %bb110.i.i.i, %bb116.i.i.i, %bb106.i.i.i, %bb104.i.i.i, %bb173.i.i.i, %bb177.i.i.i, %bb78.i.i.i, %bb180.i.i.i, %bb79.i.i.i, %bb69.i.i.i, %bb53.i.i.i, %bb53.i.i.i, %bb53.i.i.i, %bb53.i.i.i, %bb51.i.i.i, %bb50.i.i.i, %bb49.i.i.i, %bb48.i.i.i, %bb16.i.i.i, %bb159.i.i.i
  %spec.select.i15.i = phi i32 [ %spec.select.i21.i, %bb110.i.i.i ], [ %spec.select.i21.i, %bb109.i.i.i ], [ %spec.select.i21.i, %bb116.i.i.i ], [ 127988, %bb106.i.i.i ], [ %spec.select.i21.i, %bb104.i.i.i ], [ %spec.select.i21.i, %bb173.i.i.i ], [ %spec.select.i21.i, %bb177.i.i.i ], [ %spec.select.i21.i, %bb78.i.i.i ], [ %spec.select.i21.i, %bb180.i.i.i ], [ %spec.select.i21.i, %bb79.i.i.i ], [ 8419, %bb69.i.i.i ], [ 11647, %bb53.i.i.i ], [ 11647, %bb53.i.i.i ], [ 11647, %bb53.i.i.i ], [ %spec.select.i21.i, %bb51.i.i.i ], [ %spec.select.i21.i, %bb50.i.i.i ], [ %spec.select.i21.i, %bb49.i.i.i ], [ %spec.select.i21.i, %bb48.i.i.i ], [ %spec.select.i21.i, %bb16.i.i.i ], [ %spec.select.i21.i, %bb159.i.i.i ], [ 11647, %bb53.i.i.i ]
; call unicode_width::tables::lookup_width
  %16 = tail call fastcc { i8, i16 } @_RNvNtCsDJOD2kcAir_13unicode_width6tables12lookup_width(i32 noundef range(i32 0, 1114112) %spec.select.i15.i) #35
  %ret.0.i.i.i = extractvalue { i8, i16 } %16, 0
  %ret.1.i.i.i = extractvalue { i8, i16 } %16, 1
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb13.i.i7.i:                                      ; preds = %bb10.i.i.i, %bb198.i.i.i
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb9.i.i.i:                                        ; preds = %bb158.i.i.i, %bb3.i
  %next_info.sroa.0.0.i.i.i = phi i16 [ %_56.i.i.i, %bb158.i.i.i ], [ %accum.sroa.6.025.i, %bb3.i ]
  %17 = icmp samesign ult i32 %spec.select.i21.i, 161
  br i1 %17, label %bb10.i.i.i, label %bb16.i.i.i

bb16.i.i.i:                                       ; preds = %bb9.i.i.i
  %_58.not.i.i.i = icmp eq i16 %next_info.sroa.0.0.i.i.i, 0
  br i1 %_58.not.i.i.i, label %bb151.i.i.i, label %bb17.i.i5.i

bb10.i.i.i:                                       ; preds = %bb9.i.i.i
  %trunc.i.i.i = trunc nuw i32 %spec.select.i21.i to i8
  switch i8 %trunc.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i [
    i8 10, label %bb13.i.i7.i
    i8 13, label %bb12.i.i.i
  ]

bb17.i.i5.i:                                      ; preds = %bb16.i.i.i
  switch i32 %spec.select.i21.i, label %bb23.i.i.i [
    i32 65039, label %bb18.i.i.i
    i32 65025, label %bb20.i.i.i
    i32 65038, label %bb22.i.i.i
  ]

bb18.i.i.i:                                       ; preds = %bb17.i.i5.i
  %18 = and i16 %next_info.sroa.0.0.i.i.i, 12288
  %or.cond17.not.i.i.i = icmp eq i16 %18, 0
  %_63.i.i.i = or disjoint i16 %next_info.sroa.0.0.i.i.i, -32768
  %_8.sroa.0.0.i.i.i = select i1 %or.cond17.not.i.i.i, i16 -32768, i16 %_63.i.i.i
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb20.i.i.i:                                       ; preds = %bb17.i.i5.i
  %_65.i.i.i = and i16 %next_info.sroa.0.0.i.i.i, 8192
  %_64.not.i.i.i = icmp eq i16 %_65.i.i.i, 0
  %_66.i.i.i = or i16 %next_info.sroa.0.0.i.i.i, 512
  %_9.sroa.0.0.i.i.i = select i1 %_64.not.i.i.i, i16 512, i16 %_66.i.i.i
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb22.i.i.i:                                       ; preds = %bb17.i.i5.i
  %_68.i.i.i = and i16 %next_info.sroa.0.0.i.i.i, 8192
  %_67.not.i.i.i = icmp eq i16 %_68.i.i.i, 0
  %_69.i.i.i = or i16 %next_info.sroa.0.0.i.i.i, 16384
  %_10.sroa.0.0.i.i.i = select i1 %_67.not.i.i.i, i16 16384, i16 %_69.i.i.i
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb23.i.i.i:                                       ; preds = %bb17.i.i5.i
  %.not55.i.i.i = icmp samesign ult i16 %next_info.sroa.0.0.i.i.i, 16384
  br i1 %.not55.i.i.i, label %bb28.i.i.i, label %bb24.i.i.i

bb24.i.i.i:                                       ; preds = %bb23.i.i.i
  %top_bits.i93.i.i.i = lshr i32 %spec.select.i21.i, 8
  switch i32 %top_bits.i93.i.i.i, label %bb27.i.i.i [
    i32 35, label %bb12.i.i.i.i
    i32 37, label %bb12.thread.i.i.i.i
    i32 38, label %bb9.i101.i.i.i
    i32 39, label %bb8.i.i.i.i
    i32 43, label %bb7.i100.i.i.i
    i32 496, label %bb6.i99.i.i.i
    i32 499, label %bb5.i98.i.i.i
    i32 500, label %bb4.i97.i.i.i
    i32 501, label %bb3.i96.i.i.i
    i32 502, label %bb2.i94.i.i.i
  ]

bb9.i101.i.i.i:                                   ; preds = %bb24.i.i.i
  br label %bb12.i.i.i.i

bb8.i.i.i.i:                                      ; preds = %bb24.i.i.i
  br label %bb12.i.i.i.i

bb7.i100.i.i.i:                                   ; preds = %bb24.i.i.i
  br label %bb12.i.i.i.i

bb6.i99.i.i.i:                                    ; preds = %bb24.i.i.i
  br label %bb12.thread.i.i.i.i

bb5.i98.i.i.i:                                    ; preds = %bb24.i.i.i
  br label %bb12.i.i.i.i

bb4.i97.i.i.i:                                    ; preds = %bb24.i.i.i
  br label %bb12.i.i.i.i

bb3.i96.i.i.i:                                    ; preds = %bb24.i.i.i
  br label %bb12.i.i.i.i

bb2.i94.i.i.i:                                    ; preds = %bb24.i.i.i
  br label %bb12.i.i.i.i

bb12.thread.i.i.i.i:                              ; preds = %bb6.i99.i.i.i, %bb24.i.i.i
  %leaf.sroa.0.0.ph.i.i.i.i = phi ptr [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_5, %bb6.i99.i.i.i ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_1, %bb24.i.i.i ]
  %19 = trunc i32 %spec.select.i21.i to i8
  br label %bb7.i.i.i.i.i

bb12.i.i.i.i:                                     ; preds = %bb2.i94.i.i.i, %bb3.i96.i.i.i, %bb4.i97.i.i.i, %bb5.i98.i.i.i, %bb7.i100.i.i.i, %bb8.i.i.i.i, %bb9.i101.i.i.i, %bb24.i.i.i
  %leaf.sroa.11.0.i.i.i.i = phi i64 [ 15, %bb9.i101.i.i.i ], [ 10, %bb8.i.i.i.i ], [ 3, %bb7.i100.i.i.i ], [ 13, %bb5.i98.i.i.i ], [ 22, %bb4.i97.i.i.i ], [ 4, %bb3.i96.i.i.i ], [ 10, %bb2.i94.i.i.i ], [ 4, %bb24.i.i.i ]
  %leaf.sroa.0.0.i.i.i.i = phi ptr [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_2, %bb9.i101.i.i.i ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_3, %bb8.i.i.i.i ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_4, %bb7.i100.i.i.i ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_6, %bb5.i98.i.i.i ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_7, %bb4.i97.i.i.i ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_8, %bb3.i96.i.i.i ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_9, %bb2.i94.i.i.i ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables24TEXT_PRESENTATION_LEAF_0, %bb24.i.i.i ]
  %20 = trunc i32 %spec.select.i21.i to i8
  br label %bb4.i.i.i.i.i

bb7.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i, %bb12.thread.i.i.i.i
  %21 = phi i8 [ %19, %bb12.thread.i.i.i.i ], [ %20, %bb4.i.i.i.i.i ]
  %leaf.sroa.0.05.i.i.i.i = phi ptr [ %leaf.sroa.0.0.ph.i.i.i.i, %bb12.thread.i.i.i.i ], [ %leaf.sroa.0.0.i.i.i.i, %bb4.i.i.i.i.i ]
  %leaf.sroa.11.04.i.i.i.i = phi i64 [ 1, %bb12.thread.i.i.i.i ], [ %leaf.sroa.11.0.i.i.i.i, %bb4.i.i.i.i.i ]
  %base.sroa.0.0.lcssa.i.i.i.i.i = phi i64 [ 0, %bb12.thread.i.i.i.i ], [ %24, %bb4.i.i.i.i.i ]
  %_41.i.i.i.i.i = icmp ult i64 %base.sroa.0.0.lcssa.i.i.i.i.i, %leaf.sroa.11.04.i.i.i.i
  tail call void @llvm.assume(i1 %_41.i.i.i.i.i)
  %_38.i.i.i.i.i = getelementptr inbounds nuw { i8, i8 }, ptr %leaf.sroa.0.05.i.i.i.i, i64 %base.sroa.0.0.lcssa.i.i.i.i.i
  %_38.val.i.i.i.i.i = load i8, ptr %_38.i.i.i.i.i, align 1, !alias.scope !905, !noalias !908, !noundef !7
  %22 = getelementptr i8, ptr %_38.i.i.i.i.i, i64 1
  %_38.val12.i.i.i.i.i = load i8, ptr %22, align 1, !alias.scope !905, !noalias !908
  %_5.i.i.i.i.i.i = icmp uge i8 %21, %_38.val.i.i.i.i.i
  %_7.i.i.i.i.i.i = icmp ugt i8 %21, %_38.val12.i.i.i.i.i
  %not._5.i.i.i.i.i.i = xor i1 %_5.i.i.i.i.i.i, true
  %23 = select i1 %not._5.i.i.i.i.i.i, i1 true, i1 %_7.i.i.i.i.i.i
  br i1 %23, label %_RNvNtCsDJOD2kcAir_13unicode_width6tables44starts_non_ideographic_text_presentation_seq.exit.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb4.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i, %bb12.i.i.i.i
  %size.sroa.0.022.i.i.i.i.i = phi i64 [ %25, %bb4.i.i.i.i.i ], [ %leaf.sroa.11.0.i.i.i.i, %bb12.i.i.i.i ]
  %base.sroa.0.021.i.i.i.i.i = phi i64 [ %24, %bb4.i.i.i.i.i ], [ 0, %bb12.i.i.i.i ]
  %half11.i.i.i.i.i = lshr i64 %size.sroa.0.022.i.i.i.i.i, 1
  %mid.i.i.i.i.i = add i64 %half11.i.i.i.i.i, %base.sroa.0.021.i.i.i.i.i
  %_35.i.i.i.i.i = icmp ult i64 %mid.i.i.i.i.i, %leaf.sroa.11.0.i.i.i.i
  tail call void @llvm.assume(i1 %_35.i.i.i.i.i)
  %_32.i.i.i.i.i = getelementptr inbounds nuw { i8, i8 }, ptr %leaf.sroa.0.0.i.i.i.i, i64 %mid.i.i.i.i.i
  %_32.val.i.i.i.i.i = load i8, ptr %_32.i.i.i.i.i, align 1, !alias.scope !905, !noalias !908, !noundef !7
  %_5.i16.i.i.i.i.i = icmp ugt i8 %_32.val.i.i.i.i.i, %20
  %24 = select i1 %_5.i16.i.i.i.i.i, i64 %base.sroa.0.021.i.i.i.i.i, i64 %mid.i.i.i.i.i, !unpredictable !7
  %25 = sub i64 %size.sroa.0.022.i.i.i.i.i, %half11.i.i.i.i.i
  %_6.i.i.i.i.i = icmp ugt i64 %25, 1
  br i1 %_6.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb7.i.i.i.i.i

_RNvNtCsDJOD2kcAir_13unicode_width6tables44starts_non_ideographic_text_presentation_seq.exit.i.i.i: ; preds = %bb7.i.i.i.i.i
  %_29.i.i.i.i.i = select i1 %_5.i.i.i.i.i.i, i1 %_7.i.i.i.i.i.i, i1 false
  %_28.i.i.i.i.i = zext i1 %_29.i.i.i.i.i to i64
  %result.i.i.i.i.i = add nuw nsw i64 %base.sroa.0.0.lcssa.i.i.i.i.i, %_28.i.i.i.i.i
  %cond1.i.i.i.i.i = icmp ule i64 %result.i.i.i.i.i, %leaf.sroa.11.04.i.i.i.i
  tail call void @llvm.assume(i1 %cond1.i.i.i.i.i)
  br label %bb27.i.i.i

bb28.i.i.i:                                       ; preds = %bb23.i.i.i
  %_72.i.i.i = and i16 %next_info.sroa.0.0.i.i.i, 512
  %.not56.i.i.i = icmp eq i16 %_72.i.i.i, 0
  br i1 %.not56.i.i.i, label %bb34.i.i.i, label %bb29.i.i.i

bb27.i.i.i:                                       ; preds = %_RNvNtCsDJOD2kcAir_13unicode_width6tables44starts_non_ideographic_text_presentation_seq.exit.i.i.i, %bb24.i.i.i
  %_71.i.i.i = and i16 %next_info.sroa.0.0.i.i.i, 16383
  br label %bb34.i.i.i

bb34.i.i.i:                                       ; preds = %bb30.i.i.i, %bb27.i.i.i, %bb28.i.i.i
  %next_info.sroa.0.1.i.i.i = phi i16 [ %_71.i.i.i, %bb27.i.i.i ], [ %_73.i.i.i, %bb30.i.i.i ], [ %next_info.sroa.0.0.i.i.i, %bb28.i.i.i ]
  %_74.i.i.i = and i16 %next_info.sroa.0.1.i.i.i, 2048
  %.not57.i.i.i = icmp eq i16 %_74.i.i.i, 0
  br i1 %.not57.i.i.i, label %bb42.i.i.i, label %bb35.i.i.i

bb29.i.i.i:                                       ; preds = %bb28.i.i.i
  %switch.tableidx = add i32 %spec.select.i21.i, -8216
  %26 = icmp ult i32 %switch.tableidx, 6
  %switch.maskindex = trunc i32 %switch.tableidx to i8
  %switch.shifted = lshr i8 51, %switch.maskindex
  %switch.lobit = trunc i8 %switch.shifted to i1
  %or.cond = select i1 %26, i1 %switch.lobit, i1 false
  br i1 %or.cond, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb30.i.i.i

bb30.i.i.i:                                       ; preds = %bb29.i.i.i
  %_73.i.i.i = and i16 %next_info.sroa.0.0.i.i.i, 15871
  br label %bb34.i.i.i

bb35.i.i.i:                                       ; preds = %bb34.i.i.i
  switch i32 %spec.select.i21.i, label %bb2.i.i [
    i32 8205, label %bb36.i.i.i
    i32 847, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i32 6159, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
  ]

bb36.i.i.i:                                       ; preds = %bb35.i.i.i
  %_75.i.i.i = or i16 %next_info.sroa.0.1.i.i.i, 1024
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb2.i.i:                                          ; preds = %bb35.i.i.i
  %27 = insertelement <2 x i32> poison, i32 %spec.select.i21.i, i64 0
  %28 = shufflevector <2 x i32> %27, <2 x i32> poison, <2 x i32> zeroinitializer
  %29 = and <2 x i32> %28, <i32 2097150, i32 2097136>
  %30 = add nsw <2 x i32> %28, <i32 -6155, i32 -917760>
  %31 = shufflevector <2 x i32> %30, <2 x i32> poison, <4 x i32> <i32 0, i32 1, i32 poison, i32 poison>
  %32 = shufflevector <4 x i32> %31, <4 x i32> <i32 poison, i32 poison, i32 6068, i32 65024>, <4 x i32> <i32 0, i32 1, i32 6, i32 7>
  %33 = shufflevector <2 x i32> %29, <2 x i32> poison, <4 x i32> <i32 0, i32 1, i32 poison, i32 poison>
  %34 = shufflevector <4 x i32> <i32 3, i32 240, i32 poison, i32 poison>, <4 x i32> %33, <4 x i32> <i32 0, i32 1, i32 4, i32 5>
  %35 = icmp ult <4 x i32> %32, %34
  %36 = icmp eq <4 x i32> %32, %34
  %37 = shufflevector <4 x i1> %35, <4 x i1> %36, <4 x i32> <i32 0, i32 1, i32 6, i32 7>
  %38 = freeze <4 x i1> %37
  %39 = bitcast <4 x i1> %38 to i4
  %.not = icmp eq i4 %39, 0
  br i1 %.not, label %bb42.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb42.i.i.i:                                       ; preds = %bb2.i.i, %bb34.i.i.i
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb43.i.i.i [
    i16 12543, label %bb44.i.i.i
    i16 15360, label %bb48.i.i.i
    i16 15367, label %bb49.i.i.i
    i16 15361, label %bb50.i.i.i
    i16 15362, label %bb51.i.i.i
  ]

bb43.i.i.i:                                       ; preds = %bb45.i.i.i, %bb42.i.i.i
  %40 = icmp eq i32 %spec.select.i21.i, 11647
  br i1 %40, label %bb53.i.i.i, label %bb52.i.i.i

bb44.i.i.i:                                       ; preds = %bb42.i.i.i
  switch i32 %spec.select.i21.i, label %bb46.i.i.i [
    i32 1604, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i32 1898, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i32 2214, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i32 2247, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
  ]

bb48.i.i.i:                                       ; preds = %bb42.i.i.i
  switch i32 %spec.select.i21.i, label %bb54.i.i.i [
    i32 1488, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i32 11647, label %bb151.i.i.i
  ]

bb49.i.i.i:                                       ; preds = %bb42.i.i.i
  switch i32 %spec.select.i21.i, label %bb54.i.i.i [
    i32 6098, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i32 11647, label %bb151.i.i.i
  ]

bb50.i.i.i:                                       ; preds = %bb42.i.i.i
  switch i32 %spec.select.i21.i, label %bb54.i.i.i [
    i32 6679, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i32 11647, label %bb151.i.i.i
  ]

bb51.i.i.i:                                       ; preds = %bb42.i.i.i
  switch i32 %spec.select.i21.i, label %bb54.i.i.i [
    i32 6677, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i32 11647, label %bb151.i.i.i
  ]

bb46.i.i.i:                                       ; preds = %bb44.i.i.i
  %41 = add nsw i32 %spec.select.i21.i, -1717
  %or.cond.i.i.i = icmp ult i32 %41, 4
  br i1 %or.cond.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb45.i.i.i

bb45.i.i.i:                                       ; preds = %bb46.i.i.i
; call unicode_width::tables::is_transparent_zero_width
  %_44.i.i.i = tail call noundef zeroext i1 @_RNvNtCsDJOD2kcAir_13unicode_width6tables25is_transparent_zero_width(i32 noundef range(i32 0, 1114112) %spec.select.i21.i)
  br i1 %_44.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb43.i.i.i

bb53.i.i.i:                                       ; preds = %bb43.i.i.i
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb65.i.i.i [
    i16 14339, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 15363, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 2, label %bb151.i.i.i
    i16 15364, label %bb151.i.i.i
    i16 15365, label %bb151.i.i.i
    i16 15366, label %bb151.i.i.i
  ]

bb52.i.i.i:                                       ; preds = %bb43.i.i.i
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb54.i.i.i [
    i16 15363, label %bb55.i.i.i
    i16 15364, label %bb58.i.i.i
    i16 15365, label %bb61.i.i.i
    i16 15366, label %bb64.i.i.i
    i16 2, label %bb129.i.i.i
  ]

bb54.i.i.i:                                       ; preds = %bb52.i.i.i, %bb51.i.i.i, %bb50.i.i.i, %bb49.i.i.i, %bb48.i.i.i
  %42 = icmp samesign ugt i32 %spec.select.i21.i, 127461
  %43 = add nsw i32 %spec.select.i21.i, -127462
  %or.cond85.i.i.i = icmp ult i32 %43, 26
  br i1 %or.cond85.i.i.i, label %bb66.i.i.i, label %bb67.i.i.i

bb61.i.i.i:                                       ; preds = %bb52.i.i.i
  %44 = and i32 %spec.select.i21.i, 2097148
  %or.cond5.i.i.i = icmp eq i32 %44, 42232
  br i1 %or.cond5.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb190.i.i.i

bb190.i.i.i:                                      ; preds = %bb60.i.i.i, %bb57.i.i.i, %bb64.i.i.i, %bb61.i.i.i
  %45 = icmp samesign ugt i32 %spec.select.i21.i, 127461
  %46 = add nsw i32 %spec.select.i21.i, -127462
  %or.cond83.i.i.i = icmp ult i32 %46, 26
  br i1 %or.cond83.i.i.i, label %bb66.i.i.i, label %bb191.i.i.i

bb129.i.i.i:                                      ; preds = %bb52.i.i.i
; call unicode_width::tables::is_emoji_modifier_base
  %_45.i.i.i = tail call fastcc noundef zeroext i1 @_RNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base(i32 noundef range(i32 0, 1114112) %spec.select.i21.i) #35
  br i1 %_45.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb144.i.i.i

bb55.i.i.i:                                       ; preds = %bb52.i.i.i
  %47 = icmp samesign ugt i32 %spec.select.i21.i, 11568
  br i1 %47, label %bb57.i.i.i, label %bb191.i.i.i

bb58.i.i.i:                                       ; preds = %bb52.i.i.i
  %48 = icmp samesign ugt i32 %spec.select.i21.i, 11568
  br i1 %48, label %bb60.i.i.i, label %bb191.i.i.i

bb64.i.i.i:                                       ; preds = %bb52.i.i.i
  %49 = icmp eq i32 %spec.select.i21.i, 68658
  br i1 %49, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb190.i.i.i

bb57.i.i.i:                                       ; preds = %bb55.i.i.i
  %50 = icmp samesign ult i32 %spec.select.i21.i, 11622
  %51 = icmp eq i32 %spec.select.i21.i, 11631
  %or.cond2.i.i.i = or i1 %50, %51
  br i1 %or.cond2.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb190.i.i.i

bb60.i.i.i:                                       ; preds = %bb58.i.i.i
  %52 = icmp samesign ult i32 %spec.select.i21.i, 11622
  %53 = icmp eq i32 %spec.select.i21.i, 11631
  %or.cond4.i.i.i = or i1 %52, %53
  br i1 %or.cond4.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb190.i.i.i

bb191.i.i.i:                                      ; preds = %bb58.i.i.i, %bb55.i.i.i, %bb190.i.i.i
  %54 = phi i1 [ %45, %bb190.i.i.i ], [ false, %bb58.i.i.i ], [ false, %bb55.i.i.i ]
  %cond1.i.i.i = icmp eq i32 %spec.select.i21.i, 8205
  br i1 %cond1.i.i.i, label %bb68.i.i.i, label %bb71.i.i.i

bb66.i.i.i:                                       ; preds = %bb190.i.i.i, %bb54.i.i.i
  %_22.sroa.0.0.off0.i.i.i = phi i1 [ %45, %bb190.i.i.i ], [ %42, %bb54.i.i.i ]
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb71.i.i.i [
    i16 3, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 4, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 4102, label %bb75.i.i.i
  ]

bb65.i.i.i:                                       ; preds = %bb67.i.i.i, %bb53.i.i.i
  %_22.sroa.0.1.off0.i.i.i = phi i1 [ %42, %bb67.i.i.i ], [ false, %bb53.i.i.i ]
  %cond110.i.i.i = icmp eq i16 %next_info.sroa.0.1.i.i.i, 4102
  br i1 %cond110.i.i.i, label %bb75.i.i.i, label %bb71.i.i.i

bb68.i.i.i:                                       ; preds = %bb67.i.i.i, %bb191.i.i.i
  %_22.sroa.0.2.off0.i.i.i = phi i1 [ %42, %bb67.i.i.i ], [ %54, %bb191.i.i.i ]
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb71.i.i.i [
    i16 5, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 4, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 10, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 11, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 2, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 4102, label %bb75.i.i.i
  ]

bb144.i.i.i:                                      ; preds = %bb129.i.i.i
  %55 = icmp samesign ugt i32 %spec.select.i21.i, 127461
  %cond.i.i.i = icmp eq i32 %spec.select.i21.i, 8205
  br i1 %cond.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb71.i.i.i

bb67.i.i.i:                                       ; preds = %bb54.i.i.i
  switch i32 %spec.select.i21.i, label %bb65.i.i.i [
    i32 8205, label %bb68.i.i.i
    i32 8419, label %bb69.i.i.i
  ]

bb69.i.i.i:                                       ; preds = %bb67.i.i.i
  %cond.i.i = icmp eq i16 %next_info.sroa.0.1.i.i.i, 4102
  br i1 %cond.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb151.i.i.i

bb71.i.i.i:                                       ; preds = %bb144.i.i.i, %bb68.i.i.i, %bb65.i.i.i, %bb66.i.i.i, %bb191.i.i.i
  %_22.sroa.0.3.off0.i.i.i = phi i1 [ %_22.sroa.0.0.off0.i.i.i, %bb66.i.i.i ], [ %_22.sroa.0.1.off0.i.i.i, %bb65.i.i.i ], [ %_22.sroa.0.2.off0.i.i.i, %bb68.i.i.i ], [ %55, %bb144.i.i.i ], [ %54, %bb191.i.i.i ]
  %_43.i.i.i = icmp samesign ult i32 %spec.select.i21.i, 127488
  %or.cond7.i.i.i = and i1 %_43.i.i.i, %_22.sroa.0.3.off0.i.i.i
  br i1 %or.cond7.i.i.i, label %bb79.i.i.i, label %bb80.i.i.i

bb75.i.i.i:                                       ; preds = %bb68.i.i.i, %bb65.i.i.i, %bb66.i.i.i
  %_22.sroa.0.6.off0.i.i.i = phi i1 [ %_22.sroa.0.0.off0.i.i.i, %bb66.i.i.i ], [ %_22.sroa.0.1.off0.i.i.i, %bb65.i.i.i ], [ %_22.sroa.0.2.off0.i.i.i, %bb68.i.i.i ]
  %_26.i.i.i = icmp samesign ult i32 %spec.select.i21.i, 127488
  %or.cond18.i.i.i = and i1 %_26.i.i.i, %_22.sroa.0.6.off0.i.i.i
  br i1 %or.cond18.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb82.i.i.i

bb80.i.i.i:                                       ; preds = %bb71.i.i.i
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb81.i.i.i [
    i16 4102, label %bb82.i.i.i
    i16 16, label %bb86.i.i.i
    i16 25, label %bb89.i.i.i
    i16 26, label %bb92.i.i.i
    i16 27, label %bb95.i.i.i
    i16 28, label %bb98.i.i.i
    i16 29, label %bb101.i.i.i
  ]

bb79.i.i.i:                                       ; preds = %bb71.i.i.i
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb151.i.i.i [
    i16 9, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 11, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 10, label %bb122.i.i.i
    i16 4102, label %bb116.i.i.i
    i16 32, label %bb109.i.i.i
    i16 33, label %bb110.i.i.i
  ]

bb81.i.i.i:                                       ; preds = %bb84.i.i.i, %bb80.i.i.i
  %56 = add nsw i32 %spec.select.i21.i, -917552
  %or.cond92.i.i.i = icmp ult i32 %56, 10
  br i1 %or.cond92.i.i.i, label %bb104.i.i.i, label %bb105.i.i.i

bb82.i.i.i:                                       ; preds = %bb80.i.i.i, %bb75.i.i.i
  %57 = add nsw i32 %spec.select.i21.i, -127995
  %or.cond8.i.i.i = icmp ult i32 %57, 5
  br i1 %or.cond8.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb84.i.i.i

bb86.i.i.i:                                       ; preds = %bb80.i.i.i
  %58 = add nsw i32 %spec.select.i21.i, -917601
  %or.cond9.i.i.i = icmp ult i32 %58, 26
  br i1 %or.cond9.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb186.i.i.i

bb89.i.i.i:                                       ; preds = %bb80.i.i.i
  %59 = add nsw i32 %spec.select.i21.i, -917601
  %or.cond10.i.i.i = icmp ult i32 %59, 26
  br i1 %or.cond10.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb184.i.i.i

bb92.i.i.i:                                       ; preds = %bb80.i.i.i
  %60 = add nsw i32 %spec.select.i21.i, -917601
  %or.cond11.i.i.i = icmp ult i32 %60, 26
  br i1 %or.cond11.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb182.i.i.i

bb95.i.i.i:                                       ; preds = %bb80.i.i.i
  %61 = add nsw i32 %spec.select.i21.i, -917601
  %or.cond12.i.i.i = icmp ult i32 %61, 26
  br i1 %or.cond12.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb179.i.i.i

bb98.i.i.i:                                       ; preds = %bb80.i.i.i
  %62 = add nsw i32 %spec.select.i21.i, -917601
  %or.cond13.i.i.i = icmp ult i32 %62, 26
  br i1 %or.cond13.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb176.i.i.i

bb101.i.i.i:                                      ; preds = %bb80.i.i.i
  %63 = add nsw i32 %spec.select.i21.i, -917601
  %or.cond14.i.i.i = icmp ult i32 %63, 26
  br i1 %or.cond14.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb173.i.i.i

bb84.i.i.i:                                       ; preds = %bb82.i.i.i
  %64 = icmp eq i32 %spec.select.i21.i, 917631
  br i1 %64, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb81.i.i.i

bb186.i.i.i:                                      ; preds = %bb86.i.i.i
  %65 = add nsw i32 %spec.select.i21.i, -917552
  %or.cond86.i.i.i = icmp ult i32 %65, 10
  br i1 %or.cond86.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb105.i.i.i

bb105.i.i.i:                                      ; preds = %bb182.i.i.i, %bb184.i.i.i, %bb186.i.i.i, %bb81.i.i.i
  %66 = icmp eq i32 %spec.select.i21.i, 127988
  br i1 %66, label %bb106.i.i.i, label %bb78.i.i.i

bb184.i.i.i:                                      ; preds = %bb89.i.i.i
  %67 = add nsw i32 %spec.select.i21.i, -917552
  %or.cond87.i.i.i = icmp ult i32 %67, 10
  br i1 %or.cond87.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb105.i.i.i

bb182.i.i.i:                                      ; preds = %bb92.i.i.i
  %68 = add nsw i32 %spec.select.i21.i, -917552
  %or.cond88.i.i.i = icmp ult i32 %68, 10
  br i1 %or.cond88.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb105.i.i.i

bb179.i.i.i:                                      ; preds = %bb95.i.i.i
  %69 = add nsw i32 %spec.select.i21.i, -917552
  %or.cond89.i.i.i = icmp ult i32 %69, 10
  br i1 %or.cond89.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb180.i.i.i

bb180.i.i.i:                                      ; preds = %bb179.i.i.i
  %70 = icmp eq i32 %spec.select.i21.i, 127988
  br i1 %70, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb151.i.i.i

bb78.i.i.i:                                       ; preds = %bb105.i.i.i
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb151.i.i.i [
    i16 4102, label %bb116.i.i.i
    i16 32, label %bb109.i.i.i
    i16 33, label %bb110.i.i.i
  ]

bb176.i.i.i:                                      ; preds = %bb98.i.i.i
  %71 = add nsw i32 %spec.select.i21.i, -917552
  %or.cond90.i.i.i = icmp ult i32 %71, 10
  br i1 %or.cond90.i.i.i, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb177.i.i.i

bb177.i.i.i:                                      ; preds = %bb176.i.i.i
  %72 = icmp eq i32 %spec.select.i21.i, 127988
  br i1 %72, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb151.i.i.i

bb173.i.i.i:                                      ; preds = %bb101.i.i.i
  %73 = icmp eq i32 %spec.select.i21.i, 127988
  br i1 %73, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb151.i.i.i

bb104.i.i.i:                                      ; preds = %bb81.i.i.i
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb151.i.i.i [
    i16 16, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 25, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 26, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 27, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 28, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 17, label %bb119.i.i.i
    i16 18, label %bb118.i.i.i
    i16 4102, label %bb116.i.i.i
    i16 32, label %bb109.i.i.i
  ]

bb106.i.i.i:                                      ; preds = %bb105.i.i.i
  switch i16 %next_info.sroa.0.1.i.i.i, label %bb151.i.i.i [
    i16 27, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 28, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 29, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 30, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 19, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i
    i16 4102, label %bb116.i.i.i
  ]

bb116.i.i.i:                                      ; preds = %bb106.i.i.i, %bb104.i.i.i, %bb78.i.i.i, %bb79.i.i.i
; call unicode_width::tables::lookup_width
  %74 = tail call fastcc { i8, i16 } @_RNvNtCsDJOD2kcAir_13unicode_width6tables12lookup_width(i32 noundef range(i32 0, 1114112) %spec.select.i21.i) #35
  %_47.1.i.i.i = extractvalue { i8, i16 } %74, 1
  %75 = icmp eq i16 %_47.1.i.i.i, 5
  br i1 %75, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb151.i.i.i

bb109.i.i.i:                                      ; preds = %bb104.i.i.i, %bb78.i.i.i, %bb79.i.i.i
  %switch.tableidx2 = add i32 %spec.select.i21.i, -93539
  %76 = icmp ult i32 %switch.tableidx2, 7
  %switch.maskindex5 = trunc i32 %switch.tableidx2 to i8
  %switch.shifted6 = lshr i8 113, %switch.maskindex5
  %switch.lobit7 = trunc i8 %switch.shifted6 to i1
  %or.cond8 = select i1 %76, i1 %switch.lobit7, i1 false
  br i1 %or.cond8, label %switch.lookup4, label %bb151.i.i.i

bb119.i.i.i:                                      ; preds = %bb104.i.i.i
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb118.i.i.i:                                      ; preds = %bb104.i.i.i
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb122.i.i.i:                                      ; preds = %bb79.i.i.i
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

bb110.i.i.i:                                      ; preds = %bb78.i.i.i, %bb79.i.i.i
  %77 = icmp eq i32 %spec.select.i21.i, 93539
  br i1 %77, label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, label %bb151.i.i.i

bb12.i.i.i:                                       ; preds = %bb10.i.i.i
  %_6.i.i6.i = icmp ne i16 %next_info.sroa.0.0.i.i.i, 1
  %spec.select.i.i.i = zext i1 %_6.i.i6.i to i8
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

switch.lookup4:                                   ; preds = %bb109.i.i.i
  %78 = zext nneg i32 %switch.tableidx2 to i64
  %switch.gep = getelementptr inbounds nuw [7 x i16], ptr @switch.table._RNvNtCsC3JuwEIQwb_7console5utils9str_width, i64 0, i64 %78
  %switch.load = load i16, ptr %switch.gep, align 2
  %79 = shl nuw nsw i32 %switch.tableidx2, 3
  %switch.shiftamt = zext nneg i32 %79 to i56
  %switch.downshift = lshr i56 1099511627776, %switch.shiftamt
  %switch.masked = trunc i56 %switch.downshift to i8
  br label %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i

_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i: ; preds = %bb29.i.i.i, %switch.lookup4, %bb12.i.i.i, %bb110.i.i.i, %bb122.i.i.i, %bb118.i.i.i, %bb119.i.i.i, %bb116.i.i.i, %bb106.i.i.i, %bb106.i.i.i, %bb106.i.i.i, %bb106.i.i.i, %bb106.i.i.i, %bb104.i.i.i, %bb104.i.i.i, %bb104.i.i.i, %bb104.i.i.i, %bb104.i.i.i, %bb173.i.i.i, %bb177.i.i.i, %bb176.i.i.i, %bb180.i.i.i, %bb179.i.i.i, %bb182.i.i.i, %bb184.i.i.i, %bb186.i.i.i, %bb84.i.i.i, %bb101.i.i.i, %bb98.i.i.i, %bb95.i.i.i, %bb92.i.i.i, %bb89.i.i.i, %bb86.i.i.i, %bb82.i.i.i, %bb79.i.i.i, %bb79.i.i.i, %bb75.i.i.i, %bb69.i.i.i, %bb144.i.i.i, %bb68.i.i.i, %bb68.i.i.i, %bb68.i.i.i, %bb68.i.i.i, %bb68.i.i.i, %bb66.i.i.i, %bb66.i.i.i, %bb60.i.i.i, %bb57.i.i.i, %bb64.i.i.i, %bb129.i.i.i, %bb61.i.i.i, %bb53.i.i.i, %bb53.i.i.i, %bb45.i.i.i, %bb46.i.i.i, %bb51.i.i.i, %bb50.i.i.i, %bb49.i.i.i, %bb48.i.i.i, %bb44.i.i.i, %bb44.i.i.i, %bb44.i.i.i, %bb44.i.i.i, %bb2.i.i, %bb36.i.i.i, %bb35.i.i.i, %bb35.i.i.i, %bb7.i.i.i.i.i, %bb22.i.i.i, %bb20.i.i.i, %bb18.i.i.i, %bb10.i.i.i, %bb13.i.i7.i, %bb151.i.i.i, %bb198.i.i.i, %bb3.i.i8.i
  %_0.sroa.51.0.i.i.i = phi i16 [ 5, %bb3.i.i8.i ], [ %_75.i.i.i, %bb36.i.i.i ], [ 11, %bb122.i.i.i ], [ 18, %bb119.i.i.i ], [ 19, %bb118.i.i.i ], [ %_8.sroa.0.0.i.i.i, %bb18.i.i.i ], [ %_9.sroa.0.0.i.i.i, %bb20.i.i.i ], [ %_10.sroa.0.0.i.i.i, %bb22.i.i.i ], [ 0, %bb46.i.i.i ], [ 0, %bb44.i.i.i ], [ 0, %bb44.i.i.i ], [ 0, %bb44.i.i.i ], [ 0, %bb44.i.i.i ], [ 12543, %bb45.i.i.i ], [ 0, %bb48.i.i.i ], [ 0, %bb49.i.i.i ], [ 15362, %bb50.i.i.i ], [ 0, %bb51.i.i.i ], [ 15364, %bb53.i.i.i ], [ 15364, %bb53.i.i.i ], [ 0, %bb57.i.i.i ], [ 0, %bb60.i.i.i ], [ 0, %bb61.i.i.i ], [ 0, %bb64.i.i.i ], [ 5, %bb129.i.i.i ], [ 4102, %bb68.i.i.i ], [ 4102, %bb68.i.i.i ], [ 4102, %bb68.i.i.i ], [ 4102, %bb68.i.i.i ], [ 4102, %bb68.i.i.i ], [ 4103, %bb69.i.i.i ], [ 4, %bb66.i.i.i ], [ 4, %bb66.i.i.i ], [ 9, %bb75.i.i.i ], [ 2, %bb82.i.i.i ], [ 16, %bb84.i.i.i ], [ 25, %bb86.i.i.i ], [ 17, %bb186.i.i.i ], [ 17, %bb104.i.i.i ], [ 17, %bb104.i.i.i ], [ 17, %bb104.i.i.i ], [ 17, %bb104.i.i.i ], [ 17, %bb104.i.i.i ], [ 26, %bb89.i.i.i ], [ 17, %bb184.i.i.i ], [ 27, %bb92.i.i.i ], [ 17, %bb182.i.i.i ], [ 28, %bb95.i.i.i ], [ 17, %bb179.i.i.i ], [ 5, %bb106.i.i.i ], [ 5, %bb106.i.i.i ], [ 5, %bb106.i.i.i ], [ 5, %bb106.i.i.i ], [ 5, %bb106.i.i.i ], [ 5, %bb177.i.i.i ], [ 5, %bb180.i.i.i ], [ 29, %bb98.i.i.i ], [ 17, %bb176.i.i.i ], [ 30, %bb101.i.i.i ], [ 10, %bb79.i.i.i ], [ 10, %bb79.i.i.i ], [ 5, %bb116.i.i.i ], [ 0, %bb110.i.i.i ], [ 1, %bb13.i.i7.i ], [ %ret.1.i.i.i, %bb151.i.i.i ], [ 0, %bb198.i.i.i ], [ 0, %bb10.i.i.i ], [ 0, %bb12.i.i.i ], [ 0, %bb7.i.i.i.i.i ], [ 4102, %bb144.i.i.i ], [ 5, %bb173.i.i.i ], [ %next_info.sroa.0.1.i.i.i, %bb2.i.i ], [ %next_info.sroa.0.1.i.i.i, %bb35.i.i.i ], [ %next_info.sroa.0.1.i.i.i, %bb35.i.i.i ], [ %switch.load, %switch.lookup4 ], [ 0, %bb29.i.i.i ]
  %_0.sroa.0.0.i.i.i = phi i8 [ %..i.i.i, %bb3.i.i8.i ], [ 0, %bb36.i.i.i ], [ 3, %bb122.i.i.i ], [ 0, %bb119.i.i.i ], [ 0, %bb118.i.i.i ], [ 0, %bb18.i.i.i ], [ 0, %bb20.i.i.i ], [ 0, %bb22.i.i.i ], [ 0, %bb46.i.i.i ], [ 0, %bb44.i.i.i ], [ 0, %bb44.i.i.i ], [ 0, %bb44.i.i.i ], [ 0, %bb44.i.i.i ], [ 0, %bb45.i.i.i ], [ 0, %bb48.i.i.i ], [ -1, %bb49.i.i.i ], [ 0, %bb50.i.i.i ], [ 0, %bb51.i.i.i ], [ 1, %bb53.i.i.i ], [ 1, %bb53.i.i.i ], [ 0, %bb57.i.i.i ], [ -1, %bb60.i.i.i ], [ 0, %bb61.i.i.i ], [ 0, %bb64.i.i.i ], [ 0, %bb129.i.i.i ], [ 0, %bb68.i.i.i ], [ 0, %bb68.i.i.i ], [ 0, %bb68.i.i.i ], [ 0, %bb68.i.i.i ], [ 0, %bb68.i.i.i ], [ 0, %bb69.i.i.i ], [ 1, %bb66.i.i.i ], [ 1, %bb66.i.i.i ], [ 1, %bb75.i.i.i ], [ 0, %bb82.i.i.i ], [ 0, %bb84.i.i.i ], [ 0, %bb86.i.i.i ], [ 0, %bb186.i.i.i ], [ 0, %bb104.i.i.i ], [ 0, %bb104.i.i.i ], [ 0, %bb104.i.i.i ], [ 0, %bb104.i.i.i ], [ 0, %bb104.i.i.i ], [ 0, %bb89.i.i.i ], [ 0, %bb184.i.i.i ], [ 0, %bb92.i.i.i ], [ 0, %bb182.i.i.i ], [ 0, %bb95.i.i.i ], [ 0, %bb179.i.i.i ], [ 0, %bb106.i.i.i ], [ 0, %bb106.i.i.i ], [ 0, %bb106.i.i.i ], [ 0, %bb106.i.i.i ], [ 0, %bb106.i.i.i ], [ 0, %bb177.i.i.i ], [ 0, %bb180.i.i.i ], [ 0, %bb98.i.i.i ], [ 0, %bb176.i.i.i ], [ 0, %bb101.i.i.i ], [ -1, %bb79.i.i.i ], [ -1, %bb79.i.i.i ], [ 0, %bb116.i.i.i ], [ 0, %bb110.i.i.i ], [ 1, %bb13.i.i7.i ], [ %ret.0.i.i.i, %bb151.i.i.i ], [ 1, %bb198.i.i.i ], [ 1, %bb10.i.i.i ], [ %spec.select.i.i.i, %bb12.i.i.i ], [ 1, %bb7.i.i.i.i.i ], [ 0, %bb144.i.i.i ], [ 0, %bb173.i.i.i ], [ 0, %bb2.i.i ], [ 0, %bb35.i.i.i ], [ 0, %bb35.i.i.i ], [ %switch.masked, %switch.lookup4 ], [ 2, %bb29.i.i.i ]
  %_10.i.i = sext i8 %_0.sroa.0.0.i.i.i to i64
  %_9.i.i = add i64 %accum.sroa.0.026.i, %_10.i.i
  %.not.i.i = icmp eq ptr %s.0, %self.sroa.2.320.i
  br i1 %.not.i.i, label %_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator5rfoldTjNtNtCsDJOD2kcAir_13unicode_width6tables9WidthInfoENCNvB1N_9str_width0ECsC3JuwEIQwb_7console.exit, label %bb17.i.i.i

_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator5rfoldTjNtNtCsDJOD2kcAir_13unicode_width6tables9WidthInfoENCNvB1N_9str_width0ECsC3JuwEIQwb_7console.exit: ; preds = %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.i, %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i, %start
  %accum.sroa.0.0.lcssa.i = phi i64 [ 0, %start ], [ %accum.sroa.0.026.i, %_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back.exit.i ], [ %_9.i.i, %_RNCNvNtCsDJOD2kcAir_13unicode_width6tables9str_width0CsC3JuwEIQwb_7console.exit.i ]
  ret i64 %accum.sroa.0.0.lcssa.i
}

; console::unix_term::key_from_utf8
; Function Attrs: uwtable
define internal fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.0, i64 noundef range(i64 2, 5) %buf.1) unnamed_addr #1 {
start:
  %_2 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2)
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_2, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.0, i64 noundef %buf.1)
  %_3 = load i64, ptr %_2, align 8, !range !6, !noundef !7
  %0 = trunc nuw i64 %_3 to i1
  br i1 %0, label %bb3, label %bb2

bb3:                                              ; preds = %bb2, %start
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2)
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb4

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_2, i64 8
  %s.0 = load ptr, ptr %1, align 8, !nonnull !7, !align !187, !noundef !7
  %2 = getelementptr inbounds nuw i8, ptr %_2, i64 16
  %s.1 = load i64, ptr %2, align 8, !noundef !7
  %_6.i.i = icmp samesign eq i64 %s.1, 0
  br i1 %_6.i.i, label %bb3, label %bb14.i

bb14.i:                                           ; preds = %bb2
  %x.i = load i8, ptr %s.0, align 1, !noalias !910, !noundef !7
  %_6.i = icmp sgt i8 %x.i, -1
  br i1 %_6.i, label %bb3.i, label %bb4.i

bb4.i:                                            ; preds = %bb14.i
  %_16.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 1
  %_30.i = and i8 %x.i, 31
  %init.i = zext nneg i8 %_30.i to i32
  %_6.i10.i = icmp samesign ne i64 %s.1, 1
  tail call void @llvm.assume(i1 %_6.i10.i)
  %y.i = load i8, ptr %_16.i.i, align 1, !noalias !910, !noundef !7
  %_33.i = shl nuw nsw i32 %init.i, 6
  %_35.i = and i8 %y.i, 63
  %_34.i = zext nneg i8 %_35.i to i32
  %3 = or disjoint i32 %_33.i, %_34.i
  %_13.i = icmp samesign ugt i8 %x.i, -33
  br i1 %_13.i, label %bb6.i, label %bb8

bb3.i:                                            ; preds = %bb14.i
  %_7.i = zext nneg i8 %x.i to i32
  br label %bb8

bb6.i:                                            ; preds = %bb4.i
  %_16.i12.i = getelementptr inbounds nuw i8, ptr %s.0, i64 2
  %_6.i17.i = icmp samesign ne i64 %s.1, 2
  tail call void @llvm.assume(i1 %_6.i17.i)
  %z.i = load i8, ptr %_16.i12.i, align 1, !noalias !910, !noundef !7
  %_38.i = shl nuw nsw i32 %_34.i, 6
  %_40.i = and i8 %z.i, 63
  %_39.i = zext nneg i8 %_40.i to i32
  %y_z.i = or disjoint i32 %_38.i, %_39.i
  %_20.i = shl nuw nsw i32 %init.i, 12
  %4 = or disjoint i32 %y_z.i, %_20.i
  %_21.i = icmp samesign ugt i8 %x.i, -17
  br i1 %_21.i, label %bb8.i, label %bb8

bb8.i:                                            ; preds = %bb6.i
  %_16.i19.i = getelementptr inbounds nuw i8, ptr %s.0, i64 3
  %_6.i24.i = icmp samesign ne i64 %s.1, 3
  tail call void @llvm.assume(i1 %_6.i24.i)
  %w.i = load i8, ptr %_16.i19.i, align 1, !noalias !910, !noundef !7
  %_26.i = shl nuw nsw i32 %init.i, 18
  %_25.i = and i32 %_26.i, 1835008
  %_43.i = shl nuw nsw i32 %y_z.i, 6
  %_45.i = and i8 %w.i, 63
  %_44.i = zext nneg i8 %_45.i to i32
  %_27.i = or disjoint i32 %_43.i, %_44.i
  %5 = or disjoint i32 %_27.i, %_25.i
  br label %bb8

bb8:                                              ; preds = %bb3.i, %bb8.i, %bb6.i, %bb4.i
  %_0.sroa.4.0.i.ph = phi i32 [ %3, %bb4.i ], [ %4, %bb6.i ], [ %5, %bb8.i ], [ %_7.i, %bb3.i ]
  %6 = icmp samesign ult i32 %_0.sroa.4.0.i.ph, 1114112
  tail call void @llvm.assume(i1 %6)
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i32 %_0.sroa.4.0.i.ph, ptr %7, align 8
  store i64 -9223372036854775789, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2)
  br label %bb4

bb4:                                              ; preds = %bb3, %bb8
  ret void
}

; console::unix_term::terminal_size
; Function Attrs: uwtable
define i48 @_RNvNtCsC3JuwEIQwb_7console9unix_term13terminal_size(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %out) unnamed_addr #1 {
start:
  %winsize = alloca [8 x i8], align 8
; call <console::term::Term as std::os::fd::raw::AsRawFd>::as_raw_fd
  %_10 = tail call noundef i32 @_RNvXs2_NtCsC3JuwEIQwb_7console4termNtB5_4TermNtNtNtNtCs5sEH5CPMdak_3std2os2fd3raw7AsRawFd9as_raw_fd(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out)
  %_9 = tail call noundef i32 @isatty(i32 noundef %_10) #29
  %0 = icmp eq i32 %_9, 0
  br i1 %0, label %bb9, label %bb1

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %winsize)
  store i64 0, ptr %winsize, align 8
; call <console::term::Term as std::os::fd::raw::AsRawFd>::as_raw_fd
  %_4 = tail call noundef i32 @_RNvXs2_NtCsC3JuwEIQwb_7console4termNtB5_4TermNtNtNtNtCs5sEH5CPMdak_3std2os2fd3raw7AsRawFd9as_raw_fd(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out)
  %_3 = call noundef i32 (i32, i64, ...) @ioctl(i32 noundef %_4, i64 noundef 1074295912, ptr noalias noundef nonnull align 2 dereferenceable(8) %winsize) #29
  %winsize1 = load i16, ptr %winsize, align 8, !noundef !7
  %1 = getelementptr inbounds nuw i8, ptr %winsize, i64 2
  %winsize2 = load i16, ptr %1, align 2, !noundef !7
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %winsize)
  %_6 = icmp ne i16 %winsize1, 0
  %_7 = icmp ne i16 %winsize2, 0
  %or.cond = and i1 %_6, %_7
  %. = zext i1 %or.cond to i16
  br label %bb9

bb9:                                              ; preds = %bb1, %start
  %_0.sroa.5.0 = phi i16 [ undef, %start ], [ %winsize2, %bb1 ]
  %_0.sroa.4.0 = phi i16 [ undef, %start ], [ %winsize1, %bb1 ]
  %_0.sroa.0.0 = phi i16 [ 0, %start ], [ %., %bb1 ]
  %_0.sroa.5.0.insert.ext = zext i16 %_0.sroa.5.0 to i48
  %_0.sroa.5.0.insert.shift = shl nuw i48 %_0.sroa.5.0.insert.ext, 32
  %_0.sroa.4.0.insert.ext = zext i16 %_0.sroa.4.0 to i48
  %_0.sroa.4.0.insert.shift = shl nuw nsw i48 %_0.sroa.4.0.insert.ext, 16
  %_0.sroa.4.0.insert.insert = or disjoint i48 %_0.sroa.4.0.insert.shift, %_0.sroa.5.0.insert.shift
  %_0.sroa.0.0.insert.ext = zext nneg i16 %_0.sroa.0.0 to i48
  %_0.sroa.0.0.insert.insert = or disjoint i48 %_0.sroa.4.0.insert.insert, %_0.sroa.0.0.insert.ext
  ret i48 %_0.sroa.0.0.insert.insert
}

; console::unix_term::read_single_key
; Function Attrs: uwtable
define internal fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term15read_single_key(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, i1 noundef zeroext %ctrlc_key) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_2.i57.i = alloca [24 x i8], align 8
  %_2.i.i = alloca [24 x i8], align 8
  %_95.i = alloca [16 x i8], align 8
  %buf.i = alloca [4 x i8], align 1
  %_27.i = alloca [16 x i8], align 8
  %_14.i41 = alloca [16 x i8], align 8
  %_8.i42 = alloca [16 x i8], align 8
  %_3.i = alloca [16 x i8], align 8
  %_6.i = alloca [12 x i8], align 4
  %_4.i = alloca [16 x i8], align 8
  %rv = alloca [24 x i8], align 8
  %original = alloca [72 x i8], align 8
  %termios1 = alloca [72 x i8], align 8
  %termios = alloca [72 x i8], align 8
; call std::io::stdio::stdin
  %stdin.i = tail call noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio5stdin(), !noalias !913
  %_10.i = tail call noundef i32 @isatty(i32 noundef 0) #29, !noalias !913
  %.not.not.not = icmp eq i32 %_10.i, 0
  br i1 %.not.not.not, label %bb3.i, label %bb28

bb3.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4.i), !noalias !913
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_6.i), !noalias !913
  store i32 0, ptr %_6.i, align 4, !noalias !913
  %_11.sroa.4.0._6.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_6.i, i64 4
  store i16 438, ptr %_11.sroa.4.0._6.sroa_idx.i, align 4, !noalias !913
  %_11.sroa.5.0._6.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_6.i, i64 6
  %_11.sroa.6.0._6.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_6.i, i64 7
  %0 = getelementptr inbounds nuw i8, ptr %_6.i, i64 8
  store i32 0, ptr %0, align 4, !noalias !913
  store i8 1, ptr %_11.sroa.5.0._6.sroa_idx.i, align 2, !noalias !913
  store i8 1, ptr %_11.sroa.6.0._6.sroa_idx.i, align 1, !noalias !913
; call <std::fs::OpenOptions>::_open
  call void @_RNvMsl_NtCs5sEH5CPMdak_3std2fsNtB5_11OpenOptions5__open(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_4.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %_6.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_890f94c692f51df5fc72c5e0dfe1823b, i64 noundef 8), !noalias !913
  %1 = load i32, ptr %_4.i, align 8, !range !228, !noalias !913, !noundef !7
  %2 = trunc nuw i32 %1 to i1
  br i1 %2, label %bb27, label %bb10.i

bb10.i:                                           ; preds = %bb3.i
  %3 = getelementptr inbounds nuw i8, ptr %_4.i, i64 4
  %_13.i = load i32, ptr %3, align 4, !range !28, !noalias !913, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4.i), !noalias !913
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_6.i), !noalias !913
  br label %bb28

bb27:                                             ; preds = %bb3.i
  %4 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_14.i = load ptr, ptr %4, align 8, !noalias !913, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4.i), !noalias !913
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_6.i), !noalias !913
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_14.i, ptr %5, align 8
  store i64 -9223372036854775787, ptr %_0, align 8
  br label %bb23

bb28:                                             ; preds = %bb10.i, %start
  %_2.sroa.0.0.i.i = phi i32 [ 0, %start ], [ %_13.i, %bb10.i ]
  %_4.sroa.7.0.ph = phi i32 [ undef, %start ], [ %_13.i, %bb10.i ]
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %termios)
  %_0.i.i = call noundef i32 @tcgetattr(i32 noundef %_2.sroa.0.0.i.i, ptr noundef nonnull align 8 dereferenceable(72) %termios) #29
  %6 = icmp eq i32 %_0.i.i, 0
  br i1 %6, label %bb30, label %bb29

bb2.i30:                                          ; preds = %cleanup
  %_5.i.i.i.i.i.i = call noundef i32 @close(i32 noundef %_4.sroa.7.0.ph) #29
  br label %bb26

cleanup.loopexit:                                 ; preds = %.noexc45
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

cleanup.loopexit.split-lp:                        ; preds = %bb4.i.i.invoke, %bb89.i.invoke, %bb18, %bb32, %bb65.i, %bb7.i, %bb11.i, %bb15.i, %bb102.i, %bb104.i, %bb106.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

cleanup:                                          ; preds = %cleanup.loopexit.split-lp, %cleanup.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.loopexit.split-lp ]
  br i1 %.not.not.not, label %bb2.i30, label %bb26

bb29:                                             ; preds = %bb28
  %_5.i = tail call noundef ptr @__error() #29
  %_4.i28 = load i32, ptr %_5.i, align 4, !noundef !7
  %_9.i = sext i32 %_4.i28 to i64
  %_8.i = shl nsw i64 %_9.i, 32
  %_7.i = or disjoint i64 %_8.i, 2
  %_14.i29 = inttoptr i64 %_7.i to ptr
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_14.i29, ptr %7, align 8
  store i64 -9223372036854775787, ptr %_0, align 8
  br label %bb21

bb30:                                             ; preds = %bb28
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %termios1)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %termios1, ptr noundef nonnull align 8 dereferenceable(72) %termios, i64 72, i1 false)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %original)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %original, ptr noundef nonnull align 8 dereferenceable(72) %termios, i64 72, i1 false)
  call void @cfmakeraw(ptr noundef nonnull %termios1) #29
  %8 = getelementptr inbounds nuw i8, ptr %original, i64 8
  %_19 = load i64, ptr %8, align 8, !noundef !7
  %9 = getelementptr inbounds nuw i8, ptr %termios1, i64 8
  store i64 %_19, ptr %9, align 8
  %_0.i.i32 = call noundef i32 @tcsetattr(i32 noundef %_2.sroa.0.0.i.i, i32 noundef 1, ptr noundef nonnull readonly align 8 dereferenceable(72) %termios1) #29
  %10 = icmp eq i32 %_0.i.i32, 0
  br i1 %10, label %bb32, label %bb31

bb31:                                             ; preds = %bb30
  %_5.i34 = tail call noundef ptr @__error() #29
  %_4.i35 = load i32, ptr %_5.i34, align 4, !noundef !7
  %_9.i36 = sext i32 %_4.i35 to i64
  %_8.i37 = shl nsw i64 %_9.i36, 32
  %_7.i38 = or disjoint i64 %_8.i37, 2
  %_14.i39 = inttoptr i64 %_7.i38 to ptr
  %11 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_14.i39, ptr %11, align 8
  store i64 -9223372036854775787, ptr %_0, align 8
  br label %bb20

bb32:                                             ; preds = %bb30
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %rv)
  call void @llvm.experimental.noalias.scope.decl(metadata !916)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_3.i), !noalias !916
; invoke console::unix_term::read_single_char
  invoke fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term16read_single_char(ptr noalias noundef align 8 captures(none) dereferenceable(16) %_3.i, i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i)
          to label %.noexc unwind label %cleanup.loopexit.split-lp

.noexc:                                           ; preds = %bb32
  %12 = load i32, ptr %_3.i, align 8, !range !228, !noalias !916, !noundef !7
  %13 = trunc nuw i32 %12 to i1
  br i1 %13, label %bb78.i, label %bb79.lr.ph.i

bb79.lr.ph.i:                                     ; preds = %.noexc
  %14 = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  %15 = getelementptr inbounds nuw i8, ptr %_95.i, i64 8
  br label %bb79.i

bb78.i:                                           ; preds = %.noexc46, %.noexc
  %16 = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  %_115.i = load ptr, ptr %16, align 8, !noalias !916, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3.i), !noalias !916
  %17 = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %_115.i, ptr %17, align 8, !alias.scope !916
  store i64 -9223372036854775787, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb79.i:                                           ; preds = %.noexc46, %bb79.lr.ph.i
  %_114.i = load i32, ptr %14, align 4, !range !919, !noalias !916, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3.i), !noalias !916
  switch i32 %_114.i, label %bb6.i [
    i32 1114112, label %bb5.i
    i32 27, label %bb7.i
  ]

bb5.i:                                            ; preds = %bb79.i
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_95.i), !noalias !916
; call console::unix_term::select_or_poll_term_fd
  call fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term22select_or_poll_term_fd(ptr noalias noundef align 8 captures(address) dereferenceable(16) %_95.i, i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i, i32 noundef -1), !noalias !916
  %18 = load i8, ptr %_95.i, align 8, !range !29, !noalias !916, !noundef !7
  %19 = trunc nuw i8 %18 to i1
  br i1 %19, label %bb65.i, label %.noexc45

bb65.i:                                           ; preds = %bb5.i
  %_246.i = tail call noundef ptr @__error() #29
  %_245.i = load i32, ptr %_246.i, align 4, !noalias !916, !noundef !7
  %_250.i = sext i32 %_245.i to i64
  %_249.i = shl nsw i64 %_250.i, 32
  %_248.i = or disjoint i64 %_249.i, 2
  %_255.i = inttoptr i64 %_248.i to ptr
  %20 = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %_255.i, ptr %20, align 8, !alias.scope !916
  store i64 -9223372036854775787, ptr %rv, align 8, !alias.scope !916
  %_95.val30.i = load ptr, ptr %15, align 8, !noalias !916
; invoke core::ptr::drop_in_place::<core::result::Result<bool, std::io::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultbNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsC3JuwEIQwb_7console(i8 1, ptr %_95.val30.i)
          to label %.noexc44 unwind label %cleanup.loopexit.split-lp

.noexc44:                                         ; preds = %bb65.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_95.i), !noalias !916
  br label %bb6

.noexc45:                                         ; preds = %bb5.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_95.i), !noalias !916
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_3.i), !noalias !916
; invoke console::unix_term::read_single_char
  invoke fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term16read_single_char(ptr noalias noundef align 8 captures(none) dereferenceable(16) %_3.i, i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i)
          to label %.noexc46 unwind label %cleanup.loopexit

.noexc46:                                         ; preds = %.noexc45
  %21 = load i32, ptr %_3.i, align 8, !range !228, !noalias !916, !noundef !7
  %22 = trunc nuw i32 %21 to i1
  br i1 %22, label %bb78.i, label %bb79.i

bb7.i:                                            ; preds = %bb79.i
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_8.i42), !noalias !916
; invoke console::unix_term::read_single_char
  invoke fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term16read_single_char(ptr noalias noundef align 8 captures(none) dereferenceable(16) %_8.i42, i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i)
          to label %.noexc47 unwind label %cleanup.loopexit.split-lp

.noexc47:                                         ; preds = %bb7.i
  %23 = load i32, ptr %_8.i42, align 8, !range !228, !noalias !916, !noundef !7
  %24 = trunc nuw i32 %23 to i1
  br i1 %24, label %bb80.i, label %bb81.i

bb6.i:                                            ; preds = %bb79.i
  %25 = icmp samesign ult i32 %_114.i, 1114112
  call void @llvm.assume(i1 %25)
  %byte.i = trunc i32 %_114.i to i8
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %buf.i), !noalias !916
  store i8 %byte.i, ptr %buf.i, align 1, !noalias !916
  %26 = getelementptr inbounds nuw i8, ptr %buf.i, i64 1
  store i8 0, ptr %26, align 1, !noalias !916
  %27 = getelementptr inbounds nuw i8, ptr %buf.i, i64 2
  store i8 0, ptr %27, align 1, !noalias !916
  %28 = getelementptr inbounds nuw i8, ptr %buf.i, i64 3
  store i8 0, ptr %28, align 1, !noalias !916
  %_64.i = and i8 %byte.i, -32
  %29 = icmp eq i8 %_64.i, -64
  br i1 %29, label %bb41.i, label %bb44.i

bb80.i:                                           ; preds = %.noexc47
  %30 = getelementptr inbounds nuw i8, ptr %_8.i42, i64 8
  %_120.i = load ptr, ptr %30, align 8, !noalias !916, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_8.i42), !noalias !916
  br label %bb76.i

bb81.i:                                           ; preds = %.noexc47
  %31 = getelementptr inbounds nuw i8, ptr %_8.i42, i64 4
  %_119.i = load i32, ptr %31, align 4, !range !919, !noalias !916, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_8.i42), !noalias !916
  switch i32 %_119.i, label %bb39.i [
    i32 1114112, label %bb9.i43
    i32 91, label %bb11.i
  ]

bb9.i43:                                          ; preds = %bb81.i
  store i64 -9223372036854775801, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb11.i:                                           ; preds = %bb81.i
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_14.i41), !noalias !916
; invoke console::unix_term::read_single_char
  invoke fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term16read_single_char(ptr noalias noundef align 8 captures(none) dereferenceable(16) %_14.i41, i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i)
          to label %.noexc48 unwind label %cleanup.loopexit.split-lp

.noexc48:                                         ; preds = %bb11.i
  %32 = load i32, ptr %_14.i41, align 8, !range !228, !noalias !916, !noundef !7
  %33 = trunc nuw i32 %32 to i1
  br i1 %33, label %bb82.i, label %bb83.i

bb39.i:                                           ; preds = %bb81.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !916
; call __rustc::__rust_alloc
  %34 = call noundef align 4 dereferenceable_or_null(4) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 4, i64 noundef 4) #29, !noalias !916
  %35 = icmp eq ptr %34, null
  br i1 %35, label %bb89.i.invoke, label %bb99.i, !prof !5

bb82.i:                                           ; preds = %.noexc48
  %36 = getelementptr inbounds nuw i8, ptr %_14.i41, i64 8
  %_125.i = load ptr, ptr %36, align 8, !noalias !916, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_14.i41), !noalias !916
  br label %bb76.i

bb83.i:                                           ; preds = %.noexc48
  %37 = getelementptr inbounds nuw i8, ptr %_14.i41, i64 4
  %_124.i = load i32, ptr %37, align 4, !range !919, !noalias !916, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_14.i41), !noalias !916
  switch i32 %_124.i, label %bb15.i [
    i32 1114112, label %bb13.i
    i32 65, label %bb22.i
    i32 66, label %bb21.i
    i32 67, label %bb20.i
    i32 68, label %bb19.i
    i32 72, label %bb18.i
    i32 70, label %bb17.i
    i32 90, label %bb16.i
  ]

bb13.i:                                           ; preds = %bb83.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !916
; call __rustc::__rust_alloc
  %38 = call noundef align 4 dereferenceable_or_null(4) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 4, i64 noundef 4) #29, !noalias !916
  %39 = icmp eq ptr %38, null
  br i1 %39, label %bb89.i.invoke, label %bb85.i, !prof !5

bb15.i:                                           ; preds = %bb83.i
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_27.i), !noalias !916
; invoke console::unix_term::read_single_char
  invoke fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term16read_single_char(ptr noalias noundef align 8 captures(none) dereferenceable(16) %_27.i, i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i)
          to label %.noexc49 unwind label %cleanup.loopexit.split-lp

.noexc49:                                         ; preds = %bb15.i
  %40 = load i32, ptr %_27.i, align 8, !range !228, !noalias !916, !noundef !7
  %41 = trunc nuw i32 %40 to i1
  br i1 %41, label %bb87.i, label %bb88.i

bb22.i:                                           ; preds = %bb83.i
  store i64 -9223372036854775804, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb21.i:                                           ; preds = %bb83.i
  store i64 -9223372036854775803, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb20.i:                                           ; preds = %bb83.i
  store i64 -9223372036854775805, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb19.i:                                           ; preds = %bb83.i
  store i64 -9223372036854775806, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb18.i:                                           ; preds = %bb83.i
  store i64 -9223372036854775799, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb17.i:                                           ; preds = %bb83.i
  store i64 -9223372036854775798, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb16.i:                                           ; preds = %bb83.i
  store i64 -9223372036854775796, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb87.i:                                           ; preds = %.noexc49
  %42 = getelementptr inbounds nuw i8, ptr %_27.i, i64 8
  %_135.i = load ptr, ptr %42, align 8, !noalias !916, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_27.i), !noalias !916
  br label %bb76.i

bb88.i:                                           ; preds = %.noexc49
  %43 = getelementptr inbounds nuw i8, ptr %_27.i, i64 4
  %_134.i = load i32, ptr %43, align 4, !range !919, !noalias !916, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_27.i), !noalias !916
  switch i32 %_134.i, label %bb35.i [
    i32 1114112, label %bb37.i
    i32 126, label %bb25.i
  ]

bb37.i:                                           ; preds = %bb88.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !916
; call __rustc::__rust_alloc
  %44 = call noundef align 4 dereferenceable_or_null(8) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 8, i64 noundef 4) #29, !noalias !916
  %45 = icmp eq ptr %44, null
  br i1 %45, label %bb89.i.invoke, label %bb96.i, !prof !5

bb25.i:                                           ; preds = %bb88.i
  switch i32 %_124.i, label %bb26.i [
    i32 49, label %bb34.i
    i32 50, label %bb33.i
    i32 51, label %bb32.i
    i32 52, label %bb31.i
    i32 53, label %bb30.i
    i32 54, label %bb29.i
    i32 55, label %bb28.i
    i32 56, label %bb27.i
  ]

bb35.i:                                           ; preds = %bb88.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !916
; call __rustc::__rust_alloc
  %46 = call noundef align 4 dereferenceable_or_null(12) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 12, i64 noundef 4) #29, !noalias !916
  %47 = icmp eq ptr %46, null
  br i1 %47, label %bb89.i.invoke, label %bb93.i, !prof !5

bb26.i:                                           ; preds = %bb25.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #29, !noalias !916
; call __rustc::__rust_alloc
  %48 = call noundef align 4 dereferenceable_or_null(12) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 12, i64 noundef 4) #29, !noalias !916
  %49 = icmp eq ptr %48, null
  br i1 %49, label %bb89.i.invoke, label %bb90.i, !prof !5

bb34.i:                                           ; preds = %bb25.i
  store i64 -9223372036854775799, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb33.i:                                           ; preds = %bb25.i
  store i64 -9223372036854775792, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb32.i:                                           ; preds = %bb25.i
  store i64 -9223372036854775794, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb31.i:                                           ; preds = %bb25.i
  store i64 -9223372036854775798, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb30.i:                                           ; preds = %bb25.i
  store i64 -9223372036854775791, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb29.i:                                           ; preds = %bb25.i
  store i64 -9223372036854775790, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb28.i:                                           ; preds = %bb25.i
  store i64 -9223372036854775799, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb27.i:                                           ; preds = %bb25.i
  store i64 -9223372036854775798, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb89.i.invoke:                                    ; preds = %bb39.i, %bb13.i, %bb37.i, %bb35.i, %bb26.i
  %50 = phi i64 [ 12, %bb26.i ], [ 12, %bb35.i ], [ 8, %bb37.i ], [ 4, %bb13.i ], [ 4, %bb39.i ]
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 4, i64 noundef %50) #30
          to label %bb89.i.cont unwind label %cleanup.loopexit.split-lp

bb89.i.cont:                                      ; preds = %bb89.i.invoke
  unreachable

bb90.i:                                           ; preds = %bb26.i
  store i32 91, ptr %48, align 4, !noalias !916
  %51 = getelementptr inbounds nuw i8, ptr %48, i64 4
  store i32 %_124.i, ptr %51, align 4, !noalias !916
  %52 = getelementptr inbounds nuw i8, ptr %48, i64 8
  store i32 126, ptr %52, align 4, !noalias !916
  store i64 3, ptr %rv, align 8, !alias.scope !916
  %_40.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %48, ptr %_40.sroa.4.0._0.sroa_idx.i, align 8, !alias.scope !916
  %_40.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 16
  store i64 3, ptr %_40.sroa.5.0._0.sroa_idx.i, align 8, !alias.scope !916
  br label %bb6

bb93.i:                                           ; preds = %bb35.i
  store i32 91, ptr %46, align 4, !noalias !916
  %53 = getelementptr inbounds nuw i8, ptr %46, i64 4
  store i32 %_124.i, ptr %53, align 4, !noalias !916
  %54 = getelementptr inbounds nuw i8, ptr %46, i64 8
  store i32 %_134.i, ptr %54, align 4, !noalias !916
  store i64 3, ptr %rv, align 8, !alias.scope !916
  %_44.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %46, ptr %_44.sroa.4.0._0.sroa_idx.i, align 8, !alias.scope !916
  %_44.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 16
  store i64 3, ptr %_44.sroa.5.0._0.sroa_idx.i, align 8, !alias.scope !916
  br label %bb6

bb96.i:                                           ; preds = %bb37.i
  store i32 91, ptr %44, align 4, !noalias !916
  %55 = getelementptr inbounds nuw i8, ptr %44, i64 4
  store i32 %_124.i, ptr %55, align 4, !noalias !916
  store i64 2, ptr %rv, align 8, !alias.scope !916
  %_48.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %44, ptr %_48.sroa.4.0._0.sroa_idx.i, align 8, !alias.scope !916
  %_48.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 16
  store i64 2, ptr %_48.sroa.5.0._0.sroa_idx.i, align 8, !alias.scope !916
  br label %bb6

bb85.i:                                           ; preds = %bb13.i
  store i32 91, ptr %38, align 4, !noalias !916
  store i64 1, ptr %rv, align 8, !alias.scope !916
  %_52.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %38, ptr %_52.sroa.4.0._0.sroa_idx.i, align 8, !alias.scope !916
  %_52.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 16
  store i64 1, ptr %_52.sroa.5.0._0.sroa_idx.i, align 8, !alias.scope !916
  br label %bb6

bb76.i:                                           ; preds = %bb87.i, %bb82.i, %bb80.i
  %_135.sink.i = phi ptr [ %_135.i, %bb87.i ], [ %_125.i, %bb82.i ], [ %_120.i, %bb80.i ]
  %56 = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %_135.sink.i, ptr %56, align 8, !alias.scope !916
  store i64 -9223372036854775787, ptr %rv, align 8, !alias.scope !916
  br label %bb6

bb99.i:                                           ; preds = %bb39.i
  store i32 %_119.i, ptr %34, align 4, !noalias !916
  store i64 1, ptr %rv, align 8, !alias.scope !916
  %_56.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %34, ptr %_56.sroa.4.0._0.sroa_idx.i, align 8, !alias.scope !916
  %_56.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 16
  store i64 1, ptr %_56.sroa.5.0._0.sroa_idx.i, align 8, !alias.scope !916
  br label %bb6

bb41.i:                                           ; preds = %bb6.i
  %read.i.i = call noundef i64 @read(i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i, ptr noundef nonnull align 1 %26, i64 noundef 1) #29, !noalias !920
  %_7.i.i = icmp slt i64 %read.i.i, 0
  br i1 %_7.i.i, label %bb70.i.sink.split, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb41.i
  %57 = icmp eq i64 %read.i.i, 0
  br i1 %57, label %bb4.i.i.invoke, label %bb7.i.i

bb4.i.i.invoke:                                   ; preds = %bb7.i106.i, %bb3.i105.i, %bb7.i40.i, %bb3.i39.i, %bb7.i.i, %bb3.i.i
  %58 = phi i8 [ 37, %bb3.i.i ], [ 35, %bb7.i.i ], [ 37, %bb3.i39.i ], [ 35, %bb7.i40.i ], [ 37, %bb3.i105.i ], [ 35, %bb7.i106.i ]
  %59 = phi ptr [ @alloc_7ef65f9252ea7966a1063c1ab22af5b9, %bb3.i.i ], [ @alloc_50f3f282355a7f8c5c8e6d622c64bbaf, %bb7.i.i ], [ @alloc_7ef65f9252ea7966a1063c1ab22af5b9, %bb3.i39.i ], [ @alloc_50f3f282355a7f8c5c8e6d622c64bbaf, %bb7.i40.i ], [ @alloc_7ef65f9252ea7966a1063c1ab22af5b9, %bb3.i105.i ], [ @alloc_50f3f282355a7f8c5c8e6d622c64bbaf, %bb7.i106.i ]
  %60 = phi i64 [ 19, %bb3.i.i ], [ 16, %bb7.i.i ], [ 19, %bb3.i39.i ], [ 16, %bb7.i40.i ], [ 19, %bb3.i105.i ], [ 16, %bb7.i106.i ]
; invoke <std::io::error::Error>::new::<&str>
  %61 = invoke noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newReEBa_(i8 noundef %58, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %59, i64 noundef %60) #33
          to label %bb70.i unwind label %cleanup.loopexit.split-lp

bb7.i.i:                                          ; preds = %bb3.i.i
  %_10.i.i = load i8, ptr %26, align 1, !alias.scope !923, !noalias !920, !noundef !7
  %62 = icmp eq i8 %_10.i.i, 3
  br i1 %62, label %bb4.i.i.invoke, label %bb102.i

bb44.i:                                           ; preds = %bb6.i
  %_74.i = and i8 %byte.i, -16
  %63 = icmp eq i8 %_74.i, -32
  br i1 %63, label %bb45.i, label %bb48.i

bb102.i:                                          ; preds = %bb7.i.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i), !noalias !925
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_2.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.i, i64 noundef 2)
          to label %.noexc57 unwind label %cleanup.loopexit.split-lp

.noexc57:                                         ; preds = %bb102.i
  %_3.i.i = load i64, ptr %_2.i.i, align 8, !range !6, !noalias !925, !noundef !7
  %64 = trunc nuw i64 %_3.i.i to i1
  br i1 %64, label %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit.i, label %bb2.i33.i

bb2.i33.i:                                        ; preds = %.noexc57
  %65 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  %s.0.i.i = load ptr, ptr %65, align 8, !noalias !925, !nonnull !7, !align !187, !noundef !7
  %66 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 16
  %s.1.i.i = load i64, ptr %66, align 8, !noalias !925, !noundef !7
  %_6.i.i.i.i = icmp samesign eq i64 %s.1.i.i, 0
  br i1 %_6.i.i.i.i, label %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit.i, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb2.i33.i
  %x.i.i.i = load i8, ptr %s.0.i.i, align 1, !noalias !929, !noundef !7
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %s.0.i.i, i64 1
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp samesign ne i64 %s.1.i.i, 1
  call void @llvm.assume(i1 %_6.i10.i.i.i)
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !noalias !929, !noundef !7
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %67 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb8.i34.i

bb3.i.i.i:                                        ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb8.i34.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %s.0.i.i, i64 2
  %_6.i17.i.i.i = icmp samesign ne i64 %s.1.i.i, 2
  call void @llvm.assume(i1 %_6.i17.i.i.i)
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !noalias !929, !noundef !7
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %68 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i.i, label %bb8.i34.i

bb8.i.i.i:                                        ; preds = %bb6.i.i.i
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %s.0.i.i, i64 3
  %_6.i24.i.i.i = icmp samesign ne i64 %s.1.i.i, 3
  call void @llvm.assume(i1 %_6.i24.i.i.i)
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !noalias !929, !noundef !7
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %69 = or disjoint i32 %_27.i.i.i, %_25.i.i.i
  br label %bb8.i34.i

bb8.i34.i:                                        ; preds = %bb8.i.i.i, %bb6.i.i.i, %bb3.i.i.i, %bb4.i.i.i
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %67, %bb4.i.i.i ], [ %68, %bb6.i.i.i ], [ %69, %bb8.i.i.i ], [ %_7.i.i.i, %bb3.i.i.i ]
  %70 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %70)
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit.i

_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit.i: ; preds = %bb8.i34.i, %bb2.i33.i, %.noexc57
  %_71.sroa.5.0.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i, %bb8.i34.i ], [ undef, %bb2.i33.i ], [ undef, %.noexc57 ]
  %_71.sroa.0.0.i = phi i64 [ -9223372036854775789, %bb8.i34.i ], [ -9223372036854775808, %bb2.i33.i ], [ -9223372036854775808, %.noexc57 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i), !noalias !925
  br label %bb63.i.sink.split

bb63.i.sink.split:                                ; preds = %bb52.i, %bb59.i, %bb58.i, %bb57.i, %bb56.i, %bb55.i, %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit.i, %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit102.i
  %_94.sroa.0.0.i.sink = phi i64 [ %_81.sroa.0.0.i, %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit102.i ], [ %_71.sroa.0.0.i, %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit.i ], [ -9223372036854775802, %bb59.i ], [ -9223372036854775800, %bb58.i ], [ -9223372036854775797, %bb57.i ], [ -9223372036854775799, %bb56.i ], [ -9223372036854775798, %bb55.i ], [ -9223372036854775789, %bb52.i ]
  %_114.i.lcssa.sink = phi i32 [ %_81.sroa.5.0.i, %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit102.i ], [ %_71.sroa.5.0.i, %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit.i ], [ %_114.i, %bb59.i ], [ %_114.i, %bb58.i ], [ %_114.i, %bb57.i ], [ %_114.i, %bb56.i ], [ %_114.i, %bb55.i ], [ %_114.i, %bb52.i ]
  store i64 %_94.sroa.0.0.i.sink, ptr %rv, align 8, !alias.scope !916
  %_94.sroa.10.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store i32 %_114.i.lcssa.sink, ptr %_94.sroa.10.0._0.sroa_idx.i, align 8, !alias.scope !916
  br label %bb63.i

bb63.i:                                           ; preds = %bb63.i.sink.split, %bb106.i
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %buf.i), !noalias !916
  br label %bb6

bb70.i.sink.split:                                ; preds = %bb41.i, %bb49.i, %bb45.i
  %_17.i116.i = tail call noundef ptr @__error() #29
  %_16.i117.i = load i32, ptr %_17.i116.i, align 4, !noalias !916, !noundef !7
  %_21.i118.i = sext i32 %_16.i117.i to i64
  %_20.i119.i = shl nsw i64 %_21.i118.i, 32
  %_19.i120.i = or disjoint i64 %_20.i119.i, 2
  %_26.i121.i = inttoptr i64 %_19.i120.i to ptr
  br label %bb70.i

bb70.i:                                           ; preds = %bb70.i.sink.split, %bb4.i.i.invoke
  %_76.sroa.6124.0.ph.sink.i = phi ptr [ %61, %bb4.i.i.invoke ], [ %_26.i121.i, %bb70.i.sink.split ]
  %71 = getelementptr inbounds nuw i8, ptr %rv, i64 8
  store ptr %_76.sroa.6124.0.ph.sink.i, ptr %71, align 8, !alias.scope !916
  store i64 -9223372036854775787, ptr %rv, align 8, !alias.scope !916
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %buf.i), !noalias !916
  br label %bb6

bb45.i:                                           ; preds = %bb44.i
  %read.i37.i = call noundef i64 @read(i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i, ptr noundef nonnull align 1 %26, i64 noundef 2) #29, !noalias !932
  %_7.i38.i = icmp slt i64 %read.i37.i, 0
  br i1 %_7.i38.i, label %bb70.i.sink.split, label %bb3.i39.i

bb3.i39.i:                                        ; preds = %bb45.i
  %72 = icmp eq i64 %read.i37.i, 0
  br i1 %72, label %bb4.i.i.invoke, label %bb7.i40.i

bb7.i40.i:                                        ; preds = %bb3.i39.i
  %_10.i41.i = load i8, ptr %26, align 1, !alias.scope !935, !noalias !932, !noundef !7
  %73 = icmp eq i8 %_10.i41.i, 3
  br i1 %73, label %bb4.i.i.invoke, label %bb104.i

bb48.i:                                           ; preds = %bb44.i
  %_84.i = and i8 %byte.i, -8
  %74 = icmp eq i8 %_84.i, -16
  br i1 %74, label %bb49.i, label %bb52.i

bb104.i:                                          ; preds = %bb7.i40.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i57.i), !noalias !937
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_2.i57.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.i, i64 noundef 3)
          to label %.noexc60 unwind label %cleanup.loopexit.split-lp

.noexc60:                                         ; preds = %bb104.i
  %_3.i58.i = load i64, ptr %_2.i57.i, align 8, !range !6, !noalias !937, !noundef !7
  %75 = trunc nuw i64 %_3.i58.i to i1
  br i1 %75, label %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit102.i, label %bb2.i59.i

bb2.i59.i:                                        ; preds = %.noexc60
  %76 = getelementptr inbounds nuw i8, ptr %_2.i57.i, i64 8
  %s.0.i60.i = load ptr, ptr %76, align 8, !noalias !937, !nonnull !7, !align !187, !noundef !7
  %77 = getelementptr inbounds nuw i8, ptr %_2.i57.i, i64 16
  %s.1.i61.i = load i64, ptr %77, align 8, !noalias !937, !noundef !7
  %_6.i.i.i62.i = icmp samesign eq i64 %s.1.i61.i, 0
  br i1 %_6.i.i.i62.i, label %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit102.i, label %bb14.i.i63.i

bb14.i.i63.i:                                     ; preds = %bb2.i59.i
  %x.i.i64.i = load i8, ptr %s.0.i60.i, align 1, !noalias !941, !noundef !7
  %_6.i.i65.i = icmp sgt i8 %x.i.i64.i, -1
  br i1 %_6.i.i65.i, label %bb3.i.i99.i, label %bb4.i.i66.i

bb4.i.i66.i:                                      ; preds = %bb14.i.i63.i
  %_16.i.i.i67.i = getelementptr inbounds nuw i8, ptr %s.0.i60.i, i64 1
  %_30.i.i68.i = and i8 %x.i.i64.i, 31
  %init.i.i69.i = zext nneg i8 %_30.i.i68.i to i32
  %_6.i10.i.i70.i = icmp samesign ne i64 %s.1.i61.i, 1
  call void @llvm.assume(i1 %_6.i10.i.i70.i)
  %y.i.i71.i = load i8, ptr %_16.i.i.i67.i, align 1, !noalias !941, !noundef !7
  %_33.i.i72.i = shl nuw nsw i32 %init.i.i69.i, 6
  %_35.i.i73.i = and i8 %y.i.i71.i, 63
  %_34.i.i74.i = zext nneg i8 %_35.i.i73.i to i32
  %78 = or disjoint i32 %_33.i.i72.i, %_34.i.i74.i
  %_13.i.i75.i = icmp samesign ugt i8 %x.i.i64.i, -33
  br i1 %_13.i.i75.i, label %bb6.i.i79.i, label %bb8.i76.i

bb3.i.i99.i:                                      ; preds = %bb14.i.i63.i
  %_7.i.i100.i = zext nneg i8 %x.i.i64.i to i32
  br label %bb8.i76.i

bb6.i.i79.i:                                      ; preds = %bb4.i.i66.i
  %_16.i12.i.i80.i = getelementptr inbounds nuw i8, ptr %s.0.i60.i, i64 2
  %_6.i17.i.i81.i = icmp samesign ne i64 %s.1.i61.i, 2
  call void @llvm.assume(i1 %_6.i17.i.i81.i)
  %z.i.i82.i = load i8, ptr %_16.i12.i.i80.i, align 1, !noalias !941, !noundef !7
  %_38.i.i83.i = shl nuw nsw i32 %_34.i.i74.i, 6
  %_40.i.i84.i = and i8 %z.i.i82.i, 63
  %_39.i.i85.i = zext nneg i8 %_40.i.i84.i to i32
  %y_z.i.i86.i = or disjoint i32 %_38.i.i83.i, %_39.i.i85.i
  %_20.i.i87.i = shl nuw nsw i32 %init.i.i69.i, 12
  %79 = or disjoint i32 %y_z.i.i86.i, %_20.i.i87.i
  %_21.i.i88.i = icmp samesign ugt i8 %x.i.i64.i, -17
  br i1 %_21.i.i88.i, label %bb8.i.i89.i, label %bb8.i76.i

bb8.i.i89.i:                                      ; preds = %bb6.i.i79.i
  %_16.i19.i.i90.i = getelementptr inbounds nuw i8, ptr %s.0.i60.i, i64 3
  %_6.i24.i.i91.i = icmp samesign ne i64 %s.1.i61.i, 3
  call void @llvm.assume(i1 %_6.i24.i.i91.i)
  %w.i.i92.i = load i8, ptr %_16.i19.i.i90.i, align 1, !noalias !941, !noundef !7
  %_26.i.i93.i = shl nuw nsw i32 %init.i.i69.i, 18
  %_25.i.i94.i = and i32 %_26.i.i93.i, 1835008
  %_43.i.i95.i = shl nuw nsw i32 %y_z.i.i86.i, 6
  %_45.i.i96.i = and i8 %w.i.i92.i, 63
  %_44.i.i97.i = zext nneg i8 %_45.i.i96.i to i32
  %_27.i.i98.i = or disjoint i32 %_43.i.i95.i, %_44.i.i97.i
  %80 = or disjoint i32 %_27.i.i98.i, %_25.i.i94.i
  br label %bb8.i76.i

bb8.i76.i:                                        ; preds = %bb8.i.i89.i, %bb6.i.i79.i, %bb3.i.i99.i, %bb4.i.i66.i
  %_0.sroa.4.0.i.ph.i77.i = phi i32 [ %78, %bb4.i.i66.i ], [ %79, %bb6.i.i79.i ], [ %80, %bb8.i.i89.i ], [ %_7.i.i100.i, %bb3.i.i99.i ]
  %81 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i77.i, 1114112
  call void @llvm.assume(i1 %81)
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit102.i

_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8.exit102.i: ; preds = %bb8.i76.i, %bb2.i59.i, %.noexc60
  %_81.sroa.0.0.i = phi i64 [ -9223372036854775789, %bb8.i76.i ], [ -9223372036854775808, %bb2.i59.i ], [ -9223372036854775808, %.noexc60 ]
  %_81.sroa.5.0.i = phi i32 [ %_0.sroa.4.0.i.ph.i77.i, %bb8.i76.i ], [ undef, %bb2.i59.i ], [ undef, %.noexc60 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i57.i), !noalias !937
  br label %bb63.i.sink.split

bb49.i:                                           ; preds = %bb48.i
  %read.i103.i = call noundef i64 @read(i32 noundef range(i32 0, -1) %_2.sroa.0.0.i.i, ptr noundef nonnull align 1 %26, i64 noundef 3) #29, !noalias !944
  %_7.i104.i = icmp slt i64 %read.i103.i, 0
  br i1 %_7.i104.i, label %bb70.i.sink.split, label %bb3.i105.i

bb3.i105.i:                                       ; preds = %bb49.i
  %82 = icmp eq i64 %read.i103.i, 0
  br i1 %82, label %bb4.i.i.invoke, label %bb7.i106.i

bb7.i106.i:                                       ; preds = %bb3.i105.i
  %_10.i107.i = load i8, ptr %26, align 1, !alias.scope !947, !noalias !944, !noundef !7
  %83 = icmp eq i8 %_10.i107.i, 3
  br i1 %83, label %bb4.i.i.invoke, label %bb106.i

bb52.i:                                           ; preds = %bb48.i
  switch i32 %_114.i, label %bb63.i.sink.split [
    i32 10, label %bb59.i
    i32 13, label %bb59.i
    i32 127, label %bb58.i
    i32 9, label %bb57.i
    i32 1, label %bb56.i
    i32 5, label %bb55.i
    i32 8, label %bb58.i
  ]

bb106.i:                                          ; preds = %bb7.i106.i
; invoke console::unix_term::key_from_utf8
  invoke fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(24) %rv, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.i, i64 noundef 4)
          to label %bb63.i unwind label %cleanup.loopexit.split-lp

bb59.i:                                           ; preds = %bb52.i, %bb52.i
  br label %bb63.i.sink.split

bb58.i:                                           ; preds = %bb52.i, %bb52.i
  br label %bb63.i.sink.split

bb57.i:                                           ; preds = %bb52.i
  br label %bb63.i.sink.split

bb56.i:                                           ; preds = %bb52.i
  br label %bb63.i.sink.split

bb55.i:                                           ; preds = %bb52.i
  br label %bb63.i.sink.split

bb6:                                              ; preds = %bb70.i, %bb63.i, %bb99.i, %bb76.i, %bb85.i, %bb96.i, %bb93.i, %bb90.i, %bb27.i, %bb28.i, %bb29.i, %bb30.i, %bb31.i, %bb32.i, %bb33.i, %bb34.i, %bb16.i, %bb17.i, %bb18.i, %bb19.i, %bb20.i, %bb21.i, %bb22.i, %bb9.i43, %.noexc44, %bb78.i
  %_0.i.i65 = call noundef i32 @tcsetattr(i32 noundef %_2.sroa.0.0.i.i, i32 noundef 1, ptr noundef nonnull readonly align 8 dereferenceable(72) %original) #29
  %84 = icmp eq i32 %_0.i.i65, 0
  br i1 %84, label %bb37, label %bb36

bb36:                                             ; preds = %bb6
  %_5.i67 = tail call noundef ptr @__error() #29
  %_4.i68 = load i32, ptr %_5.i67, align 4, !noundef !7
  %_9.i69 = sext i32 %_4.i68 to i64
  %_8.i70 = shl nsw i64 %_9.i69, 32
  %_7.i71 = or disjoint i64 %_8.i70, 2
  %_14.i72 = inttoptr i64 %_7.i71 to ptr
  %85 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_14.i72, ptr %85, align 8
  %rv.val25.pre = load i64, ptr %rv, align 8, !range !949
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %rv, i64 8
  %rv.val26.pre = load ptr, ptr %.phi.trans.insert, align 8
  br label %bb18

bb37:                                             ; preds = %bb6
  %86 = load i64, ptr %rv, align 8, !range !949, !noundef !7
  %87 = icmp eq i64 %86, -9223372036854775787
  br i1 %87, label %bb8, label %bb16

bb8:                                              ; preds = %bb37
  %err = getelementptr inbounds nuw i8, ptr %rv, i64 8
  %err.val = load ptr, ptr %err, align 8, !nonnull !7, !noundef !7
; call <std::io::error::Error>::kind
  %_36 = call fastcc noundef i8 @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4kind(ptr nonnull %err.val)
  %88 = icmp eq i8 %_36, 35
  br i1 %88, label %bb10, label %bb16

bb16:                                             ; preds = %bb11, %bb8, %bb37
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %rv, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %rv)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %original)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %termios1)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %termios)
  br i1 %.not.not.not, label %bb2.i74, label %bb23

bb2.i74:                                          ; preds = %bb16
  %_5.i.i.i.i.i.i75 = call noundef i32 @close(i32 noundef %_4.sroa.7.0.ph) #29
  br label %bb23

bb10:                                             ; preds = %bb8
  br i1 %ctrlc_key, label %bb18, label %bb11

bb11:                                             ; preds = %bb10
  %_37 = call noundef i32 @raise(i32 noundef 2) #29
  br label %bb16

bb18:                                             ; preds = %bb10, %bb36
  %rv.val26 = phi ptr [ %rv.val26.pre, %bb36 ], [ %err.val, %bb10 ]
  %rv.val25 = phi i64 [ %rv.val25.pre, %bb36 ], [ -9223372036854775787, %bb10 ]
  %storemerge = phi i64 [ -9223372036854775787, %bb36 ], [ -9223372036854775788, %bb10 ]
  store i64 %storemerge, ptr %_0, align 8
; invoke core::ptr::drop_in_place::<core::result::Result<console::kb::Key, std::io::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsC3JuwEIQwb_7console2kb3KeyNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEEB16_(i64 %rv.val25, ptr %rv.val26)
          to label %bb19 unwind label %cleanup.loopexit.split-lp

bb23:                                             ; preds = %bb27, %bb21, %bb2.i77, %bb2.i74, %bb16
  ret void

bb19:                                             ; preds = %bb18
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %rv)
  br label %bb20

bb20:                                             ; preds = %bb31, %bb19
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %original)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %termios1)
  br label %bb21

bb21:                                             ; preds = %bb29, %bb20
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %termios)
  br i1 %.not.not.not, label %bb2.i77, label %bb23

bb2.i77:                                          ; preds = %bb21
  %_5.i.i.i.i.i.i78 = call noundef i32 @close(i32 noundef %_4.sroa.7.0.ph) #29
  br label %bb23

bb26:                                             ; preds = %bb2.i30, %cleanup
  resume { ptr, i32 } %lpad.phi
}

; console::unix_term::read_single_char
; Function Attrs: uwtable
define internal fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term16read_single_char(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(16) initializes((0, 4)) %_0, i32 noundef range(i32 0, -1) %fd) unnamed_addr #1 {
start:
  %buf = alloca [1 x i8], align 1
  %_3 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_3)
; call console::unix_term::select_or_poll_term_fd
  call fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term22select_or_poll_term_fd(ptr noalias noundef align 8 captures(address) dereferenceable(16) %_3, i32 noundef %fd, i32 noundef 0)
  %0 = load i8, ptr %_3, align 8, !range !29, !noundef !7
  %1 = trunc nuw i8 %0 to i1
  br i1 %1, label %bb9, label %bb10

bb9:                                              ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_18 = load ptr, ptr %2, align 8, !nonnull !7, !noundef !7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3)
  %3 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_18, ptr %3, align 8
  store i32 1, ptr %_0, align 8
  br label %bb8

bb10:                                             ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %_3, i64 1
  %5 = load i8, ptr %4, align 1, !range !29, !noundef !7
  %_17 = trunc nuw i8 %5 to i1
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3)
  br i1 %_17, label %bb3, label %bb5

bb5:                                              ; preds = %bb10
  store <2 x i32> <i32 0, i32 1114112>, ptr %_0, align 8
  br label %bb8

bb3:                                              ; preds = %bb10
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %buf)
  store i8 0, ptr %buf, align 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !950)
  %read.i = call noundef i64 @read(i32 noundef range(i32 0, -1) %fd, ptr noundef nonnull align 1 %buf, i64 noundef 1) #29, !noalias !953
  %_7.i = icmp slt i64 %read.i, 0
  br i1 %_7.i, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %bb3
  %6 = icmp eq i64 %read.i, 0
  br i1 %6, label %bb4.i, label %bb7.i

bb2.i:                                            ; preds = %bb3
  %_17.i = tail call noundef ptr @__error() #29
  %_16.i = load i32, ptr %_17.i, align 4, !noalias !955, !noundef !7
  %_21.i = sext i32 %_16.i to i64
  %_20.i = shl nsw i64 %_21.i, 32
  %_19.i = or disjoint i64 %_20.i, 2
  %_26.i = inttoptr i64 %_19.i to ptr
  br label %bb11

bb4.i:                                            ; preds = %bb3.i
; call <std::io::error::Error>::new::<&str>
  %_9.i = tail call noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newReEBa_(i8 noundef 37, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_7ef65f9252ea7966a1063c1ab22af5b9, i64 noundef 19) #33, !noalias !955
  br label %bb11

bb7.i:                                            ; preds = %bb3.i
  %_10.i = load i8, ptr %buf, align 1, !alias.scope !950, !noalias !953, !noundef !7
  %7 = icmp eq i8 %_10.i, 3
  br i1 %7, label %bb8.i, label %bb12

bb8.i:                                            ; preds = %bb7.i
; call <std::io::error::Error>::new::<&str>
  %_13.i = tail call noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newReEBa_(i8 noundef 35, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_50f3f282355a7f8c5c8e6d622c64bbaf, i64 noundef 16) #33, !noalias !955
  br label %bb11

bb11:                                             ; preds = %bb8.i, %bb4.i, %bb2.i
  %_8.sroa.64.0.ph = phi ptr [ %_13.i, %bb8.i ], [ %_9.i, %bb4.i ], [ %_26.i, %bb2.i ]
  %8 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_8.sroa.64.0.ph, ptr %8, align 8
  store i32 1, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 1, ptr nonnull %buf)
  br label %bb8

bb12:                                             ; preds = %bb7.i
  %_14 = zext i8 %_10.i to i32
  %9 = getelementptr inbounds nuw i8, ptr %_0, i64 4
  store i32 %_14, ptr %9, align 4
  store i32 0, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 1, ptr nonnull %buf)
  br label %bb8

bb8:                                              ; preds = %bb11, %bb9, %bb5, %bb12
  ret void
}

; console::unix_term::is_a_color_terminal
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvNtCsC3JuwEIQwb_7console9unix_term19is_a_color_terminal(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %out) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_3 = alloca [32 x i8], align 8
  %_2 = alloca [32 x i8], align 8
; call <console::term::Term as std::os::fd::raw::AsRawFd>::as_raw_fd
  %_10 = tail call noundef i32 @_RNvXs2_NtCsC3JuwEIQwb_7console4termNtB5_4TermNtNtNtNtCs5sEH5CPMdak_3std2os2fd3raw7AsRawFd9as_raw_fd(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %out)
  %_9 = tail call noundef i32 @isatty(i32 noundef %_10) #29
  %0 = icmp eq i32 %_9, 0
  br i1 %0, label %bb13, label %bb1

bb1:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_2)
; call std::env::_var
  call void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_2, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9e8821ebf06809bdf585485e81a043ea, i64 noundef 8)
  %_11 = load i64, ptr %_2, align 8, !range !6, !noundef !7
  %1 = icmp eq i64 %_11, 0
  %2 = getelementptr inbounds nuw i8, ptr %_2, i64 8
  br i1 %1, label %bb2.i, label %bb3.i9

bb13:                                             ; preds = %start, %bb16, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit
  %_0.sroa.0.0.off0 = phi i1 [ false, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit ], [ %_0.sroa.0.1.off025, %bb16 ], [ false, %start ]
  ret i1 %_0.sroa.0.0.off0

bb2.i:                                            ; preds = %bb1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !956)
  %.val.i = load i64, ptr %2, align 8, !alias.scope !956
  %3 = icmp eq i64 %.val.i, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit, label %bb1.sink.split.i

bb1.sink.split.i:                                 ; preds = %bb2.i
  %4 = getelementptr inbounds nuw i8, ptr %_2, i64 16
  %.val3.i = load ptr, ptr %4, align 8, !alias.scope !956, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i, i64 noundef %.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !956
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit: ; preds = %bb2.i, %bb1.sink.split.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_2)
  br label %bb13

bb3.i9:                                           ; preds = %bb1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !959)
  %.val.i8 = load i64, ptr %2, align 8, !alias.scope !959
  switch i64 %.val.i8, label %bb1.sink.split.i10 [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit13
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit13
  ]

bb1.sink.split.i10:                               ; preds = %bb3.i9
  %5 = getelementptr inbounds nuw i8, ptr %_2, i64 16
  %.val3.i11 = load ptr, ptr %5, align 8, !alias.scope !959, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i11, i64 noundef %.val.i8, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !959
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit13

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit13: ; preds = %bb3.i9, %bb3.i9, %bb1.sink.split.i10
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_2)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3)
; call std::env::_var
  call void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c940f1872184b67533cde325d4eb7ceb, i64 noundef 4)
  %_4 = load i64, ptr %_3, align 8, !range !6, !noundef !7
  %6 = trunc nuw i64 %_4 to i1
  br i1 %6, label %bb3.i19, label %bb11

bb11:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit13
  %7 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %term.sroa.0.0.copyload = load i64, ptr %7, align 8
  %term.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %term.sroa.5.0.copyload = load ptr, ptr %term.sroa.5.0..sroa_idx, align 8, !nonnull !7, !noundef !7
  %term.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 24
  %term.sroa.8.0.copyload = load i64, ptr %term.sroa.8.0..sroa_idx, align 8
  %_3.not.i = icmp eq i64 %term.sroa.8.0.copyload, 4
  br i1 %_3.not.i, label %bb2.i14, label %bb23

bb2.i14:                                          ; preds = %bb11
  %8 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) %term.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(4) @alloc_0cb071d804372b212169612c3e5a40ce, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !962
  %9 = icmp ne i32 %8, 0
  br label %bb23

bb23:                                             ; preds = %bb2.i14, %bb11
  %_0.sroa.0.0.off0.i = phi i1 [ %9, %bb2.i14 ], [ true, %bb11 ]
  %10 = icmp eq i64 %term.sroa.0.0.copyload, 0
  br i1 %10, label %bb16, label %bb2.i.i.i4.i.i15

bb2.i.i.i4.i.i15:                                 ; preds = %bb23
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %term.sroa.5.0.copyload, i64 noundef %term.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #29
  br label %bb16

bb3.i19:                                          ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console.exit13
  tail call void @llvm.experimental.noalias.scope.decl(metadata !966)
  %11 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %.val.i18 = load i64, ptr %11, align 8, !alias.scope !966
  switch i64 %.val.i18, label %bb1.sink.split.i20 [
    i64 -9223372036854775808, label %bb16
    i64 0, label %bb16
  ]

bb1.sink.split.i20:                               ; preds = %bb3.i19
  %12 = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %.val3.i21 = load ptr, ptr %12, align 8, !alias.scope !966, !nonnull !7, !noundef !7
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i21, i64 noundef %.val.i18, i64 noundef range(i64 1, -9223372036854775807) 1) #29, !noalias !966
  br label %bb16

bb16:                                             ; preds = %bb2.i.i.i4.i.i15, %bb23, %bb1.sink.split.i20, %bb3.i19, %bb3.i19
  %_0.sroa.0.1.off025 = phi i1 [ false, %bb3.i19 ], [ false, %bb3.i19 ], [ false, %bb1.sink.split.i20 ], [ %_0.sroa.0.0.off0.i, %bb23 ], [ %_0.sroa.0.0.off0.i, %bb2.i.i.i4.i.i15 ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3)
  br label %bb13
}

; console::unix_term::select_or_poll_term_fd
; Function Attrs: nounwind uwtable
define internal fastcc void @_RNvNtCsC3JuwEIQwb_7console9unix_term22select_or_poll_term_fd(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(16) %_0, i32 noundef range(i32 0, -1) %fd, i32 noundef range(i32 -1, 1) %timeout) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %pollfd.i = alloca [8 x i8], align 4
  %timeout_val.i = alloca [16 x i8], align 8
  %read_fd_set.i = alloca [128 x i8], align 4
  %_3 = tail call noundef i32 @isatty(i32 noundef %fd) #29
  %0 = icmp eq i32 %_3, 1
  br i1 %0, label %bb2, label %bb4

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !969)
  call void @llvm.lifetime.start.p0(i64 128, ptr nonnull %read_fd_set.i), !noalias !969
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %timeout_val.i), !noalias !969
  %_6.i = icmp slt i32 %timeout, 0
  br i1 %_6.i, label %bb3.i, label %bb2.i

bb2.i:                                            ; preds = %bb2
  store i64 0, ptr %timeout_val.i, align 8, !noalias !969
  %1 = getelementptr inbounds nuw i8, ptr %timeout_val.i, i64 8
  store i32 0, ptr %1, align 8, !noalias !969
  br label %bb3.i

bb3.i:                                            ; preds = %bb2.i, %bb2
  %timeout1.sroa.0.0.i = phi ptr [ %timeout_val.i, %bb2.i ], [ null, %bb2 ]
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 4 dereferenceable(128) %read_fd_set.i, i8 0, i64 128, i1 false), !noalias !969
  %fd1.i.i = sext i32 %fd to i64
  %_103.i.i = lshr i64 %fd1.i.i, 5
  %_11.i.i = icmp ult i32 %fd, 1024
  br i1 %_11.i.i, label %_RNvNtNtCshxypWE0Cazg_4libc4unix3bsd6FD_SET.exit.i, label %panic2.i.i

terminate.i.i:                                    ; preds = %panic2.i.i
  %2 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_cannot_unwind
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking19panic_cannot_unwind() #31, !noalias !969
  unreachable

unreachable.i.i:                                  ; preds = %panic2.i.i
  unreachable

panic2.i.i:                                       ; preds = %bb3.i
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %_103.i.i, i64 noundef 32, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e5974023cf134bdbad09697d09348ac9) #30
          to label %unreachable.i.i unwind label %terminate.i.i, !noalias !969

_RNvNtNtCshxypWE0Cazg_4libc4unix3bsd6FD_SET.exit.i: ; preds = %bb3.i
  %_8.i.i = and i32 %fd, 31
  %_7.i.i = shl nuw i32 1, %_8.i.i
  %3 = getelementptr inbounds nuw i32, ptr %read_fd_set.i, i64 %_103.i.i
  %4 = load i32, ptr %3, align 4, !noalias !969, !noundef !7
  %5 = or i32 %4, %_7.i.i
  store i32 %5, ptr %3, align 4, !noalias !969
  %_16.i = add nuw nsw i32 %fd, 1
  %ret.i = call noundef i32 @select(i32 noundef %_16.i, ptr noundef nonnull %read_fd_set.i, ptr noundef null, ptr noundef null, ptr noundef %timeout1.sroa.0.0.i) #29, !noalias !969
  %_18.i = icmp slt i32 %ret.i, 0
  br i1 %_18.i, label %bb7.i, label %_RNvNtNtCshxypWE0Cazg_4libc4unix3bsd8FD_ISSET.exit.i

_RNvNtNtCshxypWE0Cazg_4libc4unix3bsd8FD_ISSET.exit.i: ; preds = %_RNvNtNtCshxypWE0Cazg_4libc4unix3bsd6FD_SET.exit.i
  %_8.i8.i = load i32, ptr %3, align 4, !noalias !969, !noundef !7
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 1
  %7 = lshr i32 %_8.i8.i, %_8.i.i
  %8 = trunc i32 %7 to i8
  %9 = and i8 %8, 1
  store i8 %9, ptr %6, align 1, !alias.scope !969
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term9select_fd.exit

bb7.i:                                            ; preds = %_RNvNtNtCshxypWE0Cazg_4libc4unix3bsd6FD_SET.exit.i
  %_30.i = tail call noundef ptr @__error() #29
  %_29.i = load i32, ptr %_30.i, align 4, !noalias !969, !noundef !7
  %_34.i = sext i32 %_29.i to i64
  %_33.i = shl nsw i64 %_34.i, 32
  %_32.i = or disjoint i64 %_33.i, 2
  %_39.i = inttoptr i64 %_32.i to ptr
  %10 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_39.i, ptr %10, align 8, !alias.scope !969
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term9select_fd.exit

_RNvNtCsC3JuwEIQwb_7console9unix_term9select_fd.exit: ; preds = %_RNvNtNtCshxypWE0Cazg_4libc4unix3bsd8FD_ISSET.exit.i, %bb7.i
  %storemerge.i = phi i8 [ 0, %_RNvNtNtCshxypWE0Cazg_4libc4unix3bsd8FD_ISSET.exit.i ], [ 1, %bb7.i ]
  store i8 %storemerge.i, ptr %_0, align 8, !alias.scope !969
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %timeout_val.i), !noalias !969
  call void @llvm.lifetime.end.p0(i64 128, ptr nonnull %read_fd_set.i), !noalias !969
  br label %bb5

bb4:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !972)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %pollfd.i), !noalias !972
  store i32 %fd, ptr %pollfd.i, align 4, !noalias !972
  %11 = getelementptr inbounds nuw i8, ptr %pollfd.i, i64 4
  store i16 1, ptr %11, align 4, !noalias !972
  %12 = getelementptr inbounds nuw i8, ptr %pollfd.i, i64 6
  store i16 0, ptr %12, align 2, !noalias !972
  %ret.i1 = call noundef i32 @poll(ptr noundef nonnull %pollfd.i, i32 noundef 1, i32 noundef range(i32 -1, 1) %timeout) #29, !noalias !972
  %_6.i2 = icmp slt i32 %ret.i1, 0
  br i1 %_6.i2, label %bb2.i5, label %bb3.i3

bb3.i3:                                           ; preds = %bb4
  %_10.i = load i16, ptr %12, align 2, !noalias !972, !noundef !7
  %13 = getelementptr inbounds nuw i8, ptr %_0, i64 1
  %14 = trunc i16 %_10.i to i8
  %15 = and i8 %14, 1
  store i8 %15, ptr %13, align 1, !alias.scope !972
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term7poll_fd.exit

bb2.i5:                                           ; preds = %bb4
  %_12.i = tail call noundef ptr @__error() #29
  %_11.i = load i32, ptr %_12.i, align 4, !noalias !972, !noundef !7
  %_16.i6 = sext i32 %_11.i to i64
  %_15.i = shl nsw i64 %_16.i6, 32
  %_14.i = or disjoint i64 %_15.i, 2
  %_21.i = inttoptr i64 %_14.i to ptr
  %16 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_21.i, ptr %16, align 8, !alias.scope !972
  br label %_RNvNtCsC3JuwEIQwb_7console9unix_term7poll_fd.exit

_RNvNtCsC3JuwEIQwb_7console9unix_term7poll_fd.exit: ; preds = %bb3.i3, %bb2.i5
  %storemerge.i4 = phi i8 [ 0, %bb3.i3 ], [ 1, %bb2.i5 ]
  store i8 %storemerge.i4, ptr %_0, align 8, !alias.scope !972
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %pollfd.i), !noalias !972
  br label %bb5

bb5:                                              ; preds = %_RNvNtCsC3JuwEIQwb_7console9unix_term7poll_fd.exit, %_RNvNtCsC3JuwEIQwb_7console9unix_term9select_fd.exit
  ret void
}

; unicode_width::tables::lookup_width
; Function Attrs: inlinehint uwtable
define internal fastcc { i8, i16 } @_RNvNtCsDJOD2kcAir_13unicode_width6tables12lookup_width(i32 noundef range(i32 0, 1114112) %c) unnamed_addr #3 {
start:
  %cp = zext nneg i32 %c to i64
  %_5 = lshr i64 %cp, 13
  %0 = getelementptr inbounds nuw i8, ptr @_RNvNtCsDJOD2kcAir_13unicode_width6tables10WIDTH_ROOT, i64 %_5
  %t1_offset = load i8, ptr %0, align 1, !noundef !7
  %_9 = zext i8 %t1_offset to i64
  %_10 = icmp ult i8 %t1_offset, 20
  br i1 %_10, label %bb3, label %panic1

panic1:                                           ; preds = %start
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %_9, i64 noundef 20, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3c235b06addb9c2230a3beb294b85791) #34
  unreachable

bb3:                                              ; preds = %start
  %_12 = lshr i64 %cp, 7
  %_11 = and i64 %_12, 63
  %1 = getelementptr inbounds nuw [64 x i8], ptr @_RNvNtCsDJOD2kcAir_13unicode_width6tables12WIDTH_MIDDLE, i64 %_9
  %2 = getelementptr inbounds nuw i8, ptr %1, i64 %_11
  %t2_offset = load i8, ptr %2, align 1, !noundef !7
  %_16 = zext i8 %t2_offset to i64
  %_17 = icmp ult i8 %t2_offset, -70
  br i1 %_17, label %bb5, label %panic3

panic3:                                           ; preds = %bb3
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %_16, i64 noundef 186, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_54facc433374646a466a7fffff6dc929) #34
  unreachable

bb5:                                              ; preds = %bb3
  %_19 = lshr i64 %cp, 2
  %_18 = and i64 %_19, 31
  %3 = getelementptr inbounds nuw [32 x i8], ptr @_RNvNtCsDJOD2kcAir_13unicode_width6tables12WIDTH_LEAVES, i64 %_16
  %4 = getelementptr inbounds nuw i8, ptr %3, i64 %_18
  %packed_widths = load i8, ptr %4, align 1, !noundef !7
  %cp.tr = trunc i32 %c to i8
  %5 = shl i8 %cp.tr, 1
  %6 = and i8 %5, 6
  %_22 = lshr i8 %packed_widths, %6
  %width = and i8 %_22, 3
  %_25.not = icmp eq i8 %width, 3
  br i1 %_25.not, label %bb7, label %bb37

bb7:                                              ; preds = %bb5
  switch i32 %c, label %bb9 [
    i32 10, label %bb37
    i32 1500, label %bb35
    i32 6104, label %bb34
    i32 6672, label %bb33
    i32 65025, label %bb32
    i32 65038, label %bb31
    i32 65039, label %bb30
    i32 68611, label %bb29
    i32 93543, label %bb28
    i32 93544, label %bb27
  ]

bb9:                                              ; preds = %bb7
  %7 = add nsw i32 %c, -1570
  %or.cond = icmp ult i32 %7, 609
  br i1 %or.cond, label %bb37, label %bb11

bb35:                                             ; preds = %bb7
  br label %bb37

bb34:                                             ; preds = %bb7
  br label %bb37

bb33:                                             ; preds = %bb7
  br label %bb37

bb32:                                             ; preds = %bb7
  br label %bb37

bb31:                                             ; preds = %bb7
  br label %bb37

bb30:                                             ; preds = %bb7
  br label %bb37

bb29:                                             ; preds = %bb7
  br label %bb37

bb28:                                             ; preds = %bb7
  br label %bb37

bb27:                                             ; preds = %bb7
  br label %bb37

bb37:                                             ; preds = %bb17, %bb19, %bb15, %bb13, %bb11, %bb9, %bb7, %bb5, %bb27, %bb28, %bb29, %bb30, %bb31, %bb32, %bb33, %bb34, %bb35
  %_0.sroa.19.0 = phi i16 [ 14336, %bb35 ], [ 0, %bb34 ], [ 14337, %bb33 ], [ 512, %bb32 ], [ 16384, %bb31 ], [ -32768, %bb30 ], [ 14342, %bb29 ], [ 32, %bb28 ], [ 33, %bb27 ], [ 0, %bb5 ], [ 1, %bb7 ], [ 12543, %bb9 ], [ 15367, %bb11 ], [ 14339, %bb13 ], [ 15365, %bb15 ], [ %., %bb19 ], [ 3, %bb17 ]
  %_0.sroa.0.0 = phi i8 [ 1, %bb35 ], [ 3, %bb34 ], [ 1, %bb33 ], [ 0, %bb32 ], [ 0, %bb31 ], [ 0, %bb30 ], [ 1, %bb29 ], [ 1, %bb28 ], [ 1, %bb27 ], [ %width, %bb5 ], [ 1, %bb7 ], [ 1, %bb9 ], [ 1, %bb11 ], [ 1, %bb13 ], [ 1, %bb15 ], [ 2, %bb19 ], [ 1, %bb17 ]
  %8 = insertvalue { i8, i16 } poison, i8 %_0.sroa.0.0, 0
  %9 = insertvalue { i8, i16 } %8, i16 %_0.sroa.19.0, 1
  ret { i8, i16 } %9

bb11:                                             ; preds = %bb9
  %10 = add nsw i32 %c, -6016
  %or.cond5 = icmp ult i32 %10, 48
  br i1 %or.cond5, label %bb37, label %bb13

bb13:                                             ; preds = %bb11
  %11 = add nsw i32 %c, -11569
  %or.cond6 = icmp ult i32 %11, 63
  br i1 %or.cond6, label %bb37, label %bb15

bb15:                                             ; preds = %bb13
  %12 = and i32 %c, 2097150
  %or.cond7 = icmp eq i32 %12, 42236
  br i1 %or.cond7, label %bb37, label %bb17

bb17:                                             ; preds = %bb15
  %13 = add nsw i32 %c, -127462
  %or.cond8 = icmp ult i32 %13, 26
  br i1 %or.cond8, label %bb37, label %bb19

bb19:                                             ; preds = %bb17
  %14 = add nsw i32 %c, -127995
  %or.cond9 = icmp ult i32 %14, 5
  %. = select i1 %or.cond9, i16 2, i16 5
  br label %bb37
}

; unicode_width::tables::is_emoji_modifier_base
; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: write) uwtable
define internal fastcc noundef zeroext i1 @_RNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base(i32 noundef range(i32 161, 1114112) %c) unnamed_addr #10 personality ptr @rust_eh_personality {
start:
  %top_bits = lshr i32 %c, 8
  switch i32 %top_bits, label %bb12 [
    i32 38, label %bb10
    i32 39, label %bb10.thread
    i32 499, label %bb7
    i32 500, label %bb6
    i32 501, label %bb5
    i32 502, label %bb4
    i32 505, label %bb3
    i32 506, label %bb2
  ]

bb10.thread:                                      ; preds = %start
  %0 = trunc i32 %c to i8
  br label %bb7.i

bb7:                                              ; preds = %start
  br label %bb10

bb6:                                              ; preds = %start
  br label %bb10

bb5:                                              ; preds = %start
  br label %bb10

bb4:                                              ; preds = %start
  br label %bb10

bb3:                                              ; preds = %start
  br label %bb10

bb2:                                              ; preds = %start
  br label %bb10

bb10:                                             ; preds = %start, %bb2, %bb3, %bb4, %bb5, %bb6, %bb7
  %leaf.sroa.9.0 = phi i64 [ 4, %bb7 ], [ 9, %bb6 ], [ 4, %bb5 ], [ 6, %bb4 ], [ 12, %bb3 ], [ 2, %bb2 ], [ 2, %start ]
  %leaf.sroa.0.0 = phi ptr [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_2, %bb7 ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_3, %bb6 ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_4, %bb5 ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_5, %bb4 ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_6, %bb3 ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_7, %bb2 ], [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_0, %start ]
  %1 = trunc i32 %c to i8
  br label %bb4.i

bb7.i:                                            ; preds = %bb4.i, %bb10.thread
  %2 = phi i8 [ %0, %bb10.thread ], [ %1, %bb4.i ]
  %leaf.sroa.0.05 = phi ptr [ @_RNvNtCsDJOD2kcAir_13unicode_width6tables21EMOJI_MODIFIER_LEAF_1, %bb10.thread ], [ %leaf.sroa.0.0, %bb4.i ]
  %leaf.sroa.9.04 = phi i64 [ 1, %bb10.thread ], [ %leaf.sroa.9.0, %bb4.i ]
  %base.sroa.0.0.lcssa.i = phi i64 [ 0, %bb10.thread ], [ %5, %bb4.i ]
  %_41.i = icmp ult i64 %base.sroa.0.0.lcssa.i, %leaf.sroa.9.04
  tail call void @llvm.assume(i1 %_41.i)
  %_38.i = getelementptr inbounds nuw { i8, i8 }, ptr %leaf.sroa.0.05, i64 %base.sroa.0.0.lcssa.i
  %_38.val.i = load i8, ptr %_38.i, align 1, !alias.scope !975, !noalias !978, !noundef !7
  %3 = getelementptr i8, ptr %_38.i, i64 1
  %_38.val12.i = load i8, ptr %3, align 1, !alias.scope !975, !noalias !978
  %_5.i.i = icmp uge i8 %2, %_38.val.i
  %_7.i.i = icmp ugt i8 %2, %_38.val12.i
  %not._5.i.i = xor i1 %_5.i.i, true
  %4 = select i1 %not._5.i.i, i1 true, i1 %_7.i.i
  br i1 %4, label %bb10.i, label %_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base0ECsC3JuwEIQwb_7console.exit

bb4.i:                                            ; preds = %bb10, %bb4.i
  %size.sroa.0.022.i = phi i64 [ %6, %bb4.i ], [ %leaf.sroa.9.0, %bb10 ]
  %base.sroa.0.021.i = phi i64 [ %5, %bb4.i ], [ 0, %bb10 ]
  %half11.i = lshr i64 %size.sroa.0.022.i, 1
  %mid.i = add i64 %half11.i, %base.sroa.0.021.i
  %_35.i = icmp ult i64 %mid.i, %leaf.sroa.9.0
  tail call void @llvm.assume(i1 %_35.i)
  %_32.i = getelementptr inbounds nuw { i8, i8 }, ptr %leaf.sroa.0.0, i64 %mid.i
  %_32.val.i = load i8, ptr %_32.i, align 1, !alias.scope !975, !noalias !978, !noundef !7
  %_5.i16.i = icmp ugt i8 %_32.val.i, %1
  %5 = select i1 %_5.i16.i, i64 %base.sroa.0.021.i, i64 %mid.i, !unpredictable !7
  %6 = sub i64 %size.sroa.0.022.i, %half11.i
  %_6.i = icmp ugt i64 %6, 1
  br i1 %_6.i, label %bb4.i, label %bb7.i

bb10.i:                                           ; preds = %bb7.i
  %_29.i = select i1 %_5.i.i, i1 %_7.i.i, i1 false
  %_28.i = zext i1 %_29.i to i64
  %result.i = add nuw nsw i64 %base.sroa.0.0.lcssa.i, %_28.i
  %cond1.i = icmp ule i64 %result.i, %leaf.sroa.9.04
  tail call void @llvm.assume(i1 %cond1.i)
  br label %_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base0ECsC3JuwEIQwb_7console.exit

_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base0ECsC3JuwEIQwb_7console.exit: ; preds = %bb7.i, %bb10.i
  %7 = xor i1 %4, true
  br label %bb12

bb12:                                             ; preds = %start, %_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base0ECsC3JuwEIQwb_7console.exit
  %_0.sroa.0.0.off0 = phi i1 [ %7, %_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base0ECsC3JuwEIQwb_7console.exit ], [ false, %start ]
  ret i1 %_0.sroa.0.0.off0
}

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsC3JuwEIQwb_7console(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !7, !align !187, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !7
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <console::ansi::Matches as core::iter::traits::iterator::Iterator>::next
; Function Attrs: nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable
define void @_RNvXs2_NtCsC3JuwEIQwb_7console4ansiNtB5_7MatchesNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr dead_on_unwind noalias noundef writable writeonly sret([32 x i8]) align 8 captures(none) dereferenceable(32) initializes((0, 8)) %_0, ptr noalias noundef align 8 captures(none) dereferenceable(56) %self) unnamed_addr #9 {
start:
  %_2 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2)
  %_3 = getelementptr inbounds nuw i8, ptr %self, i64 16
; call console::ansi::find_ansi_code_exclusive
  call fastcc void @_RNvNtCsC3JuwEIQwb_7console4ansi24find_ansi_code_exclusive(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2, ptr noalias noundef align 8 dereferenceable(40) %_3)
  %_5 = load i64, ptr %_2, align 8, !range !6, !noundef !7
  %0 = trunc nuw i64 %_5 to i1
  br i1 %0, label %bb5, label %bb2

bb5:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_4.1 = load i64, ptr %1, align 8, !noundef !7
  %_4.0 = load ptr, ptr %self, align 8, !nonnull !7, !align !187, !noundef !7
  %2 = getelementptr inbounds nuw i8, ptr %_2, i64 8
  %_6.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_4.1, ptr %_6.sroa.4.0._0.sroa_idx, align 8
  %_6.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  %3 = load <2 x i64>, ptr %2, align 8
  store <2 x i64> %3, ptr %_6.sroa.5.0._0.sroa_idx, align 8
  br label %bb2

bb2:                                              ; preds = %start, %bb5
  %.sink = phi ptr [ %_4.0, %bb5 ], [ null, %start ]
  store ptr %.sink, ptr %_0, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2)
  ret void
}

; <console::term::Term as std::os::fd::raw::AsRawFd>::as_raw_fd
; Function Attrs: uwtable
define noundef i32 @_RNvXs2_NtCsC3JuwEIQwb_7console4termNtB5_4TermNtNtNtNtCs5sEH5CPMdak_3std2os2fd3raw7AsRawFd9as_raw_fd(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [24 x i8], align 8
  %_9 = load ptr, ptr %self, align 8, !nonnull !7, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %_9, i64 127
  %1 = load i8, ptr %0, align 1, !range !58, !noundef !7
  %2 = add nsw i8 %1, -2
  %.inv = icmp samesign ult i8 %1, 2
  %narrow = select i1 %.inv, i8 2, i8 %2
  switch i8 %narrow, label %bb1 [
    i8 0, label %bb9
    i8 1, label %bb3
    i8 2, label %bb2
  ]

bb1:                                              ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  br label %bb9

bb2:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %_9, i64 80
  %_11.0 = load ptr, ptr %3, align 8, !nonnull !7, !noundef !7
  %4 = getelementptr inbounds nuw i8, ptr %_9, i64 88
  %_11.1 = load ptr, ptr %4, align 8, !nonnull !7, !align !22, !noundef !7
  %5 = getelementptr inbounds nuw i8, ptr %_11.1, i64 16
  %6 = load i64, ptr %5, align 8, !range !24, !invariant.load !7
  %7 = tail call i64 @llvm.umax.i64(i64 %6, i64 8)
  %8 = add i64 %7, -1
  %9 = and i64 %8, -16
  %10 = getelementptr i8, ptr %_11.0, i64 %9
  %_7.0 = getelementptr i8, ptr %10, i64 16
  %11 = load atomic ptr, ptr %_7.0 acquire, align 8, !noalias !980
  %12 = icmp eq ptr %11, null
  br i1 %12, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, !prof !5

bb7.i.i:                                          ; preds = %bb2
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %13 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 %_7.0), !noalias !980
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i: ; preds = %bb7.i.i, %bb2
  %_0.sroa.0.0.i.i = phi ptr [ %13, %bb7.i.i ], [ %11, %bb2 ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !980
  %14 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !980
  %_6.i.i = and i64 %14, 9223372036854775807
  %15 = icmp eq i64 %_6.i.i, 0
  br i1 %15, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit, label %bb6.i.i, !prof !30

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %16 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33, !noalias !980
  %17 = xor i1 %16, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %17, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i ]
  %_7.i = getelementptr i8, ptr %10, i64 24
  %18 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !980
  %.not = icmp eq i8 %18, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_.exit, label %bb2.i, !prof !30

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e.i), !noalias !984
  store ptr %_7.0, ptr %e.i, align 8, !noalias !989
  %_6.sroa.7.8.e.i.sroa_idx = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store ptr %_11.1, ptr %_6.sroa.7.8.e.i.sroa_idx, align 8, !noalias !989
  %_6.sroa.9.8.e.i.sroa_idx = getelementptr inbounds nuw i8, ptr %e.i, i64 16
  store i8 %_0.sroa.3.0.i.i, ptr %_6.sroa.9.8.e.i.sroa_idx, align 8, !noalias !989
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9ead3bbeb6a5cfcbf0840e30f987d290) #30
          to label %unreachable.i unwind label %cleanup.i, !noalias !990

cleanup.i:                                        ; preds = %bb2.i
  %19 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEEB1Z_(ptr noalias noundef nonnull align 8 dereferenceable(24) %e.i) #32
          to label %common.resume unwind label %terminate.i, !noalias !990

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %20 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !990
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %19, %cleanup.i ], [ %27, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_.exit
  %21 = add i64 %6, -1
  %22 = and i64 %21, -9
  %23 = getelementptr i8, ptr %_7.0, i64 %22
  %_14.0 = getelementptr i8, ptr %23, i64 9
  %24 = getelementptr inbounds nuw i8, ptr %_11.1, i64 96
  %25 = load ptr, ptr %24, align 8, !invariant.load !7, !nonnull !7
  %26 = invoke noundef i32 %25(ptr noundef align 1 %_14.0)
          to label %bb7 unwind label %cleanup

bb9:                                              ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit, %bb3
  %_0.sroa.0.0 = phi i32 [ 2, %bb3 ], [ %26, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit ], [ 1, %start ]
  ret i32 %_0.sroa.0.0

cleanup:                                          ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_.exit
  %27 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr nonnull %_7.0, i8 %_0.sroa.3.0.i.i) #32
          to label %common.resume unwind label %terminate

bb7:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_.exit
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb7
  %28 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %28, 9223372036854775807
  %29 = icmp eq i64 %_7.i.i.i, 0
  br i1 %29, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_.exit: ; preds = %bb7, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %30 = load atomic ptr, ptr %_7.0 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %30)
  br label %bb9

terminate:                                        ; preds = %cleanup
  %31 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; <console::term::Term as std::io::Write>::flush
; Function Attrs: uwtable
define noundef ptr @_RNvXs3_NtCsC3JuwEIQwb_7console4termNtB5_4TermNtNtCs5sEH5CPMdak_3std2io5Write5flush(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 {
start:
; call <console::term::Term>::flush
  %_0 = tail call noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term5flush(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self)
  ret ptr %_0
}

; <console::term::Term as std::io::Write>::write
; Function Attrs: uwtable
define { i64, ptr } @_RNvXs3_NtCsC3JuwEIQwb_7console4termNtB5_4TermNtNtCs5sEH5CPMdak_3std2io5Write5write(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.0, i64 noundef range(i64 0, -9223372036854775808) %buf.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [16 x i8], align 8
  %_14 = load ptr, ptr %self, align 8, !nonnull !7, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %_14, i64 16
  %_5 = load i64, ptr %0, align 8, !range !6, !noundef !7
  %1 = trunc nuw i64 %_5 to i1
  br i1 %1, label %bb3, label %bb2

bb3:                                              ; preds = %start
  %buffer = getelementptr inbounds nuw i8, ptr %_14, i64 24
  %2 = load atomic ptr, ptr %buffer acquire, align 8, !noalias !991
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, !prof !5

bb7.i.i:                                          ; preds = %bb3
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %4 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 %buffer), !noalias !991
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i: ; preds = %bb7.i.i, %bb3
  %_0.sroa.0.0.i.i = phi ptr [ %4, %bb7.i.i ], [ %2, %bb3 ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !991
  %5 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !991
  %_6.i.i = and i64 %5, 9223372036854775807
  %6 = icmp eq i64 %_6.i.i, 0
  br i1 %6, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit, label %bb6.i.i, !prof !30

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %7 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33, !noalias !991
  %8 = xor i1 %7, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %8, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_14, i64 32
  %9 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !991
  %.not9 = icmp eq i8 %9, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not9, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit, label %bb2.i, !prof !30

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !994
  store ptr %buffer, ptr %e.i, align 8, !noalias !994
  %10 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %10, align 8, !noalias !994
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_da7d8ace005fc641b5ea7745ba5bdcfd) #30
          to label %unreachable.i unwind label %cleanup.i, !noalias !998

cleanup.i:                                        ; preds = %bb2.i
  %11 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<u8>>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #32
          to label %common.resume unwind label %terminate.i, !noalias !998

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !998
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %11, %cleanup.i ], [ %18, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit
  %_17 = getelementptr inbounds nuw i8, ptr %_14, i64 40
  %13 = getelementptr inbounds nuw i8, ptr %_14, i64 56
  %len.i = load i64, ptr %13, align 8, !alias.scope !999, !noundef !7
  %self2.i = load i64, ptr %_17, align 8, !range !23, !alias.scope !999, !noundef !7
  %_9.i = sub i64 %self2.i, %len.i
  %_7.i5 = icmp ugt i64 %buf.1, %_9.i
  br i1 %_7.i5, label %bb1.i, label %bb12, !prof !5

bb1.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %_17, i64 noundef %len.i, i64 noundef %buf.1, i64 noundef 1, i64 noundef 1)
          to label %bb1.i.bb12_crit_edge unwind label %cleanup

bb1.i.bb12_crit_edge:                             ; preds = %bb1.i
  %_29.pre = load i64, ptr %13, align 8
  br label %bb12

bb2:                                              ; preds = %start
; call <console::term::Term>::write_through
  %14 = tail call fastcc noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term13write_through(ptr nonnull %_14, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.0, i64 noundef %buf.1)
  %.not = icmp eq ptr %14, null
  br i1 %.not, label %bb14, label %bb8

bb14:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, %bb2
  %15 = inttoptr i64 %buf.1 to ptr
  br label %bb8

bb8:                                              ; preds = %bb2, %bb14
  %_0.sroa.3.0 = phi ptr [ %15, %bb14 ], [ %14, %bb2 ]
  %_0.sroa.0.0 = phi i64 [ 0, %bb14 ], [ 1, %bb2 ]
  %16 = insertvalue { i64, ptr } poison, i64 %_0.sroa.0.0, 0
  %17 = insertvalue { i64, ptr } %16, ptr %_0.sroa.3.0, 1
  ret { i64, ptr } %17

cleanup:                                          ; preds = %bb1.i
  %18 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr nonnull %buffer, i8 %_0.sroa.3.0.i.i) #32
          to label %common.resume unwind label %terminate

bb12:                                             ; preds = %bb1.i.bb12_crit_edge, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
  %_29 = phi i64 [ %_29.pre, %bb1.i.bb12_crit_edge ], [ %len.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit ]
  %_33 = icmp sgt i64 %_29, -1
  tail call void @llvm.assume(i1 %_33)
  %19 = getelementptr inbounds nuw i8, ptr %_14, i64 48
  %_34 = load ptr, ptr %19, align 8, !nonnull !7, !noundef !7
  %_31 = getelementptr inbounds nuw i8, ptr %_34, i64 %_29
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_31, ptr nonnull align 1 %buf.0, i64 %buf.1, i1 false)
  %20 = load i64, ptr %13, align 8, !noundef !7
  %21 = add i64 %20, %buf.1
  store i64 %21, ptr %13, align 8
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb12
  %22 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %22, 9223372036854775807
  %23 = icmp eq i64 %_7.i.i.i, 0
  br i1 %23, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit: ; preds = %bb12, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %24 = load atomic ptr, ptr %buffer monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %24)
  br label %bb14

terminate:                                        ; preds = %cleanup
  %25 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; <&console::term::Term as std::io::Write>::flush
; Function Attrs: uwtable
define noundef ptr @_RNvXs4_NtCsC3JuwEIQwb_7console4termRNtB5_4TermNtNtCs5sEH5CPMdak_3std2io5Write5flush(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 {
start:
  %_2 = load ptr, ptr %self, align 8, !nonnull !7, !align !22, !noundef !7
; call <console::term::Term>::flush
  %_0 = tail call noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term5flush(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %_2)
  ret ptr %_0
}

; <&console::term::Term as std::io::Write>::write
; Function Attrs: uwtable
define { i64, ptr } @_RNvXs4_NtCsC3JuwEIQwb_7console4termRNtB5_4TermNtNtCs5sEH5CPMdak_3std2io5Write5write(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.0, i64 noundef range(i64 0, -9223372036854775808) %buf.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [16 x i8], align 8
  %_12 = load ptr, ptr %self, align 8, !nonnull !7, !align !22, !noundef !7
  %_15 = load ptr, ptr %_12, align 8, !nonnull !7, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %_15, i64 16
  %_5 = load i64, ptr %0, align 8, !range !6, !noundef !7
  %1 = trunc nuw i64 %_5 to i1
  br i1 %1, label %bb3, label %bb2

bb3:                                              ; preds = %start
  %buffer = getelementptr inbounds nuw i8, ptr %_15, i64 24
  %2 = load atomic ptr, ptr %buffer acquire, align 8, !noalias !1002
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, !prof !5

bb7.i.i:                                          ; preds = %bb3
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %4 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECsC3JuwEIQwb_7console(ptr noundef nonnull align 8 %buffer), !noalias !1002
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i: ; preds = %bb7.i.i, %bb3
  %_0.sroa.0.0.i.i = phi ptr [ %4, %bb7.i.i ], [ %2, %bb3 ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !1002
  %5 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !1002
  %_6.i.i = and i64 %5, 9223372036854775807
  %6 = icmp eq i64 %_6.i.i, 0
  br i1 %6, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit, label %bb6.i.i, !prof !30

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %7 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33, !noalias !1002
  %8 = xor i1 %7, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %8, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECsC3JuwEIQwb_7console.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_15, i64 32
  %9 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !1002
  %.not9 = icmp eq i8 %9, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not9, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit, label %bb2.i, !prof !30

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !1005
  store ptr %buffer, ptr %e.i, align 8, !noalias !1005
  %10 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %10, align 8, !noalias !1005
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0878d7ad62607610ca18347eb669fd5d) #30
          to label %unreachable.i unwind label %cleanup.i, !noalias !1009

cleanup.i:                                        ; preds = %bb2.i
  %11 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<alloc::vec::Vec<u8>>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEEECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #32
          to label %common.resume unwind label %terminate.i, !noalias !1009

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31, !noalias !1009
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %11, %cleanup.i ], [ %18, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console.exit
  %_18 = getelementptr inbounds nuw i8, ptr %_15, i64 40
  %13 = getelementptr inbounds nuw i8, ptr %_15, i64 56
  %len.i = load i64, ptr %13, align 8, !alias.scope !1010, !noundef !7
  %self2.i = load i64, ptr %_18, align 8, !range !23, !alias.scope !1010, !noundef !7
  %_9.i = sub i64 %self2.i, %len.i
  %_7.i5 = icmp ugt i64 %buf.1, %_9.i
  br i1 %_7.i5, label %bb1.i, label %bb12, !prof !5

bb1.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsC3JuwEIQwb_7console(ptr noalias noundef nonnull align 8 dereferenceable(24) %_18, i64 noundef %len.i, i64 noundef %buf.1, i64 noundef 1, i64 noundef 1)
          to label %bb1.i.bb12_crit_edge unwind label %cleanup

bb1.i.bb12_crit_edge:                             ; preds = %bb1.i
  %_30.pre = load i64, ptr %13, align 8
  br label %bb12

bb2:                                              ; preds = %start
; call <console::term::Term>::write_through
  %14 = tail call fastcc noundef ptr @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term13write_through(ptr nonnull %_15, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.0, i64 noundef %buf.1)
  %.not = icmp eq ptr %14, null
  br i1 %.not, label %bb14, label %bb8

bb14:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, %bb2
  %15 = inttoptr i64 %buf.1 to ptr
  br label %bb8

bb8:                                              ; preds = %bb2, %bb14
  %_0.sroa.3.0 = phi ptr [ %15, %bb14 ], [ %14, %bb2 ]
  %_0.sroa.0.0 = phi i64 [ 0, %bb14 ], [ 1, %bb2 ]
  %16 = insertvalue { i64, ptr } poison, i64 %_0.sroa.0.0, 0
  %17 = insertvalue { i64, ptr } %16, ptr %_0.sroa.3.0, 1
  ret { i64, ptr } %17

cleanup:                                          ; preds = %bb1.i
  %18 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEB1G_(ptr nonnull %buffer, i8 %_0.sroa.3.0.i.i) #32
          to label %common.resume unwind label %terminate

bb12:                                             ; preds = %bb1.i.bb12_crit_edge, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit
  %_30 = phi i64 [ %_30.pre, %bb1.i.bb12_crit_edge ], [ %len.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console.exit ]
  %_34 = icmp sgt i64 %_30, -1
  tail call void @llvm.assume(i1 %_34)
  %19 = getelementptr inbounds nuw i8, ptr %_15, i64 48
  %_35 = load ptr, ptr %19, align 8, !nonnull !7, !noundef !7
  %_32 = getelementptr inbounds nuw i8, ptr %_35, i64 %_30
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_32, ptr nonnull align 1 %buf.0, i64 %buf.1, i1 false)
  %20 = load i64, ptr %13, align 8, !noundef !7
  %21 = add i64 %20, %buf.1
  store i64 %21, ptr %13, align 8
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb12
  %22 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %22, 9223372036854775807
  %23 = icmp eq i64 %_7.i.i.i, 0
  br i1 %23, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb6.i.i.i, !prof !30

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #33
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console.exit: ; preds = %bb12, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %24 = load atomic ptr, ptr %buffer monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %24)
  br label %bb14

terminate:                                        ; preds = %cleanup
  %25 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable
}

; <console::utils::Emoji as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs4_NtCsC3JuwEIQwb_7console5utilsNtB5_5EmojiNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %f) unnamed_addr #1 {
start:
  %args = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %self, ptr %args, align 8
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsC3JuwEIQwb_7console, ptr %_6.sroa.4.0..sroa_idx, align 8
  %_20.0 = load ptr, ptr %f, align 8, !nonnull !7, !align !187, !noundef !7
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_20.1 = load ptr, ptr %0, align 8, !nonnull !7, !align !22, !noundef !7
; call core::fmt::write
  %1 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_20.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_20.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  ret i1 %1
}

; <console::ansi::AnsiCodeIterator as core::iter::traits::iterator::Iterator>::next
; Function Attrs: uwtable
define void @_RNvXs5_NtCsC3JuwEIQwb_7console4ansiNtB5_16AnsiCodeIteratorNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24) initializes((0, 24)) %_0, ptr noalias noundef align 8 captures(none) dereferenceable(112) %self) unnamed_addr #1 {
start:
  %_2.i = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 32
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %0, i64 24, i1 false)
  %_32.sroa.2.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 48
  store i8 2, ptr %_32.sroa.2.0..sroa_idx, align 8
  %1 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  %2 = load i8, ptr %1, align 8, !range !789, !noundef !7
  %.not = icmp eq i8 %2, 2
  br i1 %.not, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %pending_item.sroa.0.0.copyload = load ptr, ptr %_0, align 8
  %pending_item.sroa.2.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  %pending_item.sroa.2.0.copyload = load i64, ptr %pending_item.sroa.2.0._0.sroa_idx, align 8
  %3 = icmp ne ptr %pending_item.sroa.0.0.copyload, null
  tail call void @llvm.assume(i1 %3)
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %5 = load i64, ptr %4, align 8, !noundef !7
  %6 = add i64 %5, %pending_item.sroa.2.0.copyload
  store i64 %6, ptr %4, align 8
  br label %bb13

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1013)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i), !noalias !1016
  %_3.i = getelementptr inbounds nuw i8, ptr %self, i64 72
; call console::ansi::find_ansi_code_exclusive
  call fastcc void @_RNvNtCsC3JuwEIQwb_7console4ansi24find_ansi_code_exclusive(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i, ptr noalias noundef align 8 dereferenceable(40) %_3.i), !noalias !1018
  %_5.i = load i64, ptr %_2.i, align 8, !range !6, !noalias !1016, !noundef !7
  %7 = trunc nuw i64 %_5.i to i1
  br i1 %7, label %bb4, label %bb8

bb13:                                             ; preds = %bb23, %bb20, %bb26, %bb10, %bb1
  ret void

bb4:                                              ; preds = %bb2
  %_6 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_4.1.i = load i64, ptr %8, align 8, !alias.scope !1013, !noalias !1018, !noundef !7
  %_4.0.i = load ptr, ptr %_6, align 8, !alias.scope !1013, !noalias !1018, !nonnull !7, !align !187, !noundef !7
  %9 = getelementptr inbounds nuw i8, ptr %_2.i, i64 8
  %_7.i = load i64, ptr %9, align 8, !noalias !1016, !noundef !7
  %10 = getelementptr inbounds nuw i8, ptr %_2.i, i64 16
  %_8.i = load i64, ptr %10, align 8, !noalias !1016, !noundef !7
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i), !noalias !1016
  %_27.0 = load ptr, ptr %self, align 8, !nonnull !7, !align !187, !noundef !7
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_27.1 = load i64, ptr %11, align 8, !noundef !7
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_10 = load i64, ptr %12, align 8, !noundef !7
  %_3.not.i = icmp ugt i64 %_10, %_7.i
  br i1 %_3.not.i, label %bb16, label %bb1.i

bb1.i:                                            ; preds = %bb4
  %13 = icmp eq i64 %_10, 0
  br i1 %13, label %bb2.i, label %bb9.i

bb9.i:                                            ; preds = %bb1.i
  %_11.not.i = icmp ult i64 %_10, %_27.1
  br i1 %_11.not.i, label %bb13.i, label %bb10.i

bb2.i:                                            ; preds = %bb13.i, %bb10.i, %bb1.i
  %14 = icmp eq i64 %_7.i, 0
  br i1 %14, label %bb17, label %bb15.i

bb10.i:                                           ; preds = %bb9.i
  %15 = icmp eq i64 %_10, %_27.1
  br i1 %15, label %bb2.i, label %bb16

bb13.i:                                           ; preds = %bb9.i
  %16 = getelementptr inbounds nuw i8, ptr %_27.0, i64 %_10
  %self.i = load i8, ptr %16, align 1, !alias.scope !1019, !noundef !7
  %17 = icmp sgt i8 %self.i, -65
  br i1 %17, label %bb2.i, label %bb16

bb15.i:                                           ; preds = %bb2.i
  %_17.not.i = icmp ult i64 %_7.i, %_27.1
  br i1 %_17.not.i, label %bb19.i, label %bb16.i

bb16.i:                                           ; preds = %bb15.i
  %18 = icmp eq i64 %_7.i, %_27.1
  br i1 %18, label %bb17, label %bb16

bb19.i:                                           ; preds = %bb15.i
  %19 = getelementptr inbounds nuw i8, ptr %_27.0, i64 %_7.i
  %self2.i = load i8, ptr %19, align 1, !alias.scope !1019, !noundef !7
  %20 = icmp sgt i8 %self2.i, -65
  br i1 %20, label %bb17, label %bb16

bb8:                                              ; preds = %bb2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i), !noalias !1016
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_18 = load i64, ptr %21, align 8, !noundef !7
  %22 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_28.1 = load i64, ptr %22, align 8, !noundef !7
  %_17 = icmp ult i64 %_18, %_28.1
  br i1 %_17, label %bb9, label %bb10

bb17:                                             ; preds = %bb19.i, %bb16.i, %bb2.i
  %new_len.i = sub nuw i64 %_7.i, %_10
  %data.i = getelementptr inbounds nuw i8, ptr %_27.0, i64 %_10
  store i64 %_8.i, ptr %12, align 8
  %23 = icmp eq i64 %_7.i, %_10
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_3.not.i12 = icmp ugt i64 %_7.i, %_8.i
  br i1 %23, label %bb5, label %bb6

bb16:                                             ; preds = %bb19.i, %bb13.i, %bb4, %bb10.i, %bb16.i
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_27.0, i64 noundef %_27.1, i64 noundef %_10, i64 noundef %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_ca4d880a0cd8b0a81e2f00dd95ebe9c4) #34
  unreachable

bb5:                                              ; preds = %bb17
  store i64 %_8.i, ptr %24, align 8
  br i1 %_3.not.i12, label %bb19, label %bb1.i13

bb1.i13:                                          ; preds = %bb5
  br i1 %14, label %bb2.i19, label %bb9.i14

bb9.i14:                                          ; preds = %bb1.i13
  %_11.not.i15 = icmp ult i64 %_7.i, %_4.1.i
  br i1 %_11.not.i15, label %bb13.i28, label %bb10.i16

bb2.i19:                                          ; preds = %bb13.i28, %bb10.i16, %bb1.i13
  %25 = icmp eq i64 %_8.i, 0
  br i1 %25, label %bb20, label %bb15.i20

bb10.i16:                                         ; preds = %bb9.i14
  %26 = icmp eq i64 %_7.i, %_4.1.i
  br i1 %26, label %bb2.i19, label %bb19

bb13.i28:                                         ; preds = %bb9.i14
  %27 = getelementptr inbounds nuw i8, ptr %_4.0.i, i64 %_7.i
  %self.i29 = load i8, ptr %27, align 1, !alias.scope !1022, !noundef !7
  %28 = icmp sgt i8 %self.i29, -65
  br i1 %28, label %bb2.i19, label %bb19

bb15.i20:                                         ; preds = %bb2.i19
  %_17.not.i21 = icmp ult i64 %_8.i, %_4.1.i
  br i1 %_17.not.i21, label %bb19.i26, label %bb16.i22

bb16.i22:                                         ; preds = %bb15.i20
  %29 = icmp eq i64 %_8.i, %_4.1.i
  br i1 %29, label %bb20, label %bb19

bb19.i26:                                         ; preds = %bb15.i20
  %30 = getelementptr inbounds nuw i8, ptr %_4.0.i, i64 %_8.i
  %self2.i27 = load i8, ptr %30, align 1, !alias.scope !1022, !noundef !7
  %31 = icmp sgt i8 %self2.i27, -65
  br i1 %31, label %bb20, label %bb19

bb6:                                              ; preds = %bb17
  store i64 %_7.i, ptr %24, align 8
  br i1 %_3.not.i12, label %bb22, label %bb1.i32

bb1.i32:                                          ; preds = %bb6
  br i1 %14, label %bb2.i38, label %bb9.i33

bb9.i33:                                          ; preds = %bb1.i32
  %_11.not.i34 = icmp ult i64 %_7.i, %_4.1.i
  br i1 %_11.not.i34, label %bb13.i47, label %bb10.i35

bb2.i38:                                          ; preds = %bb13.i47, %bb10.i35, %bb1.i32
  %32 = icmp eq i64 %_8.i, 0
  br i1 %32, label %bb23, label %bb15.i39

bb10.i35:                                         ; preds = %bb9.i33
  %33 = icmp eq i64 %_7.i, %_4.1.i
  br i1 %33, label %bb2.i38, label %bb22

bb13.i47:                                         ; preds = %bb9.i33
  %34 = getelementptr inbounds nuw i8, ptr %_4.0.i, i64 %_7.i
  %self.i48 = load i8, ptr %34, align 1, !alias.scope !1025, !noundef !7
  %35 = icmp sgt i8 %self.i48, -65
  br i1 %35, label %bb2.i38, label %bb22

bb15.i39:                                         ; preds = %bb2.i38
  %_17.not.i40 = icmp ult i64 %_8.i, %_4.1.i
  br i1 %_17.not.i40, label %bb19.i45, label %bb16.i41

bb16.i41:                                         ; preds = %bb15.i39
  %36 = icmp eq i64 %_8.i, %_4.1.i
  br i1 %36, label %bb23, label %bb22

bb19.i45:                                         ; preds = %bb15.i39
  %37 = getelementptr inbounds nuw i8, ptr %_4.0.i, i64 %_8.i
  %self2.i46 = load i8, ptr %37, align 1, !alias.scope !1025, !noundef !7
  %38 = icmp sgt i8 %self2.i46, -65
  br i1 %38, label %bb23, label %bb22

bb20:                                             ; preds = %bb19.i26, %bb16.i22, %bb2.i19
  %new_len.i24 = sub nuw i64 %_8.i, %_7.i
  %data.i25 = getelementptr inbounds nuw i8, ptr %_4.0.i, i64 %_7.i
  store ptr %data.i25, ptr %_0, align 8
  %_11.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %new_len.i24, ptr %_11.sroa.4.0._0.sroa_idx, align 8
  store i8 1, ptr %1, align 8
  br label %bb13

bb19:                                             ; preds = %bb19.i26, %bb13.i28, %bb5, %bb10.i16, %bb16.i22
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_4.0.i, i64 noundef %_4.1.i, i64 noundef %_7.i, i64 noundef %_8.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1b354bc18ec1773a84dec3c89f38b3b8) #34
  unreachable

bb23:                                             ; preds = %bb19.i45, %bb16.i41, %bb2.i38
  %new_len.i43 = sub nuw i64 %_8.i, %_7.i
  %data.i44 = getelementptr inbounds nuw i8, ptr %_4.0.i, i64 %_7.i
  store ptr %data.i44, ptr %0, align 8
  %_13.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 40
  store i64 %new_len.i43, ptr %_13.sroa.4.0..sroa_idx, align 8
  store i8 1, ptr %_32.sroa.2.0..sroa_idx, align 8
  store ptr %data.i, ptr %_0, align 8
  %_16.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %new_len.i, ptr %_16.sroa.4.0._0.sroa_idx, align 8
  store i8 0, ptr %1, align 8
  br label %bb13

bb22:                                             ; preds = %bb19.i45, %bb13.i47, %bb6, %bb10.i35, %bb16.i41
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_4.0.i, i64 noundef %_4.1.i, i64 noundef %_7.i, i64 noundef %_8.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1b354bc18ec1773a84dec3c89f38b3b8) #34
  unreachable

bb10:                                             ; preds = %bb8
  store i8 2, ptr %1, align 8
  br label %bb13

bb9:                                              ; preds = %bb8
  %_29.0 = load ptr, ptr %self, align 8, !nonnull !7, !align !187, !noundef !7
  %39 = icmp eq i64 %_18, 0
  br i1 %39, label %bb26, label %bb9.i55

bb9.i55:                                          ; preds = %bb9
  %40 = getelementptr inbounds nuw i8, ptr %_29.0, i64 %_18
  %self1.i = load i8, ptr %40, align 1, !alias.scope !1028, !noundef !7
  %41 = icmp sgt i8 %self1.i, -65
  br i1 %41, label %bb26, label %bb25

bb26:                                             ; preds = %bb9.i55, %bb9
  %new_len.i53 = sub nuw i64 %_28.1, %_18
  %data.i54 = getelementptr inbounds nuw i8, ptr %_29.0, i64 %_18
  %42 = getelementptr inbounds nuw i8, ptr %self, i64 24
  store i64 %_28.1, ptr %42, align 8
  store i64 %_28.1, ptr %21, align 8
  store ptr %data.i54, ptr %_0, align 8
  %_25.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %new_len.i53, ptr %_25.sroa.4.0._0.sroa_idx, align 8
  store i8 0, ptr %1, align 8
  br label %bb13

bb25:                                             ; preds = %bb9.i55
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_29.0, i64 noundef %_28.1, i64 noundef %_18, i64 noundef %_28.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_173e90a6bd2787cc8478b93b2c321f76) #34
  unreachable
}

; <console::term::Term as std::io::Read>::read
; Function Attrs: uwtable
define { i64, ptr } @_RNvXs5_NtCsC3JuwEIQwb_7console4termNtB5_4TermNtNtCs5sEH5CPMdak_3std2io4Read4read(ptr noalias noundef readnone align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef nonnull align 1 %buf.0, i64 noundef range(i64 0, -9223372036854775808) %buf.1) unnamed_addr #1 {
start:
  %_4 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_4)
; call std::io::stdio::stdin
  %0 = tail call noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio5stdin()
  store ptr %0, ptr %_4, align 8
; call <std::io::stdio::Stdin as std::io::Read>::read
  %1 = call { i64, ptr } @_RNvXs3_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_5StdinNtB7_4Read4read(ptr noalias noundef nonnull align 8 dereferenceable(8) %_4, ptr noalias noundef nonnull align 1 %buf.0, i64 noundef %buf.1)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_4)
  ret { i64, ptr } %1
}

; <&console::term::Term as std::io::Read>::read
; Function Attrs: uwtable
define { i64, ptr } @_RNvXs6_NtCsC3JuwEIQwb_7console4termRNtB5_4TermNtNtCs5sEH5CPMdak_3std2io4Read4read(ptr noalias noundef readnone align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef nonnull align 1 %buf.0, i64 noundef range(i64 0, -9223372036854775808) %buf.1) unnamed_addr #1 {
start:
  %_4 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_4)
; call std::io::stdio::stdin
  %0 = tail call noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio5stdin()
  store ptr %0, ptr %_4, align 8
; call <std::io::stdio::Stdin as std::io::Read>::read
  %1 = call { i64, ptr } @_RNvXs3_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_5StdinNtB7_4Read4read(ptr noalias noundef nonnull align 8 dereferenceable(8) %_4, ptr noalias noundef nonnull align 1 %buf.0, i64 noundef %buf.1)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_4)
  ret { i64, ptr } %1
}

; <std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<dyn console::term::TermWrite>> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1q_(ptr noalias readonly align 8 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
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

; <console::utils::STDERR_COLORS::{closure#0} as core::ops::function::FnOnce<()>>::call_once
; Function Attrs: inlinehint uwtable
define internal range(i8 0, 2) i8 @_RNvYNCNvNtCsC3JuwEIQwb_7console5utils13STDERR_COLORS0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_onceB8_() unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %_5.i = alloca [168 x i8], align 8
  %_4.i = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4.i)
  call void @llvm.lifetime.start.p0(i64 168, ptr nonnull %_5.i)
  %_6.sroa.3.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 111
  store i8 3, ptr %_6.sroa.3.0..sroa_idx.i, align 1
  store i64 0, ptr %_5.i, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_5.i, i64 112
  store i64 0, ptr %0, align 8
  %_8.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 120
  store i8 0, ptr %_8.sroa.4.0..sroa_idx.i, align 8
  %_8.sroa.5.sroa.3.0._8.sroa.5.0..sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 128
  store i64 0, ptr %_8.sroa.5.sroa.3.0._8.sroa.5.0..sroa_idx.sroa_idx.i, align 8
  %_8.sroa.5.sroa.4.0._8.sroa.5.0..sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 136
  store ptr inttoptr (i64 1 to ptr), ptr %_8.sroa.5.sroa.4.0._8.sroa.5.0..sroa_idx.sroa_idx.i, align 8
  %_8.sroa.5.sroa.5.0._8.sroa.5.0..sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 144
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(17) %_8.sroa.5.sroa.5.0._8.sroa.5.0..sroa_idx.sroa_idx.i, i8 0, i64 17, i1 false)
; call <console::term::Term>::with_inner
  call void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term10with_inner(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_4.i, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(168) %_5.i)
  call void @llvm.lifetime.end.p0(i64 168, ptr nonnull %_5.i)
; invoke console::utils::default_colors_enabled
  %_2.i = invoke noundef zeroext i1 @_RNvNtCsC3JuwEIQwb_7console5utils22default_colors_enabled(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %_4.i)
          to label %bb1.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1031)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1034)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1037)
  %_10.i.i.i.i = load ptr, ptr %_4.i, align 8, !alias.scope !1040, !nonnull !7, !noundef !7
  %2 = atomicrmw sub ptr %_10.i.i.i.i, i64 1 release, align 8, !noalias !1040
  %3 = icmp eq i64 %2, 1
  br i1 %3, label %bb2.i.i.i.i, label %bb4.i

bb2.i.i.i.i:                                      ; preds = %cleanup.i
  fence acquire
; invoke <alloc::sync::Arc<console::term::TermInner>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerE9drop_slowBK_(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %_4.i) #33
          to label %bb4.i unwind label %terminate.i

bb1.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1041)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1044)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1047)
  %_10.i.i.i4.i = load ptr, ptr %_4.i, align 8, !alias.scope !1050, !nonnull !7, !noundef !7
  %4 = atomicrmw sub ptr %_10.i.i.i4.i, i64 1 release, align 8, !noalias !1050
  %5 = icmp eq i64 %4, 1
  br i1 %5, label %bb2.i.i.i5.i, label %_RNCNvNtCsC3JuwEIQwb_7console5utils13STDERR_COLORS0B5_.exit

bb2.i.i.i5.i:                                     ; preds = %bb1.i
  fence acquire
; call <alloc::sync::Arc<console::term::TermInner>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerE9drop_slowBK_(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %_4.i) #33
  br label %_RNCNvNtCsC3JuwEIQwb_7console5utils13STDERR_COLORS0B5_.exit

terminate.i:                                      ; preds = %bb2.i.i.i.i
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

bb4.i:                                            ; preds = %bb2.i.i.i.i, %cleanup.i
  resume { ptr, i32 } %1

_RNCNvNtCsC3JuwEIQwb_7console5utils13STDERR_COLORS0B5_.exit: ; preds = %bb1.i, %bb2.i.i.i5.i
  %_21.i = zext i1 %_2.i to i8
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4.i)
  ret i8 %_21.i
}

; <console::utils::STDOUT_COLORS::{closure#0} as core::ops::function::FnOnce<()>>::call_once
; Function Attrs: inlinehint uwtable
define internal range(i8 0, 2) i8 @_RNvYNCNvNtCsC3JuwEIQwb_7console5utils13STDOUT_COLORS0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_onceB8_() unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %_5.i = alloca [168 x i8], align 8
  %_4.i = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4.i)
  call void @llvm.lifetime.start.p0(i64 168, ptr nonnull %_5.i)
  %_6.sroa.3.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 111
  store i8 2, ptr %_6.sroa.3.0..sroa_idx.i, align 1
  store i64 0, ptr %_5.i, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_5.i, i64 112
  store i64 0, ptr %0, align 8
  %_8.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 120
  store i8 0, ptr %_8.sroa.4.0..sroa_idx.i, align 8
  %_8.sroa.5.sroa.3.0._8.sroa.5.0..sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 128
  store i64 0, ptr %_8.sroa.5.sroa.3.0._8.sroa.5.0..sroa_idx.sroa_idx.i, align 8
  %_8.sroa.5.sroa.4.0._8.sroa.5.0..sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 136
  store ptr inttoptr (i64 1 to ptr), ptr %_8.sroa.5.sroa.4.0._8.sroa.5.0..sroa_idx.sroa_idx.i, align 8
  %_8.sroa.5.sroa.5.0._8.sroa.5.0..sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 144
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(17) %_8.sroa.5.sroa.5.0._8.sroa.5.0..sroa_idx.sroa_idx.i, i8 0, i64 17, i1 false)
; call <console::term::Term>::with_inner
  call void @_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term10with_inner(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_4.i, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(168) %_5.i)
  call void @llvm.lifetime.end.p0(i64 168, ptr nonnull %_5.i)
; invoke console::utils::default_colors_enabled
  %_2.i = invoke noundef zeroext i1 @_RNvNtCsC3JuwEIQwb_7console5utils22default_colors_enabled(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %_4.i)
          to label %bb1.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1051)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1054)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1057)
  %_10.i.i.i.i = load ptr, ptr %_4.i, align 8, !alias.scope !1060, !nonnull !7, !noundef !7
  %2 = atomicrmw sub ptr %_10.i.i.i.i, i64 1 release, align 8, !noalias !1060
  %3 = icmp eq i64 %2, 1
  br i1 %3, label %bb2.i.i.i.i, label %bb4.i

bb2.i.i.i.i:                                      ; preds = %cleanup.i
  fence acquire
; invoke <alloc::sync::Arc<console::term::TermInner>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerE9drop_slowBK_(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %_4.i) #33
          to label %bb4.i unwind label %terminate.i

bb1.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1061)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1064)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1067)
  %_10.i.i.i4.i = load ptr, ptr %_4.i, align 8, !alias.scope !1070, !nonnull !7, !noundef !7
  %4 = atomicrmw sub ptr %_10.i.i.i4.i, i64 1 release, align 8, !noalias !1070
  %5 = icmp eq i64 %4, 1
  br i1 %5, label %bb2.i.i.i5.i, label %_RNCNvNtCsC3JuwEIQwb_7console5utils13STDOUT_COLORS0B5_.exit

bb2.i.i.i5.i:                                     ; preds = %bb1.i
  fence acquire
; call <alloc::sync::Arc<console::term::TermInner>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerE9drop_slowBK_(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %_4.i) #33
  br label %_RNCNvNtCsC3JuwEIQwb_7console5utils13STDOUT_COLORS0B5_.exit

terminate.i:                                      ; preds = %bb2.i.i.i.i
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #31
  unreachable

bb4.i:                                            ; preds = %bb2.i.i.i.i, %cleanup.i
  resume { ptr, i32 } %1

_RNCNvNtCsC3JuwEIQwb_7console5utils13STDOUT_COLORS0B5_.exit: ; preds = %bb1.i, %bb2.i.i.i5.i
  %_21.i = zext i1 %_2.i to i8
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4.i)
  ret i8 %_21.i
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #11

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #11

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #12

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #13

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #14

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #15

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #14

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #14

; <std::fs::OpenOptions>::_open
; Function Attrs: uwtable
declare void @_RNvMsl_NtCs5sEH5CPMdak_3std2fsNtB5_11OpenOptions5__open(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(none) dereferenceable(16), ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(12), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; core::str::converts::from_utf8
; Function Attrs: uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; std::env::_var
; Function Attrs: uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; Function Attrs: nofree nosync nounwind memory(none) uwtable
declare noundef ptr @__error() unnamed_addr #16

; <std::io::Guard as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXNtCs5sEH5CPMdak_3std2ioNtB2_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #1

; <std::sys::sync::mutex::pthread::Mutex as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXs_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #1

; <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memmove.p0.p0.i64(ptr writeonly captures(none), ptr readonly captures(none), i64, i1 immarg) #13

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #17

; <std::sys::pal::unix::sync::mutex::Mutex>::init
; Function Attrs: uwtable
declare void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4init(ptr noundef nonnull align 8) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare ptr @llvm.ptrmask.p0.i64(ptr, i64) #18

; Function Attrs: nounwind uwtable
declare noundef i32 @tcgetattr(i32 noundef, ptr noundef) unnamed_addr #2

; Function Attrs: nounwind uwtable
declare noundef i32 @tcsetattr(i32 noundef, i32 noundef, ptr noundef) unnamed_addr #2

; alloc::fmt::format::format_inner
; Function Attrs: uwtable
declare void @_RNvNvNtCsdJPVW0sQgAG_5alloc3fmt6format12format_inner(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #1

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #19

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #20

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #21

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #2

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #22

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #23

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #14

; std::panicking::panic_count::is_zero_slow_path
; Function Attrs: cold noinline uwtable
declare noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() unnamed_addr #4

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #18

; <std::io::stdio::Stdin>::read_line
; Function Attrs: uwtable
declare { i64, ptr } @_RNvMs1_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_5Stdin9read_line(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; Function Attrs: nounwind uwtable
declare noundef i32 @isatty(i32 noundef) unnamed_addr #2

; <std::sys::sync::rwlock::queue::RwLock>::lock_contended
; Function Attrs: cold uwtable
declare void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock14lock_contended(ptr noundef nonnull align 8, i1 noundef zeroext) unnamed_addr #0

; std::io::stdio::stdout
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6stdout() unnamed_addr #1

; <std::io::stdio::Stdout as std::io::Write>::write_all
; Function Attrs: uwtable
declare noundef ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write9write_all(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; <std::io::stdio::Stdout as std::io::Write>::flush
; Function Attrs: uwtable
declare noundef ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write5flush(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #1

; <std::io::stdio::Stderr as std::io::Write>::write_all
; Function Attrs: uwtable
declare noundef ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write9write_all(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; <std::io::stdio::Stderr as std::io::Write>::flush
; Function Attrs: uwtable
declare noundef ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write5flush(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #1

; <std::io::error::Error>::new::<&str>
; Function Attrs: noinline uwtable
declare noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newReEBa_(i8 noundef range(i8 0, 42), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #7

; core::str::slice_error_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #14

; <std::sys::pal::unix::sync::mutex::Mutex>::lock
; Function Attrs: uwtable
declare void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8) unnamed_addr #1

; <alloc::raw_vec::RawVec<u8>>::grow_one
; Function Attrs: noinline uwtable
declare void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVechE8grow_oneB7_(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #7

; std::io::stdio::stdin
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio5stdin() unnamed_addr #1

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #17

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i8, i1 } @llvm.umul.with.overflow.i8(i8, i8) #18

; <usize as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare range(i8 -1, 2) i8 @llvm.ucmp.i8.i64(i64, i64) #18

; Function Attrs: nofree nounwind uwtable
declare noundef i64 @read(i32 noundef, ptr noundef captures(none), i64 noundef) unnamed_addr #24

; Function Attrs: nounwind uwtable
declare noundef i32 @ioctl(i32 noundef, i64 noundef, ...) unnamed_addr #2

; Function Attrs: nounwind uwtable
declare void @cfmakeraw(ptr noundef) unnamed_addr #2

; Function Attrs: nounwind uwtable
declare noundef i32 @raise(i32 noundef) unnamed_addr #2

; Function Attrs: nounwind uwtable
declare noundef i32 @poll(ptr noundef, i32 noundef, i32 noundef) unnamed_addr #2

; Function Attrs: nounwind uwtable
declare noundef i32 @select(i32 noundef, ptr noundef, ptr noundef, ptr noundef, ptr noundef) unnamed_addr #2

; unicode_width::tables::is_transparent_zero_width
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsDJOD2kcAir_13unicode_width6tables25is_transparent_zero_width(i32 noundef range(i32 0, 1114112)) unnamed_addr #1

; core::panicking::panic_cannot_unwind
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking19panic_cannot_unwind() unnamed_addr #15

; core::slice::memchr::memchr_aligned
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; <std::fs::File as std::io::Read>::read_buf
; Function Attrs: uwtable
declare noundef ptr @_RNvXsa_NtCs5sEH5CPMdak_3std2fsNtB5_4FileNtNtB7_2io4Read8read_buf(ptr noalias noundef align 4 dereferenceable(4), ptr noalias noundef align 8 dereferenceable(32)) unnamed_addr #1

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #25

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #26

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #1

; <std::io::stdio::Stdin as std::io::Read>::read
; Function Attrs: uwtable
declare { i64, ptr } @_RNvXs3_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_5StdinNtB7_4Read4read(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull align 1, i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; Function Attrs: nounwind uwtable
declare noundef i32 @close(i32 noundef) unnamed_addr #2

; <core::fmt::Formatter>::debug_struct
; Function Attrs: uwtable
declare void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(address) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; <core::fmt::builders::DebugStruct>::finish_non_exhaustive
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #1

; <std::sys::pal::unix::sync::mutex::Mutex>::unlock
; Function Attrs: uwtable
declare void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8) unnamed_addr #1

; <std::sys::sync::rwlock::queue::RwLock>::read_unlock_contended
; Function Attrs: cold uwtable
declare void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock21read_unlock_contended(ptr noundef nonnull align 8, ptr noundef) unnamed_addr #0

; <std::sys::sync::rwlock::queue::RwLock>::unlock_contended
; Function Attrs: cold uwtable
declare void @_RNvMs1_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync6rwlock5queueNtB5_6RwLock16unlock_contended(ptr noundef nonnull align 8, ptr noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare range(i8 -1, 2) i8 @llvm.scmp.i8.i64(i64, i64) #18

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #27

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #28

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64) #27

attributes #0 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { cold noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { inlinehint mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read, inaccessiblemem: write) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { inlinehint nofree norecurse nosync nounwind memory(argmem: read) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { inlinehint nofree norecurse nosync nounwind memory(read, inaccessiblemem: write) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #12 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #13 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #14 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { nofree nosync nounwind memory(none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #17 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #18 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #19 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #20 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #21 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #22 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #23 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #24 = { nofree nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #25 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #26 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #27 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #28 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #29 = { nounwind }
attributes #30 = { noreturn }
attributes #31 = { cold noreturn nounwind }
attributes #32 = { cold }
attributes #33 = { noinline }
attributes #34 = { noinline noreturn }
attributes #35 = { inlinehint }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCsC3JuwEIQwb_7console: %x"}
!4 = distinct !{!4, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCsC3JuwEIQwb_7console"}
!5 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!6 = !{i64 0, i64 2}
!7 = !{}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console: %_1"}
!10 = distinct !{!10, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console"}
!11 = !{!12}
!12 = distinct !{!12, !13, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!13 = distinct !{!13, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!14 = !{!12, !9, !15, !17}
!15 = distinct !{!15, !16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console: %_1"}
!16 = distinct !{!16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console"}
!17 = distinct !{!17, !18, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console: %_1"}
!18 = distinct !{!18, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEECsC3JuwEIQwb_7console"}
!19 = !{!12, !9}
!20 = !{!17}
!21 = !{!"branch_weights", i32 2000, i32 6001}
!22 = !{i64 8}
!23 = !{i64 0, i64 -9223372036854775808}
!24 = !{i64 1, i64 0}
!25 = !{!26}
!26 = distinct !{!26, !27, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console: %_1"}
!27 = distinct !{!27, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console"}
!28 = !{i32 0, i32 -1}
!29 = !{i8 0, i8 2}
!30 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!31 = !{!32}
!32 = distinct !{!32, !33, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console: %_1"}
!33 = distinct !{!33, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console"}
!34 = !{!35}
!35 = distinct !{!35, !36, !"_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!36 = distinct !{!36, !"_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!37 = !{!35, !32}
!38 = !{!"branch_weights", i32 1073205, i32 2146410443}
!39 = !{!40}
!40 = distinct !{!40, !41, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!41 = distinct !{!41, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!42 = !{!43}
!43 = distinct !{!43, !44, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console: %_1"}
!44 = distinct !{!44, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console"}
!45 = !{!46}
!46 = distinct !{!46, !47, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!47 = distinct !{!47, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!48 = !{!46, !43, !49}
!49 = distinct !{!49, !50, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console: %_1"}
!50 = distinct !{!50, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console"}
!51 = !{!46, !43}
!52 = !{!53}
!53 = distinct !{!53, !54, !"_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!54 = distinct !{!54, !"_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!55 = !{!56}
!56 = distinct !{!56, !57, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term10TermTargetEBK_: %_1"}
!57 = distinct !{!57, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term10TermTargetEBK_"}
!58 = !{i8 0, i8 4}
!59 = !{!60}
!60 = distinct !{!60, !61, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term13ReadWritePairEBK_: %_1"}
!61 = distinct !{!61, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term13ReadWritePairEBK_"}
!62 = !{!63}
!63 = distinct !{!63, !64, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EEEB28_: %_1"}
!64 = distinct !{!64, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EEEB28_"}
!65 = !{!66}
!66 = distinct !{!66, !67, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1A_: %self"}
!67 = distinct !{!67, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term8TermReadEL_EENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1A_"}
!68 = !{!66, !63, !60, !56}
!69 = !{!70}
!70 = distinct !{!70, !71, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEEB28_: %_1"}
!71 = distinct !{!71, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEEB28_"}
!72 = !{!73}
!73 = distinct !{!73, !74, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1A_: %self"}
!74 = distinct !{!74, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1A_"}
!75 = !{!73, !70, !60, !56}
!76 = !{!77}
!77 = distinct !{!77, !78, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEEB28_: %_1"}
!78 = distinct !{!78, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EEEB28_"}
!79 = !{!80}
!80 = distinct !{!80, !81, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1A_: %self"}
!81 = distinct !{!81, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1A_"}
!82 = !{!80, !77, !60, !56}
!83 = !{!84}
!84 = distinct !{!84, !85, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console5utils5StyleEBK_: %_1"}
!85 = distinct !{!85, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console5utils5StyleEBK_"}
!86 = !{!87}
!87 = distinct !{!87, !88, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3set8BTreeSetNtNtCsC3JuwEIQwb_7console5utils9AttributeEEB1J_: %_1"}
!88 = distinct !{!88, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3set8BTreeSetNtNtCsC3JuwEIQwb_7console5utils9AttributeEEB1J_"}
!89 = !{!90}
!90 = distinct !{!90, !91, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBL_7set_val9SetValZSTEEB1J_: %_1"}
!91 = distinct !{!91, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBL_7set_val9SetValZSTEEB1J_"}
!92 = !{!93}
!93 = distinct !{!93, !94, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_: %self"}
!94 = distinct !{!94, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_"}
!95 = !{!93, !90, !87, !84, !60, !56}
!96 = !{!97, !99, !93, !90, !87, !84, !60, !56}
!97 = distinct !{!97, !98, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1b_: %self"}
!98 = distinct !{!98, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1b_"}
!99 = distinct !{!99, !100, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBL_7set_val9SetValZSTEEB1J_: %_1"}
!100 = distinct !{!100, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBL_7set_val9SetValZSTEEB1J_"}
!101 = !{!60, !56}
!102 = !{!103}
!103 = distinct !{!103, !104, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console: %_1"}
!104 = distinct !{!104, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console"}
!105 = !{!106}
!106 = distinct !{!106, !107, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!107 = distinct !{!107, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!108 = !{!106, !103, !109, !111}
!109 = distinct !{!109, !110, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console: %_1"}
!110 = distinct !{!110, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console"}
!111 = distinct !{!111, !112, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECsC3JuwEIQwb_7console: %_1"}
!112 = distinct !{!112, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECsC3JuwEIQwb_7console"}
!113 = !{!106, !103}
!114 = !{!115}
!115 = distinct !{!115, !116, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3set8BTreeSetNtNtCsC3JuwEIQwb_7console5utils9AttributeEEB1J_: %_1"}
!116 = distinct !{!116, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3set8BTreeSetNtNtCsC3JuwEIQwb_7console5utils9AttributeEEB1J_"}
!117 = !{!118}
!118 = distinct !{!118, !119, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBL_7set_val9SetValZSTEEB1J_: %_1"}
!119 = distinct !{!119, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBL_7set_val9SetValZSTEEB1J_"}
!120 = !{!121}
!121 = distinct !{!121, !122, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_: %self"}
!122 = distinct !{!122, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB18_"}
!123 = !{!121, !118, !115}
!124 = !{!125, !127, !121, !118, !115}
!125 = distinct !{!125, !126, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1b_: %self"}
!126 = distinct !{!126, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB1b_"}
!127 = distinct !{!127, !128, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBL_7set_val9SetValZSTEEB1J_: %_1"}
!128 = distinct !{!128, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBL_7set_val9SetValZSTEEB1J_"}
!129 = !{!130}
!130 = distinct !{!130, !131, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsC3JuwEIQwb_7console: %self"}
!131 = distinct !{!131, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsC3JuwEIQwb_7console"}
!132 = !{i64 0, i64 -9223372036854775807}
!133 = !{!134}
!134 = distinct !{!134, !135, !"_RINvMNtCsjMrxcFdYDNN_4core3stre16trim_end_matchesRScECsC3JuwEIQwb_7console: %self.0"}
!135 = distinct !{!135, !"_RINvMNtCsjMrxcFdYDNN_4core3stre16trim_end_matchesRScECsC3JuwEIQwb_7console"}
!136 = !{!137, !139, !141, !143, !144, !146, !147, !149}
!137 = distinct !{!137, !138, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations23next_code_point_reverseINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!138 = distinct !{!138, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations23next_code_point_reverseINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!139 = distinct !{!139, !140, !"_RNvXs4_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back: %self"}
!140 = distinct !{!140, !"_RNvXs4_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back"}
!141 = distinct !{!141, !142, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherRScENtB5_15ReverseSearcher9next_backCsC3JuwEIQwb_7console: %_0"}
!142 = distinct !{!142, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherRScENtB5_15ReverseSearcher9next_backCsC3JuwEIQwb_7console"}
!143 = distinct !{!143, !142, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherRScENtB5_15ReverseSearcher9next_backCsC3JuwEIQwb_7console: %self"}
!144 = distinct !{!144, !145, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherRScENtB5_15ReverseSearcher16next_reject_backCsC3JuwEIQwb_7console: %_0"}
!145 = distinct !{!145, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherRScENtB5_15ReverseSearcher16next_reject_backCsC3JuwEIQwb_7console"}
!146 = distinct !{!146, !145, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherRScENtB5_15ReverseSearcher16next_reject_backCsC3JuwEIQwb_7console: %self"}
!147 = distinct !{!147, !148, !"_RNvXsk_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_17CharSliceSearcherNtB5_15ReverseSearcher16next_reject_back: %_0"}
!148 = distinct !{!148, !"_RNvXsk_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_17CharSliceSearcherNtB5_15ReverseSearcher16next_reject_back"}
!149 = distinct !{!149, !148, !"_RNvXsk_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_17CharSliceSearcherNtB5_15ReverseSearcher16next_reject_back: %self"}
!150 = !{!151}
!151 = distinct !{!151, !152, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8truncate: %self"}
!152 = distinct !{!152, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8truncate"}
!153 = !{!154}
!154 = distinct !{!154, !155, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCsC3JuwEIQwb_7console4term9TermInnerEE3newB14_: %x"}
!155 = distinct !{!155, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCsC3JuwEIQwb_7console4term9TermInnerEE3newB14_"}
!156 = !{!157, !159, !161}
!157 = distinct !{!157, !158, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_: %self"}
!158 = distinct !{!158, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_"}
!159 = distinct !{!159, !160, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_: %_1"}
!160 = distinct !{!160, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_"}
!161 = distinct !{!161, !162, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_: %_1"}
!162 = distinct !{!162, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_"}
!163 = !{!164}
!164 = distinct !{!164, !165, !"_RNvMsd_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringE3newCsC3JuwEIQwb_7console: %_0"}
!165 = distinct !{!165, !"_RNvMsd_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringE3newCsC3JuwEIQwb_7console"}
!166 = !{!167}
!167 = distinct !{!167, !168, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!168 = distinct !{!168, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!169 = !{!170, !172}
!170 = distinct !{!170, !171, !"_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!171 = distinct !{!171, !"_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!172 = distinct !{!172, !173, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console: %_1"}
!173 = distinct !{!173, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console"}
!174 = !{!175}
!175 = distinct !{!175, !176, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console: %_0"}
!176 = distinct !{!176, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console"}
!177 = !{!178, !180}
!178 = distinct !{!178, !179, !"_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!179 = distinct !{!179, !"_RNvXsh_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!180 = distinct !{!180, !181, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console: %_1"}
!181 = distinct !{!181, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock15RwLockReadGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEECsC3JuwEIQwb_7console"}
!182 = !{!183, !185}
!183 = distinct !{!183, !184, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!184 = distinct !{!184, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!185 = distinct !{!185, !184, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: argument 1"}
!186 = !{!183}
!187 = !{i64 1}
!188 = !{!189}
!189 = distinct !{!189, !190, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!190 = distinct !{!190, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!191 = !{!192}
!192 = distinct !{!192, !193, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechE8push_mutCsC3JuwEIQwb_7console: %self"}
!193 = distinct !{!193, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechE8push_mutCsC3JuwEIQwb_7console"}
!194 = !{!195}
!195 = distinct !{!195, !196, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!196 = distinct !{!196, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!197 = !{!198}
!198 = distinct !{!198, !199, !"_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common: %bytes.0"}
!199 = distinct !{!199, !"_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term20write_through_common"}
!200 = !{!201, !203, !198}
!201 = distinct !{!201, !202, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_: %_0"}
!202 = distinct !{!202, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_"}
!203 = distinct !{!203, !202, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_: %self.1"}
!204 = !{!205, !207, !208, !198}
!205 = distinct !{!205, !206, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_: %t"}
!206 = distinct !{!206, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_"}
!207 = distinct !{!207, !206, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_: %self"}
!208 = distinct !{!208, !206, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_: argument 2"}
!209 = !{!205, !208, !198}
!210 = !{!205, !207, !198}
!211 = !{!212}
!212 = distinct !{!212, !213, !"_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up: %out"}
!213 = distinct !{!213, !"_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up"}
!214 = !{!215}
!215 = distinct !{!215, !216, !"_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up: %out"}
!216 = distinct !{!216, !"_RNvNtCsC3JuwEIQwb_7console11common_term14move_cursor_up"}
!217 = !{!218}
!218 = distinct !{!218, !219, !"_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down: %out"}
!219 = distinct !{!219, !"_RNvNtCsC3JuwEIQwb_7console11common_term16move_cursor_down"}
!220 = !{!221}
!221 = distinct !{!221, !222, !"_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure: %_0"}
!222 = distinct !{!222, !"_RNvNtCsC3JuwEIQwb_7console9unix_term11read_secure"}
!223 = !{!224, !226, !221}
!224 = distinct !{!224, !225, !"_RNvMs_NtCsC3JuwEIQwb_7console9unix_termINtB4_5InputNtNtCs5sEH5CPMdak_3std2fs4FileE10unbuffered: %_0"}
!225 = distinct !{!225, !"_RNvMs_NtCsC3JuwEIQwb_7console9unix_termINtB4_5InputNtNtCs5sEH5CPMdak_3std2fs4FileE10unbuffered"}
!226 = distinct !{!226, !227, !"_RNvMNtCsC3JuwEIQwb_7console9unix_termINtB2_5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBU_2fs4FileEE8buffered: %_0"}
!227 = distinct !{!227, !"_RNvMNtCsC3JuwEIQwb_7console9unix_termINtB2_5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBU_2fs4FileEE8buffered"}
!228 = !{i32 0, i32 2}
!229 = !{!230, !232, !226, !221}
!230 = distinct !{!230, !231, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console: %_0"}
!231 = distinct !{!231, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console"}
!232 = distinct !{!232, !233, !"_RNvMNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB2_9BufReaderNtNtB8_2fs4FileE13with_capacityCsC3JuwEIQwb_7console: %_0"}
!233 = distinct !{!233, !"_RNvMNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB2_9BufReaderNtNtB8_2fs4FileE13with_capacityCsC3JuwEIQwb_7console"}
!234 = !{!232, !226, !221}
!235 = !{!226, !221}
!236 = !{!237}
!237 = distinct !{!237, !238, !"_RNvMs0_NtCsC3JuwEIQwb_7console9unix_termINtB5_5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBX_2fs4FileEE9read_lineB7_: %self"}
!238 = distinct !{!238, !"_RNvMs0_NtCsC3JuwEIQwb_7console9unix_termINtB5_5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBX_2fs4FileEE9read_lineB7_"}
!239 = !{!240}
!240 = distinct !{!240, !238, !"_RNvMs0_NtCsC3JuwEIQwb_7console9unix_termINtB5_5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBX_2fs4FileEE9read_lineB7_: %buf"}
!241 = !{!240, !221}
!242 = !{!243}
!243 = distinct !{!243, !244, !"_RNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBb_2fs4FileENtB9_7BufRead9read_lineCsC3JuwEIQwb_7console: %self"}
!244 = distinct !{!244, !"_RNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBb_2fs4FileENtB9_7BufRead9read_lineCsC3JuwEIQwb_7console"}
!245 = !{!246}
!246 = distinct !{!246, !244, !"_RNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBb_2fs4FileENtB9_7BufRead9read_lineCsC3JuwEIQwb_7console: %buf"}
!247 = !{!248}
!248 = distinct !{!248, !249, !"_RINvNtCs5sEH5CPMdak_3std2io16append_to_stringNCNvYINtNtNtB2_8buffered9bufreader9BufReaderNtNtB4_2fs4FileENtB2_7BufRead9read_line0ECsC3JuwEIQwb_7console: %buf"}
!249 = distinct !{!249, !"_RINvNtCs5sEH5CPMdak_3std2io16append_to_stringNCNvYINtNtNtB2_8buffered9bufreader9BufReaderNtNtB4_2fs4FileENtB2_7BufRead9read_line0ECsC3JuwEIQwb_7console"}
!250 = !{!251}
!251 = distinct !{!251, !249, !"_RINvNtCs5sEH5CPMdak_3std2io16append_to_stringNCNvYINtNtNtB2_8buffered9bufreader9BufReaderNtNtB4_2fs4FileENtB2_7BufRead9read_line0ECsC3JuwEIQwb_7console: %f"}
!252 = !{!248, !251, !243, !246, !237, !240, !221}
!253 = !{!254}
!254 = distinct !{!254, !255, !"_RNCNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBd_2fs4FileENtBb_7BufRead9read_line0CsC3JuwEIQwb_7console: %_1"}
!255 = distinct !{!255, !"_RNCNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBd_2fs4FileENtBb_7BufRead9read_line0CsC3JuwEIQwb_7console"}
!256 = !{!257}
!257 = distinct !{!257, !255, !"_RNCNvYINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBd_2fs4FileENtBb_7BufRead9read_line0CsC3JuwEIQwb_7console: %b"}
!258 = !{!259}
!259 = distinct !{!259, !260, !"_RINvNtCs5sEH5CPMdak_3std2io10read_untilINtNtNtB2_8buffered9bufreader9BufReaderNtNtB4_2fs4FileEECsC3JuwEIQwb_7console: %r"}
!260 = distinct !{!260, !"_RINvNtCs5sEH5CPMdak_3std2io10read_untilINtNtNtB2_8buffered9bufreader9BufReaderNtNtB4_2fs4FileEECsC3JuwEIQwb_7console"}
!261 = !{!262}
!262 = distinct !{!262, !260, !"_RINvNtCs5sEH5CPMdak_3std2io10read_untilINtNtNtB2_8buffered9bufreader9BufReaderNtNtB4_2fs4FileEECsC3JuwEIQwb_7console: %buf"}
!263 = !{!264}
!264 = distinct !{!264, !265, !"_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console: %self"}
!265 = distinct !{!265, !"_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console"}
!266 = !{!267}
!267 = distinct !{!267, !268, !"_RINvMNtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader6bufferNtB3_6Buffer8fill_bufQNtNtBb_2fs4FileECsC3JuwEIQwb_7console: %self"}
!268 = distinct !{!268, !"_RINvMNtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader6bufferNtB3_6Buffer8fill_bufQNtNtBb_2fs4FileECsC3JuwEIQwb_7console"}
!269 = !{!267, !264, !259, !254, !251, !243, !237}
!270 = !{!271, !272, !273, !262, !257, !248, !246, !240, !221}
!271 = distinct !{!271, !268, !"_RINvMNtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader6bufferNtB3_6Buffer8fill_bufQNtNtBb_2fs4FileECsC3JuwEIQwb_7console: %_0"}
!272 = distinct !{!272, !268, !"_RINvMNtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader6bufferNtB3_6Buffer8fill_bufQNtNtBb_2fs4FileECsC3JuwEIQwb_7console: argument 2"}
!273 = distinct !{!273, !265, !"_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead8fill_bufCsC3JuwEIQwb_7console: %_0"}
!274 = !{!271, !267, !272, !273, !264, !259, !262, !254, !257, !248, !251, !243, !246, !237, !240, !221}
!275 = !{!"branch_weights", i32 1, i32 2000, i32 2000, i32 2000, i32 2000}
!276 = !{!277}
!277 = distinct !{!277, !278, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!278 = distinct !{!278, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!279 = !{!262, !257, !221}
!280 = !{!281}
!281 = distinct !{!281, !282, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console: %self"}
!282 = distinct !{!282, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console"}
!283 = !{!284, !281, !262, !257, !248, !246, !240}
!284 = distinct !{!284, !285, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!285 = distinct !{!285, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!286 = !{!259, !254, !251, !243, !237, !221}
!287 = !{!281, !262, !257, !248, !246, !240}
!288 = !{!281, !262, !257, !221}
!289 = !{!290, !259, !254, !251, !243, !237}
!290 = distinct !{!290, !291, !"_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead7consumeCsC3JuwEIQwb_7console: %self"}
!291 = distinct !{!291, !"_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead7consumeCsC3JuwEIQwb_7console"}
!292 = !{!262, !257, !248, !246, !240, !221}
!293 = !{!294}
!294 = distinct !{!294, !295, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console: %self"}
!295 = distinct !{!295, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console"}
!296 = !{!297, !294, !262, !257, !248, !246, !240}
!297 = distinct !{!297, !298, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!298 = distinct !{!298, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!299 = !{!294, !262, !257, !248, !246, !240}
!300 = !{!294, !262, !257, !221}
!301 = !{!302, !259, !254, !251, !243, !237}
!302 = distinct !{!302, !303, !"_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead7consumeCsC3JuwEIQwb_7console: %self"}
!303 = distinct !{!303, !"_RNvXs4_NtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreaderINtB5_9BufReaderNtNtBb_2fs4FileENtB9_7BufRead7consumeCsC3JuwEIQwb_7console"}
!304 = !{i8 0, i8 42}
!305 = !{!306}
!306 = distinct !{!306, !307, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsC3JuwEIQwb_7console9unix_term5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtB1v_2fs4FileEEEBL_: %_1"}
!307 = distinct !{!307, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsC3JuwEIQwb_7console9unix_term5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtB1v_2fs4FileEEEBL_"}
!308 = !{!309}
!309 = distinct !{!309, !310, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console: %_1"}
!310 = distinct !{!310, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console"}
!311 = !{!309, !306}
!312 = !{!309, !306, !221}
!313 = !{!314}
!314 = distinct !{!314, !315, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsC3JuwEIQwb_7console9unix_term5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtB1v_2fs4FileEEEBL_: %_1"}
!315 = distinct !{!315, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsC3JuwEIQwb_7console9unix_term5InputINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtB1v_2fs4FileEEEBL_"}
!316 = !{!317}
!317 = distinct !{!317, !318, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console: %_1"}
!318 = distinct !{!318, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std2io8buffered9bufreader9BufReaderNtNtBP_2fs4FileEECsC3JuwEIQwb_7console"}
!319 = !{!317, !314}
!320 = !{!317, !314, !221}
!321 = !{!"branch_weights", i32 2000, i32 6004}
!322 = !{!323}
!323 = distinct !{!323, !324, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console: %_0"}
!324 = distinct !{!324, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console"}
!325 = !{!326}
!326 = distinct !{!326, !327, !"_RNvMs9_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_6RwLockNtNtCsdJPVW0sQgAG_5alloc6string6StringE5writeCsC3JuwEIQwb_7console: %_0"}
!327 = distinct !{!327, !"_RNvMs9_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_6RwLockNtNtCsdJPVW0sQgAG_5alloc6string6StringE5writeCsC3JuwEIQwb_7console"}
!328 = !{!329, !331}
!329 = distinct !{!329, !330, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!330 = distinct !{!330, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!331 = distinct !{!331, !330, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: argument 1"}
!332 = !{!329}
!333 = !{!334}
!334 = distinct !{!334, !335, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCsC3JuwEIQwb_7console: %_0"}
!335 = distinct !{!335, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCsC3JuwEIQwb_7console"}
!336 = !{!337}
!337 = distinct !{!337, !338, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!338 = distinct !{!338, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!339 = !{!340}
!340 = distinct !{!340, !341, !"_RNvNvMs1_NtCsC3JuwEIQwb_7console4termNtB7_4Term22read_line_initial_text18read_line_internal: argument 2"}
!341 = distinct !{!341, !"_RNvNvMs1_NtCsC3JuwEIQwb_7console4termNtB7_4Term22read_line_initial_text18read_line_internal"}
!342 = !{!343, !344, !340}
!343 = distinct !{!343, !341, !"_RNvNvMs1_NtCsC3JuwEIQwb_7console4termNtB7_4Term22read_line_initial_text18read_line_internal: %_0"}
!344 = distinct !{!344, !341, !"_RNvNvMs1_NtCsC3JuwEIQwb_7console4termNtB7_4Term22read_line_initial_text18read_line_internal: %slf"}
!345 = !{!346}
!346 = distinct !{!346, !347, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VeccEINtB2_18SpecFromIterNestedcNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsE9from_iterCsC3JuwEIQwb_7console: %_0"}
!347 = distinct !{!347, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VeccEINtB2_18SpecFromIterNestedcNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsE9from_iterCsC3JuwEIQwb_7console"}
!348 = !{!346, !343, !344, !340}
!349 = !{!350, !352, !346, !343, !344}
!350 = distinct !{!350, !351, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!351 = distinct !{!351, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!352 = distinct !{!352, !353, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!353 = distinct !{!353, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!354 = !{!"branch_weights", i32 6004, i32 2000}
!355 = !{!356, !346, !343, !344}
!356 = distinct !{!356, !357, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console: %_0"}
!357 = distinct !{!357, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console"}
!358 = !{!346, !343, !344}
!359 = !{!360}
!360 = distinct !{!360, !361, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VeccEINtB2_10SpecExtendcNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsE11spec_extendCsC3JuwEIQwb_7console: %self"}
!361 = distinct !{!361, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VeccEINtB2_10SpecExtendcNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsE11spec_extendCsC3JuwEIQwb_7console"}
!362 = !{!363}
!363 = distinct !{!363, !364, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VeccE16extend_desugaredNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsECsC3JuwEIQwb_7console: %self"}
!364 = distinct !{!364, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VeccE16extend_desugaredNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsECsC3JuwEIQwb_7console"}
!365 = !{!366, !368, !363, !360, !346, !343, !344}
!366 = distinct !{!366, !367, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!367 = distinct !{!367, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!368 = distinct !{!368, !369, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!369 = distinct !{!369, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!370 = !{!363, !360}
!371 = !{!363, !360, !346, !343, !344}
!372 = !{!343, !344}
!373 = !{!374}
!374 = distinct !{!374, !375, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VeccE8push_mutCsC3JuwEIQwb_7console: %self"}
!375 = distinct !{!375, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VeccE8push_mutCsC3JuwEIQwb_7console"}
!376 = !{!374, !343, !344}
!377 = !{!378}
!378 = distinct !{!378, !379, !"_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw: %dst.0"}
!379 = distinct !{!379, !"_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw"}
!380 = !{!343}
!381 = !{i32 0, i32 1114112}
!382 = !{!383, !343, !344, !340}
!383 = distinct !{!383, !384, !"_RNvNtCsC3JuwEIQwb_7console11common_term11clear_chars: %out"}
!384 = distinct !{!384, !"_RNvNtCsC3JuwEIQwb_7console11common_term11clear_chars"}
!385 = !{!383, !343, !344}
!386 = !{!387, !389, !343, !344, !340}
!387 = distinct !{!387, !388, !"_RINvXs6_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorRcE9from_iterINtNtNtBS_8adapters4skip4SkipINtNtNtBU_5slice4iter4ItercEEECsC3JuwEIQwb_7console: %_0"}
!388 = distinct !{!388, !"_RINvXs6_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorRcE9from_iterINtNtNtBS_8adapters4skip4SkipINtNtNtBU_5slice4iter4ItercEEECsC3JuwEIQwb_7console"}
!389 = distinct !{!389, !388, !"_RINvXs6_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorRcE9from_iterINtNtNtBS_8adapters4skip4SkipINtNtNtBU_5slice4iter4ItercEEECsC3JuwEIQwb_7console: %iter"}
!390 = !{!391}
!391 = distinct !{!391, !392, !"_RINvXse_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendRcE6extendINtNtNtBS_8adapters4skip4SkipINtNtNtBU_5slice4iter4ItercEEECsC3JuwEIQwb_7console: %self"}
!392 = distinct !{!392, !"_RINvXse_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendRcE6extendINtNtNtBS_8adapters4skip4SkipINtNtNtBU_5slice4iter4ItercEEECsC3JuwEIQwb_7console"}
!393 = !{!394}
!394 = distinct !{!394, !395, !"_RINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendcE6extendINtNtNtBS_8adapters6cloned6ClonedINtNtB1S_4skip4SkipINtNtNtBU_5slice4iter4ItercEEEECsC3JuwEIQwb_7console: %self"}
!395 = distinct !{!395, !"_RINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendcE6extendINtNtNtBS_8adapters6cloned6ClonedINtNtB1S_4skip4SkipINtNtNtBU_5slice4iter4ItercEEEECsC3JuwEIQwb_7console"}
!396 = !{!387, !389, !343, !344}
!397 = !{!398}
!398 = distinct !{!398, !399, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6cloned6ClonedINtNtB8_4skip4SkipINtNtNtBc_5slice4iter4ItercEEENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2z_6StringINtNtB1N_7collect6ExtendcE6extendB3_E0ECsC3JuwEIQwb_7console: %f"}
!399 = distinct !{!399, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6cloned6ClonedINtNtB8_4skip4SkipINtNtNtBc_5slice4iter4ItercEEENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2z_6StringINtNtB1N_7collect6ExtendcE6extendB3_E0ECsC3JuwEIQwb_7console"}
!400 = !{!401}
!401 = distinct !{!401, !402, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters6clonedINtB5_6ClonedINtNtB7_4skip4SkipINtNtNtBb_5slice4iter4ItercEEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNvB1O_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB31_6StringINtNtB1S_7collect6ExtendcE6extendBP_E0E0ECsC3JuwEIQwb_7console: %f"}
!402 = distinct !{!402, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters6clonedINtB5_6ClonedINtNtB7_4skip4SkipINtNtNtBb_5slice4iter4ItercEEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNvB1O_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB31_6StringINtNtB1S_7collect6ExtendcE6extendBP_E0E0ECsC3JuwEIQwb_7console"}
!403 = !{!404}
!404 = distinct !{!404, !405, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB5_4SkipINtNtNtBb_5slice4iter4ItercEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldRccuNvYcNtNtBb_5clone5Clone5cloneNCINvNvB1r_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3y_6StringINtNtB1v_7collect6ExtendcE6extendINtNtB7_6cloned6ClonedBN_EE0E0E0ECsC3JuwEIQwb_7console: %fold"}
!405 = distinct !{!405, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB5_4SkipINtNtNtBb_5slice4iter4ItercEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldRccuNvYcNtNtBb_5clone5Clone5cloneNCINvNvB1r_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3y_6StringINtNtB1v_7collect6ExtendcE6extendINtNtB7_6cloned6ClonedBN_EE0E0E0ECsC3JuwEIQwb_7console"}
!406 = !{!344, !340}
!407 = !{!408}
!408 = distinct !{!408, !409, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4ItercENtNtNtNtBb_4iter6traits8iterator8Iterator4folduNCINvNtNtBY_8adapters3map8map_foldRccuNvYcNtNtBb_5clone5Clone5cloneNCINvNvBS_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3g_6StringINtNtBW_7collect6ExtendcE6extendINtNtB1K_6cloned6ClonedINtNtB1K_4skip4SkipBF_EEE0E0E0ECsC3JuwEIQwb_7console: argument 0"}
!409 = distinct !{!409, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4ItercENtNtNtNtBb_4iter6traits8iterator8Iterator4folduNCINvNtNtBY_8adapters3map8map_foldRccuNvYcNtNtBb_5clone5Clone5cloneNCINvNvBS_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3g_6StringINtNtBW_7collect6ExtendcE6extendINtNtB1K_6cloned6ClonedINtNtB1K_4skip4SkipBF_EEE0E0E0ECsC3JuwEIQwb_7console"}
!410 = !{!411}
!411 = distinct !{!411, !412, !"_RNvXsF_NtNtCsjMrxcFdYDNN_4core5clone5implscNtB7_5Clone5clone: %self"}
!412 = distinct !{!412, !"_RNvXsF_NtNtCsjMrxcFdYDNN_4core5clone5implscNtB7_5Clone5clone"}
!413 = !{!408, !414, !404, !415, !401, !416, !398, !394, !417, !391, !418, !387, !389, !343, !344}
!414 = distinct !{!414, !405, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB5_4SkipINtNtNtBb_5slice4iter4ItercEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNtB7_3map8map_foldRccuNvYcNtNtBb_5clone5Clone5cloneNCINvNvB1r_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3y_6StringINtNtB1v_7collect6ExtendcE6extendINtNtB7_6cloned6ClonedBN_EE0E0E0ECsC3JuwEIQwb_7console: %self"}
!415 = distinct !{!415, !402, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters6clonedINtB5_6ClonedINtNtB7_4skip4SkipINtNtNtBb_5slice4iter4ItercEEENtNtNtB9_6traits8iterator8Iterator4folduNCINvNvB1O_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB31_6StringINtNtB1S_7collect6ExtendcE6extendBP_E0E0ECsC3JuwEIQwb_7console: %self"}
!416 = distinct !{!416, !399, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6cloned6ClonedINtNtB8_4skip4SkipINtNtNtBc_5slice4iter4ItercEEENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2z_6StringINtNtB1N_7collect6ExtendcE6extendB3_E0ECsC3JuwEIQwb_7console: %self"}
!417 = distinct !{!417, !395, !"_RINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendcE6extendINtNtNtBS_8adapters6cloned6ClonedINtNtB1S_4skip4SkipINtNtNtBU_5slice4iter4ItercEEEECsC3JuwEIQwb_7console: %iter"}
!418 = distinct !{!418, !392, !"_RINvXse_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendRcE6extendINtNtNtBS_8adapters4skip4SkipINtNtNtBU_5slice4iter4ItercEEECsC3JuwEIQwb_7console: %iter"}
!419 = !{!420}
!420 = distinct !{!420, !421, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!421 = distinct !{!421, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!422 = !{!423, !420, !408, !404, !401, !398, !394, !391}
!423 = distinct !{!423, !424, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!424 = distinct !{!424, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!425 = !{!414, !415, !416, !417, !418, !387, !389, !343, !344, !340}
!426 = !{!420, !408, !404, !401, !398, !394, !391}
!427 = !{!420, !408, !414, !404, !415, !401, !416, !398, !394, !417, !391, !418, !387, !389, !343, !344}
!428 = !{!429}
!429 = distinct !{!429, !430, !"_RNvMs9_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_6RwLockNtNtCsdJPVW0sQgAG_5alloc6string6StringE5writeCsC3JuwEIQwb_7console: %_0"}
!430 = distinct !{!430, !"_RNvMs9_NtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlockINtB5_6RwLockNtNtCsdJPVW0sQgAG_5alloc6string6StringE5writeCsC3JuwEIQwb_7console"}
!431 = !{!432, !434}
!432 = distinct !{!432, !433, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!433 = distinct !{!433, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!434 = distinct !{!434, !433, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison6rwlock16RwLockWriteGuardNtNtCsdJPVW0sQgAG_5alloc6string6StringEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: argument 1"}
!435 = !{!432}
!436 = !{!437}
!437 = distinct !{!437, !438, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console: %_0"}
!438 = distinct !{!438, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console"}
!439 = !{!440, !442}
!440 = distinct !{!440, !441, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!441 = distinct !{!441, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!442 = distinct !{!442, !441, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: argument 1"}
!443 = !{!440}
!444 = !{!445}
!445 = distinct !{!445, !446, !"_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term8read_key: %self"}
!446 = distinct !{!446, !"_RNvMs1_NtCsC3JuwEIQwb_7console4termNtB5_4Term8read_key"}
!447 = !{!448}
!448 = distinct !{!448, !449, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console: %_0"}
!449 = distinct !{!449, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console"}
!450 = !{!451, !453}
!451 = distinct !{!451, !452, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!452 = distinct !{!452, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!453 = distinct !{!453, !452, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: argument 1"}
!454 = !{!451}
!455 = !{!456}
!456 = distinct !{!456, !457, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!457 = distinct !{!457, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!458 = !{!459}
!459 = distinct !{!459, !460, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!460 = distinct !{!460, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!461 = !{!462, !464, !465}
!462 = distinct !{!462, !463, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %_0"}
!463 = distinct !{!463, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match"}
!464 = distinct !{!464, !463, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %self"}
!465 = distinct !{!465, !466, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCsC3JuwEIQwb_7console: %self"}
!466 = distinct !{!466, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCsC3JuwEIQwb_7console"}
!467 = !{!468, !470}
!468 = distinct !{!468, !469, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!469 = distinct !{!469, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!470 = distinct !{!470, !469, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!471 = !{!472, !474}
!472 = distinct !{!472, !473, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!473 = distinct !{!473, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!474 = distinct !{!474, !473, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!475 = !{!476, !478}
!476 = distinct !{!476, !477, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!477 = distinct !{!477, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!478 = distinct !{!478, !477, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!479 = !{!480, !482}
!480 = distinct !{!480, !481, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!481 = distinct !{!481, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!482 = distinct !{!482, !481, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!483 = !{!484, !486}
!484 = distinct !{!484, !485, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!485 = distinct !{!485, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!486 = distinct !{!486, !485, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!487 = !{!488, !490}
!488 = distinct !{!488, !489, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!489 = distinct !{!489, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!490 = distinct !{!490, !489, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!491 = !{!492, !494}
!492 = distinct !{!492, !493, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!493 = distinct !{!493, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!494 = distinct !{!494, !493, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!495 = !{!496, !498}
!496 = distinct !{!496, !497, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!497 = distinct !{!497, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!498 = distinct !{!498, !497, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!499 = !{!500, !502}
!500 = distinct !{!500, !501, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!501 = distinct !{!501, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!502 = distinct !{!502, !501, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!503 = !{!504, !506}
!504 = distinct !{!504, !505, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!505 = distinct !{!505, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!506 = distinct !{!506, !505, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!507 = !{!508, !510}
!508 = distinct !{!508, !509, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!509 = distinct !{!509, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!510 = distinct !{!510, !509, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!511 = !{!512, !514}
!512 = distinct !{!512, !513, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!513 = distinct !{!513, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!514 = distinct !{!514, !513, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!515 = !{!516, !518}
!516 = distinct !{!516, !517, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!517 = distinct !{!517, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!518 = distinct !{!518, !517, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!519 = !{!520, !522}
!520 = distinct !{!520, !521, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!521 = distinct !{!521, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!522 = distinct !{!522, !521, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!523 = !{!524, !526}
!524 = distinct !{!524, !525, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!525 = distinct !{!525, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!526 = distinct !{!526, !525, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!527 = !{!528, !530}
!528 = distinct !{!528, !529, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!529 = distinct !{!529, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!530 = distinct !{!530, !529, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!531 = !{!532, !534}
!532 = distinct !{!532, !533, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!533 = distinct !{!533, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!534 = distinct !{!534, !533, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!535 = !{!536, !538}
!536 = distinct !{!536, !537, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!537 = distinct !{!537, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!538 = distinct !{!538, !537, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!539 = !{!540, !542}
!540 = distinct !{!540, !541, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!541 = distinct !{!541, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!542 = distinct !{!542, !541, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!543 = !{!544, !546}
!544 = distinct !{!544, !545, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!545 = distinct !{!545, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!546 = distinct !{!546, !545, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!547 = !{!548, !550}
!548 = distinct !{!548, !549, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!549 = distinct !{!549, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!550 = distinct !{!550, !549, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!551 = !{!552, !554}
!552 = distinct !{!552, !553, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!553 = distinct !{!553, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!554 = distinct !{!554, !553, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!555 = !{!556, !558}
!556 = distinct !{!556, !557, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!557 = distinct !{!557, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!558 = distinct !{!558, !557, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!559 = !{!560, !562}
!560 = distinct !{!560, !561, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!561 = distinct !{!561, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!562 = distinct !{!562, !561, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!563 = !{!564, !566}
!564 = distinct !{!564, !565, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!565 = distinct !{!565, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!566 = distinct !{!566, !565, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!567 = !{!568, !570}
!568 = distinct !{!568, !569, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!569 = distinct !{!569, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!570 = distinct !{!570, !569, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!571 = !{!572, !574}
!572 = distinct !{!572, !573, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!573 = distinct !{!573, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!574 = distinct !{!574, !573, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!575 = !{!576}
!576 = distinct !{!576, !577, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!577 = distinct !{!577, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!578 = !{!579}
!579 = distinct !{!579, !580, !"_RNvMsi_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE6insertB1b_: %self"}
!580 = distinct !{!580, !"_RNvMsi_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE6insertB1b_"}
!581 = !{!582}
!582 = distinct !{!582, !583, !"_RNvMsi_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE5entryB1b_: %self"}
!583 = distinct !{!583, !"_RNvMsi_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE5entryB1b_"}
!584 = !{!582, !579}
!585 = !{!586}
!586 = distinct !{!586, !583, !"_RNvMsi_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8BTreeMapNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE5entryB1b_: %_0"}
!587 = !{!588, !586, !582, !579}
!588 = distinct !{!588, !589, !"_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree6searchINtNtB7_4node7NodeRefNtNtBY_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1i_14LeafOrInternalE11search_treeB1y_EB1C_: %_0"}
!589 = distinct !{!589, !"_RINvMs_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree6searchINtNtB7_4node7NodeRefNtNtBY_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1i_14LeafOrInternalE11search_treeB1y_EB1C_"}
!590 = !{i8 0, i8 9}
!591 = !{!592}
!592 = distinct !{!592, !593, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertNtNtCsC3JuwEIQwb_7console5utils9AttributeEB18_: %slice.0"}
!593 = distinct !{!593, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertNtNtCsC3JuwEIQwb_7console5utils9AttributeEB18_"}
!594 = !{!595, !597, !598, !600, !601, !603, !604, !606, !579}
!595 = distinct !{!595, !596, !"_RNvMsJ_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_4LeafENtB1m_4EdgeE10insert_fitB1G_: %_0"}
!596 = distinct !{!596, !"_RNvMsJ_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_4LeafENtB1m_4EdgeE10insert_fitB1G_"}
!597 = distinct !{!597, !596, !"_RNvMsJ_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_4LeafENtB1m_4EdgeE10insert_fitB1G_: %self"}
!598 = distinct !{!598, !599, !"_RINvMsK_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_4EdgeE6insertNtNtBc_5alloc6GlobalEB1H_: %_0"}
!599 = distinct !{!599, !"_RINvMsK_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_4EdgeE6insertNtNtBc_5alloc6GlobalEB1H_"}
!600 = distinct !{!600, !599, !"_RINvMsK_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_4EdgeE6insertNtNtBc_5alloc6GlobalEB1H_: %self"}
!601 = distinct !{!601, !602, !"_RINvMsN_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_4EdgeE16insert_recursingNtNtBc_5alloc6GlobalNCNvMs6_NtNtB8_3map5entryINtB3P_11VacantEntryB1D_B2i_E12insert_entry0EB1H_: %_0"}
!602 = distinct !{!602, !"_RINvMsN_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_4EdgeE16insert_recursingNtNtBc_5alloc6GlobalNCNvMs6_NtNtB8_3map5entryINtB3P_11VacantEntryB1D_B2i_E12insert_entry0EB1H_"}
!603 = distinct !{!603, !602, !"_RINvMsN_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_4EdgeE16insert_recursingNtNtBc_5alloc6GlobalNCNvMs6_NtNtB8_3map5entryINtB3P_11VacantEntryB1D_B2i_E12insert_entry0EB1H_: %self"}
!604 = distinct !{!604, !605, !"_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_: %_0"}
!605 = distinct !{!605, !"_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_"}
!606 = distinct !{!606, !605, !"_RNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB5_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB9_7set_val9SetValZSTE12insert_entryB1n_: %self"}
!607 = !{!608, !610, !598, !600, !601, !603, !604, !606, !579}
!608 = distinct !{!608, !609, !"_RINvMsV_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_: %_0"}
!609 = distinct !{!609, !"_RINvMsV_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_"}
!610 = distinct !{!610, !609, !"_RINvMsV_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_4LeafENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_: %self"}
!611 = !{!612}
!612 = distinct !{!612, !613, !"_RNvMsU_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_4LeafENtB1m_2KVE15split_leaf_dataB1G_: %new_node"}
!613 = distinct !{!613, !"_RNvMsU_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_4LeafENtB1m_2KVE15split_leaf_dataB1G_"}
!614 = !{!612, !608, !610, !598, !600, !601, !603, !604, !606, !579}
!615 = !{!"branch_weights", i32 4000000, i32 4001}
!616 = !{!617, !619}
!617 = distinct !{!617, !618, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceNtNtCsC3JuwEIQwb_7console5utils9AttributeEB19_: %src.0"}
!618 = distinct !{!618, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceNtNtCsC3JuwEIQwb_7console5utils9AttributeEB19_"}
!619 = distinct !{!619, !618, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceNtNtCsC3JuwEIQwb_7console5utils9AttributeEB19_: %dst.0"}
!620 = !{!621, !623, !598, !600, !601, !603, !604, !606, !579}
!621 = distinct !{!621, !622, !"_RNvMsJ_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_4LeafENtB1m_4EdgeE10insert_fitB1G_: %_0"}
!622 = distinct !{!622, !"_RNvMsJ_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_4LeafENtB1m_4EdgeE10insert_fitB1G_"}
!623 = distinct !{!623, !622, !"_RNvMsJ_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_4LeafENtB1m_4EdgeE10insert_fitB1G_: %self"}
!624 = !{!625}
!625 = distinct !{!625, !626, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertNtNtCsC3JuwEIQwb_7console5utils9AttributeEB18_: %slice.0"}
!626 = distinct !{!626, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertNtNtCsC3JuwEIQwb_7console5utils9AttributeEB18_"}
!627 = !{!628, !601, !603, !604, !606, !579}
!628 = distinct !{!628, !629, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB19_14LeafOrInternalE6ascendB1t_: %_0"}
!629 = distinct !{!629, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB19_14LeafOrInternalE6ascendB1t_"}
!630 = !{!631}
!631 = distinct !{!631, !632, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node7NodeRefNtNtB10_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1k_14LeafOrInternalEuNCINvB2_8take_mutBX_NCINvMss_B10_BX_19push_internal_levelNtNtB8_5alloc6GlobalE0E0EB1H_: %v"}
!632 = distinct !{!632, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node7NodeRefNtNtB10_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1k_14LeafOrInternalEuNCINvB2_8take_mutBX_NCINvMss_B10_BX_19push_internal_levelNtNtB8_5alloc6GlobalE0E0EB1H_"}
!633 = !{!631, !634, !601, !603, !604, !606, !579}
!634 = distinct !{!634, !635, !"_RNCNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB7_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBb_7set_val9SetValZSTE12insert_entry0B1p_: %ins"}
!635 = distinct !{!635, !"_RNCNvMs6_NtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map5entryINtB7_11VacantEntryNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBb_7set_val9SetValZSTE12insert_entry0B1p_"}
!636 = !{!637, !639, !631, !634, !601, !603, !604, !606, !579}
!637 = distinct !{!637, !638, !"_RINvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_8InternalE30correct_childrens_parent_linksINtNtNtCsjMrxcFdYDNN_4core3ops5range14RangeInclusivejEEB1u_: %range"}
!638 = distinct !{!638, !"_RINvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_8InternalE30correct_childrens_parent_linksINtNtNtCsjMrxcFdYDNN_4core3ops5range14RangeInclusivejEEB1u_"}
!639 = distinct !{!639, !640, !"_RINvMs9_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_8InternalE17from_new_internalNtNtBc_5alloc6GlobalEB1w_: %internal"}
!640 = distinct !{!640, !"_RINvMs9_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_8InternalE17from_new_internalNtNtBc_5alloc6GlobalEB1w_"}
!641 = !{!637, !631, !634, !601, !603, !604, !606, !579}
!642 = !{!631, !579}
!643 = !{!634, !601, !603, !604, !606}
!644 = !{!634, !601, !603, !604, !606, !579}
!645 = !{!646, !648, !601, !603, !604, !606, !579}
!646 = distinct !{!646, !647, !"_RINvMsM_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_4EdgeE6insertNtNtBc_5alloc6GlobalEB1H_: %_0"}
!647 = distinct !{!647, !"_RINvMsM_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_4EdgeE6insertNtNtBc_5alloc6GlobalEB1H_"}
!648 = distinct !{!648, !647, !"_RINvMsM_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_4EdgeE6insertNtNtBc_5alloc6GlobalEB1H_: %self"}
!649 = !{!650}
!650 = distinct !{!650, !651, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertNtNtCsC3JuwEIQwb_7console5utils9AttributeEB18_: %slice.0"}
!651 = distinct !{!651, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertNtNtCsC3JuwEIQwb_7console5utils9AttributeEB18_"}
!652 = !{!653, !646, !648, !601, !603, !604, !606, !579}
!653 = distinct !{!653, !654, !"_RNvMsM_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_8InternalENtB1m_4EdgeE10insert_fitB1G_: %self"}
!654 = distinct !{!654, !"_RNvMsM_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_8InternalENtB1m_4EdgeE10insert_fitB1G_"}
!655 = !{!656}
!656 = distinct !{!656, !657, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_: %slice.0"}
!657 = distinct !{!657, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_"}
!658 = !{!659, !661, !646, !648, !601, !603, !604, !606, !579}
!659 = distinct !{!659, !660, !"_RINvMsW_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_: %_0"}
!660 = distinct !{!660, !"_RINvMsW_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_"}
!661 = distinct !{!661, !660, !"_RINvMsW_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_6HandleINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1n_8InternalENtB1n_2KVE5splitNtNtBc_5alloc6GlobalEB1H_: %self"}
!662 = !{!663}
!663 = distinct !{!663, !664, !"_RNvMsU_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_8InternalENtB1m_2KVE15split_leaf_dataB1G_: %new_node"}
!664 = distinct !{!664, !"_RNvMsU_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_8InternalENtB1m_2KVE15split_leaf_dataB1G_"}
!665 = !{!663, !659, !661, !646, !648, !601, !603, !604, !606, !579}
!666 = !{!667, !669}
!667 = distinct !{!667, !668, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceNtNtCsC3JuwEIQwb_7console5utils9AttributeEB19_: %src.0"}
!668 = distinct !{!668, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceNtNtCsC3JuwEIQwb_7console5utils9AttributeEB19_"}
!669 = distinct !{!669, !668, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceNtNtCsC3JuwEIQwb_7console5utils9AttributeEB19_: %dst.0"}
!670 = !{!671, !673}
!671 = distinct !{!671, !672, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB29_: %src.0"}
!672 = distinct !{!672, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB29_"}
!673 = distinct !{!673, !672, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node13move_to_sliceINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB29_: %dst.0"}
!674 = !{!675}
!675 = distinct !{!675, !676, !"_RINvMs9_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_8InternalE17from_new_internalNtNtBc_5alloc6GlobalEB1w_: %internal"}
!676 = distinct !{!676, !"_RINvMs9_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5OwnedNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_8InternalE17from_new_internalNtNtBc_5alloc6GlobalEB1w_"}
!677 = !{!678, !659, !661, !646, !648, !601, !603, !604, !606, !579}
!678 = distinct !{!678, !679, !"_RINvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_8InternalE30correct_childrens_parent_linksINtNtNtCsjMrxcFdYDNN_4core3ops5range14RangeInclusivejEEB1u_: %range"}
!679 = distinct !{!679, !"_RINvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_8InternalE30correct_childrens_parent_linksINtNtNtCsjMrxcFdYDNN_4core3ops5range14RangeInclusivejEEB1u_"}
!680 = !{!678, !675, !659, !661, !646, !648, !601, !603, !604, !606, !579}
!681 = !{!682, !646, !648, !601, !603, !604, !606, !579}
!682 = distinct !{!682, !683, !"_RNvMsM_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_8InternalENtB1m_4EdgeE10insert_fitB1G_: %self"}
!683 = distinct !{!683, !"_RNvMsM_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_6HandleINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1m_8InternalENtB1m_4EdgeE10insert_fitB1G_"}
!684 = !{!685}
!685 = distinct !{!685, !686, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertNtNtCsC3JuwEIQwb_7console5utils9AttributeEB18_: %slice.0"}
!686 = distinct !{!686, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertNtNtCsC3JuwEIQwb_7console5utils9AttributeEB18_"}
!687 = !{!688}
!688 = distinct !{!688, !689, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_: %slice.0"}
!689 = distinct !{!689, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4node12slice_insertINtNtNtCsjMrxcFdYDNN_4core3ptr8non_null7NonNullINtB2_8LeafNodeNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTEEEB28_"}
!690 = !{!604, !606, !579}
!691 = !{!604, !606}
!692 = !{!693, !604, !606, !579}
!693 = distinct !{!693, !694, !"_RNvMsu_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB19_4LeafE16push_with_handleB1t_: %_0"}
!694 = distinct !{!694, !"_RNvMsu_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker3MutNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB19_4LeafE16push_with_handleB1t_"}
!695 = !{!696}
!696 = distinct !{!696, !697, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsC3JuwEIQwb_7console: %self"}
!697 = distinct !{!697, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsC3JuwEIQwb_7console"}
!698 = !{!699}
!699 = distinct !{!699, !700, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!700 = distinct !{!700, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!701 = !{!702}
!702 = distinct !{!702, !703, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range7RangeTojEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!703 = distinct !{!703, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range7RangeTojEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!704 = !{!"branch_weights", i32 2002, i32 2000}
!705 = !{!706}
!706 = distinct !{!706, !707, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console: %_1"}
!707 = distinct !{!707, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECsC3JuwEIQwb_7console"}
!708 = !{!709}
!709 = distinct !{!709, !710, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console: %self"}
!710 = distinct !{!710, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsC3JuwEIQwb_7console"}
!711 = !{!709, !706, !712}
!712 = distinct !{!712, !713, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console: %_1"}
!713 = distinct !{!713, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECsC3JuwEIQwb_7console"}
!714 = !{!709, !706}
!715 = !{!716}
!716 = distinct !{!716, !717, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE16deallocating_endNtNtBc_5alloc6GlobalEB1O_: %self"}
!717 = distinct !{!717, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE16deallocating_endNtNtBc_5alloc6GlobalEB1O_"}
!718 = !{!719}
!719 = distinct !{!719, !720, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10take_frontB1N_: %self"}
!720 = distinct !{!720, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10take_frontB1N_"}
!721 = !{!719, !716}
!722 = !{!723}
!723 = distinct !{!723, !720, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10take_frontB1N_: %_0"}
!724 = !{!723, !719, !716}
!725 = !{!726, !728, !716}
!726 = distinct !{!726, !727, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB19_14LeafOrInternalE6ascendB1v_: %_0"}
!727 = distinct !{!727, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB19_14LeafOrInternalE6ascendB1v_"}
!728 = distinct !{!728, !729, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalEB1w_: %ret"}
!729 = distinct !{!729, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalEB1w_"}
!730 = !{!728, !716}
!731 = !{!732}
!732 = distinct !{!732, !733, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalEB1O_: %self"}
!733 = distinct !{!733, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalEB1O_"}
!734 = !{!735}
!735 = distinct !{!735, !736, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10init_frontB1N_: %self"}
!736 = distinct !{!736, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTE10init_frontB1N_"}
!737 = !{!735, !732}
!738 = !{!739}
!739 = distinct !{!739, !733, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalEB1O_: %_0"}
!740 = !{!741, !732}
!741 = distinct !{!741, !742, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2w_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0EB1V_: %v"}
!742 = distinct !{!742, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2w_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0EB1V_"}
!743 = !{!744, !739}
!744 = distinct !{!744, !742, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB4_7set_val9SetValZSTNtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2w_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0EB1V_: %ret"}
!745 = !{!735, !739, !732}
!746 = !{!741}
!747 = !{!748, !750, !751, !753, !744, !741, !739, !732}
!748 = distinct !{!748, !749, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalEB1W_: %_0"}
!749 = distinct !{!749, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalEB1W_"}
!750 = distinct !{!750, !749, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalEB1W_: %self"}
!751 = distinct !{!751, !752, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBa_7set_val9SetValZSTNtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0B1Y_: %val"}
!752 = distinct !{!752, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBa_7set_val9SetValZSTNtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0B1Y_"}
!753 = distinct !{!753, !752, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtBa_7set_val9SetValZSTNtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0B1Y_: %leaf_edge"}
!754 = !{!755, !757, !748, !750, !751, !753, !744, !741, !739, !732}
!755 = distinct !{!755, !756, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB19_14LeafOrInternalE6ascendB1v_: %_0"}
!756 = distinct !{!756, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB19_14LeafOrInternalE6ascendB1v_"}
!757 = distinct !{!757, !758, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalEB1w_: %ret"}
!758 = distinct !{!758, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB8_7set_val9SetValZSTNtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalEB1w_"}
!759 = !{!760, !762, !748, !750, !751, !753, !744, !741, !739, !732}
!760 = distinct !{!760, !761, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeB1V_: %_0"}
!761 = distinct !{!761, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeB1V_"}
!762 = distinct !{!762, !761, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtCsC3JuwEIQwb_7console5utils9AttributeNtNtB7_7set_val9SetValZSTNtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeB1V_: %self"}
!763 = !{!757, !748, !750, !751, !753, !744, !741, !739, !732}
!764 = !{!744, !741, !739, !732}
!765 = !{!739, !732}
!766 = !{!767, !769}
!767 = distinct !{!767, !768, !"_RINvXs7_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorReE9from_iterINtNtNtBS_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2F_16strip_ansi_codes0EEB2H_: %_0"}
!768 = distinct !{!768, !"_RINvXs7_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorReE9from_iterINtNtNtBS_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2F_16strip_ansi_codes0EEB2H_"}
!769 = distinct !{!769, !768, !"_RINvXs7_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorReE9from_iterINtNtNtBS_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2F_16strip_ansi_codes0EEB2H_: %iter"}
!770 = !{!771}
!771 = distinct !{!771, !772, !"_RNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB1C_16strip_ansi_codes0ENtB5_13SpecExtendStr16spec_extend_intoB1E_: %target"}
!772 = distinct !{!772, !"_RNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB1C_16strip_ansi_codes0ENtB5_13SpecExtendStr16spec_extend_intoB1E_"}
!773 = !{!774}
!774 = distinct !{!774, !775, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB17_16strip_ansi_codes0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringB3_NtB38_13SpecExtendStr16spec_extend_into0EB19_: %f"}
!775 = distinct !{!775, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB17_16strip_ansi_codes0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringB3_NtB38_13SpecExtendStr16spec_extend_into0EB19_"}
!776 = !{!777}
!777 = distinct !{!777, !778, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_mapINtB6_9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB1d_16strip_ansi_codes0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB2p_8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringBV_NtB3C_13SpecExtendStr16spec_extend_into0E0EB1f_: %fold"}
!778 = distinct !{!778, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_mapINtB6_9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB1d_16strip_ansi_codes0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB2p_8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringBV_NtB3C_13SpecExtendStr16spec_extend_into0E0EB1f_"}
!779 = !{!780, !777, !781, !774, !782, !771, !767, !769}
!780 = distinct !{!780, !778, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_mapINtB6_9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB1d_16strip_ansi_codes0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB2p_8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringBV_NtB3C_13SpecExtendStr16spec_extend_into0E0EB1f_: %self"}
!781 = distinct !{!781, !775, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB17_16strip_ansi_codes0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringB3_NtB38_13SpecExtendStr16spec_extend_into0EB19_: %self"}
!782 = distinct !{!782, !772, !"_RNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB1C_16strip_ansi_codes0ENtB5_13SpecExtendStr16spec_extend_intoB1E_: %self"}
!783 = !{!777, !774, !771, !767}
!784 = !{!785}
!785 = distinct !{!785, !786, !"_RINvYNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4folduNCINvNtNtBV_8adapters10filter_map15filter_map_foldTRebEB2F_uNCNvB5_16strip_ansi_codes0NCINvNvBP_8for_each4callB2F_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB1V_9FilterMapB3_B2O_ENtB3O_13SpecExtendStr16spec_extend_into0E0E0EB7_: argument 1"}
!786 = distinct !{!786, !"_RINvYNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4folduNCINvNtNtBV_8adapters10filter_map15filter_map_foldTRebEB2F_uNCNvB5_16strip_ansi_codes0NCINvNvBP_8for_each4callB2F_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB1V_9FilterMapB3_B2O_ENtB3O_13SpecExtendStr16spec_extend_into0E0E0EB7_"}
!787 = !{!788, !785, !780, !777, !781, !774, !782, !771, !767, !769}
!788 = distinct !{!788, !786, !"_RINvYNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4folduNCINvNtNtBV_8adapters10filter_map15filter_map_foldTRebEB2F_uNCNvB5_16strip_ansi_codes0NCINvNvBP_8for_each4callB2F_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB1V_9FilterMapB3_B2O_ENtB3O_13SpecExtendStr16spec_extend_into0E0E0EB7_: %self"}
!789 = !{i8 0, i8 3}
!790 = !{!791}
!791 = distinct !{!791, !792, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console: %self"}
!792 = distinct !{!792, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCsC3JuwEIQwb_7console"}
!793 = !{!794, !791, !785, !777, !774, !771}
!794 = distinct !{!794, !795, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!795 = distinct !{!795, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!796 = !{!797, !799, !801, !788, !780, !781, !782, !767, !769}
!797 = distinct !{!797, !798, !"_RNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB1E_16strip_ansi_codes0ENtB7_13SpecExtendStr16spec_extend_into0B1G_: %s.0"}
!798 = distinct !{!798, !"_RNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB1E_16strip_ansi_codes0ENtB7_13SpecExtendStr16spec_extend_into0B1G_"}
!799 = distinct !{!799, !800, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtBc_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2z_16strip_ansi_codes0ENtB1p_13SpecExtendStr16spec_extend_into0E0B2B_: %item.0"}
!800 = distinct !{!800, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callReNCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtNtNtBc_8adapters10filter_map9FilterMapNtNtCsC3JuwEIQwb_7console4ansi16AnsiCodeIteratorNCNvB2z_16strip_ansi_codes0ENtB1p_13SpecExtendStr16spec_extend_into0E0B2B_"}
!801 = distinct !{!801, !802, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map15filter_map_foldTRebEB1b_uNCNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1b_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB4_9FilterMapNtB1o_16AnsiCodeIteratorB1k_ENtB3e_13SpecExtendStr16spec_extend_into0E0E0B1q_: %item"}
!802 = distinct !{!802, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters10filter_map15filter_map_foldTRebEB1b_uNCNvNtCsC3JuwEIQwb_7console4ansi16strip_ansi_codes0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1b_NCNvXsg_NtCsdJPVW0sQgAG_5alloc6stringINtB4_9FilterMapNtB1o_16AnsiCodeIteratorB1k_ENtB3e_13SpecExtendStr16spec_extend_into0E0E0B1q_"}
!803 = !{!791, !785, !777, !774, !771}
!804 = !{!791, !801, !788, !785, !780, !777, !781, !774, !782, !771, !767, !769}
!805 = !{!806}
!806 = distinct !{!806, !807, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console: %self"}
!807 = distinct !{!807, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console"}
!808 = !{!809}
!809 = distinct !{!809, !807, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console: %f"}
!810 = !{!811}
!811 = distinct !{!811, !812, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console: %_1"}
!812 = distinct !{!812, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console"}
!813 = !{!814}
!814 = distinct !{!814, !815, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!815 = distinct !{!815, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!816 = !{!817}
!817 = distinct !{!817, !818, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!818 = distinct !{!818, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!819 = !{!820, !817, !814, !811, !809}
!820 = distinct !{!820, !821, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCsC3JuwEIQwb_7console: %self"}
!821 = distinct !{!821, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCsC3JuwEIQwb_7console"}
!822 = !{!817, !814, !811, !806, !809}
!823 = !{!824, !817, !814, !811, !809}
!824 = distinct !{!824, !825, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCsC3JuwEIQwb_7console: %self"}
!825 = distinct !{!825, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCsC3JuwEIQwb_7console"}
!826 = !{!827, !817, !814, !811, !809}
!827 = distinct !{!827, !828, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCsC3JuwEIQwb_7console: %self"}
!828 = distinct !{!828, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCsC3JuwEIQwb_7console"}
!829 = !{!830, !817, !814, !811, !809}
!830 = distinct !{!830, !831, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCsC3JuwEIQwb_7console: %self"}
!831 = distinct !{!831, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCsC3JuwEIQwb_7console"}
!832 = !{!814, !811, !809}
!833 = !{!834, !836, !838, !840}
!834 = distinct !{!834, !835, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!835 = distinct !{!835, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!836 = distinct !{!836, !837, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!837 = distinct !{!837, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!838 = distinct !{!838, !839, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console: %_1"}
!839 = distinct !{!839, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0CsC3JuwEIQwb_7console"}
!840 = distinct !{!840, !841, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console: %f"}
!841 = distinct !{!841, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console"}
!842 = !{!843}
!843 = distinct !{!843, !841, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECsC3JuwEIQwb_7console: %self"}
!844 = !{!836, !838, !840}
!845 = !{!840}
!846 = !{!838}
!847 = !{!836}
!848 = !{!834}
!849 = !{!834, !836, !838, !843, !840}
!850 = distinct !{!850, !851}
!851 = !{!"llvm.loop.peeled.count", i32 1}
!852 = !{!853}
!853 = distinct !{!853, !854, !"_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width: %s.0"}
!854 = distinct !{!854, !"_RNvNtCsC3JuwEIQwb_7console5utils18measure_text_width"}
!855 = !{!856}
!856 = distinct !{!856, !857, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!857 = distinct !{!857, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!858 = !{!859, !856}
!859 = distinct !{!859, !860, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!860 = distinct !{!860, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!861 = !{!862}
!862 = distinct !{!862, !863, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!863 = distinct !{!863, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!864 = !{!865}
!865 = distinct !{!865, !866, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!866 = distinct !{!866, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!867 = !{!868, !865}
!868 = distinct !{!868, !869, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!869 = distinct !{!869, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!870 = !{!871}
!871 = distinct !{!871, !872, !"_RNvMs4_NtCsC3JuwEIQwb_7console4ansiNtB5_16AnsiCodeIterator13current_slice: %self"}
!872 = distinct !{!872, !"_RNvMs4_NtCsC3JuwEIQwb_7console4ansiNtB5_16AnsiCodeIterator13current_slice"}
!873 = !{!874}
!874 = distinct !{!874, !875, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range7RangeTojEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!875 = distinct !{!875, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range7RangeTojEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!876 = !{!877}
!877 = distinct !{!877, !878, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!878 = distinct !{!878, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!879 = !{!880}
!880 = distinct !{!880, !881, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range7RangeTojEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!881 = distinct !{!881, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range7RangeTojEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!882 = !{!883}
!883 = distinct !{!883, !884, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console: %_0"}
!884 = distinct !{!884, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console"}
!885 = !{!886}
!886 = distinct !{!886, !887, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!887 = distinct !{!887, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!888 = !{!889, !891}
!889 = distinct !{!889, !890, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console: %_0"}
!890 = distinct !{!890, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console"}
!891 = distinct !{!891, !892, !"_RNCNvNtCsC3JuwEIQwb_7console5utils22default_colors_enabled0B5_: %_0"}
!892 = distinct !{!892, !"_RNCNvNtCsC3JuwEIQwb_7console5utils22default_colors_enabled0B5_"}
!893 = !{!891}
!894 = !{!895, !897}
!895 = distinct !{!895, !896, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console: %_0"}
!896 = distinct !{!896, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsC3JuwEIQwb_7console"}
!897 = distinct !{!897, !898, !"_RNCNvNtCsC3JuwEIQwb_7console5utils22default_colors_enableds_0B5_: %_0"}
!898 = distinct !{!898, !"_RNCNvNtCsC3JuwEIQwb_7console5utils22default_colors_enableds_0B5_"}
!899 = !{!897}
!900 = !{!901, !903}
!901 = distinct !{!901, !902, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations23next_code_point_reverseINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!902 = distinct !{!902, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations23next_code_point_reverseINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!903 = distinct !{!903, !904, !"_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back: %self"}
!904 = distinct !{!904, !"_RNvXs0_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5CharsNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back"}
!905 = !{!906}
!906 = distinct !{!906, !907, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables44starts_non_ideographic_text_presentation_seq0ECsC3JuwEIQwb_7console: %self.0"}
!907 = distinct !{!907, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables44starts_non_ideographic_text_presentation_seq0ECsC3JuwEIQwb_7console"}
!908 = !{!909}
!909 = distinct !{!909, !907, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables44starts_non_ideographic_text_presentation_seq0ECsC3JuwEIQwb_7console: argument 1"}
!910 = !{!911}
!911 = distinct !{!911, !912, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!912 = distinct !{!912, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!913 = !{!914}
!914 = distinct !{!914, !915, !"_RNvMs_NtCsC3JuwEIQwb_7console9unix_termINtB4_5InputNtNtCs5sEH5CPMdak_3std2fs4FileE10unbuffered: %_0"}
!915 = distinct !{!915, !"_RNvMs_NtCsC3JuwEIQwb_7console9unix_termINtB4_5InputNtNtCs5sEH5CPMdak_3std2fs4FileE10unbuffered"}
!916 = !{!917}
!917 = distinct !{!917, !918, !"_RNvNtCsC3JuwEIQwb_7console9unix_term20read_single_key_impl: %_0"}
!918 = distinct !{!918, !"_RNvNtCsC3JuwEIQwb_7console9unix_term20read_single_key_impl"}
!919 = !{i32 0, i32 1114113}
!920 = !{!921, !917}
!921 = distinct !{!921, !922, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes: %_0"}
!922 = distinct !{!922, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes"}
!923 = !{!924}
!924 = distinct !{!924, !922, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes: %buf.0"}
!925 = !{!926, !928, !917}
!926 = distinct !{!926, !927, !"_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8: %_0"}
!927 = distinct !{!927, !"_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8"}
!928 = distinct !{!928, !927, !"_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8: %buf.0"}
!929 = !{!930, !926, !917}
!930 = distinct !{!930, !931, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!931 = distinct !{!931, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!932 = !{!933, !917}
!933 = distinct !{!933, !934, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes: %_0"}
!934 = distinct !{!934, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes"}
!935 = !{!936}
!936 = distinct !{!936, !934, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes: %buf.0"}
!937 = !{!938, !940, !917}
!938 = distinct !{!938, !939, !"_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8: %_0"}
!939 = distinct !{!939, !"_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8"}
!940 = distinct !{!940, !939, !"_RNvNtCsC3JuwEIQwb_7console9unix_term13key_from_utf8: %buf.0"}
!941 = !{!942, !938, !917}
!942 = distinct !{!942, !943, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console: %bytes"}
!943 = distinct !{!943, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECsC3JuwEIQwb_7console"}
!944 = !{!945, !917}
!945 = distinct !{!945, !946, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes: %_0"}
!946 = distinct !{!946, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes"}
!947 = !{!948}
!948 = distinct !{!948, !946, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes: %buf.0"}
!949 = !{i64 0, i64 -9223372036854775786}
!950 = !{!951}
!951 = distinct !{!951, !952, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes: %buf.0"}
!952 = distinct !{!952, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes"}
!953 = !{!954}
!954 = distinct !{!954, !952, !"_RNvNtCsC3JuwEIQwb_7console9unix_term10read_bytes: %_0"}
!955 = !{!954, !951}
!956 = !{!957}
!957 = distinct !{!957, !958, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console: %_1"}
!958 = distinct !{!958, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console"}
!959 = !{!960}
!960 = distinct !{!960, !961, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console: %_1"}
!961 = distinct !{!961, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console"}
!962 = !{!963, !965}
!963 = distinct !{!963, !964, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %self.0"}
!964 = distinct !{!964, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console"}
!965 = distinct !{!965, !964, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsC3JuwEIQwb_7console: %other.0"}
!966 = !{!967}
!967 = distinct !{!967, !968, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console: %_1"}
!968 = distinct !{!968, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsC3JuwEIQwb_7console"}
!969 = !{!970}
!970 = distinct !{!970, !971, !"_RNvNtCsC3JuwEIQwb_7console9unix_term9select_fd: %_0"}
!971 = distinct !{!971, !"_RNvNtCsC3JuwEIQwb_7console9unix_term9select_fd"}
!972 = !{!973}
!973 = distinct !{!973, !974, !"_RNvNtCsC3JuwEIQwb_7console9unix_term7poll_fd: %_0"}
!974 = distinct !{!974, !"_RNvNtCsC3JuwEIQwb_7console9unix_term7poll_fd"}
!975 = !{!976}
!976 = distinct !{!976, !977, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base0ECsC3JuwEIQwb_7console: %self.0"}
!977 = distinct !{!977, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base0ECsC3JuwEIQwb_7console"}
!978 = !{!979}
!979 = distinct !{!979, !977, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSThhE16binary_search_byNCNvNtCsDJOD2kcAir_13unicode_width6tables22is_emoji_modifier_base0ECsC3JuwEIQwb_7console: argument 1"}
!980 = !{!981, !983}
!981 = distinct !{!981, !982, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_: %_0"}
!982 = distinct !{!982, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_"}
!983 = distinct !{!983, !982, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_E4lockB12_: %self.1"}
!984 = !{!985, !987, !988}
!985 = distinct !{!985, !986, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_: %t"}
!986 = distinct !{!986, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_"}
!987 = distinct !{!987, !986, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_: %self"}
!988 = distinct !{!988, !986, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardDNtNtCsC3JuwEIQwb_7console4term9TermWriteEL_EINtBM_11PoisonErrorBH_EE6unwrapB1H_: argument 2"}
!989 = !{!985, !988}
!990 = !{!985, !987}
!991 = !{!992}
!992 = distinct !{!992, !993, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console: %_0"}
!993 = distinct !{!993, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console"}
!994 = !{!995, !997}
!995 = distinct !{!995, !996, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!996 = distinct !{!996, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!997 = distinct !{!997, !996, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: argument 1"}
!998 = !{!995}
!999 = !{!1000}
!1000 = distinct !{!1000, !1001, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!1001 = distinct !{!1001, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!1002 = !{!1003}
!1003 = distinct !{!1003, !1004, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console: %_0"}
!1004 = distinct !{!1004, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexINtNtCsdJPVW0sQgAG_5alloc3vec3VechEE4lockCsC3JuwEIQwb_7console"}
!1005 = !{!1006, !1008}
!1006 = distinct !{!1006, !1007, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: %self"}
!1007 = distinct !{!1007, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console"}
!1008 = distinct !{!1008, !1007, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardINtNtCsdJPVW0sQgAG_5alloc3vec3VechEEINtBM_11PoisonErrorBH_EE6unwrapCsC3JuwEIQwb_7console: argument 1"}
!1009 = !{!1006}
!1010 = !{!1011}
!1011 = distinct !{!1011, !1012, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console: %self"}
!1012 = distinct !{!1012, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsC3JuwEIQwb_7console"}
!1013 = !{!1014}
!1014 = distinct !{!1014, !1015, !"_RNvXs2_NtCsC3JuwEIQwb_7console4ansiNtB5_7MatchesNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!1015 = distinct !{!1015, !"_RNvXs2_NtCsC3JuwEIQwb_7console4ansiNtB5_7MatchesNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!1016 = !{!1017, !1014}
!1017 = distinct !{!1017, !1015, !"_RNvXs2_NtCsC3JuwEIQwb_7console4ansiNtB5_7MatchesNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %_0"}
!1018 = !{!1017}
!1019 = !{!1020}
!1020 = distinct !{!1020, !1021, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1021 = distinct !{!1021, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1022 = !{!1023}
!1023 = distinct !{!1023, !1024, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1024 = distinct !{!1024, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1025 = !{!1026}
!1026 = distinct !{!1026, !1027, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1027 = distinct !{!1027, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1028 = !{!1029}
!1029 = distinct !{!1029, !1030, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1030 = distinct !{!1030, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1031 = !{!1032}
!1032 = distinct !{!1032, !1033, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_: %_1"}
!1033 = distinct !{!1033, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_"}
!1034 = !{!1035}
!1035 = distinct !{!1035, !1036, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_: %_1"}
!1036 = distinct !{!1036, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_"}
!1037 = !{!1038}
!1038 = distinct !{!1038, !1039, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_: %self"}
!1039 = distinct !{!1039, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_"}
!1040 = !{!1038, !1035, !1032}
!1041 = !{!1042}
!1042 = distinct !{!1042, !1043, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_: %_1"}
!1043 = distinct !{!1043, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_"}
!1044 = !{!1045}
!1045 = distinct !{!1045, !1046, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_: %_1"}
!1046 = distinct !{!1046, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_"}
!1047 = !{!1048}
!1048 = distinct !{!1048, !1049, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_: %self"}
!1049 = distinct !{!1049, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_"}
!1050 = !{!1048, !1045, !1042}
!1051 = !{!1052}
!1052 = distinct !{!1052, !1053, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_: %_1"}
!1053 = distinct !{!1053, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_"}
!1054 = !{!1055}
!1055 = distinct !{!1055, !1056, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_: %_1"}
!1056 = distinct !{!1056, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_"}
!1057 = !{!1058}
!1058 = distinct !{!1058, !1059, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_: %self"}
!1059 = distinct !{!1059, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_"}
!1060 = !{!1058, !1055, !1052}
!1061 = !{!1062}
!1062 = distinct !{!1062, !1063, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_: %_1"}
!1063 = distinct !{!1063, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsC3JuwEIQwb_7console4term4TermEBK_"}
!1064 = !{!1065}
!1065 = distinct !{!1065, !1066, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_: %_1"}
!1066 = distinct !{!1066, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerEEB1i_"}
!1067 = !{!1068}
!1068 = distinct !{!1068, !1069, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_: %self"}
!1069 = distinct !{!1069, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCsC3JuwEIQwb_7console4term9TermInnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_"}
!1070 = !{!1068, !1065, !1062}
