# OpenCV

clone
checkout 4.x
mkdir build && cd build

rm -rf ./* && cmake -DBUILD_SHARED_LIBS=off -DBUILD_LIST=core,dnn,imgcodecs -DCMAKE_INSTALL_PREFIX=install -DCMAKE_BUILD_TYPE=RELEASE -DWITH_OPENEXR=off -DWITH_TIFF=off -DWITH_WEBP=off -DWITH_OPENJPEG=off -DWITH_JASPER=off ..  && cmake --build . -j 8 && make install



# App

RUSTFLAGS='-L /Users/ben/Projects/opencv/build/3rdparty/lib  -l ittnotify -l libjpeg-turbo  -l libpng -l libprotobuf -l tegra_hal -l zlib -l framework=OpenCL -l framework=Accelerate'  OPENCV_DISABLE_PROBES=1  OPENCV_LINK_PATHS='/Users/ben/Projects/opencv/build/install/lib/opencv4/3rdparty/lib,/Users/ben/Projects/opencv/build/install/lib' OPENCV_LINK_LIBS='opencv_core,opencv_imgproc,opencv_imgcodecs,opencv_dnn' OPENCV_INCLUDE_PATHS='/Users/ben/Projects/opencv/build/install/include/opencv4' npx tauri build --features builtin
