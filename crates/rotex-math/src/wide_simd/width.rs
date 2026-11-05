use std::sync::OnceLock;

/// The widest SIMD path supported by the current CPU.
///
/// Detected once at runtime via [`detected_width`] and cached for the
/// process lifetime.
///
/// | Variant | x86_64 | aarch64 |
/// |---------|--------|---------|
/// | `X8`    | AVX2   | —       |
/// | `X4`    | SSE2   | NEON    |
/// | `Scalar`| —      | —       |
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdWidth {
    /// No SIMD — use the scalar per-AABB loop.
    Scalar,
    /// 4-wide SIMD (SSE2 on x86_64, NEON on aarch64).
    X4,
    /// 8-wide SIMD (AVX2 on x86_64).
    X8,
}

/// Returns the widest SIMD path supported by the current CPU.
///
/// The result is cached in a [`OnceLock`] so the detection runs at most once.
///
/// # Examples
///
/// ```rust
/// use rotex_math::wide_simd::width::SimdWidth;
/// let width = rotex_math::wide_simd::width::detected_width();
/// assert!(matches!(width, SimdWidth::X4 | SimdWidth::X8 | SimdWidth::Scalar));
/// ```
pub fn detected_width() -> SimdWidth {
    static WIDTH: OnceLock<SimdWidth> = OnceLock::new();
    *WIDTH.get_or_init(detect)
}

#[cfg(target_arch = "x86_64")]
fn detect() -> SimdWidth {
    if std::is_x86_feature_detected!("avx2") {
        SimdWidth::X8
    } else {
        SimdWidth::X4
    }
}

#[cfg(target_arch = "aarch64")]
fn detect() -> SimdWidth {
    SimdWidth::X4
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
fn detect() -> SimdWidth {
    SimdWidth::Scalar
}
