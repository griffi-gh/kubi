<h1 align="center">Kubi</h1>
<p align="center">
  Voxel engine written in Rust
</p>
<div align="center">
  <img src=".readme/game.gif" width="512">
</div>

<h2>features</h2>

<p>
  <ul>
    <li>multithreaded procedural world generation</li>
    <li>procedural structures</li>
    <li>multithreaded mesh generation</li>
    <li>cubic chunks (32x32x32)</li>
    <li>low-level OpenGL renderer, targetting OpenGL ES 3.0</li>
    <li>frustum culling</li>
    <li>work-in-progress multiplayer support</li>
    <li>block placement system</li>
    <li>partial gamepad input support</li>
    <li>(experimental) android support</li>
  </ul>
</p>

<h2>download</h2>
<a href="https://github.com/griffi-gh/kubi/releases/tag/nightly">Latest nightly release</a>

<h2>building</h2>

build/run

```bash
cargo build --bin kubi
cargo run --bin kubi
```

build with nightly features

```bash
cargo +nightly build --bin kubi -r --features nightly
```

build for android

please note that android support is purely experimental!
gamepad, keyboard and mouse input is currently borked, and touch controls are not available.
srgb and blending are broken too, which leads to many rendering issues

prerequisites: Android SDK, NDK, platform-tools, latest JDK (all should be in $PATH)

Setup:

```bash
cargo install cargo-apk
cargo target add aarch64-linux-android
```

Build:
`--no-default-features` is required for keyboard input!

```bash
cargo apk build -p kubi --no-default-features
# or, with nighly optimizations:
cargo +nightly apk build -p kubi --no-default-features --features nightly
```

Run:

```bash
cargo apk run -p kubi --features nightly
# or, with nighly optimizations:
cargo +nightly apk run -p kubi --no-default-features --features nightly
```

<h2>mutiplayer</h2>

to join a multiplayer server, just pass the ip address as an argument

```sh
cargo run -p kubi -- 127.0.0.1:1234
```

<h2>server configuration</h2>

```
[server]
address = "0.0.0.0:12345"     # ip address to bind to
max_clients = 32              # max amount of connected clients
timeout_ms = 10000            # client timeout in ms

[world]
seed = 0xfeb_face_dead_cafe   # worldgen seed to use

[query]
name = "Kubi Server"          # server name
```

<h6 align="right"><i>~ uwu</i></h6>
