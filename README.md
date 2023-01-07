<p align="center">
  <img src="app-icon.svg" alt="drawing" width="120"/>
</p>

<h1 align="center">CamTrap Detector</h1>

<p align="center">Detect Animals, Humans and Vehicles with a friendly graphical interface. Powered by MegaDetector v5.</p>

## Features

* 👀 Detects **Animals, Humans and Vehicles** in Camera Trap Imagery
* 🧑‍💻 Runs on **Windows, macOS & Ubuntu**
* ➡️ Multiple Export Formats
  * **CSV** for working in Excel, Numbers etc.
  * **JSON** for integration with other tooling.
  * **Images** filtered by animal/vehicle/human occupancy with bounding boxes/detections drawn on.
* 🔌 Run anywhere, **no internet required**.
* 🕵️ **Privacy Preserving**: No need to share images with a 3rd party
* 🚀 Acceleration using NVIDIA GPUs with **CUDA**
* 💰 **Free and Open Source**... contributions and/or sponsorship are very welcome.


## Installation

Installers are available from the [releases](https://github.com/bencevans/camtrap-detector/releases) page on GitHub under assets of the most recent verion. To identify which file is suitable for your computer, please look for the file with the extention identified in the following table. Once the installer has been downloaded, no internet is required to install and run the application.

| Platform      | CPU               | CUDA            |
|---------------|-------------------|-----------------|
| Windows       | x64_en-US.msi     | en-US-cu117.msi |
| macOS (Intel) | x64.dmg           | n/a             |
| macOS (M1/M2) | aarch64.dmg       | n/a             |
| Linux         | .deb or .AppImage | n/a             |



## Screenshots


| Select  | Process | Export |
|---------|---------|--------|
| <img width="306" alt="macos_selection" src="https://user-images.githubusercontent.com/638535/210103770-8b3c3730-9cb7-4a7c-85a6-8530ed13ad7c.png"> | <img width="306" alt="macos_processing" src="https://user-images.githubusercontent.com/638535/210103781-8418d75c-7543-4d7b-9f68-f191986d8321.png"> | <img width="356" alt="macos_export" src="https://user-images.githubusercontent.com/638535/210103789-9bf1de1d-2f0f-4099-8f22-c8e79e1b4065.png"> |
|![windows_select png-shadow](https://user-images.githubusercontent.com/638535/210595909-1856abf5-a92f-4264-aace-ad724dee1144.png)|![windows_process png-shadow](https://user-images.githubusercontent.com/638535/210595978-deb9eff2-9dec-4b6d-b96f-d0ba53485798.png)|![windows_export png-shadow](https://user-images.githubusercontent.com/638535/210596005-104d8257-d24d-4127-af86-0f3bce43ee3b.png)|










## ❤️ Sponsors and Contributors

Great big thank you to all that have supported this project. Including [London HogWatch](https://www.zsl.org/conservation/species/mammals/london-hogwatch) for inital funding and continued guidance. [Blue Chimp Limited](https://bluechimp.io) for development and compute resources. [MegaDetector](https://github.com/microsoft/CameraTraps/graphs/contributors) Team for their work collating, developing and producing generalisable detection models. The many other Open Source projects this work sits upon ([nodejs modules](package.json), [rust crates](src-tauri/Cargo.toml)).

## Contributing

Contributions of all kinds are welcome!

* Found a bug? Please open an [issue](https://github.com/bencevans/camtrap-detector/issues/new) and let us know
* Looking for a feature? Start a [discussion](https://github.com/bencevans/camtrap-detector/discussions/new)
* Fancy hacking on the project? Please see [CONTRIBUTING.md](CONTRIBUTING.md) for more information on getting set up.
* Looking to help fund work on CamTrap Detector and futher open source projects? [https://github.com/sponsors/bencevans](https://github.com/sponsors/bencevans)


