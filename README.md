# ndstrim

`ndstrim` is a trimmer for Nintendo DS(i) ROMs, created as a project to familiarize myself with
Rust.

It takes into account whether a game bundles a RSA certificate in order for Download Play
functionality to work, and preserves it while trimming the excess space from the ROM file.

-----

## Usage

`ndstrim` can trim any amount of files in a single run. To execute it, use:

```bash
ndstrim foo.nds bar.nds baz.nds
```

Alternatively, simply launch it without arguments for a small usage message.

**For the time being, the files are trimmed in-place. Prior backups are recommended.**

-----

## Building

Building `ndstrim` is a straightforward process that can be done using `cargo`.

For a debug build, run:

```bash
cargo b
```

For a release build, which produces an optimized and stripped binary with full LTO, run:

```bash
cargo b --release
```

-----

## Credits

This program is based on an adaptation of the trimming algorithm included in [GodMode9][1].

Additionally, to validate NDS ROMs, it adapts the CRC-16 algorithm included in [TWiLightMenu][2].

-----

## License

This program is licensed under the terms of the [MIT][3] license.

See [LICENSE.txt][4] for further info.


[1]:https://github.com/d0k3/GodMode9
[2]:https://github.com/DS-Homebrew/TWiLightMenu
[3]:https://choosealicense.com/licenses/mit/
[4]:./LICENSE.txt
