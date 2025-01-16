# Tauri + Vanilla

This template should help get you started developing with Tauri in vanilla HTML, CSS and Javascript.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## iOS steps

- npm run tauri ios init
- npm run tauri ios dev open  , dann einmal das Team einstellen

* In Xcode, open the src-tauri/gen/apple/places.xcodeproj/project.xcworkspace.
* Select your project (places_iOS).
* Go to the Signing & Capabilities tab.
* Make sure Automatically manage signing is enabled.
* Select your team from the team dropdown.

<img width="1061" alt="image" src="https://github.com/user-attachments/assets/ad5eae1d-9cdc-4ade-a2c7-faac8df1431e" />

Attention: if error here, check if the app identifier is gloablly unqiue. if it's not, change in tauri.conf.json! Run again.

- npm run tauri ios dev --host

## Build errors 

Thanks to SIMD on ios

> The problem we keep facing is that the accelerate feature from candle-core and candle-transformers enables SIMD operations, including those that use fullfp16 which the target architecture is not compatible with (or we are not building for). While we try to disable it for some dependencies, it seems to be trickling in through transitive dependencies, and when the architecture is aarch64-apple-ios the default features will be enabled, causing the compiler to try to use instructions that are not supported.

Feed Gemini with this info: https://github.com/sarah-quinones/gemm/issues/31


