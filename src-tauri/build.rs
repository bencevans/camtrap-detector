fn main() {
    println!("cargo:rustc-link-search=vendor/opencv/build/3rdparty/lib");
    println!("cargo:rustc-link-lib=ittnotify");
    println!("cargo:rustc-link-lib=libjpeg-turbo");
    println!("cargo:rustc-link-lib=libpng");
    println!("cargo:rustc-link-lib=libprotobuf");
    println!("cargo:rustc-link-lib=tegra_hal");
    println!("cargo:rustc-link-lib=zlib");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=OpenCL");
        println!("cargo:rustc-link-lib=framework=Accelerate");
    }

    println!("cargo:rustc-env=OPENCV_DISABLE_PROBES=1");
    println!("cargo:rustc-env=OPENCV_LINK_PATHS=vendor/install/lib/opencv4/3rdparty/lib,vendor/opencv/build/install/lib");
    println!(
        "cargo:rustc-env=OPENCV_LINK_LIBS=opencv_core,opencv_imgproc,opencv_imgcodecs,opencv_dnn"
    );
    println!("cargo:rustc-env=OPENCV_INCLUDE_PATHS=vendor/opencv/build/install/include/opencv4");

    tauri_build::build()
}
