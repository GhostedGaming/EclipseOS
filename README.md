# eclipse_os (Async/Await)

[![Build Status](https://github.com/phil-opp/eclipse_os/workflows/Code/badge.svg?branch=post-12)](https://github.com/phil-opp/eclipse_os/actions?query=workflow%3A%22Code%22+branch%3Apost-12)

This repository contains the source code for the [Async/Await][post] post of the [Writing an OS in Rust](https://os.phil-opp.com) series.

[post]: https://os.phil-opp.com/async-await/

**Check out the [master branch](https://github.com/phil-opp/eclipse_os) for more information.**

## Building

Install QEMU using this command for Windows:
```sh
winget install --id=SoftwareFreedomConservancy.QEMU -e
```
or use this command for Linux:
```sh
sudo apt-get install qemu-system
```

This project requires a nightly version of Rust because it uses some unstable features. At least nightly _2020-07-15_ is required for building. You might need to run `rustup update nightly --force` to update to the latest nightly even if some components such as `rustfmt` are missing.

You can build the project by running:
```sh
cargo build
```

To create a bootable disk image from the compiled kernel, you need to install the [`bootimage`] tool:

[`bootimage`]: https://github.com/rust-osdev/bootimage

```sh
cargo install bootimage
```

After installing, you can create the bootable disk image by running:
```sh
cargo bootimage
```

This creates a bootable disk image in the `target/x86_64-eclipse_os/debug` directory.

Please file an issue if you have any problems.

## Running

You can run the disk image in [QEMU] through:

[QEMU]: https://www.qemu.org/

```sh
cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.

I also made a shell script to run both of these:
For Linux:
```sh
chmod +x ./run.sh
./run.sh
```
For Windows:
```sh
.\run.sh
```

You can also write the image to a USB stick for booting it on a real machine. On Linux, the command for this is:
```sh
dd if=target/x86_64-eclipse_os/debug/bootimage-eclipse_os.bin of=/dev/sdX && sync
```
You can also download Balena Etcher, which is excellent for writing bootable images to drives.

Where `sdX` is the device name of your USB stick. **Be careful** to choose the correct device name, because everything on that device is overwritten.

## Testing

To run the unit and integration tests, execute:
```sh
cargo xtest
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Note that this only applies to this git branch, other branches might be licensed differently.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
