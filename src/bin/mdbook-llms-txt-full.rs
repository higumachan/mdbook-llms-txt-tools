use anyhow::Result;
use mdbook::book::BookItem;
use mdbook::renderer::RenderContext;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();

    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin)?;

    let output = render_llm_txt_full(&ctx)?;

    let output_path = PathBuf::from(&ctx.destination).join("llms-full.txt");
    fs::write(output_path, output)?;

    Ok(())
}

pub fn render_llm_txt_full(ctx: &RenderContext) -> anyhow::Result<String> {
    let mut output = String::new();
    let book = &ctx.book;

    // Use the title from book.toml
    let title = ctx
        .config
        .book
        .title
        .as_deref()
        .expect("book.title is required");

    output.push_str(&format!("# {}\n\n", title));

    // Add description if exists
    if let Some(description) = &ctx.config.book.description {
        output.push_str(&format!("> {}\n\n", description));
    }

    // Process chapters
    for item in &book.sections {
        process_book_item(item, &mut output);
    }

    Ok(output)
}

fn process_book_item(item: &BookItem, output: &mut String) {
    if let BookItem::Chapter(chapter) = item {
        // チャプターの内容を追加
        output.push_str(&chapter.content);
        output.push_str("\n\n");

        // サブチャプターの処理を再帰的に行う
        for sub_item in &chapter.sub_items {
            process_book_item(sub_item, output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use similar_asserts::assert_eq;

    #[test]
    fn test_render_llm_txt_full() -> Result<()> {
        let json_str = include_str!("../../assets/test_render_contexts/simple-project.json");
        let ctx: RenderContext = serde_json::from_str(json_str)?;

        let output = render_llm_txt_full(&ctx)?;

        // 出力に必要なヘッダー情報が含まれていることを確認
        assert!(output.contains("# サンプルブック"), "Check: {}", output);
        assert!(output.contains("> これはサンプルブックです。"), "Check: {}", output);
        assert!(output.contains("# はじめに"), "Check: {}", output);

        Ok(())
    }

    #[test]
    fn test_render_llm_txt_full_deep_chapters() -> Result<()> {
        let json_str = include_str!("../../assets/test_render_contexts/deep-chapter-project.json");
        let ctx: RenderContext = serde_json::from_str(json_str)?;

        let output = render_llm_txt_full(&ctx)?;

        // タイトルと説明が含まれていることを確認
        assert!(output.contains("# 深い階層のテストブック"), "Check: {}", output);
        assert!(output.contains("> 4段以上の深い階層構造を持つテスト用のブックです。"), "Check: {}", output);

        // 深い階層のコンテンツが含まれていることを確認
        assert!(output.contains("# 1.1.1.1.1 細目"), "Check: {}", output);
        assert!(output.contains("これは1.1.1.1.1細目のコンテンツです。"), "Check: {}", output);

        // 並列の階層構造も含まれていることを確認
        assert!(output.contains("# 1.2.1.1 目"), "Check: {}", output);
        assert!(output.contains("これは1.2.1.1目のコンテンツです。"), "Check: {}", output);

        Ok(())
    }
}
