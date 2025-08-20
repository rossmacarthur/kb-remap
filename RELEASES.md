# 📝 Release notes

## 0.4.1

*Unreleased*

- [Auto upload a release binary for macOS on tag pushes][20ae6f8f]
- [Add `--completions` option to generate shell completions][fbb8a22a]

[20ae6f8f]: https://github.com/rossmacarthur/kb-remap/commit/20ae6f8f6f738dbfb50bdd183753374c9406a29e
[fbb8a22a]: https://github.com/rossmacarthur/kb-remap/commit/fbb8a22afb27be02ac43f2599ce49c5167c4c6bd

## 0.4.0

*September 11th, 2024*

- [Support setting Usage Page and Usage ID when using hex][d79d9d93].
  Previously, the USB Usage Page was assumed to be `0x07`, now if you specify a
  key using both the USB Usage Page and USB Usage ID it works.

[d79d9d93]: https://github.com/rossmacarthur/kb-remap/commit/d79d9d93bc2ae173e4eb1d415e38040180124a41

## 0.3.4

*February 11th, 2024*

- [Handle empty lines in hidutil output][d637bcce]

[d637bcce]: https://github.com/rossmacarthur/kb-remap/commit/d637bcce3fec15868ec7859816777bd4a4284bfd

## 0.3.3

*August 18th, 2023*

- [Fix bug when matching keyboards][3cb0bea7]. It turns out hidutil does not
  error if you pass invalid JSON for the `--matching` key and instead it just
  matches all keyboards.

[3cb0bea7]: https://github.com/rossmacarthur/kb-remap/commit/3cb0bea7206b1e0d803b344c429cf08588c75377

## 0.3.2

*August 15th, 2023*

- [Handle newlines in `hidutil list` column values][5f048606]

[5f048606]: https://github.com/rossmacarthur/kb-remap/commit/5f048606c0a08c165729977714e9156074a55009

## 0.3.1

*July 15th, 2023*

- [Add -R short alias for --reset][b1daa747]

[b1daa747]: https://github.com/rossmacarthur/kb-remap/commit/b1daa747f692bcb98c567096c60c89a0e3990ff3

## 0.3.0

*June 19th, 2023*

- [Add --RESET alias for --reset][1cbe786d]
- [Dump numbers with hex, refactor some type names][ecb62b41]
- [Add aliases for Fn and F1 to F24][06f2679f]

[1cbe786d]: https://github.com/rossmacarthur/kb-remap/commit/1cbe786d5a25784535b51aa73b8ad1aa99cb26ca
[ecb62b41]: https://github.com/rossmacarthur/kb-remap/commit/ecb62b417f315cfa1a27ca79a0a4aaf532bf54f6
[06f2679f]: https://github.com/rossmacarthur/kb-remap/commit/06f2679f26059ad7187c7e697e78dbff211d4024

## 0.2.1

*April 9th, 2022*

No user facing changes

## 0.2.0

*April 6th, 2022*

- [Add key names to README][0ad963b1]
- [Improve support for control, shift, command and option][69cea725]

[0ad963b1]: https://github.com/rossmacarthur/kb-remap/commit/0ad963b10233125933c194956a1f02955ee21b41
[69cea725]: https://github.com/rossmacarthur/kb-remap/commit/69cea725f3720f109f4d4be316f3591c1776694a

## 0.1.0

*March 13th, 2022*

- [Add --dump option][6ec37b1c]
- [Allow filtering by vendor ID and/or product ID][891b41a3]

[6ec37b1c]: https://github.com/rossmacarthur/kb-remap/commit/6ec37b1cd0a498a514949f803f5d100d88cf7d7b
[891b41a3]: https://github.com/rossmacarthur/kb-remap/commit/891b41a3abed08e60a75756aee89f6360e69c2b2

## 0.0.5

*April 22nd, 2021*

- [Tabulate vendor and product output][ecea027e]

[ecea027e]: https://github.com/rossmacarthur/kb-remap/commit/ecea027eaa2cd5b5fde358b07c4ad2c2baba9a8a

## 0.0.4

*March 22nd, 2021*

- [Make key specification simpler][1d53d2c7]

[1d53d2c7]: https://github.com/rossmacarthur/kb-remap/commit/1d53d2c712c0d51129cf7da2214083df990f1a7e

## 0.0.3

*March 19th, 2021*

- [Fix plist parse error][b3ca0142]

[b3ca0142]: https://github.com/rossmacarthur/kb-remap/commit/b3ca01426728564de3cde89b1f3f8fc118f5c510

## 0.0.2

*March 18th, 2021*

- [List USB devices when none given][7e572b72]
- [Parse keyboard information from plist data][936e41e2]
- [Allow specifying key names in mapping][eba9e0ac]

[7e572b72]: https://github.com/rossmacarthur/kb-remap/commit/7e572b728d802094c6e36a88ee744e3806d85f14
[936e41e2]: https://github.com/rossmacarthur/kb-remap/commit/936e41e2623f2fb14c7562ccb5bbf7c24d98f1d4
[eba9e0ac]: https://github.com/rossmacarthur/kb-remap/commit/eba9e0ac5501e629783fb5a302c414176ab1b7a9

## 0.0.1

*March 18th, 2021*

First version
