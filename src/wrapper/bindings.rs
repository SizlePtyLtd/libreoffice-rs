#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const __API_TO_BE_DEPRECATED: u32 = 100000;
pub const __MAC_10_0: u32 = 1000;
pub const __MAC_10_1: u32 = 1010;
pub const __MAC_10_2: u32 = 1020;
pub const __MAC_10_3: u32 = 1030;
pub const __MAC_10_4: u32 = 1040;
pub const __MAC_10_5: u32 = 1050;
pub const __MAC_10_6: u32 = 1060;
pub const __MAC_10_7: u32 = 1070;
pub const __MAC_10_8: u32 = 1080;
pub const __MAC_10_9: u32 = 1090;
pub const __MAC_10_10: u32 = 101000;
pub const __MAC_10_10_2: u32 = 101002;
pub const __MAC_10_10_3: u32 = 101003;
pub const __MAC_10_11: u32 = 101100;
pub const __MAC_10_11_2: u32 = 101102;
pub const __MAC_10_11_3: u32 = 101103;
pub const __MAC_10_11_4: u32 = 101104;
pub const __MAC_10_12: u32 = 101200;
pub const __MAC_10_12_1: u32 = 101201;
pub const __MAC_10_12_2: u32 = 101202;
pub const __MAC_10_12_4: u32 = 101204;
pub const __MAC_10_13: u32 = 101300;
pub const __MAC_10_13_1: u32 = 101301;
pub const __MAC_10_13_2: u32 = 101302;
pub const __MAC_10_13_4: u32 = 101304;
pub const __MAC_10_14: u32 = 101400;
pub const __MAC_10_14_1: u32 = 101401;
pub const __MAC_10_14_4: u32 = 101404;
pub const __MAC_10_14_6: u32 = 101406;
pub const __MAC_10_15: u32 = 101500;
pub const __MAC_10_15_1: u32 = 101501;
pub const __MAC_10_15_4: u32 = 101504;
pub const __MAC_10_16: u32 = 101600;
pub const __MAC_11_0: u32 = 110000;
pub const __MAC_11_1: u32 = 110100;
pub const __MAC_11_3: u32 = 110300;
pub const __MAC_11_4: u32 = 110400;
pub const __MAC_11_5: u32 = 110500;
pub const __MAC_11_6: u32 = 110600;
pub const __MAC_12_0: u32 = 120000;
pub const __MAC_12_1: u32 = 120100;
pub const __MAC_12_2: u32 = 120200;
pub const __MAC_12_3: u32 = 120300;
pub const __IPHONE_2_0: u32 = 20000;
pub const __IPHONE_2_1: u32 = 20100;
pub const __IPHONE_2_2: u32 = 20200;
pub const __IPHONE_3_0: u32 = 30000;
pub const __IPHONE_3_1: u32 = 30100;
pub const __IPHONE_3_2: u32 = 30200;
pub const __IPHONE_4_0: u32 = 40000;
pub const __IPHONE_4_1: u32 = 40100;
pub const __IPHONE_4_2: u32 = 40200;
pub const __IPHONE_4_3: u32 = 40300;
pub const __IPHONE_5_0: u32 = 50000;
pub const __IPHONE_5_1: u32 = 50100;
pub const __IPHONE_6_0: u32 = 60000;
pub const __IPHONE_6_1: u32 = 60100;
pub const __IPHONE_7_0: u32 = 70000;
pub const __IPHONE_7_1: u32 = 70100;
pub const __IPHONE_8_0: u32 = 80000;
pub const __IPHONE_8_1: u32 = 80100;
pub const __IPHONE_8_2: u32 = 80200;
pub const __IPHONE_8_3: u32 = 80300;
pub const __IPHONE_8_4: u32 = 80400;
pub const __IPHONE_9_0: u32 = 90000;
pub const __IPHONE_9_1: u32 = 90100;
pub const __IPHONE_9_2: u32 = 90200;
pub const __IPHONE_9_3: u32 = 90300;
pub const __IPHONE_10_0: u32 = 100000;
pub const __IPHONE_10_1: u32 = 100100;
pub const __IPHONE_10_2: u32 = 100200;
pub const __IPHONE_10_3: u32 = 100300;
pub const __IPHONE_11_0: u32 = 110000;
pub const __IPHONE_11_1: u32 = 110100;
pub const __IPHONE_11_2: u32 = 110200;
pub const __IPHONE_11_3: u32 = 110300;
pub const __IPHONE_11_4: u32 = 110400;
pub const __IPHONE_12_0: u32 = 120000;
pub const __IPHONE_12_1: u32 = 120100;
pub const __IPHONE_12_2: u32 = 120200;
pub const __IPHONE_12_3: u32 = 120300;
pub const __IPHONE_12_4: u32 = 120400;
pub const __IPHONE_13_0: u32 = 130000;
pub const __IPHONE_13_1: u32 = 130100;
pub const __IPHONE_13_2: u32 = 130200;
pub const __IPHONE_13_3: u32 = 130300;
pub const __IPHONE_13_4: u32 = 130400;
pub const __IPHONE_13_5: u32 = 130500;
pub const __IPHONE_13_6: u32 = 130600;
pub const __IPHONE_13_7: u32 = 130700;
pub const __IPHONE_14_0: u32 = 140000;
pub const __IPHONE_14_1: u32 = 140100;
pub const __IPHONE_14_2: u32 = 140200;
pub const __IPHONE_14_3: u32 = 140300;
pub const __IPHONE_14_5: u32 = 140500;
pub const __IPHONE_14_6: u32 = 140600;
pub const __IPHONE_14_7: u32 = 140700;
pub const __IPHONE_14_8: u32 = 140800;
pub const __IPHONE_15_0: u32 = 150000;
pub const __IPHONE_15_1: u32 = 150100;
pub const __IPHONE_15_2: u32 = 150200;
pub const __IPHONE_15_3: u32 = 150300;
pub const __IPHONE_15_4: u32 = 150400;
pub const __TVOS_9_0: u32 = 90000;
pub const __TVOS_9_1: u32 = 90100;
pub const __TVOS_9_2: u32 = 90200;
pub const __TVOS_10_0: u32 = 100000;
pub const __TVOS_10_0_1: u32 = 100001;
pub const __TVOS_10_1: u32 = 100100;
pub const __TVOS_10_2: u32 = 100200;
pub const __TVOS_11_0: u32 = 110000;
pub const __TVOS_11_1: u32 = 110100;
pub const __TVOS_11_2: u32 = 110200;
pub const __TVOS_11_3: u32 = 110300;
pub const __TVOS_11_4: u32 = 110400;
pub const __TVOS_12_0: u32 = 120000;
pub const __TVOS_12_1: u32 = 120100;
pub const __TVOS_12_2: u32 = 120200;
pub const __TVOS_12_3: u32 = 120300;
pub const __TVOS_12_4: u32 = 120400;
pub const __TVOS_13_0: u32 = 130000;
pub const __TVOS_13_2: u32 = 130200;
pub const __TVOS_13_3: u32 = 130300;
pub const __TVOS_13_4: u32 = 130400;
pub const __TVOS_14_0: u32 = 140000;
pub const __TVOS_14_1: u32 = 140100;
pub const __TVOS_14_2: u32 = 140200;
pub const __TVOS_14_3: u32 = 140300;
pub const __TVOS_14_5: u32 = 140500;
pub const __TVOS_14_6: u32 = 140600;
pub const __TVOS_14_7: u32 = 140700;
pub const __TVOS_15_0: u32 = 150000;
pub const __TVOS_15_1: u32 = 150100;
pub const __TVOS_15_2: u32 = 150200;
pub const __TVOS_15_3: u32 = 150300;
pub const __TVOS_15_4: u32 = 150400;
pub const __WATCHOS_1_0: u32 = 10000;
pub const __WATCHOS_2_0: u32 = 20000;
pub const __WATCHOS_2_1: u32 = 20100;
pub const __WATCHOS_2_2: u32 = 20200;
pub const __WATCHOS_3_0: u32 = 30000;
pub const __WATCHOS_3_1: u32 = 30100;
pub const __WATCHOS_3_1_1: u32 = 30101;
pub const __WATCHOS_3_2: u32 = 30200;
pub const __WATCHOS_4_0: u32 = 40000;
pub const __WATCHOS_4_1: u32 = 40100;
pub const __WATCHOS_4_2: u32 = 40200;
pub const __WATCHOS_4_3: u32 = 40300;
pub const __WATCHOS_5_0: u32 = 50000;
pub const __WATCHOS_5_1: u32 = 50100;
pub const __WATCHOS_5_2: u32 = 50200;
pub const __WATCHOS_5_3: u32 = 50300;
pub const __WATCHOS_6_0: u32 = 60000;
pub const __WATCHOS_6_1: u32 = 60100;
pub const __WATCHOS_6_2: u32 = 60200;
pub const __WATCHOS_7_0: u32 = 70000;
pub const __WATCHOS_7_1: u32 = 70100;
pub const __WATCHOS_7_2: u32 = 70200;
pub const __WATCHOS_7_3: u32 = 70300;
pub const __WATCHOS_7_4: u32 = 70400;
pub const __WATCHOS_7_5: u32 = 70500;
pub const __WATCHOS_7_6: u32 = 70600;
pub const __WATCHOS_8_0: u32 = 80000;
pub const __WATCHOS_8_1: u32 = 80100;
pub const __WATCHOS_8_3: u32 = 80300;
pub const __WATCHOS_8_4: u32 = 80400;
pub const __WATCHOS_8_5: u32 = 80500;
pub const MAC_OS_X_VERSION_10_0: u32 = 1000;
pub const MAC_OS_X_VERSION_10_1: u32 = 1010;
pub const MAC_OS_X_VERSION_10_2: u32 = 1020;
pub const MAC_OS_X_VERSION_10_3: u32 = 1030;
pub const MAC_OS_X_VERSION_10_4: u32 = 1040;
pub const MAC_OS_X_VERSION_10_5: u32 = 1050;
pub const MAC_OS_X_VERSION_10_6: u32 = 1060;
pub const MAC_OS_X_VERSION_10_7: u32 = 1070;
pub const MAC_OS_X_VERSION_10_8: u32 = 1080;
pub const MAC_OS_X_VERSION_10_9: u32 = 1090;
pub const MAC_OS_X_VERSION_10_10: u32 = 101000;
pub const MAC_OS_X_VERSION_10_10_2: u32 = 101002;
pub const MAC_OS_X_VERSION_10_10_3: u32 = 101003;
pub const MAC_OS_X_VERSION_10_11: u32 = 101100;
pub const MAC_OS_X_VERSION_10_11_2: u32 = 101102;
pub const MAC_OS_X_VERSION_10_11_3: u32 = 101103;
pub const MAC_OS_X_VERSION_10_11_4: u32 = 101104;
pub const MAC_OS_X_VERSION_10_12: u32 = 101200;
pub const MAC_OS_X_VERSION_10_12_1: u32 = 101201;
pub const MAC_OS_X_VERSION_10_12_2: u32 = 101202;
pub const MAC_OS_X_VERSION_10_12_4: u32 = 101204;
pub const MAC_OS_X_VERSION_10_13: u32 = 101300;
pub const MAC_OS_X_VERSION_10_13_1: u32 = 101301;
pub const MAC_OS_X_VERSION_10_13_2: u32 = 101302;
pub const MAC_OS_X_VERSION_10_13_4: u32 = 101304;
pub const MAC_OS_X_VERSION_10_14: u32 = 101400;
pub const MAC_OS_X_VERSION_10_14_1: u32 = 101401;
pub const MAC_OS_X_VERSION_10_14_4: u32 = 101404;
pub const MAC_OS_X_VERSION_10_14_6: u32 = 101406;
pub const MAC_OS_X_VERSION_10_15: u32 = 101500;
pub const MAC_OS_X_VERSION_10_15_1: u32 = 101501;
pub const MAC_OS_X_VERSION_10_16: u32 = 101600;
pub const MAC_OS_VERSION_11_0: u32 = 110000;
pub const MAC_OS_VERSION_12_0: u32 = 120000;
pub const __DRIVERKIT_19_0: u32 = 190000;
pub const __DRIVERKIT_20_0: u32 = 200000;
pub const __DRIVERKIT_21_0: u32 = 210000;
pub const __MAC_OS_X_VERSION_MAX_ALLOWED: u32 = 120300;
pub const __ENABLE_LEGACY_MAC_AVAILABILITY: u32 = 1;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_EXTSN: &[u8; 14usize] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __has_ptrcheck: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const __DARWIN_WCHAR_MIN: i32 = -2147483648;
pub const _FORTIFY_SOURCE: u32 = 2;
pub const __DARWIN_NSIG: u32 = 32;
pub const NSIG: u32 = 32;
pub const _ARM_SIGNAL_: u32 = 1;
pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;
pub const SIGEMT: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGBUS: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGSYS: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGURG: u32 = 16;
pub const SIGSTOP: u32 = 17;
pub const SIGTSTP: u32 = 18;
pub const SIGCONT: u32 = 19;
pub const SIGCHLD: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGIO: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGINFO: u32 = 29;
pub const SIGUSR1: u32 = 30;
pub const SIGUSR2: u32 = 31;
pub const __DARWIN_OPAQUE_ARM_THREAD_STATE64: u32 = 0;
pub const SIGEV_NONE: u32 = 0;
pub const SIGEV_SIGNAL: u32 = 1;
pub const SIGEV_THREAD: u32 = 3;
pub const ILL_NOOP: u32 = 0;
pub const ILL_ILLOPC: u32 = 1;
pub const ILL_ILLTRP: u32 = 2;
pub const ILL_PRVOPC: u32 = 3;
pub const ILL_ILLOPN: u32 = 4;
pub const ILL_ILLADR: u32 = 5;
pub const ILL_PRVREG: u32 = 6;
pub const ILL_COPROC: u32 = 7;
pub const ILL_BADSTK: u32 = 8;
pub const FPE_NOOP: u32 = 0;
pub const FPE_FLTDIV: u32 = 1;
pub const FPE_FLTOVF: u32 = 2;
pub const FPE_FLTUND: u32 = 3;
pub const FPE_FLTRES: u32 = 4;
pub const FPE_FLTINV: u32 = 5;
pub const FPE_FLTSUB: u32 = 6;
pub const FPE_INTDIV: u32 = 7;
pub const FPE_INTOVF: u32 = 8;
pub const SEGV_NOOP: u32 = 0;
pub const SEGV_MAPERR: u32 = 1;
pub const SEGV_ACCERR: u32 = 2;
pub const BUS_NOOP: u32 = 0;
pub const BUS_ADRALN: u32 = 1;
pub const BUS_ADRERR: u32 = 2;
pub const BUS_OBJERR: u32 = 3;
pub const TRAP_BRKPT: u32 = 1;
pub const TRAP_TRACE: u32 = 2;
pub const CLD_NOOP: u32 = 0;
pub const CLD_EXITED: u32 = 1;
pub const CLD_KILLED: u32 = 2;
pub const CLD_DUMPED: u32 = 3;
pub const CLD_TRAPPED: u32 = 4;
pub const CLD_STOPPED: u32 = 5;
pub const CLD_CONTINUED: u32 = 6;
pub const POLL_IN: u32 = 1;
pub const POLL_OUT: u32 = 2;
pub const POLL_MSG: u32 = 3;
pub const POLL_ERR: u32 = 4;
pub const POLL_PRI: u32 = 5;
pub const POLL_HUP: u32 = 6;
pub const SA_ONSTACK: u32 = 1;
pub const SA_RESTART: u32 = 2;
pub const SA_RESETHAND: u32 = 4;
pub const SA_NOCLDSTOP: u32 = 8;
pub const SA_NODEFER: u32 = 16;
pub const SA_NOCLDWAIT: u32 = 32;
pub const SA_SIGINFO: u32 = 64;
pub const SA_USERTRAMP: u32 = 256;
pub const SA_64REGSET: u32 = 512;
pub const SA_USERSPACE_MASK: u32 = 127;
pub const SIG_BLOCK: u32 = 1;
pub const SIG_UNBLOCK: u32 = 2;
pub const SIG_SETMASK: u32 = 3;
pub const SI_USER: u32 = 65537;
pub const SI_QUEUE: u32 = 65538;
pub const SI_TIMER: u32 = 65539;
pub const SI_ASYNCIO: u32 = 65540;
pub const SI_MESGQ: u32 = 65541;
pub const SS_ONSTACK: u32 = 1;
pub const SS_DISABLE: u32 = 4;
pub const MINSIGSTKSZ: u32 = 32768;
pub const SIGSTKSZ: u32 = 131072;
pub const SV_ONSTACK: u32 = 1;
pub const SV_INTERRUPT: u32 = 2;
pub const SV_RESETHAND: u32 = 4;
pub const SV_NODEFER: u32 = 16;
pub const SV_NOCLDSTOP: u32 = 8;
pub const SV_SIGINFO: u32 = 64;
pub const __WORDSIZE: u32 = 64;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const PRIO_PROCESS: u32 = 0;
pub const PRIO_PGRP: u32 = 1;
pub const PRIO_USER: u32 = 2;
pub const PRIO_DARWIN_THREAD: u32 = 3;
pub const PRIO_DARWIN_PROCESS: u32 = 4;
pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: u32 = 20;
pub const PRIO_DARWIN_BG: u32 = 4096;
pub const PRIO_DARWIN_NONUI: u32 = 4097;
pub const RUSAGE_SELF: u32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
pub const RUSAGE_INFO_V0: u32 = 0;
pub const RUSAGE_INFO_V1: u32 = 1;
pub const RUSAGE_INFO_V2: u32 = 2;
pub const RUSAGE_INFO_V3: u32 = 3;
pub const RUSAGE_INFO_V4: u32 = 4;
pub const RUSAGE_INFO_V5: u32 = 5;
pub const RUSAGE_INFO_CURRENT: u32 = 5;
pub const RU_PROC_RUNS_RESLIDE: u32 = 1;
pub const RLIMIT_CPU: u32 = 0;
pub const RLIMIT_FSIZE: u32 = 1;
pub const RLIMIT_DATA: u32 = 2;
pub const RLIMIT_STACK: u32 = 3;
pub const RLIMIT_CORE: u32 = 4;
pub const RLIMIT_AS: u32 = 5;
pub const RLIMIT_RSS: u32 = 5;
pub const RLIMIT_MEMLOCK: u32 = 6;
pub const RLIMIT_NPROC: u32 = 7;
pub const RLIMIT_NOFILE: u32 = 8;
pub const RLIM_NLIMITS: u32 = 9;
pub const _RLIMIT_POSIX_FLAG: u32 = 4096;
pub const RLIMIT_WAKEUPS_MONITOR: u32 = 1;
pub const RLIMIT_CPU_USAGE_MONITOR: u32 = 2;
pub const RLIMIT_THREAD_CPULIMITS: u32 = 3;
pub const RLIMIT_FOOTPRINT_INTERVAL: u32 = 4;
pub const WAKEMON_ENABLE: u32 = 1;
pub const WAKEMON_DISABLE: u32 = 2;
pub const WAKEMON_GET_PARAMS: u32 = 4;
pub const WAKEMON_SET_DEFAULTS: u32 = 8;
pub const WAKEMON_MAKE_FATAL: u32 = 16;
pub const CPUMON_MAKE_FATAL: u32 = 4096;
pub const FOOTPRINT_INTERVAL_RESET: u32 = 1;
pub const IOPOL_TYPE_DISK: u32 = 0;
pub const IOPOL_TYPE_VFS_ATIME_UPDATES: u32 = 2;
pub const IOPOL_TYPE_VFS_MATERIALIZE_DATALESS_FILES: u32 = 3;
pub const IOPOL_TYPE_VFS_STATFS_NO_DATA_VOLUME: u32 = 4;
pub const IOPOL_TYPE_VFS_TRIGGER_RESOLVE: u32 = 5;
pub const IOPOL_TYPE_VFS_IGNORE_CONTENT_PROTECTION: u32 = 6;
pub const IOPOL_TYPE_VFS_IGNORE_PERMISSIONS: u32 = 7;
pub const IOPOL_TYPE_VFS_SKIP_MTIME_UPDATE: u32 = 8;
pub const IOPOL_TYPE_VFS_ALLOW_LOW_SPACE_WRITES: u32 = 9;
pub const IOPOL_SCOPE_PROCESS: u32 = 0;
pub const IOPOL_SCOPE_THREAD: u32 = 1;
pub const IOPOL_SCOPE_DARWIN_BG: u32 = 2;
pub const IOPOL_DEFAULT: u32 = 0;
pub const IOPOL_IMPORTANT: u32 = 1;
pub const IOPOL_PASSIVE: u32 = 2;
pub const IOPOL_THROTTLE: u32 = 3;
pub const IOPOL_UTILITY: u32 = 4;
pub const IOPOL_STANDARD: u32 = 5;
pub const IOPOL_APPLICATION: u32 = 5;
pub const IOPOL_NORMAL: u32 = 1;
pub const IOPOL_ATIME_UPDATES_DEFAULT: u32 = 0;
pub const IOPOL_ATIME_UPDATES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_DEFAULT: u32 = 0;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ON: u32 = 2;
pub const IOPOL_VFS_STATFS_NO_DATA_VOLUME_DEFAULT: u32 = 0;
pub const IOPOL_VFS_STATFS_FORCE_NO_DATA_VOLUME: u32 = 1;
pub const IOPOL_VFS_TRIGGER_RESOLVE_DEFAULT: u32 = 0;
pub const IOPOL_VFS_TRIGGER_RESOLVE_OFF: u32 = 1;
pub const IOPOL_VFS_CONTENT_PROTECTION_DEFAULT: u32 = 0;
pub const IOPOL_VFS_CONTENT_PROTECTION_IGNORE: u32 = 1;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_OFF: u32 = 0;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_ON: u32 = 1;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_OFF: u32 = 0;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_ON: u32 = 1;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_OFF: u32 = 0;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_ON: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WCOREFLAG: u32 = 128;
pub const _WSTOPPED: u32 = 127;
pub const WEXITED: u32 = 4;
pub const WSTOPPED: u32 = 8;
pub const WCONTINUED: u32 = 16;
pub const WNOWAIT: u32 = 32;
pub const WAIT_ANY: i32 = -1;
pub const WAIT_MYPGRP: u32 = 0;
pub const _QUAD_HIGHWORD: u32 = 1;
pub const _QUAD_LOWWORD: u32 = 0;
pub const __DARWIN_LITTLE_ENDIAN: u32 = 1234;
pub const __DARWIN_BIG_ENDIAN: u32 = 4321;
pub const __DARWIN_PDP_ENDIAN: u32 = 3412;
pub const __DARWIN_BYTE_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const RAND_MAX: u32 = 2147483647;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub const RENAME_SECLUDE: u32 = 1;
pub const RENAME_SWAP: u32 = 2;
pub const RENAME_EXCL: u32 = 4;
pub const RENAME_RESERVED1: u32 = 8;
pub const RENAME_NOFOLLOW_ANY: u32 = 16;
pub const __SLBF: u32 = 1;
pub const __SNBF: u32 = 2;
pub const __SRD: u32 = 4;
pub const __SWR: u32 = 8;
pub const __SRW: u32 = 16;
pub const __SEOF: u32 = 32;
pub const __SERR: u32 = 64;
pub const __SMBF: u32 = 128;
pub const __SAPP: u32 = 256;
pub const __SSTR: u32 = 512;
pub const __SOPT: u32 = 1024;
pub const __SNPT: u32 = 2048;
pub const __SOFF: u32 = 4096;
pub const __SMOD: u32 = 8192;
pub const __SALC: u32 = 16384;
pub const __SIGN: u32 = 32768;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 1024;
pub const EOF: i32 = -1;
pub const FOPEN_MAX: u32 = 20;
pub const FILENAME_MAX: u32 = 1024;
pub const P_tmpdir: &[u8; 10usize] = b"/var/tmp/\0";
pub const L_tmpnam: u32 = 1024;
pub const TMP_MAX: u32 = 308915776;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const L_ctermid: u32 = 1024;
pub const _USE_FORTIFY_LEVEL: u32 = 2;
pub const __HAS_FIXED_CHK_PROTOTYPES: u32 = 1;
pub const S_IFMT: u32 = 61440;
pub const S_IFIFO: u32 = 4096;
pub const S_IFCHR: u32 = 8192;
pub const S_IFDIR: u32 = 16384;
pub const S_IFBLK: u32 = 24576;
pub const S_IFREG: u32 = 32768;
pub const S_IFLNK: u32 = 40960;
pub const S_IFSOCK: u32 = 49152;
pub const S_IFWHT: u32 = 57344;
pub const S_IRWXU: u32 = 448;
pub const S_IRUSR: u32 = 256;
pub const S_IWUSR: u32 = 128;
pub const S_IXUSR: u32 = 64;
pub const S_IRWXG: u32 = 56;
pub const S_IRGRP: u32 = 32;
pub const S_IWGRP: u32 = 16;
pub const S_IXGRP: u32 = 8;
pub const S_IRWXO: u32 = 7;
pub const S_IROTH: u32 = 4;
pub const S_IWOTH: u32 = 2;
pub const S_IXOTH: u32 = 1;
pub const S_ISUID: u32 = 2048;
pub const S_ISGID: u32 = 1024;
pub const S_ISVTX: u32 = 512;
pub const S_ISTXT: u32 = 512;
pub const S_IREAD: u32 = 256;
pub const S_IWRITE: u32 = 128;
pub const S_IEXEC: u32 = 64;
pub const ACCESSPERMS: u32 = 511;
pub const ALLPERMS: u32 = 4095;
pub const DEFFILEMODE: u32 = 438;
pub const S_BLKSIZE: u32 = 512;
pub const UF_SETTABLE: u32 = 65535;
pub const UF_NODUMP: u32 = 1;
pub const UF_IMMUTABLE: u32 = 2;
pub const UF_APPEND: u32 = 4;
pub const UF_OPAQUE: u32 = 8;
pub const UF_COMPRESSED: u32 = 32;
pub const UF_TRACKED: u32 = 64;
pub const UF_DATAVAULT: u32 = 128;
pub const UF_HIDDEN: u32 = 32768;
pub const SF_SUPPORTED: u32 = 10420224;
pub const SF_SETTABLE: u32 = 1073676288;
pub const SF_SYNTHETIC: u32 = 3221225472;
pub const SF_ARCHIVED: u32 = 65536;
pub const SF_IMMUTABLE: u32 = 131072;
pub const SF_APPEND: u32 = 262144;
pub const SF_RESTRICTED: u32 = 524288;
pub const SF_NOUNLINK: u32 = 1048576;
pub const SF_FIRMLINK: u32 = 8388608;
pub const SF_DATALESS: u32 = 1073741824;
pub const EF_MAY_SHARE_BLOCKS: u32 = 1;
pub const EF_NO_XATTRS: u32 = 2;
pub const EF_IS_SYNC_ROOT: u32 = 4;
pub const EF_IS_PURGEABLE: u32 = 8;
pub const EF_IS_SPARSE: u32 = 16;
pub const EF_IS_SYNTHETIC: u32 = 32;
pub const UTIME_NOW: i32 = -1;
pub const UTIME_OMIT: i32 = -2;
pub const RTLD_LAZY: u32 = 1;
pub const RTLD_NOW: u32 = 2;
pub const RTLD_LOCAL: u32 = 4;
pub const RTLD_GLOBAL: u32 = 8;
pub const RTLD_NOLOAD: u32 = 16;
pub const RTLD_NODELETE: u32 = 128;
pub const RTLD_FIRST: u32 = 256;
pub const TARGET_LIB: &[u8; 20usize] = b"libsofficeapp.dylib\0";
pub const TARGET_MERGED_LIB: &[u8; 18usize] = b"libmergedlo.dylib\0";
pub const SEPARATOR: u8 = 47u8;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type __darwin_nl_item = ::std::os::raw::c_int;
pub type __darwin_wctrans_t = ::std::os::raw::c_int;
pub type __darwin_wctype_t = __uint32_t;
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = ::std::os::raw::c_uint;
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type sig_atomic_t = ::std::os::raw::c_int;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = i64;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = i64;
pub type user_long_t = i64;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = i64;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state {
    pub __exception: __uint32_t,
    pub __fsr: __uint32_t,
    pub __far: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state64 {
    pub __far: __uint64_t,
    pub __esr: __uint32_t,
    pub __exception: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state {
    pub __r: [__uint32_t; 13usize],
    pub __sp: __uint32_t,
    pub __lr: __uint32_t,
    pub __pc: __uint32_t,
    pub __cpsr: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state64 {
    pub __x: [__uint64_t; 29usize],
    pub __fp: __uint64_t,
    pub __lr: __uint64_t,
    pub __sp: __uint64_t,
    pub __pc: __uint64_t,
    pub __cpsr: __uint32_t,
    pub __pad: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_vfp_state {
    pub __r: [__uint32_t; 64usize],
    pub __fpscr: __uint32_t,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_neon_state64 {
    pub __v: [__uint128_t; 32usize],
    pub __fpsr: __uint32_t,
    pub __fpcr: __uint32_t,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_neon_state {
    pub __v: [__uint128_t; 16usize],
    pub __fpsr: __uint32_t,
    pub __fpcr: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __arm_pagein_state {
    pub __pagein_error: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __arm_legacy_debug_state {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_debug_state32 {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_debug_state64 {
    pub __bvr: [__uint64_t; 16usize],
    pub __bcr: [__uint64_t; 16usize],
    pub __wvr: [__uint64_t; 16usize],
    pub __wcr: [__uint64_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_cpmu_state64 {
    pub __ctrs: [__uint64_t; 16usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mcontext32 {
    pub __es: __darwin_arm_exception_state,
    pub __ss: __darwin_arm_thread_state,
    pub __fs: __darwin_arm_vfp_state,
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mcontext64 {
    pub __es: __darwin_arm_exception_state64,
    pub __ss: __darwin_arm_thread_state64,
    pub __ns: __darwin_arm_neon_state64,
}
pub type mcontext_t = *mut __darwin_mcontext64;
pub type pthread_attr_t = __darwin_pthread_attr_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_sigaltstack {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_size: __darwin_size_t,
    pub ss_flags: ::std::os::raw::c_int,
}
pub type stack_t = __darwin_sigaltstack;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_ucontext {
    pub uc_onstack: ::std::os::raw::c_int,
    pub uc_sigmask: __darwin_sigset_t,
    pub uc_stack: __darwin_sigaltstack,
    pub uc_link: *mut __darwin_ucontext,
    pub uc_mcsize: __darwin_size_t,
    pub uc_mcontext: *mut __darwin_mcontext64,
}
pub type ucontext_t = __darwin_ucontext;
pub type sigset_t = __darwin_sigset_t;
pub type size_t = __darwin_size_t;
pub type uid_t = __darwin_uid_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::std::os::raw::c_int,
    pub sival_ptr: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_notify: ::std::os::raw::c_int,
    pub sigev_signo: ::std::os::raw::c_int,
    pub sigev_value: sigval,
    pub sigev_notify_function: ::std::option::Option<unsafe extern "C" fn(arg1: sigval)>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __siginfo {
    pub si_signo: ::std::os::raw::c_int,
    pub si_errno: ::std::os::raw::c_int,
    pub si_code: ::std::os::raw::c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: ::std::os::raw::c_int,
    pub si_addr: *mut ::std::os::raw::c_void,
    pub si_value: sigval,
    pub si_band: ::std::os::raw::c_long,
    pub __pad: [::std::os::raw::c_ulong; 7usize],
}
pub type siginfo_t = __siginfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sigaction_u {
    pub __sa_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub __sa_sigaction: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut __siginfo,
            arg3: *mut ::std::os::raw::c_void,
        ),
    >,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_tramp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *mut siginfo_t,
            arg5: *mut ::std::os::raw::c_void,
        ),
    >,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
pub type sig_t = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigvec {
    pub sv_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub sv_mask: ::std::os::raw::c_int,
    pub sv_flags: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigstack {
    pub ss_sp: *mut ::std::os::raw::c_char,
    pub ss_onstack: ::std::os::raw::c_int,
}
extern "C" {
    pub fn signal(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        ),
    >;
}
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
pub type rlim_t = __uint64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: ::std::os::raw::c_long,
    pub ru_ixrss: ::std::os::raw::c_long,
    pub ru_idrss: ::std::os::raw::c_long,
    pub ru_isrss: ::std::os::raw::c_long,
    pub ru_minflt: ::std::os::raw::c_long,
    pub ru_majflt: ::std::os::raw::c_long,
    pub ru_nswap: ::std::os::raw::c_long,
    pub ru_inblock: ::std::os::raw::c_long,
    pub ru_oublock: ::std::os::raw::c_long,
    pub ru_msgsnd: ::std::os::raw::c_long,
    pub ru_msgrcv: ::std::os::raw::c_long,
    pub ru_nsignals: ::std::os::raw::c_long,
    pub ru_nvcsw: ::std::os::raw::c_long,
    pub ru_nivcsw: ::std::os::raw::c_long,
}
pub type rusage_info_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v0 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v1 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v2 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v3 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v4 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v5 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
    pub ri_flags: u64,
}
pub type rusage_info_current = rusage_info_v5;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct proc_rlimit_control_wakeupmon {
    pub wm_flags: u32,
    pub wm_rate: i32,
}
extern "C" {
    pub fn getpriority(arg1: ::std::os::raw::c_int, arg2: id_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getiopolicy_np(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrlimit(arg1: ::std::os::raw::c_int, arg2: *mut rlimit) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrusage(arg1: ::std::os::raw::c_int, arg2: *mut rusage) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpriority(
        arg1: ::std::os::raw::c_int,
        arg2: id_t,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setiopolicy_np(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setrlimit(arg1: ::std::os::raw::c_int, arg2: *const rlimit) -> ::std::os::raw::c_int;
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _OSUnalignedU16 {
    pub __val: u16,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _OSUnalignedU32 {
    pub __val: u32,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct _OSUnalignedU64 {
    pub __val: u64,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wait {
    pub w_status: ::std::os::raw::c_int,
    pub w_T: wait__bindgen_ty_1,
    pub w_S: wait__bindgen_ty_2,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl wait__bindgen_ty_1 {
    #[inline]
    pub fn w_Termsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_w_Termsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Coredump(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_w_Coredump(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Retcode(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Retcode(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Termsig: ::std::os::raw::c_uint,
        w_Coredump: ::std::os::raw::c_uint,
        w_Retcode: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let w_Termsig: u32 = unsafe { ::std::mem::transmute(w_Termsig) };
            w_Termsig as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let w_Coredump: u32 = unsafe { ::std::mem::transmute(w_Coredump) };
            w_Coredump as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Retcode: u32 = unsafe { ::std::mem::transmute(w_Retcode) };
            w_Retcode as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_2 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
impl wait__bindgen_ty_2 {
    #[inline]
    pub fn w_Stopval(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopval(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Stopsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Stopval: ::std::os::raw::c_uint,
        w_Stopsig: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let w_Stopval: u32 = unsafe { ::std::mem::transmute(w_Stopval) };
            w_Stopval as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Stopsig: u32 = unsafe { ::std::mem::transmute(w_Stopsig) };
            w_Stopsig as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
extern "C" {
    pub fn wait(arg1: *mut ::std::os::raw::c_int) -> pid_t;
}
extern "C" {
    pub fn waitpid(
        arg1: pid_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> pid_t;
}
extern "C" {
    pub fn waitid(
        arg1: idtype_t,
        arg2: id_t,
        arg3: *mut siginfo_t,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wait3(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *mut rusage,
    ) -> pid_t;
}
extern "C" {
    pub fn wait4(
        arg1: pid_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut rusage,
    ) -> pid_t;
}
extern "C" {
    pub fn alloca(arg1: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
pub type ct_rune_t = __darwin_ct_rune_t;
pub type rune_t = __darwin_rune_t;
pub type wchar_t = __darwin_wchar_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
extern "C" {
    pub static mut __mb_cur_max: ::std::os::raw::c_int;
}
extern "C" {
    pub fn malloc(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(
        __count: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(arg1: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn aligned_alloc(__alignment: size_t, __size: size_t) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn abs(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atexit(arg1: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atof(arg1: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn div(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn exit(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn getenv(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn labs(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn ldiv(arg1: ::std::os::raw::c_long, arg2: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn llabs(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lldiv(arg1: ::std::os::raw::c_longlong, arg2: ::std::os::raw::c_longlong) -> lldiv_t;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(
        arg1: *mut wchar_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn mbtowc(
        arg1: *mut wchar_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(arg1: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn strtod(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtol(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtold(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtoll(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoul(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoull(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn system(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcstombs(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const wchar_t,
        arg3: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn wctomb(arg1: *mut ::std::os::raw::c_char, arg2: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _Exit(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn a64l(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn ecvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn erand48(arg1: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn fcvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getsubopt(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *const *mut ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grantpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate(
        arg1: ::std::os::raw::c_uint,
        arg2: *mut ::std::os::raw::c_char,
        arg3: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn jrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn l64a(arg1: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn lcong48(arg1: *mut ::std::os::raw::c_ushort);
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mktemp(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn posix_openpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ptsname(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ptsname_r(
        fildes: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_char,
        buflen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putenv(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rand_r(arg1: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_realpath$DARWIN_EXTSN"]
    pub fn realpath(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn seed48(arg1: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __overwrite: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setkey(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn setstate(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn srand48(arg1: ::std::os::raw::c_long);
}
extern "C" {
    pub fn srandom(arg1: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn unlockpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
extern "C" {
    pub fn arc4random() -> u32;
}
extern "C" {
    pub fn arc4random_addrandom(arg1: *mut ::std::os::raw::c_uchar, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn arc4random_buf(__buf: *mut ::std::os::raw::c_void, __nbytes: size_t);
}
extern "C" {
    pub fn arc4random_stir();
}
extern "C" {
    pub fn arc4random_uniform(__upper_bound: u32) -> u32;
}
extern "C" {
    pub fn atexit_b(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bsearch_b(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn cgetcap(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn cgetclose() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetent(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetfirst(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetmatch(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetnext(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetnum(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetset(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetstr(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetustr(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn daemon(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn devname(arg1: dev_t, arg2: mode_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn devname_r(
        arg1: dev_t,
        arg2: mode_t,
        buf: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getbsize(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_long,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getloadavg(arg1: *mut f64, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getprogname() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn setprogname(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn heapsort(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn heapsort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mergesort(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mergesort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn psort(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn psort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn psort_r(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        arg1: *mut ::std::os::raw::c_void,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
                arg3: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn qsort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn qsort_r(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        arg1: *mut ::std::os::raw::c_void,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
                arg3: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn radixsort(
        __base: *mut *const ::std::os::raw::c_uchar,
        __nel: ::std::os::raw::c_int,
        __table: *const ::std::os::raw::c_uchar,
        __endbyte: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rpmatch(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sradixsort(
        __base: *mut *const ::std::os::raw::c_uchar,
        __nel: ::std::os::raw::c_int,
        __table: *const ::std::os::raw::c_uchar,
        __endbyte: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sranddev();
}
extern "C" {
    pub fn srandomdev();
}
extern "C" {
    pub fn reallocf(
        __ptr: *mut ::std::os::raw::c_void,
        __size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strtonum(
        __numstr: *const ::std::os::raw::c_char,
        __minval: ::std::os::raw::c_longlong,
        __maxval: ::std::os::raw::c_longlong,
        __errstrp: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoq(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub static mut suboptarg: *mut ::std::os::raw::c_char;
}
pub type rsize_t = ::std::os::raw::c_ulong;
pub type max_align_t = f64;
pub type LibreOfficeKitCallback = ::std::option::Option<
    unsafe extern "C" fn(
        nType: ::std::os::raw::c_int,
        pPayload: *const ::std::os::raw::c_char,
        pData: *mut ::std::os::raw::c_void,
    ),
>;
pub type LibreOfficeKitPollCallback = ::std::option::Option<
    unsafe extern "C" fn(
        pData: *mut ::std::os::raw::c_void,
        timeoutUs: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type LibreOfficeKitWakeCallback =
    ::std::option::Option<unsafe extern "C" fn(pData: *mut ::std::os::raw::c_void)>;
pub type LibreOfficeKit = _LibreOfficeKit;
pub type LibreOfficeKitClass = _LibreOfficeKitClass;
pub type LibreOfficeKitDocument = _LibreOfficeKitDocument;
pub type LibreOfficeKitDocumentClass = _LibreOfficeKitDocumentClass;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _LibreOfficeKit {
    pub pClass: *mut LibreOfficeKitClass,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _LibreOfficeKitClass {
    pub nSize: size_t,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(pThis: *mut LibreOfficeKit)>,
    pub documentLoad: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            pURL: *const ::std::os::raw::c_char,
        ) -> *mut LibreOfficeKitDocument,
    >,
    pub getError: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut LibreOfficeKit) -> *mut ::std::os::raw::c_char,
    >,
    pub documentLoadWithOptions: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            pURL: *const ::std::os::raw::c_char,
            pOptions: *const ::std::os::raw::c_char,
        ) -> *mut LibreOfficeKitDocument,
    >,
    pub freeError: ::std::option::Option<unsafe extern "C" fn(pFree: *mut ::std::os::raw::c_char)>,
    pub registerCallback: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            pCallback: LibreOfficeKitCallback,
            pData: *mut ::std::os::raw::c_void,
        ),
    >,
    pub getFilterTypes: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut LibreOfficeKit) -> *mut ::std::os::raw::c_char,
    >,
    pub setOptionalFeatures: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut LibreOfficeKit, features: ::std::os::raw::c_ulonglong),
    >,
    pub setDocumentPassword: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            pURL: *const ::std::os::raw::c_char,
            pPassword: *const ::std::os::raw::c_char,
        ),
    >,
    pub getVersionInfo: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut LibreOfficeKit) -> *mut ::std::os::raw::c_char,
    >,
    pub runMacro: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            pURL: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub signDocument: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            pUrl: *const ::std::os::raw::c_char,
            pCertificateBinary: *const ::std::os::raw::c_uchar,
            nCertificateBinarySize: ::std::os::raw::c_int,
            pPrivateKeyBinary: *const ::std::os::raw::c_uchar,
            nPrivateKeyBinarySize: ::std::os::raw::c_int,
        ) -> bool,
    >,
    pub runLoop: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            pPollCallback: LibreOfficeKitPollCallback,
            pWakeCallback: LibreOfficeKitWakeCallback,
            pData: *mut ::std::os::raw::c_void,
        ),
    >,
    pub sendDialogEvent: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            nLOKWindowId: ::std::os::raw::c_ulonglong,
            pArguments: *const ::std::os::raw::c_char,
        ),
    >,
    pub setOption: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKit,
            pOption: *const ::std::os::raw::c_char,
            pValue: *const ::std::os::raw::c_char,
        ),
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _LibreOfficeKitDocument {
    pub pClass: *mut LibreOfficeKitDocumentClass,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _LibreOfficeKitDocumentClass {
    pub nSize: size_t,
    pub destroy: ::std::option::Option<unsafe extern "C" fn(pThis: *mut LibreOfficeKitDocument)>,
    pub saveAs: ::std::option::Option<
        unsafe extern "C" fn(
            pThis: *mut LibreOfficeKitDocument,
            pUrl: *const ::std::os::raw::c_char,
            pFormat: *const ::std::os::raw::c_char,
            pFilterOptions: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >,
    pub getDocumentType: ::std::option::Option<
        unsafe extern "C" fn(pThis: *mut LibreOfficeKitDocument) -> ::std::os::raw::c_int,
    >,
}
pub type va_list = __darwin_va_list;
extern "C" {
    pub fn renameat(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renamex_np(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameatx_np(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: *const ::std::os::raw::c_char,
        arg5: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
extern "C" {
    pub static mut __stdinp: *mut FILE;
}
extern "C" {
    pub static mut __stdoutp: *mut FILE;
}
extern "C" {
    pub static mut __stderrp: *mut FILE;
}
extern "C" {
    pub fn clearerr(arg1: *mut FILE);
}
extern "C" {
    pub fn fclose(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn feof(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetpos(arg1: *mut FILE, arg2: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fprintf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputs(arg1: *const ::std::os::raw::c_char, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __nitems: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn freopen(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fscanf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseek(
        arg1: *mut FILE,
        arg2: ::std::os::raw::c_long,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(arg1: *mut FILE, arg2: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(arg1: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __nitems: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn getc(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gets(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn perror(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn printf(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn remove(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewind(arg1: *mut FILE);
}
extern "C" {
    pub fn scanf(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuf(arg1: *mut FILE, arg2: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        arg1: *mut FILE,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ungetc(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        arg3: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        arg1: *const ::std::os::raw::c_char,
        arg2: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fdopen(arg1: ::std::os::raw::c_int, arg2: *const ::std::os::raw::c_char) -> *mut FILE;
}
extern "C" {
    pub fn fileno(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pclose(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn __srget(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __svfscanf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        arg3: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __swbuf(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flockfile(arg1: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(arg1: *mut FILE);
}
extern "C" {
    pub fn getc_unlocked(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __prefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type off_t = __darwin_off_t;
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __offset: off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> off_t;
}
extern "C" {
    pub fn snprintf(
        __str: *mut ::std::os::raw::c_char,
        __size: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        arg1: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        arg1: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __str: *mut ::std::os::raw::c_char,
        __size: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        arg1: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __str: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        arg1: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
pub type ssize_t = __darwin_ssize_t;
extern "C" {
    pub fn dprintf(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdelim(
        __linep: *mut *mut ::std::os::raw::c_char,
        __linecapp: *mut size_t,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> ssize_t;
}
extern "C" {
    pub fn getline(
        __linep: *mut *mut ::std::os::raw::c_char,
        __linecapp: *mut size_t,
        __stream: *mut FILE,
    ) -> ssize_t;
}
extern "C" {
    pub fn fmemopen(
        __buf: *mut ::std::os::raw::c_void,
        __size: size_t,
        __mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufp: *mut *mut ::std::os::raw::c_char,
        __sizep: *mut size_t,
    ) -> *mut FILE;
}
extern "C" {
    pub static sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn asprintf(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid_r(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fgetln(arg1: *mut FILE, arg2: *mut size_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fmtcheck(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn fpurge(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(
        arg1: *mut FILE,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn setlinebuf(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vasprintf(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funopen(
        arg1: *const ::std::os::raw::c_void,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_char,
                arg3: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_char,
                arg3: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        arg4: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: fpos_t,
                arg3: ::std::os::raw::c_int,
            ) -> fpos_t,
        >,
        arg5: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
    ) -> *mut FILE;
}
extern "C" {
    pub fn __sprintf_chk(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: size_t,
        arg4: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __snprintf_chk(
        arg1: *mut ::std::os::raw::c_char,
        arg2: size_t,
        arg3: ::std::os::raw::c_int,
        arg4: size_t,
        arg5: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __vsprintf_chk(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: size_t,
        arg4: *const ::std::os::raw::c_char,
        arg5: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __vsnprintf_chk(
        arg1: *mut ::std::os::raw::c_char,
        arg2: size_t,
        arg3: ::std::os::raw::c_int,
        arg4: size_t,
        arg5: *const ::std::os::raw::c_char,
        arg6: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memcpy(
        __dst: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memmove(
        __dst: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __len: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset(
        __b: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __len: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcat(
        __s1: *mut ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __charset: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strncat(
        __s1: *mut ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strpbrk(
        __s: *const ::std::os::raw::c_char,
        __charset: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __charset: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strstr(
        __big: *const ::std::os::raw::c_char,
        __little: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok(
        __str: *mut ::std::os::raw::c_char,
        __sep: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strxfrm(
        __s1: *mut ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtok_r(
        __str: *mut ::std::os::raw::c_char,
        __sep: *const ::std::os::raw::c_char,
        __lasts: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __strerrbuf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strdup(__s1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn memccpy(
        __dst: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn stpcpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpncpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strndup(
        __s1: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strnlen(__s1: *const ::std::os::raw::c_char, __n: size_t) -> size_t;
}
extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
pub type errno_t = ::std::os::raw::c_int;
extern "C" {
    pub fn memset_s(
        __s: *mut ::std::os::raw::c_void,
        __smax: rsize_t,
        __c: ::std::os::raw::c_int,
        __n: rsize_t,
    ) -> errno_t;
}
extern "C" {
    pub fn memmem(
        __big: *const ::std::os::raw::c_void,
        __big_len: size_t,
        __little: *const ::std::os::raw::c_void,
        __little_len: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset_pattern4(
        __b: *mut ::std::os::raw::c_void,
        __pattern4: *const ::std::os::raw::c_void,
        __len: size_t,
    );
}
extern "C" {
    pub fn memset_pattern8(
        __b: *mut ::std::os::raw::c_void,
        __pattern8: *const ::std::os::raw::c_void,
        __len: size_t,
    );
}
extern "C" {
    pub fn memset_pattern16(
        __b: *mut ::std::os::raw::c_void,
        __pattern16: *const ::std::os::raw::c_void,
        __len: size_t,
    );
}
extern "C" {
    pub fn strcasestr(
        __big: *const ::std::os::raw::c_char,
        __little: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strnstr(
        __big: *const ::std::os::raw::c_char,
        __little: *const ::std::os::raw::c_char,
        __len: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strlcat(
        __dst: *mut ::std::os::raw::c_char,
        __source: *const ::std::os::raw::c_char,
        __size: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strlcpy(
        __dst: *mut ::std::os::raw::c_char,
        __source: *const ::std::os::raw::c_char,
        __size: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strmode(__mode: ::std::os::raw::c_int, __bp: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn swab(
        arg1: *const ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
        arg3: ssize_t,
    );
}
extern "C" {
    pub fn timingsafe_bcmp(
        __b1: *const ::std::os::raw::c_void,
        __b2: *const ::std::os::raw::c_void,
        __len: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strsignal_r(
        __sig: ::std::os::raw::c_int,
        __strsignalbuf: *mut ::std::os::raw::c_char,
        __buflen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcmp(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcopy(
        arg1: *const ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
        arg3: size_t,
    );
}
extern "C" {
    pub fn bzero(arg1: *mut ::std::os::raw::c_void, arg2: ::std::os::raw::c_ulong);
}
extern "C" {
    pub fn index(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rindex(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ffs(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsl(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsll(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fls(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flsl(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flsll(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __darwin_time_t,
    pub tv_nsec: ::std::os::raw::c_long,
}
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type ino_t = __darwin_ino_t;
pub type ino64_t = __darwin_ino64_t;
pub type nlink_t = __uint16_t;
pub type gid_t = __darwin_gid_t;
pub type time_t = __darwin_time_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ostat {
    pub st_dev: __uint16_t,
    pub st_ino: ino_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_uid: __uint16_t,
    pub st_gid: __uint16_t,
    pub st_rdev: __uint16_t,
    pub st_size: __int32_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_blksize: __int32_t,
    pub st_blocks: __int32_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_mode: mode_t,
    pub st_nlink: nlink_t,
    pub st_ino: __darwin_ino64_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub st_rdev: dev_t,
    pub st_atimespec: timespec,
    pub st_mtimespec: timespec,
    pub st_ctimespec: timespec,
    pub st_birthtimespec: timespec,
    pub st_size: off_t,
    pub st_blocks: blkcnt_t,
    pub st_blksize: blksize_t,
    pub st_flags: __uint32_t,
    pub st_gen: __uint32_t,
    pub st_lspare: __int32_t,
    pub st_qspare: [__int64_t; 2usize],
}
extern "C" {
    pub fn chmod(arg1: *const ::std::os::raw::c_char, arg2: mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchmod(arg1: ::std::os::raw::c_int, arg2: mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fstat(arg1: ::std::os::raw::c_int, arg2: *mut stat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lstat(arg1: *const ::std::os::raw::c_char, arg2: *mut stat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdir(arg1: *const ::std::os::raw::c_char, arg2: mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkfifo(arg1: *const ::std::os::raw::c_char, arg2: mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn stat(arg1: *const ::std::os::raw::c_char, arg2: *mut stat) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mknod(
        arg1: *const ::std::os::raw::c_char,
        arg2: mode_t,
        arg3: dev_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn umask(arg1: mode_t) -> mode_t;
}
extern "C" {
    pub fn fchmodat(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: mode_t,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fstatat(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut stat,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdirat(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: mode_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn futimens(__fd: ::std::os::raw::c_int, __times: *const timespec)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn utimensat(
        __fd: ::std::os::raw::c_int,
        __path: *const ::std::os::raw::c_char,
        __times: *const timespec,
        __flag: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _filesec {
    _unused: [u8; 0],
}
pub type filesec_t = *mut _filesec;
extern "C" {
    pub fn chflags(arg1: *const ::std::os::raw::c_char, arg2: __uint32_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn chmodx_np(arg1: *const ::std::os::raw::c_char, arg2: filesec_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchflags(arg1: ::std::os::raw::c_int, arg2: __uint32_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fchmodx_np(arg1: ::std::os::raw::c_int, arg2: filesec_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fstatx_np(
        arg1: ::std::os::raw::c_int,
        arg2: *mut stat,
        arg3: filesec_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lchflags(arg1: *const ::std::os::raw::c_char, arg2: __uint32_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lchmod(arg1: *const ::std::os::raw::c_char, arg2: mode_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lstatx_np(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut stat,
        arg3: filesec_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdirx_np(arg1: *const ::std::os::raw::c_char, arg2: filesec_t)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkfifox_np(
        arg1: *const ::std::os::raw::c_char,
        arg2: filesec_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn statx_np(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut stat,
        arg3: filesec_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn umaskx_np(arg1: filesec_t) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dl_info {
    pub dli_fname: *const ::std::os::raw::c_char,
    pub dli_fbase: *mut ::std::os::raw::c_void,
    pub dli_sname: *const ::std::os::raw::c_char,
    pub dli_saddr: *mut ::std::os::raw::c_void,
}
pub type Dl_info = dl_info;
extern "C" {
    pub fn dladdr(arg1: *const ::std::os::raw::c_void, arg2: *mut Dl_info)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dlclose(__handle: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dlerror() -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn dlopen(
        __path: *const ::std::os::raw::c_char,
        __mode: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn dlsym(
        __handle: *mut ::std::os::raw::c_void,
        __symbol: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn dlopen_preflight(__path: *const ::std::os::raw::c_char) -> bool;
}
pub type LokHookFunction = ::std::option::Option<
    unsafe extern "C" fn(install_path: *const ::std::os::raw::c_char) -> *mut LibreOfficeKit,
>;
pub type LokHookFunction2 = ::std::option::Option<
    unsafe extern "C" fn(
        install_path: *const ::std::os::raw::c_char,
        user_profile_url: *const ::std::os::raw::c_char,
    ) -> *mut LibreOfficeKit,
>;
pub type LokHookPreInit = ::std::option::Option<
    unsafe extern "C" fn(
        install_path: *const ::std::os::raw::c_char,
        user_profile_url: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,
>;
pub type LokHookPreInit2 = ::std::option::Option<
    unsafe extern "C" fn(
        install_path: *const ::std::os::raw::c_char,
        user_profile_url: *const ::std::os::raw::c_char,
        kit: *mut *mut LibreOfficeKit,
    ) -> ::std::os::raw::c_int,
>;
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
pub type __uint128_t = u128;
