# mdbook-llms-txt

mdbookをllmstxt.org形式に変換するためのツールです。

## 概要

このツールは[mdbook](https://rust-lang.github.io/mdBook/)で作成されたドキュメントを[llmstxt.org](https://llmstxt.org/)形式に変換するためのプリプロセッサです。これにより、mdbookで作成された技術文書をLLMsに最適化された形式で共有することができます。

## インストール方法

```bash
cargo install mdbook-llms-txt
```

## 使用方法

1. プロジェクトの`book.toml`に以下の設定を追加します：

```toml
[preprocessor.llms-txt]
command = "mdbook-llms-txt"
```

2. 通常通り`mdbook build`を実行すると、出力されるHTMLがllmstxt.org形式に変換されます。

## ライセンス

MIT

## コントリビューション

バグ報告や機能要望は[GitHub Issues](https://github.com/higumachan/mdbook-llms-txt/issues)にお願いします。
プルリクエストも歓迎です。 