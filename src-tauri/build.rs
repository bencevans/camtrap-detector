fn main() {
    if cfg!(target_arch = "aarch64") {
        println!("cargo:rustc-link-lib=tegra_hal");
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenCL");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    tauri_build::build()
}
