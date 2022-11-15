# sakuramml-rust の開発メモ

以下に開発記を書きました。
- [Rustでテキスト音楽「サクラ」を書き直している件](https://qiita.com/kujirahand/items/df2918b70c5715b7dd6b)

## ソースコードの構造

- メタ情報
  - [サクラのバージョン](src/sakura_version.rs)
  - [ライブラリをまとめる](src/lib.rs)
- 曲情報
  - [曲情報およびコンパイル情報](src/song.rs)
- 基本的な流れ
  - [コマンドラインを解析して実行](src/main.rs)
  - [日本語表記(ストトン表記)をMMLに変換](src/sutoton.rs)
  - [MMLを解析してトークンに分割](src/lexer.rs)
  - [トークンを分割して実行する](src/runner.rs)
  - [MIDIファイルを出力](src/midi.rs)
- データ構造や低レベルライブラリ
  - [基本トークン](src/token.rs)
  - [データ型(svalue)](src/svalue.rs)
  - [ソースコード解析用](src/cursor.rs)
- 定義など
  - [MMLや機能を定義したもの](src/mml_def.rs)

