[![maintenance-status: passively-maintained (as of 2022-10-01)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2022--10--01%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

[![Build](https://img.shields.io/github/actions/workflow/status/pentamassiv/gtk4-layer-shell-gir/build.yaml?branch=main)](https://github.com/pentamassiv/gtk4-layer-shell-gir/actions/workflows/build.yaml)

gtk4-layer-shell-sys:
[![Crate](https://img.shields.io/crates/v/gtk4-layer-shell-sys.svg)](https://crates.io/crates/gtk4-layer-shell-sys)
[![docs.rs](https://docs.rs/gtk4-layer-shell-sys/badge.svg)](https://docs.rs/gtk4-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk4-layer-shell-sys/0.1.2/status.svg)](https://deps.rs/crate/gtk4-layer-shell-sys/0.1.2)

# gtk4-layer-shell-sys
These are the unsafe FFI bindings for [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell). They were automatically generated from its [.gir file](../Gtk4LayerShell-1.0.gir). You need to have gtk4-layer-shell installed on your system to use this crate. Because it is new, you probably have to [build it from source](https://github.com/wmww/gtk4-layer-shell#building-from-source). If you did that, you might also have to set the following two environment variables:
```bash
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
export LD_LIBRARY_PATH=/usr/local/lib
```
This crate is GTK4 only. Checkout [gtk-layer-shell-sys](https://crates.io/crates/gtk-layer-shell-sys) if you want the GTK3 version.

## Usage
These are the unsafe bindings. You most likely want to use the safe [wrapper](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell).

## Generating the bindings
Generating the bindings yourself is not necessary to be able to use it. If you want to do it anyways, here are the steps you can follow to generate the bindings yourself.

You need to have Rust, and Gtk4 and gtk4-layer-shell installed. Clone the repository AND the submodule "gir".
```bash
git clone --recurse-submodules -j8 https://github.com/pentamassiv/gtk4-layer-shell-gir.git
git pull --recurse-submodules
cd ./gtk4-layer-shell-gir
```
Then you need to install gir.
```bash
cd gir
cargo install --path .
cd ..
```
If you regenerate the binding, because you have a new version of the Gtk4LayerShell gir file, copy it into the [gir files](../gir-files) folder.
Now you can generate, build and test the bindings.
```bash
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
cd gtk4-layer-shell-sys # Needed when the lib is built from source
gir -o .       # Regenerate the bindings
cargo build    # Build the created bindings
cargo test     # Test the created bindings
cd ..
```

There should not have been any errors. If you do encounter an error complaining about the `dox` feature not being available for the `gtk4` crate, you have to change the name of the `gtk4` crate in the `Cargo.toml` file. You should now continue and generate the [safe wrapper](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell/README.md#generating-the-wrapper).
If you want to learn more about gir, have a look at its [repo](https://github.com/gtk-rs/gir) or its [book](https://gtk-rs.org/gir/book/).

## Maintenance status
This crate is just an unsafe wrapper for the C library so it is feature complete and not actively worked on. There are Github Actions keeping the dependencies up-to-date. If you encounter any problems, feel free to open a PR.

## Contributing
Pull requests are very welcome but please keep the maintenance status in mind.

## License
[MIT](https://choosealicense.com/licenses/mit/)
