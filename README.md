See LICENSE.md

# Tips and Tricks
Just things that are good to have.

## Dependencies
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel wayland-devel libxkbcommon-devel

## Wayland vs X11
For wayland support, add the "wayland" feature. Then, to specify explicitly which one to use, use one of the following:
export WINIT_UNIX_BACKEND=x11
export WINIT_UNIX_BACKEND=wayland