# AGENTS.md - sakuramml-rust プロジェクト概要

## プロジェクト概要

**sakuramml-rust** は、MML(Music Macro Language)からMIDIファイルに変換するコンパイラです。テキスト表記の音楽(例: `cdefgab`)をMIDIファイルに変換し、簡単に音楽を制作できるツールです。

## エージェントの役割

- レビューするときは、日本語でレビューを応答してください。
- プログラムのコメントも日本語で書いてください。
- ドキュメントは、最初に日本語を書き、必要に応じて英語の翻訳を追加してください。
  - `README_ja.md`が日本語版、`README.md`が英語版です。両方のドキュメントを一度に更新してください。
- 機能を実装したら、必ずテストコードを追加してください。
- 機能を実装したら、`docs/*.md`にドキュメントを追加してください。ドキュメントは日本語で書いて。
- 実装で迷ったらオリジナル実装のPascal/Delphi版を参考にしてください。`sakuramml/`ディレクトリにソースコードがあります。
- `command.md`は、コマンドの一覧をまとめたもので、`make doc`コマンドで`scripts/extract_command.py`から自動生成されます。手動で編集せず、ソースコードにコメントを追加して、`make doc`で更新してください。

### 技術スタック

- **言語**: Rust (Edition 2021)
- **バージョン**: 0.1.45
- **対応プラットフォーム**: macOS, Windows, Linux, WebAssembly
- **ライセンス**: LICENSE ファイル参照

## プロジェクト構造

### ディレクトリ構成

```text
sakuramml-rust/
├── docs/                  # 文法定義・仕様などのドキュメント
├── src/                   # ソースコード
│   ├── lib.rs             # ライブラリエントリーポイント
│   ├── main.rs            # CLIエントリーポイント
│   ├── lexer.rs           # 字句解析器のエントリーポイント
│   ├── lexer/             # 字句解析器の機能別サブモジュール
│   │   ├── args.rs        # コマンド引数の読み取り
│   │   ├── calc.rs        # 計算式・制御構文の読み取り
│   │   ├── cc.rs          # CC・RPN・NRPN・SysExの読み取り
│   │   ├── command.rs     # 大文字コマンドと特殊構文の読み取り
│   │   ├── error.rs       # 字句解析時のエラー・警告生成
│   │   ├── note.rs        # 音符・音長・演奏パラメータの読み取り
│   │   └── variable.rs    # 変数・ユーザー定義関数の読み取り
│   ├── runner.rs          # トークン実行エンジンのエントリーポイント
│   ├── runner/            # 実行処理の機能別サブモジュール
│   │   ├── cc.rs          # CC・テンポ・音色の実行
│   │   ├── control.rs     # 条件分岐・ループの実行
│   │   ├── function.rs    # ユーザー定義関数・システム関数の実行
│   │   ├── meta.rs        # メタイベント・ログ・直接SMF出力
│   │   ├── note.rs        # 音符・休符の実行
│   │   ├── structure.rs   # サブルーチン・連符・和音・タイムポインタの実行
│   │   ├── sysex.rs       # SysExの実行
│   │   ├── tie.rs         # タイ・スラーの実行
│   │   ├── track_state.rs # トラックと演奏状態の更新
│   │   └── variable.rs    # 変数・計算式・配列の実行
│   ├── song.rs            # 曲全体の状態管理
│   ├── song/              # 曲を構成するデータ型のサブモジュール
│   │   ├── event.rs       # MIDIイベントの型と生成
│   │   ├── flags.rs       # 実行時フラグの管理
│   │   ├── function.rs    # ユーザー定義・システム関数の型
│   │   └── track.rs       # トラックと演奏パラメータの管理
│   ├── midi.rs            # MIDI出力生成
│   ├── sutoton.rs         # 日本語表記変換
│   ├── token.rs           # トークン定義
│   ├── svalue.rs          # データ型定義
│   ├── mml_def.rs         # MML定義の公開エントリーポイント
│   ├── mml_def/           # MMLの予約語・組み込み定義
│   │   ├── reserved_words.rs   # 予約語の定義
│   │   ├── rhythm.rs           # リズムマクロの初期定義
│   │   ├── system_functions.rs # システム関数とコマンドの定義
│   │   ├── tie_mode.rs         # タイ・スラーモードの定義
│   │   └── variables.rs        # 予約変数の初期定義
│   ├── note_length.rs     # 音長文字列からステップ数への変換
│   ├── sakura_functions.rs # 組み込み関数
│   ├── sakura_message.rs  # メッセージ管理
│   ├── sakura_version.rs  # バージョン情報
│   ├── source_cursor.rs   # ソースコード解析用カーソル
│   ├── lexer_test.rs      # 字句解析器のテスト
│   ├── runner_test.rs     # 実行エンジンのテスト
│   ├── song_test.rs       # 曲・MIDIイベントのテスト
│   ├── batch_gen_voice_var.nako3   # 音色変数生成スクリプト
│   └── batch_version.nako3         # バージョン更新スクリプト
├── scripts/
│   └── extract_command.py      # コマンド情報抽出スクリプト
├── pkg/                   # WebAssemblyパッケージ
├── samples/               # サンプルMMLファイル
├── target/                # ビルド出力
├── sakuramml/            # オリジナル実装(Pascal/Delphi)のソースコード
├── Cargo.toml            # Rustプロジェクト設定
└── README.md             # プロジェクトドキュメント
```

