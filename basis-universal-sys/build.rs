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
    // Compile this separately as c code
    build_with_common_settings()
        .file("vendor/basis_universal/encoder/apg_bmp.c")
        .compile("basisuniversalc");

    build_with_common_settings()
        .cpp(true)
        .define("BASISD_SUPPORT_KTX2_ZSTD", "0")
        .define("BASISU_SUPPORT_SSE", "1")
        .flag_if_supported("-msse4.1")
        .flag_if_supported("--std=c++11")
        .file("unity.cpp")
        .flag("-w")
        .debug(false)
        .compile("basisuniversal");

    // We regenerate binding code and check it in. (See generate_bindings.sh)
}
