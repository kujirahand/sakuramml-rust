# Sakura Text Music (Rust Edition) / PicoSakura

Sakura is a compiler that converts MML (Music Macro Language) into MIDI files.
It converts musical text such as Japanese solfège (`ドレミ`) or `cde` into MIDI files.
Written in Rust, it runs on multiple platforms: macOS, Windows, Linux, and WebAssembly.

Sakura is a long-standing music-production tool originally developed before 2000.
MML is a notation for representing music as text, such as `cde`.
It makes it easy to create music. Written in Rust, it runs on macOS, Windows, Linux, and WebAssembly.
It received an award in Japan's Online Software Grand Prize 2001 and was featured in Japanese high-school information-technology textbooks.

## Samples

- [mmlbbs6](https://sakuramml.com/mmlbbs6/index.php?action=pico) --- Many songs have been posted to the Sakura MML Bulletin Board 6.

## Tutorial

An online tutorial is available. You can learn the commands while listening to the music in your browser.

- [Tutorial](https://sakuramml.com/index.php?%E7%B0%A1%E5%8D%98%E3%81%AA%E4%BD%BF%E3%81%84%E6%96%B9)

## PicoSakura: Installation

The web version, [PicoSakura](https://sakuramml.com/go.php?15), lets you easily play MIDI files in your browser.

For local use, the command-line edition is also available. Download binaries for each operating system below.

- [Command-line binaries (Windows/macOS)](https://github.com/kujirahand/sakuramml-rust/releases/)

## Building the Latest Version (Rust)

First, install [Rust](https://www.rust-lang.org/tools/install).

```
$ git clone https://github.com/kujirahand/sakuramml-rust.git
$ cd sakuramml-rust
$ cargo build --release
```

This creates `target/release/sakuramml`.

## Usage

### Command-line Edition

Write the musical score as text, for example in a file named `test.mml`.
To convert `test.mml` to `test.mid`, run the following command from the command line:

```sh
$ sakuramml test.mml
```

### Basic Usage

```mml
音階4 ドレミファソラシ↑ド↓シラソファミレド
o4 cdefgab>c<bagfedc
```

```mml
トラック1 チャンネル1 音符1 ドミソー
TR=1 CH=1 l1 ceg^
```

## Chords

```mml
音符1「ドミソ」
l4 'ceg' 'dfa'8 'egb'8 'ceg'
```

Enclose chords in single quotes. After the closing quote, you can specify the note length, gate time, and velocity, as in `'ceg'4,90,120`.

## Moving the Time Pointer

Use `TIME(measure:beat:step)` to move to any position in the score.

```mml
// Move to the beginning
TIME(1:1:0) cdef
TIME(1:1:0) efga

// Move to the second measure
TIME(2:1:0) cdef
```

Use `SUB{...}` to return the time pointer to just before the `SUB`. This lets you play chords easily.

```mml
SUB{ cdef  c }
SUB{ efga  e }
     rrrr  g
```

## Rhythm Macros

In a rhythm macro, each character is treated as one instruction, regardless of case.
Define a rhythm macro in the form `$character{definition}`.

```mml
// Define rhythm macros (the following are defined by default, but can be redefined)
$b{n36,}
$h{n42,}
$o{n46,}
// Define a new rhythm macro
$S{n37,}
CH(10)
// Rhythm sample
Rhythm{
　[4　l8
　　　brSr bbsr r-1
　　　hoho hoho
　]
}
```

## Differences from Sakura v1/v2

This edition intentionally differs from Sakura v1/v2 in several respects.

### Specifying Step Mode

This version uses a different method for specifying step mode. In v1/v2, specifying a note length such as `l%96` caused all subsequent notes to use step mode.
However, since notes are rarely specified in step mode, this version allows a step specification only temporarily; it does not continue to affect subsequent notes.

```mml
// The following two lines have the same meaning
l%96 cde
c4d4e4
```

### Tuplet Notation

Previously, tuplets were written as `Div{...}`. You can now omit `Div` and write them as `{ceg}`.

```mml
l4 Div{cde} f Div{gab} >c<
l4 {cde} f {gab} >c<
```

The sustain mark `^` also counts as one note, which is convenient.

```mml
l4 {cde}c {gfe}d {c^d} e {d^e} f
```

Tuplets can be nested.

```mml
l1 { c d {efe} d } c
```

### Relative Volume Notation

`(` decreases velocity by 8, and `)` increases velocity by 8.

```mml
v127 c ( c ( c (( c )) c ) c ) c
```

### Chord Notation

Chords using zero-valued notes, such as `c0e0g`, are not supported. Use ordinary chord notation instead.

```mml
'ceg' 'dfa' 'egb' 'ceg'
「ドミソ」「レファラ」「ミソシ」「ドミソ」
```

### Reservation Notation and Continuous CC/PB Writing

Reservation notation is available.

- `v.onTime(low, high, len, ...)` / abbreviated form: `v.T(low,high,len,...)`
- `v.onNote(v1, v2, v3, ...)` / abbreviated form: `v.N(v1,v2,v3,...)`
- `v__n.onTime/onNote/onCycle(...)` / per-layer advance specification added to the base velocity (abbreviated forms: `T`/`N`/`C`)
- `v__n.Random(width)` / per-layer random adjustment (use 0 to disable it)
- `t.onNote(v1, v2, v3, ...)` / abbreviated form: `t.N(v1,v2,v3,...)`
- `(ControlChange`, `PB`, or `p`).onTime(low, high, len, ...)

```mml
v.onTime(0,127,!1)l8cccccccc
v70 v__1.onCycle(10,-10) cdef // velocities: 80,60,80,60
BR(2) PB.onTime(-8192,0,!4) l4c PB(0) efg^
```

`v__n(value)` (`n` is 1 or greater) defines an independent adjustment layer added to the
base velocity. Multiple layers are summed, and the final value is clamped to 0–127. See the
[MML syntax reference](docs/syntax-note.md#サブベロシティ-v__n) for completion and reset behavior.

## Macros

Define macros as follows:

```mml
// Define macros
STR P1 = {cdefg}
#P1 = {cdefg}
// Expand macros
P1
#P1
```

You can provide arguments to a macro and substitute them into its contents. Define `#?1`, `#?2`, `#?3`, and so on inside the macro; each placeholder is replaced with the corresponding argument written immediately after the macro.

```mml
// Define a macro
#Unison = { Key=#?2 Sub{ #?1 } Key=0 #?1 }
// Expand the macro
#Unison{cde},7
```

## References

- **MML syntax reference --- [docs/syntax.md](docs/syntax.md)**
- Sakura (Rust Edition) command list --- [command.md](command.md)
  - Sakura (v2 Edition) command list --- https://sakuramml.com/doc/command/index.htm
- Instrument list --- [voice.md](voice.md)

## Repository

- [GitHub](https://github.com/kujirahand/sakuramml-rust)
- [crate.io](https://crates.io/crates/sakuramml)
- [npm/sakuramml](https://www.npmjs.com/package/sakuramml)
