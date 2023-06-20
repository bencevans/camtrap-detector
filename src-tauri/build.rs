fn main() {
    if cfg!(feature = "cuda") {
        let cuda_path = std::env::var("CUDA_PATH").expect("CUDA_PATH not set");
        println!("cargo:rustc-link-search=native={}\\lib\\x64", cuda_path);
        println!("cargo:rustc-link-lib=static=cudart_static");
        println!("cargo:rustc-link-lib=static=cublas");
        println!("cargo:rustc-link-lib=static=cublasLt");
        println!("cargo:rustc-link-lib=static=cudnn");
    }

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenCL");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    tauri_build::build()
}