### 主要モジュール

#### 1. メタ情報

- `sakura_version.rs`: バージョン管理
- `lib.rs`: ライブラリ統合・WebAssembly公開API

#### 2. コンパイルフロー

- `main.rs`: コマンドライン引数解析と実行
- `sutoton.rs`: 日本語表記(「ドレミ」など)をMML(`cde`)に変換
- `lexer.rs`, `lexer/`: MMLテキストをトークンに分割。機能別の読み取り処理は`lexer/`以下に配置
- `runner.rs`, `runner/`: トークンを実行し、MIDIイベントを生成。機能別の実行処理は`runner/`以下に配置
- `midi.rs`: MIDIファイル形式で出力

#### 3. データ構造

- `song.rs`, `song/`: 曲全体の状態と、トラック・イベント・関数・実行時フラグを管理
- `token.rs`: トークン型定義(TokenType, Token構造体)
- `svalue.rs`: 値の型(整数、文字列、配列、ユーザー関数)
- `mml_def.rs`, `mml_def/`: MMLの予約語、システム関数、予約変数、リズムマクロ、タイモードを定義
- `note_length.rs`: 音長指定をMIDIステップ数に変換

#### 4. 補助機能

- `sakura_functions.rs`: 組み込み関数(数学関数、文字列操作など)
- `sakura_message.rs`: 多言語メッセージ管理
- `source_cursor.rs`: テキストパース用カーソル

## 主要機能

### MML基本機能

- **音階指定**: `cdefgab` (ドレミファソラシ)
- **オクターブ**: `o4`, `>`, `<`
- **音長**: `l4` (4分音符)、`l8` (8分音符)
- **休符**: `r`
- **和音**: `` `ceg` `` (ハーモニー)
- **トラック/チャンネル**: `TR(1)`, `CH(1)`
- **音色**: `@1`
- **音量**: `v100`
- **ゲートタイム**: `q90`
- **テンポ**: `Tempo(120)`

### 高度な機能

- **タイムポインタ**: `TIME(小節:拍:ステップ)`
- **サブルーチン**: `SUB{...}`
- **リズムマクロ**: `$文字{定義}`
- **ループ**: `[4 cde]`
- **条件分岐**: `If(条件){真}{偽}`
- **変数**: `Int A = 100`
- **関数定義**: `Function 名前(引数){...}`
- **SysEx**: `SysEx{0xF0, ...}`
- **CC**: `CC(番号, 値)`
- **RPN/NRPN**: `RPN(MSB, LSB, 値)`

