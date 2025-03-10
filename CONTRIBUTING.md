# Contributing

CamTrap Detector is wriiten in the Rust and JavaScript languages using the Tauri application framework.

## Dependencies

### Install Rust

Instructions for installing Rust can be found [here](https://www.rust-lang.org/tools/install).

### Install Node.js

Instructions for installing Node.js can be found [here](https://nodejs.org/en/download/).

### Clone the repository

Assuming [Git](https://git-scm.com/) is installed, clone the repository:

    git clone https://github.com/bencevans/camtrap-detector.git
    cd camtrap-detector

### Install `npm` packages

Install the npm dependencies, this should be repeated each time any of the `package.json` files are updated.

    npm install

### Download Model

    wget -O md_v5a.0.0-dynamic.onnx https://github.com/bencevans/megadetector-onnx/releases/download/v0.2.0/md_v5a.0.0-dynamic.onnx

## Development

### Run the application with reload

To run the application in development mode, run:

    npm run tauri dev

This will start the application in development mode, with reloading enabled, so any changes to the source code will be automatically reloaded.

### Build the application

To build the application, run:

    npm run tauri build

This will build the application for the current platform.

### Test the application

To run the frontend lint tests, run:

    npm run lint

To run the backend tests, run:

    cargo test

Please note that the backend tests require the `md_v5a.0.0-dynamic.onnx` model to be present in the root of the repository.
