# I3 Move

Move the focus to next or previous workspace without a insane looping.

## Compiling

```sh
cargo build
```

## Install

```sh
cargo install --git https://github.com/nsfilho/i3_move_next.git
```

## Config Example

Add or change in your `$HOME/.config/i3/config` file:

```txt
bindsym $mod+Left exec --no-startup-id "$HOME/.cargo/bin/i3_move_next -1"
bindsym $mod+Right exec --no-startup-id "$HOME/.cargo/bin/i3_move_next"
bindsym $mod+n exec --no-startup-id "$HOME/.cargo/bin/new_i3_workspace"
```

Another nice utility could be: [new_i3_workspace](https://github.com/nsfilho/i3_new_workspace.git)

