use basis_universal_sys as sys;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{LazyLock, Mutex};

mod compressor_image;
pub use compressor_image::*;

mod compressor_params;
pub use compressor_params::*;

mod compressor;
pub use compressor::*;

/// A single uncompressed pixel value
pub use basis_universal_sys::ColorU8;

#[cfg(test)]
mod encoding_tests;


/// The underlying C++ library requires that encoder_init() has been called before a .basis file can
/// be encoded. This function allows a user to do this early in the application explicitly. It is
/// protected by a lock and AtomicBool flag so it is safe and cheap to call multiple times, and
/// correctly handles multiple threads trying to initialize at the same time.
pub fn encoder_init() {
    unsafe {
        // LazyLock đảm bảo hàm bên trong chỉ chạy DUY NHẤT một lần
        // và an toàn tuyệt đối giữa các thread (thread-safe).
        static INIT: LazyLock<()> = LazyLock::new(|| {
            unsafe {
                sys::basisu_encoder_init();
            }
        });

        // Chỉ cần "chạm" vào INIT, Rust sẽ tự lo việc lock và khởi tạo
        let _ = *INIT;
    }
}
