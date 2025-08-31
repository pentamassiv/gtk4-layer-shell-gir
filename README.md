gtk4-layer-shell:
[![Crate](https://img.shields.io/crates/v/gtk4-layer-shell.svg)](https://crates.io/crates/gtk4-layer-shell)
[![docs.rs](https://docs.rs/gtk4-layer-shell/badge.svg)](https://docs.rs/gtk4-layer-shell)
[![dependency status](https://deps.rs/crate/gtk4-layer-shell/0.5.0/status.svg)](https://deps.rs/crate/gtk4-layer-shell/0.5.0)

gtk4-layer-shell-sys:
[![Crate](https://img.shields.io/crates/v/gtk4-layer-shell-sys.svg)](https://crates.io/crates/gtk4-layer-shell-sys)
[![docs.rs](https://docs.rs/gtk4-layer-shell-sys/badge.svg)](https://docs.rs/gtk4-layer-shell-sys)
[![dependency status](https://deps.rs/crate/gtk4-layer-shell-sys/0.4.0/status.svg)](https://deps.rs/crate/gtk4-layer-shell-sys/0.4.0)

gtk4-session-lock:
[![Crate](https://img.shields.io/crates/v/gtk4-session-lock.svg)](https://crates.io/crates/gtk4-session-lock)
[![docs.rs](https://docs.rs/gtk4-session-lock/badge.svg)](https://docs.rs/gtk4-session-lock)
[![dependency status](https://deps.rs/crate/gtk4-session-lock/0.2.0/status.svg)](https://deps.rs/crate/gtk4-session-lock/0.2.0)

gtk4-session-lock-sys:
[![Crate](https://img.shields.io/crates/v/gtk4-session-lock-sys.svg)](https://crates.io/crates/gtk4-session-lock-sys)
[![docs.rs](https://docs.rs/gtk4-session-lock-sys/badge.svg)](https://docs.rs/gtk4-session-lock-sys)
[![dependency status](https://deps.rs/crate/gtk4-session-lock-sys/0.2.0/status.svg)](https://deps.rs/crate/gtk4-session-lock-sys/0.2.0)

[![maintenance-status: passively-maintained (as of 2023-04-12)](https://img.shields.io/badge/maintenance--status-passively--maintained_%28as_of_2023--04--12%29-forestgreen)](https://gist.github.com/rusty-snake/574a91f1df9f97ec77ca308d6d731e29)
![dependabot status](https://img.shields.io/badge/dependabot-enabled-025e8c?logo=Dependabot)
[![Build](https://img.shields.io/github/actions/workflow/status/pentamassiv/gtk4-layer-shell-gir/build.yaml?branch=main)](https://github.com/pentamassiv/gtk4-layer-shell-gir/actions/workflows/build.yaml)

# Contents
This repo contains the following crates:

- [gtk4-layer-shell](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell): Build desktop shell components such as panels, notifications and wallpapers with GTK4. It can be used to anchor your windows to a corner or edge of the output, or stretch them across the entire output
- [gtk4-layer-shell-sys](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell-sys): Unsafe bindings used by gtk-layer-shell
- [gtk4-session-lock](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-session-lock): Build lockscreens with GTK4
- [gtk4-session-lock-sys](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-session-lock-sys): Unsafe bindings used by gtk-session-lock


The crates are language bindings to the underlying C library [gtk4-layer-shell](https://github.com/wmww/gtk4-layer-shell). Feature development is done upstream. The crates in this repo are automatically generated from their .gir files ([Gtk4LayerShell](Gtk4LayerShell-1.0.gir), [GTK4SessionLock](Gtk4SessionLock-1.0.gir)). For details on how to use the crates, have a look at the individual crate's README, which can be found in their respective folders. The crates are GTK4 only.

## Dependencies
You need to have `gtk4` and `gtk4-layer-shell` (the C library) installed on your system to use the crates in this repo. `gtk4-layer-shell` version 1.1.0 or higher is needed if you want to use `gtk4-session-lock`.
If you want to use [gtk4-layer-shell](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-layer-shell) and [gtk4-session-lock](https://github.com/pentamassiv/gtk4-layer-shell-gir/tree/main/gtk4-session-lock) together in a project, make sure to use the same .so file of `gtk4-layer-shell` for both.

If your distribution does not provide a current enough version of `gtk4-layer-shell`, you can [build it from source](https://github.com/wmww/gtk4-layer-shell#building-from-source). If you did that, you might also have to set the following two environment variables:
```bash
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
export LD_LIBRARY_PATH=/usr/local/lib
```

## Maintenance status
These crates are just wrappers for the C library so the crates in this repo are feature complete and not actively worked on. The C library is actively developed. I try to keep up with them so the crate might get frequent updates and could break things. If you encounter any problems, feel free to open a PR.

## Contributing
Pull requests are very welcome but please keep the maintenance status in mind.

## License
[MIT](https://choosealicense.com/licenses/mit/)
