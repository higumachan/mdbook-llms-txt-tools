use anyhow::Result;
use clap::{Arg, Command};
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();

    let matches = Command::new("mdbook-llms-txt")
        .about("A mdbook backend for generating llms.txt files")
        .subcommand(Command::new("supports").arg(Arg::new("renderer").required(true)))
        .get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        let renderer = sub_args
            .get_one::<String>("renderer")
            .expect("Required argument");
        if renderer == "llms-txt" {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }

    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin)?;

    let output = render_llm_txt(&ctx)?;

    let output_path = PathBuf::from(&ctx.destination).join("llms.txt");
    fs::write(output_path, output)?;

    Ok(())
}

pub fn render_llm_txt(ctx: &RenderContext) -> anyhow::Result<String> {
    let mut output = String::new();
    let book = &ctx.book;

    // book.tomlの内容を確認
    println!("Config: {:?}", serde_json::to_string_pretty(&ctx)?);

    // book.tomlのtitleを使用
    let title = ctx
        .config
        .book
        .title
        .as_deref()
        .expect("book.title is required");
    eprintln!("Title: {}", title);

    output.push_str(&format!("# {}\n\n", title));

    // descriptionがある場合は追加
    if let Some(description) = &ctx.config.book.description {
        output.push_str(&format!("> {}\n\n", description));
    }

    // チャプターの処理
    for item in &book.sections {
        match item {
            BookItem::Chapter(chapter) => {
                // セクション名の追加
                output.push_str(&format!("## {}\n\n", chapter.name));

                // チャプターの内容をリンクとして追加
                if let Some(path) = &chapter.path {
                    output.push_str(&format!("- [{}]({})\n", chapter.name, path.display()));
                }

                // サブチャプターの処理
                for sub_item in &chapter.sub_items {
                    if let BookItem::Chapter(sub_chapter) = sub_item {
                        if let Some(path) = &sub_chapter.path {
                            output.push_str(&format!(
                                "- [{}]({})\n",
                                sub_chapter.name,
                                path.display()
                            ));
                        }
                    }
                }
                output.push('\n');
            }
            _ => {}
        }
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    const SAMPLE_RENDERED_OUTPUT: &str = "\
# サンプルブック

> これはサンプルブックです。

## はじめに

- [はじめに](introduction.md)

## 第1章: サンプル

- [第1章: サンプル](chapter_1.md)
- [1.1 サブセクション](chapter_1/section_1.md)

## 第2章: 機能紹介

- [第2章: 機能紹介](chapter_2.md)

";

    #[test]
    fn simple_project_json() -> Result<()> {
        let json_str = include_str!("../../assets/test_render_contexts/simple-project.json");
        let ctx: RenderContext = serde_json::from_str(json_str)?;

        let output = render_llm_txt(&ctx)?;

        let expected = SAMPLE_RENDERED_OUTPUT;

        assert_eq!(output, expected);
        Ok(())
    }
}
