# コマンド一覧の自動生成

`command.md`は、Rustソースに記述した注釈から`scripts/extract_command.py`で生成します。
生成ファイルを直接編集しないでください。

## 生成と検査

```sh
make doc
make doc-check
```

`make doc`はPython 3の標準ライブラリだけで動作します。なでしこ3の`cnako3`は、
WebAssemblyのビルドには引き続き必要ですが、ドキュメント生成には不要です。

## Rustソースの注釈

抽出範囲は、`// <SYSTEM_FUNCTION>`と`// </SYSTEM_FUNCTION>`のような
開始・終了マーカーで指定します。コマンド説明は、対象のRust構文に続けて記述します。

```rust
sysfunc_cc_add!(
    sf,
    "PortamentoTime",
    TokenType::ControlChangeCommand,
    '*',
    5,
); // CC#5 Portamento Time range:0-127
```

Pythonスクリプトは関数呼び出し・マクロ呼び出し・`match`のアーム・`if`条件を
複数行の構文として読み取ります。このため、`cargo fmt`が引数やコメントを改行しても
同じ`command.md`を生成できます。

抽出対象の各セクションが空の場合はエラーになります。さらに、テストでは全Rustソースの
一時コピーに`cargo fmt --all`を適用し、整形前後の生成結果が一致することを確認します。

## 将来の改善案

コメント抽出より堅牢な方法は、コマンド名・引数形式・説明をTOMLやJSONなどの構造化データに
集約し、そのデータからRustの登録コードとMarkdownの両方を生成する方法です。情報源が一つに
なるため、コメントと実装のずれを防げます。一方で、既存のコマンド定義を移行し、Rustコード
生成をビルド工程へ組み込む必要があるため、今回のPython化より変更範囲が大きくなります。
