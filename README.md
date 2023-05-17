## What is this?

Creates a blank window using gtk. On my 43" screen, tiled windows can create visual challenges. Rather than having the window float, I pick up some screen real estate using a blank window. In the past, I've started terminals for the same purpose and that's OK too, of course.

![Screenshot](/img/20230517_213216.png)
  
I'm using the following keybinding in my sway config:

```sway
bindsym $mod+Shift+Return exec exec blankwindow-gtk-rs
```

## Styling

See default style in repository. Can be overwritten by `$HOME/.config/blankwindow.css`.

## Compile and install

From the repo directory, run
```sh
cargo install --path . 
```
to install to `$HOME/.cargo`.  
  
Or just use `cargo build` or `cargo build --release` to produce binaries beneath `target/` and copy manually if needed.

## Transparency while testing

Works with wayland, but apparently not Xwayland. E.g. when developing in code and code is running in Xwayland, the program started by `cargo run` also defaults to Xwayland.
