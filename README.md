# esp-rust-board-bsp

A board support package for the [ESP Rust Board]. Includes the dependencies required to interact with the various sensors and external peripherals present on the board, and additionally provides some helpers to improve quality of life.

[esp rust board]: https://github.com/esp-rs/esp-rust-board/

## Debugging with Visual Studio Code

You can debug your applications for the ESP Rust Board using [Visual Studio Code] together with the [Cortex-Debug plugin].
This repository provides a [VSCode launch configuration] for debugging all the provided examples.
Feel free to use this file as a template for your applications.
Note that you need to install the [riscv32-esp-elf] tools provided by Xtensa to successfully debug with VSCode.

**IMPORTANT:** in the `launch.json` file of your application, you must properly modify `gdbPath` and `serverpath` to point to the location of the [riscv32-esp-elf] tools in your machine.

**IMPORTANT:** as the launch configuration request is set to `attach`, VSCode won't flash your application to your Rust Board.
Instead, it assumes that your board has been already flashed with the target application.
Therefore, you *must* flash your application to your board before starting to debug with VSCode.

We have not been able to use the features of the Cortex-Debug plugin to see the logs of the applications while debugging.
As a temporary solution, you can use the [Serial Monitor] plugin for VSCode.
You can use simultaneously the Cortex-Debug plugin for proper debugging and the Serial Monitor plugin for reading the logs while debugging.

[visual studio code]: https://code.visualstudio.com
[cortex-debug plugin]: https://marketplace.visualstudio.com/items?itemName=marus25.cortex-debug
[vscode launch configuration]: .vscode/launch.json
[riscv32-esp-elf]: https://docs.espressif.com/projects/esp-idf/en/latest/esp32s2/api-guides/tools/idf-tools.html#riscv32-esp-elf
[serial monitor]: https://marketplace.visualstudio.com/items?itemName=ms-vscode.vscode-serial-monitor

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
