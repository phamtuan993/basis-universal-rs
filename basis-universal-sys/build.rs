use std::env;

// args from the basis cmake file
fn build_with_common_settings() -> cc::Build {
    let mut build = cc::Build::new();
    build
        .flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-fno-strict-aliasing")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-Wno-unused-local-typedefs")
        .flag_if_supported("-Wno-unused-value")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-variable");

    build
}

fn main() {
    // 2. Duyệt qua các flag để tìm target-feature
    let mut build = build_with_common_settings()
        .cpp(true)
        //.define("BASISD_SUPPORT_KTX2_ZSTD", "0")
        //.define("BASISU_SUPPORT_SSE", "1") TODO: expose this in a futher release
        //.define("BASISD_SUPPORT_KTX2", "0")
        .define("BASISD_SUPPORT_KTX2_ZSTD", "0")

        // Enable required formats:
        .define("BASISD_SUPPORT_UASTC", "1")
        .define("BASISD_SUPPORT_UASTC_HDR", "1")
        .define("BASISD_SUPPORT_DXT5A", "1")
        .define("BASISD_SUPPORT_BC7", "1")
        .define("BASISD_SUPPORT_BC7_MODE5", "1")
        // Disable the rest.
        .define("BASISD_SUPPORT_DXT1", "0")
        .define("BASISD_SUPPORT_PVRTC1", "0")
        .define("BASISD_SUPPORT_ETC2_EAC_A8", "0")
        .define("BASISD_SUPPORT_ASTC", "0")
        .define("BASISD_SUPPORT_ATC", "0")
        .define("BASISD_SUPPORT_ASTC_HIGHER_OPAQUE_QUALITY", "0")
        .define("BASISD_SUPPORT_ETC2_EAC_RG11", "0")
        .define("BASISD_SUPPORT_FXT1", "0")
        .define("BASISD_SUPPORT_PVRTC2", "0")
        .std("c++11").to_owned();
    // 1. Lấy thông tin compiler hiện tại
    let compiler = build.get_compiler();
    let is_msvc = compiler.is_like_msvc();
    // 1. Lấy toàn bộ flag mà Cargo đang dùng để build Rust
    let encoded_flags = env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default();
    let flags: Vec<&str> = encoded_flags.split('\x1f').collect();
    println!("cargo:warning=ENCODED_FLAGS: {:?}", std::env::var("CARGO_ENCODED_RUSTFLAGS"));
    let mut baisu_sse = 0;
    for flag in flags {
        if flag.starts_with("target-feature="){
            let features = flag.trim_start_matches("target-feature=").split(',');
            for feature in features {
                match feature {
                    "+avx" => {
                        if is_msvc { build.flag("/arch:AVX"); }
                        else { build.flag("-mavx"); }
                        baisu_sse = 1;
                    }
                    "+avx2" => {
                        if is_msvc { build.flag("/arch:AVX2"); }
                        else { build.flag("-mavx2"); }
                        baisu_sse = 1;
                    }
                    "+sse4.1" => {
                        if is_msvc { build.flag("/arch:SSE4.1"); }
                        else { build.flag("-msse4.1"); }
                        baisu_sse = 1;
                    }
                    _ => {}
                }
            }
        }
    }
    build.define("BASISU_SUPPORT_SSE", if baisu_sse == 0 { "0" }else{"1"});
    #[cfg(feature = "encoding")]
    {
        build
            .file("vendor/basis_universal/encoder/pvpngreader.cpp")
            .file("vendor/basis_universal/encoder/jpgd.cpp")
            .file("vendor/basis_universal/encoder/basisu_uastc_enc.cpp")
            .file("vendor/basis_universal/encoder/basisu_ssim.cpp")
            .file("vendor/basis_universal/encoder/basisu_resampler.cpp")
            .file("vendor/basis_universal/encoder/basisu_resample_filters.cpp")
            .file("vendor/basis_universal/encoder/basisu_pvrtc1_4.cpp")
            .file("vendor/basis_universal/encoder/basisu_opencl.cpp")
            .file("vendor/basis_universal/encoder/basisu_kernels_sse.cpp")
            .file("vendor/basis_universal/encoder/basisu_gpu_texture.cpp")
            .file("vendor/basis_universal/encoder/basisu_frontend.cpp")
            .file("vendor/basis_universal/encoder/basisu_etc.cpp")
            .file("vendor/basis_universal/encoder/basisu_enc.cpp")
            .file("vendor/basis_universal/encoder/basisu_comp.cpp")
            .file("vendor/basis_universal/encoder/basisu_bc7enc.cpp")
            .file("vendor/basis_universal/encoder/basisu_basis_file.cpp")
            .file("vendor/basis_universal/encoder/basisu_backend.cpp")
            .file("vendor/encoding_wrapper.cpp");
    }
    #[cfg(feature = "transcoding")]
    {
        build
            .file("vendor/basis_universal/transcoder/basisu_transcoder.cpp")
            .file("vendor/transcoding_wrapper.cpp");
    }

    build.compile("basisuniversal");

    // We regenerate binding code and check it in. (See generate_bindings.sh)
}
