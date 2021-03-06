# Simple Game Engine

[![](https://badgen.net/crates/v/simple-game-engine)](https://crates.io/crates/simple-game-engine)
[![](https://docs.rs/simple-game-engine/badge.svg)](https://docs.rs/simple-game-engine)

This project aims to create a minimal, yet usable, game engine. It is heavily inspired by [the Pixel Game Engine][pge],
with the goal of creating something that abstracts away the complexities of creating graphical, interactive apps and
games. Right now, it's a thin wrapper around [SDL2][sdl2] (using the [sdl2
crate][sdl2-crate]) for visuals.

[pge]: <https://github.com/OneLoneCoder/olcPixelGameEngine>
[sdl2]: <https://www.libsdl.org>
[sdl2-crate]: <https://crates.io/crates/sdl2>

This is highly experimental so far, so if you have any ideas or would like to contribute, please [sub mit an
issue](https;//github.com/mcb2003/audio-game-engine-rs/issues).

## Features

* **Very simple to use:** Just implement the [`Application trait][sge::Application] on a type of your choice, then pass an instance of this type to [Engine::new()][Engine::new].
* **Powerful:** Anything you can do with sdl2 from Rust, you can do with this library, and we provide thin abstractions over some of the more convoluted sdl2 interfaces.
* **Built-in text rendering:** No need to find a TTF font and distribute it with your application, just call the [Canvas::draw_text()][Canvas::draw_text] method. ([see below](#caveats-with-text-rendering))

[sge::Application]: <https://docs.rs/simple-game-engine/0.6.1/simple_game_engine/trait.Application.html>
[Engine::new]: <https://docs.rs/simple-game-engine/latest/simple_game_engine/struct.Engine.html#method.new>
[Canvas::draw_text]: <https://docs.rs/simple_game_engine/latest/canvas/struct.Canvas.html#method.draw_text>

## Caveats With Text Rendering

This crate uses the [GNU Unifont][unifont] for built-in text rendering. As such, if you wish to use this feature, you
must distribute your project under the [GPL][gpl]. As this is not desirable for many projects, this feature is only
enabled if this crate is built with the "unifont" cargo feature.

```toml
[dependencies.simple-game-engine]
version = "0.8.2"
features = ["unifont"]
```

If you'd like to render text without using this font, consider checking out the [SDL2 TTF module][sdl2-ttf]

[unifont]: <http://unifoundry.com/unifont/index.html>
[gpl]: <https://www.gnu.org/licenses/old-licenses/gpl-2.0-standalone.html>
[sdl2-ttf]: <https://docs.rs/sdl2/latest/sdl2/ttf/index.html>

## Install

Add the following to your `Cargo.toml`:

```toml
[dependencies]
simple-game-engine = "0.8.2"
```

This crate depends on [sdl2][sdl2], which can be built and linked in several ways:

* By default, the system version of sdl is used if it exists.
* With the "bundled_sdl" feature, the latest version of sdl2 will be downloaded, compiled and dynamically linked to your program. This requires you install [cmake][cmake].
* With the "static_sdl" feature, sdl2 will be linked statically.

Please see the [sdl2 crate README][sdl2-readme] for installation instructions for each platform.

[cmake]: <https://cmake.org>
[sdl2-readme]: <https://github.com/Rust-SDL2/rust-sdl2#readme>

## Example

The simplest SGE program looks like this:

```rust
use simple_game_engine::{self as sge, prelude::*};
use std::error::Error;

struct App {}

impl sge::Application for App {
    fn on_create(
        &mut self,
        canvas: &mut WindowCanvas,
        input: &InputState,
    ) -> sge::ApplicationResult {
        // Do one-time initialisation here
        Ok(true) // `true` indicates to continue running the application
    }

    fn on_update(
        &mut self,
        canvas: &mut WindowCanvas,
        input: &InputState,
        elapsed_time: f64,
    ) -> sge::ApplicationResult {
        // Handle user input, update the canvas, and perform any other tasks to be ran on each frame
        Ok(true) // `true` indicates to continue running the application
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App {};
    let mut engine = sge::Engine::new(
        &mut app,   // Application instance
        "Test App", // Window title
        640,        // Window width
        480,        // Window height
    )?;
    engine.start(true) // `true` starts the app with vsync enabled
}
```

`on_create` and `on_update` are optional, but their default implementation does nothing, so
you'll probably want to define some logic for at least `on_update`, which is called for every
frame.
