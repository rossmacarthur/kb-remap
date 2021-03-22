# kb-remap

Tool to assist remapping macOS keyboard keys.

## Examples

```sh
# The following remaps capslock to backspace and swaps § and ` on a the internal macOS keyboard.
$ kb-remap --name "Apple Internal Keyboard / Trackpad" --map capslock:delete --swap '0x64:`'
Apple Internal Keyboard / Trackpad
  • Raw(100) -> Char('`')
  • Char('`') -> Raw(100)
  • CapsLock -> Delete

# This remaps capslock to backspace on a keyboard called "USB Keyboard"
$ kb-remap --name "USB Keyboard" --map 0x39:0x2A
USB Keyboard
  • Raw(57) -> Raw(42)
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
