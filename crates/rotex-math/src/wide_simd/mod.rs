/// Runtime CPU-width detection for SIMD batch frustum culling.
#[cfg(feature = "wide-simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "wide-simd")))]
pub mod width;

/// 4-wide SSE2/NEON batch frustum culling implementation.
#[cfg(feature = "wide-simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "wide-simd")))]
pub mod x4;

/// 8-wide AVX2 batch frustum culling implementation.
#[cfg(feature = "wide-simd")]
#[cfg_attr(docsrs, doc(cfg(feature = "wide-simd")))]
pub mod x8;
