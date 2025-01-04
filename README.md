# sakuramml-rust

`sakruamml` is a MML/ABC to MIDI compier.

This compiler that converts the MML/ABC into MIDI files. 
MML is a notation system that represents sheet music using text such as `cde`.
It is a tool that allows you to easily create music.
It is made with Rust and works on multiple platforms (macOS/Windows/Linux/WebAssembly).

`sakuramml` is a historic music production tool developed around the year 2000.
It was recognized as an award-winning entry in the "Online Software Grand Prize 2001" in Japan, and was even featured in high school IT textbooks.

- [README(日本語)](https://github.com/kujirahand/sakuramml-rust/blob/main/README_ja.md)

# Samples

Many users have composed music using `sakuramml`, and over 2,000 songs have been posted on this forum.
Additionally, more than 100 songs are compatible with `sakuramml-rust`.

- [mmlbbs6](https://sakuramml.com/mmlbbs6/index.php?action=pico) --- Many songs are posted on mmlbbs6.

# Web Version

- [PicoSakura](https://sakuramml.com/picosakura/)
  - [sakuramml.com(Web)](https://sakuramml.com)

# Install

## Binary - command line tool

- [Command line binary(win/mac)](https://github.com/kujirahand/sakuramml-rust/releases/)

## Compile with Rust

Please install [Rust compier](https://www.rust-lang.org/tools/install).

```
$ git clone https://github.com/kujirahand/sakuramml-rust.git
$ cd sakuramml-rust
$ cargo build --release
```

`target/release/sakuramml` is compiler.


# Sakura MML

Please make text file "test.mml". 　Execute the following command to generate a MIDI file.

```
$ sakuramml test.mml
```

## Basic

```
o4 cdefgab>c<bagfedc
```

```
TR=1 CH=1 l1 ceg^
```

## Harmony

```
l4 `ceg` `dfa`8 `egb`8 `ceg`
```

## Set TIME Pointer


```
// top
TIME(1:1:0) cdef
TIME(1:1:0) efga

// 2mes
TIME(2:1:0) cdef
```

`SUB{...}` command let time pointer back.

```
SUB{ cdef  c }
SUB{ efga  e }
     rrrr  g
```

## Rhythm macro

In the rhythm macro, one character is treated as one instruction regardless of uppercase or lowercase letters.
Rhythm macro definitions are described as "$(char){definition}".


```
// define macro
$b{n36,}
$h{n42,}
$o{n46,}
// new macro
$S{n37,}
CH(10)
// sample
Rhythm{
　[4　l8
　　　brSr bbsr r-1
　　　hoho hoho
　]
}
```


### How to specify tuplets

The tuplets are written as "DIV{...}", but "DIV" can be omitted and written as "{ceg}".

```
l4 DIV{cde} f DIV{gab} >c<
l4 {cde} f {gab} >c<
```


```
l4 {cde}c {gfe}d {c^d} e {d^e} f
```

The tuplets can nest.

```
l1 { c d {efe} d } c
```

### Velocity

"(" decreases the velocity by 8, and ")" increases the velocity by 8.

```
v127 c ( c ( c (( c )) c ) c ) c  
```

### Harmony


```
`ceg` `dfa` `egb` `ceg`
```

### Reservation notation

- v.onTime(low, high, len, ...)　/ 省略形 v.T(low,high,len,...)
  - The value that should be specified for `len` is the tick. When specifying the note length, it should be written as `!4`, for example.
- v.onNote(v1, v2, v3, ...)　/ abbreviation: v.N(v1,v2,v3,...)
- t.onNote(v1, v2, v3, ...)　/ abbreviation: t.N(v1,v2,v3,...)
- (ControllChange or PB or p).onTime(low, high, len, ...)

```
v.onTime(0,127,!1) l8cccccccc
BR(2) PB.onTime(-8192,0,!4) l4c PB(0) efg^
```

## Macro

It can define Macro.

```
// define Macro
STR P1 = {cdefg}
#P1 = {cdefg}
// expand Macro
P1
#P1
```

The macro can replace with arguments. 
It replaces `#?1` `#?2` `#?3` ...

```
// define macro
#Unison = { Key=#?2 Sub{ #?1 } Key=0 #?1 }
// expand macro with arguments 
#Unison{cde},7
```

## Script

It can use IF/FOR/WHILE/FUNCTION script.

```
// IF 
INT A = 3
INT B = 5
IF (A == B) { PRINT({A == B}) } ELSE { PRINT({A != B}) }

// FOR
FOR (INT N=1; N < 5; N++) {
  PRINT(N)
}
```

## Variables

 It can define variables of INT, STR, and ARRAY types.

```
// define variables
INT I1=30
STR S1={abcd}
ARRAY A1=(1,2,3)

// use variables
PRINT(I1) // 30
PRINT(S1) // abcd
PRINT(A1) // (1,2,3)
PRINT(A1(2)) // 3
```

## reference

- Command List(ja) --- [command.md](command.md)
- Voice List - [voice.md](voice.md)

## Related Repository

- [GitHub/sakuramml-rust](https://github.com/kujirahand/sakuramml-rust)
- [crate.io/sakuramml](https://crates.io/crates/sakuramml)
- [npm/sakuramml](https://www.npmjs.com/package/sakuramml)
- Player
  - [GitHub/picosakura](https://github.com/kujirahand/picosakura) ... web player
  - [GitHub/picosakura-rust](https://github.com/kujirahand/picosakura-rust) ... local player
