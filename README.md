# kb-remap

[![Crates.io Version](https://badgers.space/crates/version/kb-remap)](https://crates.io/crates/kb-remap)
[![Build Status](https://badgers.space/github/checks/rossmacarthur/kb-remap/trunk?label=build)](https://github.com/rossmacarthur/kb-remap/actions/workflows/build.yaml)

Tool to assist remapping macOS keyboard keys.

## 🚀 Getting started

Install the tool using Cargo.

```sh
cargo install kb-remap
```

## 🤸 Usage

To globally remap keys you can just run the tool with one or more `--map` or
`--swap` options. For example the following remaps capslock to backspace and
swaps § (section) and ` (backtick) on all keyboards.
```sh
kb-remap --map capslock:delete --swap '0x64:`'
```

Reset the mapping using
```sh
kb-remap --reset
```

### Filtering keyboards

You can optionally filter which keyboards are remapped by using the `--name`
option. Use the `--list` option to list the available HID devices and their
names.

```sh
kb-remap --list
```
```text
Vendor ID  Product ID  Name
---------  ----------  ----------------------------------
0x0        0x0         BTM
0x0        0x0         Headset
0x4c       0x269       Magic Mouse
0x5ac      0x342       Apple Internal Keyboard / Trackpad
0x5ac      0x342       Keyboard Backlight
0xc45      0x7692      USB Keyboard
0x1050     0x407       YubiKey OTP+FIDO+CCID
```

Using the name listed above as `--name` you can remap any key you want using the
`--map` or `--swap` options. For example the following remaps capslock to
backspace and swaps § (section) and ` (backtick) on a the internal macOS
keyboard.
```sh
kb-remap --name "Apple Internal Keyboard / Trackpad" --map capslock:delete --swap '0x64:`'
```

You can reset the mapping using:
```sh
kb-remap --name "Apple Internal Keyboard / Trackpad" --reset
```

If you want you can inspect the raw `hidutil` command that would be run for a
particular command using the `--dump` option.
```
kb-remap --name "Apple Internal Keyboard / Trackpad" --map capslock:delete --dump
```

Would output the following:
```text
hidutil property \
    --matching '{"VendorID":1452,"ProductID":834}' \
    --set '{"UserKeyMapping":[{"HIDKeyboardModifierMappingSrc":30064771129,"HIDKeyboardModifierMappingDst":30064771114}]}'
```

## Specifying keys

The `--map` and `--swap` options both expect the source and destination keys to
be specified and separated by a ":" (colon).

There are three ways to specify keys:

### Name

Some keys you can specify using their name. For example the Return (Enter) key
"⏎" can be specified as "return". These are added on a convenience basis.

| Key                | Code                    | USB Usage Page | USB Usage ID |
| ------------------ | ----------------------- | -------------: | -----------: |
| Return (Enter)     | `return`                |           0x07 |         0x28 |
| Escape             | `escape`                |           0x07 |         0x29 |
| Delete (Backspace) | `delete`                |           0x07 |         0x2A |
| Caps Lock          | `capslock`              |           0x07 |         0x39 |
| Left Control       | `lcontrol`              |           0x07 |         0xE0 |
| Left Shift         | `lshift`                |           0x07 |         0xE1 |
| Left Option        | `loption`               |           0x07 |         0xE2 |
| Left Command       | `lcommand`              |           0x07 |         0xE3 |
| Right Control      | `rcontrol`              |           0x07 |         0xE4 |
| Right Shift        | `rshift`                |           0x07 |         0xE5 |
| Right Option       | `roption`               |           0x07 |         0xE6 |
| Right Command      | `rcommand`              |           0x07 |         0xE7 |
| Function           | `fn`                    |           0xFF |         0x03 |
| F1, F2, ..., F12   | `f1`, `f2`, ..., `f12`  |           0x07 | 0x3A -> 0x45 |
| F13, F14, ..., F24 | `f13`, `f4`, ..., `f24` |           0x07 | 0x68 -> 0x73 |

Additionally, the following special names are available and map multiple keys if
they are used.

| Keys                 | Code      |
| -------------------- | --------- |
| Left & Right Control | `control` |
| Left & Right Shift   | `shift`   |
| Left & Right Option  | `option`  |
| Left & Right Command | `command` |

### Character

Most typeable keys can be specified using their character. For example: the A
key can be specified using "A" or "a". The USB Usage ID used will the one that
the key corresponds to on a US keyboard.

### Number

Any key can be specified by using the USB Usage ID in hex. For example: Z has a
Usage ID of "0x1d", in this case the USB Usage Page is assumed to be `0x07`
(keyboard). You can also specify the Usage Page and the Usage ID together like
this: "0x7_0000_001D".

## 🤔 Why? How?

Powerful applications to remap macOS keys like [Karabiner-Elements] are often
overkill for simple remappings. Additionally, they can sometimes take a while to
support the latest macOS version. I wanted a simple reliable solution.

Instead of a constantly running application `kb-remap` simply subprocesses to
the built-in macOS  `hidutil` command to fetch keyboard information and to remap
keys. This remapping does not persist if keyboards are unplugged or **if your
Mac is restarted**. `kb-remap` does not solve this problem for you yet. One
option is to install a launchd service to automatically run `kb-remap` on boot.

[Karabiner-Elements]: https://github.com/pqrs-org/Karabiner-Elements

## License

This project is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
