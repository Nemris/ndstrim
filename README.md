# ndstrim

`ndstrim` is a program to trim the excess padding space found in Nintendo DS(i) ROMs, created as a
project to familiarize myself with Rust.

It takes into account whether a game bundles a RSA certificate in order for Download Play
functionality to work, and preserves it while trimming the excess space from the ROM file.

-----

## Usage

### Standard

`ndstrim` can trim any amount of files in a single run. To execute it, use:

```bash
ndstrim foo.nds bar.nds baz.nds
```

This will produce three files called `foo.trim.nds`, `bar.trim.nds` and `baz.trim.nds` in
the same directory as the original ROM files.

You can optionally provide a custom extension to use in place of `trim.nds` by passing the `-e`
flag. Ensure that the extension you provide contains no leading dot.

### In-place

If you don't care about preserving the original ROMs, you can run:

```bash
ndstrim -i foo.nds bar.nds baz.nds
```

This will trim the files in-place, and **is irreversible**.

### Simulated

If you want to check what `ndstrim` would do, you can use:

```bash
ndstrim -s foo.nds bar.nds baz.nds
```

This option can be combined with `-i`.

### Help

Launching `ndstrim` without arguments will display a brief usage message, but you can get a more
helpful one by passing the `-h` flag.

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

## Detection as malware

On Windows, it might happen that Defender quarantines the prebuilt .exe as a malware. Likewise,
some VirusTotal engines may flag the binary, even if sandbox analysis shows that the file is clean.

This is a false positive, most likely triggered by the use of UPX to minify the binary's size.
If you don't trust the binary, however, you can always review the source and build `ndstrim` on
your system by following the instructions above.

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
