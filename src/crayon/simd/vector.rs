use std::arch;

#[repr(C)]
#[repr(align(16))]
struct I32X4AlignedLanes {
    pub v: [i32; 4],
}

#[cfg(target_arch = "x86_64")]
struct I32X4 {
    v: arch::x86_64::__m128i,
}

impl From<&I32X4AlignedLanes> for I32X4 {
    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    fn from(xyzw: &I32X4AlignedLanes) -> Self {
        unsafe {
            Self {
                v: arch::x86_64::_mm_load_si128(xyzw.v.as_ptr() as *const arch::x86_64::__m128i),
            }
        }
    }
}

impl I32X4 {
    #[cfg(target_arch = "x86_64")]
    #[inline(always)]
    pub fn with_replicated_scalar(xyzw: i32) -> Self {
        unsafe {
            Self {
                v: arch::x86_64::_mm_set1_epi32(xyzw),
            }
        }
    }
}
