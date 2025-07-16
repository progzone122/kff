# Kindle Fucking Forge (KFF)
A simple tool for setting up the environment (SDK/Toolchain) and generating projects from templates for Kindle development.

## Installation
### Arch Linux
Just use the [package from AUR](https://aur.archlinux.org/packages/kff)
```shell
paru -S kff
```
### Other distro
Download the binary file from the [Releases tab](https://github.com/progzone122/kff/releases/latest)

## Usage
### Setting up the development environment
1. Install Toolchain
```shell
kff install toolchain <TARGET>
```
2. Install SDK
```shell
kff install sdk <TARGET>
```
3. Add the path to the `meson-crosscompile.txt` file to the `KSDK` environment variable
```shell
echo "export KSDK=${HOME}/x-tools/arm-kindlehf-linux-gnueabihf/meson-crosscompile.txt" >> .zshrc
```
4. Verify successful installation and configuration of the environment
```shell
kff doctor
```
```text
--- KFF Doctor ---
KSDK env: [OK]
KSDK: /home/diablo/x-tools/arm-kindlehf-linux-gnueabihf/meson-crosscompile.txt

meson-crosscompile.txt file (SDK): 
[binaries]
c = '/home/diablo/x-tools/arm-kindlehf-linux-gnueabihf/bin/arm-kindlehf-linux-gnueabihf-gcc'
cpp = '/home/diablo/x-tools/arm-kindlehf-linux-gnueabihf/bin/arm-kindlehf-linux-gnueabihf-g++'
ar = '/home/diablo/x-tools/arm-kindlehf-linux-gnueabihf/bin/arm-kindlehf-linux-gnueabihf-ar'
strip = '/home/diablo/x-tools/arm-kindlehf-linux-gnueabihf/bin/arm-kindlehf-linux-gnueabihf-strip'
...
```
### Generating a project from a template
Use templates from the [repository](./templates.json) or local templates from the `~/.local/share/kff/templates` directory (default).<br />
**Example:**
```shell
kff generate gtk2
```
