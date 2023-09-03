[![maintenance-status: passively-maintained (as of 2023-04-12)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2023--04--12%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)

[![Build](https://img.shields.io/github/actions/workflow/status/pentamassiv/gtk4-layer-shell-gir/build.yaml?branch=main)](https://github.com/pentamassiv/gtk4-layer-shell-gir/actions/workflows/build.yaml)

gtk4-layer-shell:
[![Crate](https://img.shields.io/crates/v/gtk4-layer-shell.svg)](https://crates.io/crates/gtk4-layer-shell)
[![docs.rs](https://docs.rs/gtk4-layer-shell/badge.svg)](https://docs.rs/gtk4-layer-shell)
[![dependency status](https://deps.rs/crate/gtk4-layer-shell/0.1.5/status.svg)](https://deps.rs/crate/gtk4-layer-shell/0.1.5)

gtk4-layer-shell-sys:
[![Crate](https://img.shields.io/crates/v/gtk4-layer-shell-sys.svg)](https://crates.io/crates/gtk4-layer-shell-sys)
[![docs.rs](https://docs.rs/gtk4-layer-shell-sys/badge.svg)](https://docs.rs/gtk4-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk4-layer-shell-sys/0.1.2/status.svg)](https://deps.rs/crate/gtk4-layer-shell-sys/0.1.2)


# gtk4-layer-shell
This is the safe wrapper for [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell), automatically generated from its [.gir file](Gtk4LayerShell-1.0.gir). For details on how to use it or how to generate the code yourself, have a look at the crate's [README](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell/README.md). The unsafe bindings can be found in this [folder](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell-sys).
You need to have gtk4-layer-shell installed on your system to use this crate. Because it is new, you probably have to [build it from source](https://github.com/wmww/gtk4-layer-shell#building-from-source). If you did that, you might also have to set the following two environment variables:
```bash
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
export LD_LIBRARY_PATH=/usr/local/lib
```
These crates are GTK4 only. Checkout [gtk-layer-shell](https://crates.io/crates/gtk-layer-shell) or [gtk-layer-shell-sys](https://crates.io/crates/gtk-layer-shell-sys) if you want the GTK3 versions.

## Maintenance status
These crates are just wrappers for the C library so it is feature complete and not actively worked on. The C library was recently released so there might be frequent changes. I try to keep up with them so the crate might get frequent updates and could break things. If you encounter any problems, feel free to open a PR.

## Contributing
Pull requests are very welcome but please keep the maintenance status in mind.

## License
[MIT](https://choosealicense.com/licenses/mit/)
