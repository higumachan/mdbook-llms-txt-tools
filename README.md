# mdbook-llms-txt

mdbookをllmstxt.org形式に変換するためのツールです。

## 概要

このツールは[mdbook](https://rust-lang.github.io/mdBook/)で作成されたドキュメントを[llmstxt.org](https://llmstxt.org/)形式に変換するためのプリプロセッサです。これにより、mdbookで作成された技術文書をLLMsに最適化された形式で共有することができます。

## インストール方法

```bash
cargo install mdbook-llms-txt-tools
```

## 使用方法

1. プロジェクトの`book.toml`に以下の設定を追加します：

```toml
# 基本的なllmstxt.org形式の出力
[output.llms-txt]

# より詳細な情報を含むllmstxt.org形式の出力
[output.llms-full-txt]

```

2. 通常通り`mdbook build`を実行すると、選択した出力形式でファイルが生成されます。

## 出力形式について

- `llms-txt`: 基本的なllmstxt.org形式の出力を生成します。一般的な用途に適しています。
- `llms-full-txt`: より詳細な情報を含むllmstxt.org形式の出力を生成します。より高度な分析や処理が必要な場合に使用します。

## ライセンス

MIT

## コントリビューション

バグ報告や機能要望は[GitHub Issues](https://github.com/higumachan/mdbook-llms-txt/issues)にお願いします。
プルリクエストも歓迎です。 