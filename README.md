<div align="center"><table><tr><th><div align="center">

## ***wgpu branch***

<b>Highly experimental very early work-in-progress wgpu version of Kubi!</b><br>
(will also include the new gui system)

<hr>

<h3><i>Status: doesn't even compile</i><br></h3>
If you want to play kubi, build the glium-based <a href="https://github.com/griffi-gh/kubi"><code>master</code></a> branch instead (<a href="https://github.com/griffi-gh/kubi/releases/tag/nightly">binary nightly releases</a>).

<hr>

### *Android is not supported*
android builds need some significant changes to work with wgpu\
but if you still want to try it latest git version of cargo-apk may be required:
```bash
cargo install --git https://github.com/rust-mobile/cargo-apk cargo-apk
```

</div></th></tr></table></div>

<h1 align="center">Kubi</h1>
<p align="center">
  Voxel engine written in Rust
</p>
<p align="center">
  
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
    <li>multiplayer support <sup><code>[1]</code></sup></li>
    <li>block placement system</li>
    <li>basic gui<sup><code>[5]</code></sup></li>
    <li>cross platform: windows, linux, osx <sup><code>[2]</code></sup>, android <sup><code>[3]</code></sup></li>
    <li>universal input system: supports keyboard, mouse, gamepad and touch input <sup><code>[4]</code></sup></li>
  </ul>
  <h6>
    <code>[1]</code> - multiplayer is work-in-progress<br>
    <code>[2]</code> - not tested on macos<br>
    <code>[3]</code> - android support is experimental<br>
    <code>[4]</code> - mouse/gamepad input is not supported on android<br>
  <code>[5]</code> - currently only used on the loading screen 
  </h6>
</p>

<h2>download</h2>
<a href="https://github.com/griffi-gh/kubi/releases/tag/nightly">Latest nightly release</a>

<h2>build for windows/linux</h2>

**build/run**

```bash
cargo build --bin kubi
cargo run --bin kubi
```

**build in release mode, with nightly optimizations**

```bash
cargo +nightly build --bin kubi --features nightly --release
```

<h2>build for android</h2>

please note that android support is highly experimental!\
gamepad, mouse input is currently borked, and proper touch controls are not available.\
srgb and blending are broken too, which leads to many rendering issues

prerequisites: Android SDK, command line tools, NDK, platform-tools, latest JDK\
(make sure that your $PATH variable is configured properly)

**Setup:**

```bash
cargo install cargo-apk
cargo target add aarch64-linux-android
```

**Build:**

`--no-default-features` is required for keyboard input!\
(`prefer-raw-events` feature *must* be disabled on android)

Mouse input is not implemented, touch only!

```bash
cargo apk build -p kubi --no-default-features
```

**Run:**

```bash
cargo apk run -p kubi --no-default-features
```

<h2>touch controls</h2>

<img src=".readme/touch_controls.png" alt="touch control scheme" width="300">

- Left side: **Movement**
- Rigth side: **Camera controls**
- Bottom right corner:
  - **B** (e.g. place blocks)
  - **A** (e.g. break, attack)

<h2>mutiplayer</h2>

to join a multiplayer server, just pass the ip address as an argument

```sh
cargo run -p kubi -- 127.0.0.1:1234
```

<h2>server configuration</h2>

```toml
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
