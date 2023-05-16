<h1 align="center">Kubi</h1>
<p align="center">
  Minecraft clone written in Rust
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
  </ul>
</p>

<h2>building</h2>

build/run

```sh
cargo build -p kubi
cargo run -p kubi
```

build with nightly features

```sh
RUSTFLAGS="-C target-cpu=native" cargo +nightly build -p kubi -r --features nightly --
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

<p>
  <ul>
    <li>multithreaded procedural world generation</li>
  </ul>
</p>

<h6 align="right"><i>~ uwu</i></h6>
