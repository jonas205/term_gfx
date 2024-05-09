# Terminal Graphics

A small graphics engine that runs in the terminal written in Rust (mostly, I did a small unsafe thing in C). This just a fun little project written to test some things, do not use this for anything important.

It uses [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code) for the graphics, and [termios.h](https://www.man7.org/linux/man-pages/man0/termios.h.0p.html) to enable better user input (disabling the user having to press enter for input to be sent to the application).
Because of that this library is probably only compatible with some GNU/Linux systems :/

## Cool Features I implemented myself
- Draw Pixel in any color (One Pixel is the size of one char in the terminal)
- Line Rasterisation (Drawing a non straight line is harder than you think)
- Image Drawing
- Image Resizing (I wrote my own piece of code for image resizing with linear sampling (wich you should not use in production), but it looks relatively good)
- Event System (Char / Window Events)

## Examples

### Sandbox
A small Sandbox app I use for testing that shows of some features.
You can use `wasd` to resize the right image.

```bash
cargo run --example sandbox
```

### Image
A picture of my cat that resizes with the terminal.
```bash
cargo run --example image
```

### [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
This is only a very small demo with a single glider and no user input, I have implemented Conway's Game of Life so often that I didn't want to add more, maybe I'll make it usable in the future (probably not).
```bash
cargo run --example game_of_life
```
