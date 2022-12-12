use std::env::var;

fn main() {
    let manifest_dir = var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/vendor/opencv/build/3rdparty/lib", manifest_dir);
    println!("cargo:rustc-link-lib=libjpeg-turbo");
    println!("cargo:rustc-link-lib=libpng");
    println!("cargo:rustc-link-lib=libprotobuf");
    println!("cargo:rustc-link-lib=zlib");

    if cfg!(target_arch = "aarch64") {
        println!("cargo:rustc-link-lib=tegra_hal");
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenCL");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    tauri_build::build()
}
