# sakuramml-rust

"sakruamml" is a MML/ABC to MIDI compier.

- [README(日本語)](README_ja.md)

This compiler that converts the text of "cde" into MIDI files. 
It is a tool that allows you to easily create music.
It is made with Rust and works on multiple platforms (macOS/Windows/Linux/WebAssembly).

# Repository

- [GitHub](https://github.com/kujirahand/sakuramml-rust)
- [crate.io](https://crates.io/crates/sakuramml)

# Web Version (WASM)

- [Pico-Sakura](https://sakuramml.com/go.php?15) --- WebAssembly version
  - [Sakura's Web](https://sakuramml.com)

# Install

## Binary - command line

- [Command line v0.1.13(win/mac/ubuntu)](https://github.com/kujirahand/sakuramml-rust/releases/tag/0.1.13)

## Compile

Please install [Rust compier](https://www.rust-lang.org/tools/install).

```
$ git clone https://github.com/kujirahand/sakuramml-rust.git
$ cd sakuramml-rust
$ cargo build --relase
```

`sakuramml` command in `target/release/` dir.


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

The tuplets are written as "Div{...}", but "Div" can be omitted and written as "{ceg}".

```
l4 Div{cde} f Div{gab} >c<
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
- v.onNote(v1, v2, v3, ...)　/ 省略形 v.N(v1,v2,v3,...)
- t.onNote(v1, v2, v3, ...)　/ 省略形 t.N(v1,v2,v3,...)
- (ControllChange または PB または p).onTime(low, high, len, ...)

```
v.onTime(0,127,!1)l8cccccccc
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

## memo

- Command List(ja) --- [command.md](src/command.md)

