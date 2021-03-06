# kb-remap

Tool to assist remapping macOS keyboard keys.

## ๐ Getting started

Install the tool using Cargo.

```sh
cargo install kb-remap
```

## ๐คธ Usage

Running the tool without any options will list the available USB devices.
```sh
$ kb-remap
```
```text
| Vendor Name           | Product Name                       |
| --------------------- | ---------------------------------- |
| Apple Inc.            | Apple T2 Controller                |
| Apple                 | Headset                            |
| Apple Inc.            | Touch Bar Display                  |
| Apple Inc.            | FaceTime HD Camera (Built-in)      |
| Apple Inc.            | Touch Bar Backlight                |
| Apple Inc.            | Ambient Light Sensor               |
| Apple Inc.            | Apple Internal Keyboard / Trackpad |
| Microchip Tech        | USB5734                            |
| VIA Technologies Inc. | USB 2.0 BILLBOARD                  |
| Microchip Tech        | Hub Controller                     |
| SONiX                 | USB Keyboard                       |
| Yubico                | YubiKey OTP+FIDO+CCID              |
```

Usually it's pretty simple to pick out which devices are keyboards. Using the
name listed above as `--name` you can remap any key you want using the `--map`
or `--swap` options. For example the following remaps capslock to backspace and
swaps ยง (section) and ` (backtick) on a the internal macOS keyboard.
```sh
$ kb-remap \
  --name "Apple Internal Keyboard / Trackpad" \
  --map capslock:delete --swap '0x64:`'
```

You can reset the mapping using:
```sh
$ kb-remap --name "Apple Internal Keyboard / Trackpad" --reset
```

The `--map` and `--swap` options both expect the source and destination keys to
be specified and separated by a ":" (colon).

There are three ways to specify keys:

- **Name:** some keys you can specify using their name. For example the Return
  key "โ" can be specified as "return". These are added on a convenience basis.
- **Character:** most typeable keys can be specified using their character. For
  example: the A key can be specified using "A" or "a".
- **Number:** any key can be specified by using the USB usage ID in decimal or
  hex. For example: Z has a usage ID of "29", which can also be specified as
  "0x1d".

## ๐ค Why? How?

Powerful applications to remap macOS keys like [Karabiner-Elements] are often
overkill for simple remappings. Additionally, they can sometimes take a while to
support the latest macOS version. I wanted a simple reliable solution.

Instead of a constantly running application `kb-remap` simply subprocesses to
built-in macOS `ioreg` and `hidutil` commands to fetch keyboard information and
to remap keys. This remapping does not persist if keyboards are unplugged or
**if your Mac goes to sleep**. `kb-remap` does not solve this problem for you
yet. One option is to install a launchd service to automatically run `kb-remap`.

[Karabiner-Elements]: https://github.com/pqrs-org/Karabiner-Elements

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
