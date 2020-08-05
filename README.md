# Sway Tray Status

This is a helper program to show basic information in swaybar. See [Sway](https://github.com/swaywm/sway) a Wayland Compositor similar to i3 for more information.

The Information shown includes:

- Battery Name
- Battery level in percent
- Date and time

## Configuration

```
#
# Status Bar:
#
# Read `man 5 sway-bar` for more information about this section.
bar {
    position bottom 
    status_command /<some_dir>/sway_tray_status BAT0 BAT1
    colors {
        statusline #ffffff
        background #323232
        inactive_workspace #32323200 #32323200 #5c5c5c
    }
}
```

