# ttlego-jitpatcher

Patches the code of classic (LEGO Batman 1 and older) Traveler's Tales LEGO Games without modifying the EXE!

## Compatability

So far tested with:

- LEGO Star Wars: The Complete Saga (GOG version)

Please let me know if you find other games that work!

## Installation

1. Download the `dinput8.dll` from the [mod_loader](https://github.com/lcdr/mod_loader/releases) project. (don't worry that it says LEGO Universe, it works for TT games too)
2. Put the mod_loader's `dinput8.dll` into the same folder as your game EXE.
3. In the same folder as your game EXE, create a `mods/jitpatcher` folder.
4. Download the latest [`mod.dll`](https://github.com/meanwhile-labs/ttlego-jitpatcher/releases) file from this project and place it in `mods/jitpatcher`.
5. Again in your game EXE folder, create a `patches` folder.
6. Add `.toml` files to the `patches` folder. You can find examples in the [examples](./examples/) folder in this project.

## Creating patches

All patch files should end in `.toml` and be in the following format:

```toml
name = "[Human readable name]"
enabled = true

[[patches]]
offset = "000000" # hex offset of address to modify
original = "00 00 00 00" # the original hex value that lives at that offset
patch =    "00 00 00 00" # the hex value you want to replace it with

[[patches]] # you can add multiple patches in a single file
offset = "0000ff"
original = "00 00 00 00 00 00 00"
patch =    "00 00 00 00 00 00 00"
```

Hex values aren't case sensitive; uppercase or lowercase letters work.

The spaces also aren't strictly required in `original` or `patch`, but highly recommended.

### `offset`

This is the hex address that will be modified. This is relative to the start of the game code, so...

- If you see something like `LEGOStarWarsSaga.exe+CAD10` in a tool like Cheat Engine, then `CAD10` will be your offset.
- If you see something like `004cad10` in a tool like Ghidra, then subtract hex `00400000` (or 4,194,304 in decimal / normal numbers) to get your address, because the game code starts there (at least in my testing, and it seems to be pretty standard - let me know if there are any games that don't work like that!)
  - Note that there might be addresses above `00500000`, so it's not just a matter of removing the `4`. If you see something like `005cccc`, that would be an offset of `1cccc`.
- If you have an address from a hex editor looking at the executable, that actually is the offset; no further transformation required. So `0x000CA6CE` would be `ca6ce`.

### `original`

These are the bytes, written in hexadecimal, that are expected to already exist in the game code. This lets the patcher make sure it has the right place before modifying the game code - otherwise unpredictable but probably crash-related things could happen.

This can be any length.

### `patch`

The new bytes, again written in hexadecimal, to replace the bytes in `original`. This must be the same length as `original`.

Remember that game code is stored in little-endian format. If you don't know what that means, be careful when modifying numbers - probably best to test your changes in something like Cheat Engine before making a patch.

## How it works

TODO

## Development

DLL must be built with `cargo build --target i686-pc-windows-msvc` to actually work with the game!

## Roadmap/ideas

- More documentation, especially of the internals
- More testing on various games
- More example patch files for more games
- Maybe build our own dll injection instead of relying on `mod_loader`, since
  1. it makes the installation instructions confusing when we're suddenly talking about LEGO Universe
  2. it hasn't been maintained for 4 years
  3. it calls `LoadLibrary` inside `DllMain` which is apparently a Win32 no-no and might cause problems (freezing up on startup, probably) down the road
- Auto-detect different versions of the same game (ex. Steam vs. GOG) and adjust patches to fit
- Support for code caves, to support more ambitious modifications
- Compatibility/interoperability with [Reloaded 2](https://github.com/Reloaded-Project/Reloaded-II), since the TT LEGO modding community is starting to lean on that tool for complex mods.
