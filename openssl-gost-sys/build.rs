use std::{env, path::PathBuf};

fn main() {
    let openssl_package = vcpkg::find_package("openssl").unwrap();

    let mut builder = bindgen::builder()
        .layout_tests(false)
        .ctypes_prefix("::libc")
        .header("../openssl-gost-engine/gost_grasshopper.h")
        .clang_arg("-I")
        .clang_arg("./engine");

    for path in openssl_package.include_paths {
        builder = builder
            .clang_arg("-I")
            .clang_arg(path.into_os_string().into_string().unwrap());
    }

    let result = builder.generate();

    let out_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());

    result
        .unwrap()
        .write_to_file(out_dir.join("src").join("bindings.rs"))
        .unwrap();
}
