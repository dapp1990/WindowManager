# CPLWM Backend

This is the code of the X11 backend and runner for the window manager of the
CPL Rust project.

If you find a bug, please file an issue. Pull requests are also accepted.



## How to use it

1. Start by cloning this repository.

2. Copy over the `runner` and `x11` folders, and the `Cargo.toml` file from
   this repository to the `cplwm` folder you already have. This folder should
   already contain amongst others the `api` and `assignment` folders. The
   `Cargo.toml` file from this repository should replace the one you already
   have.

3. Start reading the [documentation][runner] of the runner.

4. Optional: read the [documentation][backend] of the X11 backend.

[runner]: https://people.cs.kuleuven.be/~thomas.winant/cpl/doc/cplwm_runner/
[backend]: https://people.cs.kuleuven.be/~thomas.winant/cpl/doc/cplwm_x11/