## ビルド・実行方法

### ビルド

```bash
cargo build --release
```

### CLIの使い方

```bash
# MMLファイルをMIDIに変換
./sakuramml test.mml test.mid

# 出力ファイル名省略(自動生成)
./sakuramml test.mml

# MMLを直接評価
./sakuramml --eval "o4l4 cege c1"

# MIDIファイルをダンプ
./sakuramml --dump test.mid
```

### WebAssemblyビルド

```bash
./build_wasm.sh
```

## 開発ガイドライン

### コードの読み方

1. **エントリーポイント**: `main.rs`でコマンドライン処理
2. **変換フロー**: 
   - `sutoton::convert()` → 日本語をMMLに
   - `lexer::lex()` → トークン化
   - `runner::exec()` → 実行
   - `midi::generate()` → MIDI出力
3. **データフロー**: `Song`構造体がすべての状態を保持

### 主要な構造体

#### `Song` (song.rs)

曲全体の情報を管理

- `tracks`: トラック配列
- `tempo`: テンポ
- `timebase`: タイムベース
- `variables`: 変数マップ
- `functions`: ユーザー定義関数
- `events`: イベントリスト

#### `Token` (token.rs)

パース結果のトークン

- `ttype`: トークンタイプ
- `value_i`: 整数値
- `value_s`: 文字列値
- `children`: 子トークン
- `data`: 付加データ

#### `Event` (song/event.rs)

MIDIイベント

- `etype`: イベントタイプ(NoteOn, CC, Meta等)
- `time`: タイムポジション
- `channel`: チャンネル
- `v1, v2, v3`: パラメータ

### テスト

```bash
cargo test
```

## リソース

### ドキュメント

- [公式サイト](https://sakuramml.com)
- [チュートリアル](https://sakuramml.com/index.php?Tutorial)
- [曲掲示板](https://sakuramml.com/mmlbbs6/)
- [Qiita開発記](https://qiita.com/kujirahand/items/df2918b70c5715b7dd6b)

### Web版

- [PicoSakura (Web Player)](https://sakuramml.com/picosakura/)

### リリース

- [GitHub Releases](https://github.com/kujirahand/sakuramml-rust/releases/)

## 貢献

このプロジェクトは歴史あるオープンソースプロジェクトです。バグ報告、機能リクエスト、プルリクエストを歓迎します。

### 開発者向けメモ

詳細な開発メモは `dev_memo.md` を参照してください。

## AIエージェント向けヒント

### このプロジェクトで作業する際のポイント

1. **コンパイラフロー**: `sutoton` → `lexer` → `runner` → `midi`の順に処理
2. **状態管理**: `Song`構造体がすべての状態を保持
3. **トークン**: `TokenType`列挙型で命令を識別
4. **runner.rs / runner/**: 実行エンジンの入口と機能別の実行処理
5. **MIDI出力**: `Event`構造体をMIDI形式に変換

### よくある変更箇所

- 新しいMMLコマンド追加: `mml_def/system_functions.rs`でコマンドを定義し、必要に応じて`lexer/command.rs`で解析、対応する`runner/`のサブモジュールで実行
- 音符・音長の変更: `lexer/note.rs`, `note_length.rs`, `runner/note.rs`
- CC・RPN・NRPN・SysExの変更: `lexer/cc.rs`, `runner/cc.rs`, `runner/sysex.rs`
- 変数・関数・計算式の変更: `lexer/variable.rs`, `lexer/calc.rs`, `runner/variable.rs`, `runner/function.rs`
- トラック・イベント・実行状態の変更: `song/track.rs`, `song/event.rs`, `song/flags.rs`
- 新機能: `token.rs`に`TokenType`追加 → `lexer/`でパース → `runner/`で実行 → 対応する`*_test.rs`にテスト追加
