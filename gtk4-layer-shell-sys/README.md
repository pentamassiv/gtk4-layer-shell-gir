[![maintenance-status: passively-maintained (as of 2022-10-01)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2022--10--01%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

[![Build](https://img.shields.io/github/actions/workflow/status/pentamassiv/gtk4-layer-shell-gir/build_x86.yaml?branch=main)](https://github.com/pentamassiv/gtk4-layer-shell-gir/actions/workflows/build_x86.yaml)

gtk4-layer-shell-sys:
[![Crate](https://img.shields.io/crates/v/gtk4-layer-shell-sys.svg)](https://crates.io/crates/gtk4-layer-shell-sys)
[![docs.rs](https://docs.rs/gtk4-layer-shell-sys/badge.svg)](https://docs.rs/gtk4-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk4-layer-shell-sys/0.6.0/status.svg)](https://deps.rs/crate/gtk4-layer-shell-sys/0.6.0)

# gtk4-layer-shell-sys
These are the unsafe FFI bindings for [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell). They were automatically generated from its [.gir file](../Gtk4LayerShell-1.0.gir). Unfortunately this crate is GTK3 only because upstream does not yet support GTK4 (https://github.com/wmww/gtk4-layer-shell/issues/37).

## Usage
These are the unsafe bindings. You most likely want to use the safe [wrapper](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell). If you are sure you want the unsafe bindings, you can use the features to select the version of gtk4-layer-shell. Default currently is v0_6.

## Generating the bindings
Generating the bindings yourself is not necessary to be able to use it. If you want to do it anyways, here are the steps you can follow to generate the bindings yourself.

You need to have Rust, and Gtk3 installed. Clone the repository AND the submodule "gir".
```bash
git clone --recurse-submodules -j8 https://github.com/pentamassiv/gtk4-layer-shell-gir.git
cd ./gtk4-layer-shell-gir
```
Then you need to install gir.
```bash
cd gir
cargo install --path .
cd ..
```
If you regenerate the binding, because you have a new version of the GtkLayerShell gir file, copy it into the [gir files](../gir-files) folder.
Now you can generate, build and test the bindings.
```bash
cd gtk4-layer-shell-sys
gir -o .       # Regenerate the bindings
cargo build    # Build the created bindings
cargo test     # Test the created bindings
cd ..
```

There should not have been any errors. You should now continue and generate the [safe wrapper](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell/README.md#generating-the-wrapper).
If you want to learn more about gir, have a look at its [repo](https://github.com/gtk-rs/gir) or its [book](https://gtk-rs.org/gir/book/).

## Maintenance status
This crate is just an unsafe wrapper for the C library so it is feature complete and not actively worked on. There are Github Actions keeping the dependencies up-to-date. If you encounter any problems, feel free to open a PR.

## Contributing
Pull requests are very welcome but please keep the maintenance status in mind.

## License
[MIT](https://choosealicense.com/licenses/mit/)
