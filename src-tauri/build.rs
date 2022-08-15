fn main() {
    println!("cargo:rustc-link-arg=-Lvendor/opencv/build/3rdparty/lib");
    println!("cargo:rustc-link-arg=-littnotify");
    println!("cargo:rustc-link-arg=-llibjpeg-turbo");
    println!("cargo:rustc-link-arg=-llibpng");
    println!("cargo:rustc-link-arg=-llibprotobuf");
    println!("cargo:rustc-link-arg=-ltegra_hal");
    println!("cargo:rustc-link-arg=-lzlib");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-lframework=OpenCL");
        println!("cargo:rustc-link-arg=-lframework=Accelerate");
    }

    println!("cargo:rustc-env=OPENCV_DISABLE_PROBES=1");
    println!("cargo:rustc-env=OPENCV_LINK_PATHS=vendor/install/lib/opencv4/3rdparty/lib,vendor/opencv/build/install/lib");
    println!(
        "cargo:rustc-env=OPENCV_LINK_LIBS=opencv_core,opencv_imgproc,opencv_imgcodecs,opencv_dnn"
    );
    println!("cargo:rustc-env=OPENCV_INCLUDE_PATHS=vendor/opencv/build/install/include/opencv4");

    tauri_build::build()
}
