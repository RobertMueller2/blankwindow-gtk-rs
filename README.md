## What is this?

Creates a blank window using gtk. On my 43" 4k screen, tiled windows can create visual challenges. Rather than having the window float, I pick up some screen real estate using a blank window. In the past, I've started terminals for the same purpose and that's OK too, of course.

![Screenshot](/img/20230517_213216.png)
  
I'm using the following keybinding in my sway config:

```sway
bindsym $mod+Shift+Return exec exec blankwindow-gtk-rs
```

### Why no `smart_gaps` or `gaps`?

I feel this depends on the screen used. With an ultra wide screen, I tend to prefer hotkeys for setting outer gaps on the fly. That said, I'm still finding the window useful for shadowing floating game windows.

![Screenshot](/img/20250528_225344.png)

## Styling

See default style in repository. Can be overwritten by `$HOME/.config/blankwindow.css`.

## Compile and install

From the repo directory, run
```sh
cargo install --path . 
```
to install to `$HOME/.cargo`.  
  
Or just use `cargo build` or `cargo build --release` to produce binaries beneath `target/` and copy manually if needed.

## Run

`blankwindow-gtk-rs [--app-id <app_id>]`

Note that `app_id` needs to be a valid gtk application id, e.g. `com.github.RobertMueller2.blankwindow` or something.


## Transparency while testing

Works with wayland, but apparently not Xwayland. E.g. when developing in code and code is running in Xwayland, the program started by `cargo run` also defaults to Xwayland.
