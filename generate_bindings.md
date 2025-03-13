# Generating the bindings
Generating the bindings yourself is not necessary. You can just use the version published on crates.io. If you want to do it anyways, here are the steps you need to follow. Generating the crates is a two step process. First the unsafe "-sys" crates need to be generated they are the FFI bindings to the C library. Afterwards the safe wrapper crates can get generated.

## Prerequisites
You need to have the [dependencies](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/README.md#Dependencies) installed on your system.

## Generating the unsafe FFI bindings
The unsafe FFI crates are automatically generated from their respective .gir file ([Gtk4LayerShell](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/Gtk4LayerShell-1.0.gir) or [Gtk4SessionLock](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/Gtk4SessionLock-1.0.gir)). If you want to learn more about gir, have a look at its [repo](https://github.com/gtk-rs/gir) or its [book](https://gtk-rs.org/gir/book/). 

Clone the repository AND the submodules "gir" and "gir-files".
```bash
git clone --recurse-submodules  -j8 https://github.com/pentamassiv/gtk4-layer-shell-gir.git
cd ./gtk4-layer-shell-gir
```
Then you need to install gir.
```bash
cd gir
cargo install --path .
cd ..
```
If you regenerate the binding, because you have a new version of the gir file, replace the old file with the new version. Now you can generate, build and test the bindings.
```bash
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
# Change the directory to the folder of the sys crate you want to rebuild
gir -o .       # Regenerate the bindings
cargo build    # Build the created bindings
cargo test     # Test the created bindings
cd ..
```

There should not have been any errors.

## Generating the safe wrapper crate
Now that you have generated the bindings you will want to generate the safe wrapper.
```bash
# Change the directory to the folder of the wrapper crate you want to rebuild
gir -o .
cargo build
cargo test
```
There should not have been any errors.
To make sure everything you need was created, run the following command.
```bash
gir -o . -m not_bound
```
There should not be any output to this command. Let me know if there is so I can fix it. 
In order to build the documentation, you have to run
```
gir -c Gir.toml --doc-target-path docs.md -m doc
cargo install rustdoc-stripper
rustdoc-stripper -s -n
rustdoc-stripper -g -o docs.md
cargo doc
```
Congratulations, you've done it :-)
