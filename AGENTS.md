# AGENTS.md - sakuramml-rust プロジェクト概要

## プロジェクト概要

**sakuramml-rust** は、MML(Music Macro Language)からMIDIファイルに変換するコンパイラです。テキスト表記の音楽(例: `cdefgab`)をMIDIファイルに変換し、簡単に音楽を制作できるツールです。

### プロジェクトの歴史
- 2000年頃に開発された歴史ある音楽制作ツール
- 「オンラインソフト大賞2001」で入賞
- 高校の情報の教科書にも掲載された実績あり
- 2,000曲以上がユーザーにより作曲されている

### 技術スタック
- **言語**: Rust (Edition 2021)
- **バージョン**: 0.1.45
- **対応プラットフォーム**: macOS, Windows, Linux, WebAssembly
- **ライセンス**: LICENSE ファイル参照

## プロジェクト構造

### ディレクトリ構成

```
sakuramml-rust/
├── src/                    # ソースコード
│   ├── lib.rs             # ライブラリエントリーポイント
│   ├── main.rs            # CLIエントリーポイント
│   ├── lexer.rs           # 字句解析器
│   ├── runner.rs          # トークン実行エンジン
│   ├── song.rs            # 曲情報・コンパイル情報管理
│   ├── midi.rs            # MIDI出力生成
│   ├── sutoton.rs         # 日本語表記変換
│   ├── token.rs           # トークン定義
│   ├── svalue.rs          # データ型定義
│   ├── mml_def.rs         # MML機能定義
│   ├── sakura_functions.rs # 組み込み関数
│   ├── sakura_message.rs  # メッセージ管理
│   ├── sakura_version.rs  # バージョン情報
│   └── source_cursor.rs   # ソースコード解析用
├── pkg/                   # WebAssemblyパッケージ
├── samples/               # サンプルMMLファイル
├── target/                # ビルド出力
├── Cargo.toml            # Rustプロジェクト設定
└── README.md             # プロジェクトドキュメント
```

### 主要モジュール

#### 1. **メタ情報**
- `sakura_version.rs`: バージョン管理
- `lib.rs`: ライブラリ統合・WebAssembly公開API

#### 2. **コンパイルフロー**
- `main.rs`: コマンドライン引数解析と実行
- `sutoton.rs`: 日本語表記(「ドレミ」など)をMML(`cde`)に変換
- `lexer.rs`: MMLテキストをトークンに分割
- `runner.rs`: トークンを実行し、MIDI イベントを生成
- `midi.rs`: MIDIファイル形式で出力

#### 3. **データ構造**
- `song.rs`: 曲全体の情報、トラック管理、イベント管理
- `token.rs`: トークン型定義(TokenType, Token構造体)
- `svalue.rs`: 値の型(整数、文字列、配列、ユーザー関数)
- `mml_def.rs`: MMLコマンド定義、タイモード等

#### 4. **補助機能**
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

#### `Event` (song.rs)
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
4. **runner.rs**: 実行エンジンのメインロジック(約1,700行以上)
5. **MIDI出力**: `Event`構造体をMIDI形式に変換

### よくある変更箇所
- 新しいMMLコマンド追加: `mml_def.rs`, `lexer.rs`, `runner.rs`
- バグ修正: 主に`runner.rs`の実行ロジック
- 新機能: `TokenType`追加 → `lexer`でパース → `runner`で実行
