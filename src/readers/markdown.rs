use eyre::{eyre, Report, WrapErr};
use markdown::mdast::Node;
use markdown::{to_mdast, ParseOptions};
use tracing::debug;

use crate::entities::core::block::{Block, Blocks};
use crate::entities::core::block_range::BlockRange;
use crate::entities::core::freeout::FreeoutOptions;
use crate::entities::core::position::Position;
use crate::entities::core::words_statistics::WordStatistics;
use crate::entities::reader::reader_trait::ReaderTrait;

pub struct MarkdownReader {}

impl Default for MarkdownReader {
    fn default() -> Self {
        Self {}
    }
}

pub struct PastBlocks {
    pub past_block_ids_and_depths: Vec<(usize, usize)>,
}

// method to take in the current block's depth and return its parent id;
//
// the logic is:
impl PastBlocks {
    pub fn last(&self) -> Option<&(usize, usize)> {
        self.past_block_ids_and_depths.last()
    }
    // Add a new block id and its depth to the tracker
    pub fn add_block(&mut self, id: usize, depth: usize) {
        self.past_block_ids_and_depths.push((id, depth));
    }

    // Determine the parent id for a given block's depth
    pub fn get_parent_id_for_depth(&self, depth: usize) -> Option<usize> {
        // Traverse the list in reverse
        for &(id, d) in self.past_block_ids_and_depths.iter().rev() {
            if d < depth {
                // Found a block with lesser depth, return its id
                return Some(id);
            }
        }
        // No parent found
        None
    }

    pub fn get_new_id(&self) -> usize {
        self.past_block_ids_and_depths.len() + 1
    }
}

impl ReaderTrait for MarkdownReader {
    fn read(&self, source: &str, opt: &FreeoutOptions) -> Result<Blocks, Report> {
        let ast = markdown_to_ast(source)?;

        let mut blocks = Blocks::new();

        fn attach_content_to_parent_block(parent_id: usize, content: String, blocks: &mut Blocks) {
            if let Some(parent) = blocks.get_mut(&parent_id) {
                // check if the parent has content, if so append the content to the parent's content joined by a space
                if let Some(parent_content) = &mut parent.content {
                    parent_content.push_str(&format!("\n{}", content.trim()));
                } else {
                    // if the parent doesn't have content, set the content to the parent's content
                    parent.content = Some(content.to_string());
                }
            }
        }

        fn process_node(
            nodes: &[Node],
            source: &str,
            blocks: &mut Blocks,
            opt: &FreeoutOptions,
            past_block_ids_and_depths: &mut PastBlocks,
        ) {
            for node in nodes {
                match node {
                    Node::Heading(heading) => {
                        let position = heading.position.as_ref().unwrap();
                        let start = Position {
                            line: position.start.line,
                            column: position.start.column,
                            offset: position.start.offset,
                        };
                        let end = Position {
                            line: position.end.line,
                            column: position.end.column,
                            offset: position.end.offset,
                        };

                        let title = if let Some(Node::Text(text)) = heading.children.first() {
                            text.value.clone()
                        } else {
                            "".to_string()
                        };

                        let depth = heading.depth as usize;

                        // because Markdown-rs AST is already an ordered tree, we can just use the id as the order
                        let id = past_block_ids_and_depths.get_new_id();


                        let parent_id = past_block_ids_and_depths.get_parent_id_for_depth(depth);

                        // if there is parent, add the current id to the parent's children in the blocks hashmap
                        if let Some(parent_id) = parent_id {
                            if let Some(parent) = blocks.get_mut(&parent_id) {
                                parent.children_ids.push(id);
                            }
                        }

                        let block = Block {
                            id,
                            depth,
                            marker: "#".repeat(heading.depth as usize),
                            title: title.clone(),
                            content: None, // Do not attach any content to heading itself. Instead, its children's text or code content will be attached to its parent at a later stage, if `include_content` option is enabled.
                            note: None,
                            parent_id,
                            header_range: BlockRange { start, end },
                            self_stats: WordStatistics::default(),
                            aggregate_stats: WordStatistics::default(),
                            exclude: false,
                            hash: None,
                            children_ids: vec![],
                            block_range: None,
                        };

                        blocks.insert(id, block);
                        // insert into blocks hashmap by order
                        past_block_ids_and_depths.add_block(id, depth);

                        process_node(
                            &heading.children,
                            source,
                            blocks,
                            opt,
                            past_block_ids_and_depths,
                        );
                    }
                    // Handle other node types as necessary
                    _ => {
                        // add children content
                        if !opt.include_content {
                            continue;
                        }
                        // for any other types, add the content to the parent block
                        // use the last block id and depth to determine the parent
                        let depth = past_block_ids_and_depths
                            .last()
                            .map(|(_, depth)| depth + 1)
                            .unwrap_or(0);

                        let mut content: String = "".to_string();

                        match node {
                            Node::Text(text) => {
                                content = text.value.clone();
                            }
                            Node::Paragraph(paragraph) => {
                                for child in &paragraph.children {
                                    match child {
                                        Node::Text(text) => {
                                            content.push_str(&text.value);
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            Node::Code(code) => {
                                content = code.value.clone();
                            }
                            _ => {}
                        }

                        let parent_id = past_block_ids_and_depths.get_parent_id_for_depth(depth);
                        if let Some(parent_id) = parent_id {
                            attach_content_to_parent_block(parent_id, content, blocks);
                        }
                    }
                }
            }
        }

        if let Node::Root(root) = &ast {
            process_node(
                &root.children,
                source,
                &mut blocks,
                opt,
                &mut PastBlocks {
                    past_block_ids_and_depths: vec![],
                },
            );
        }

        Ok(blocks)
    }
}

fn markdown_to_ast(text: &str) -> Result<Node, Report> {
    let markdown_ask = to_mdast(text, &ParseOptions::default())
        .map_err(|err| eyre!("Failed to parse markdown: {}", err))?;
    Ok(markdown_ask)
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use tracing_test::traced_test;
    use words_count::count;

    use crate::entities::core::freeout::Freeout;
    use crate::entities::core::outline::Outline;
    use crate::utils::block_range_utils::PositionUtils;
    use crate::utils::test_utils::test_utils::{get_temp_folder, md_get_long_chinese};

    use super::*;

    #[test]
    fn test_markdown_to_ast() {
        // get rust root folder tests
        let markdown_file = md_get_long_chinese();
        // print file path
        let file_content = std::fs::read_to_string(markdown_file).unwrap();

        let ast = markdown_to_ast(&file_content).unwrap();

        // write json to file
        let json_file = get_temp_folder().join("long_chinese_md_ast.json");
        let json_content = serde_json::to_string_pretty(&ast).unwrap();
        std::fs::write(json_file, json_content).unwrap();
    }

    #[test]
    fn test_makrdown_to_blocks() {
        // markdown file
        let markdown_file = md_get_long_chinese();
        // generate blocks
        let mut reader = MarkdownReader::default();

        let options = FreeoutOptions {
            include_content: true,
        };

        let blocks = reader
            .read(&std::fs::read_to_string(markdown_file).unwrap(), &options)
            .unwrap();

        assert!(!blocks.is_empty());

        // write json to file
        let json_file = get_temp_folder().join("long_chinese_md_blocks.json");

        let json_content = serde_json::to_string_pretty(&blocks).unwrap();

        std::fs::write(json_file, json_content).unwrap();
    }

    #[test]
    fn test_markdown_to_out() {
        // markdown file
        let markdown_file = md_get_long_chinese();
        // generate blocks
        let mut reader = MarkdownReader::default();

        let options = FreeoutOptions {
            include_content: true,
        };

        let blocks = reader
            .read(&std::fs::read_to_string(markdown_file).unwrap(), &options)
            .unwrap();

        let outline = Outline::build_outline(&blocks).unwrap();

        // write json to file
        let json_file = get_temp_folder().join("long_chinese_md_outline.json");
        let json_content = serde_json::to_string_pretty(&outline).unwrap();

        std::fs::write(json_file, json_content).unwrap();
    }
    #[traced_test]
    #[test]
    fn test_markdown_generate_long_chinese_outline() {
        // markdown file
        let markdown_file = md_get_long_chinese();
        // generate blocks
        let mut reader = MarkdownReader::default();

        let options = FreeoutOptions {
            include_content: true,
        };

        let mut freeout = Freeout::new(
            std::fs::read_to_string(markdown_file).unwrap(),
            Some(options),
        );

        let outline = freeout.outline(&reader).unwrap();

        // write json to file
        let json_file = get_temp_folder().join("long_chinese_md_outline.json");

        let json_content = serde_json::to_string_pretty(&outline).unwrap();

        std::fs::write(json_file, json_content).unwrap();
    }

    #[traced_test]
    #[test]
    fn should_markdown_content_be_processed() {
        let markdown_content = r#"
        One two three four
        
        Five six seven eight"#;

        // use raw string
        let markdown = format!("# Title 1\n{}\n## Title 2", markdown_content.trim());

        let mut reader = MarkdownReader::default();

        let outline = Freeout::new(markdown.clone(), None).outline(&reader).unwrap();

        let root_blocks = outline.items;

        assert_eq!(root_blocks.len(), 1);

        let first_block = root_blocks.get(0).unwrap().block.clone();

        let first_block_content = first_block.content.unwrap().clone();

        let word_count = count(first_block_content);

        assert_eq!(word_count.words, 10);

        // content should have hash and self_word_count
        assert!(first_block.hash.is_some());
        assert!(first_block.self_stats.count.words > 0);

        // the first block's range should also be able to retrieve the same content.
        // the block_range should be some
        assert!(first_block.block_range.is_some());
        let first_block_range = first_block.block_range.clone().unwrap();
        let extractd_text_by_range = PositionUtils::get_text_by_range(&markdown, &first_block_range);

        assert_eq!(extractd_text_by_range, markdown_content.trim());
    }

    // #[traced_test]
    #[test]
    fn should_handle_thousands_of_depths() {
        let mut markdown_with_thousands_of_depths = "".to_string();

        for i in 0..1600 {
            // for a batch of 5, add a new depth and start again at 1
            for j in 0..5 {
                let marker = "#".repeat(j);
                markdown_with_thousands_of_depths.push_str(&format!("{} Title {}\n", marker, i));
            }
        }

        // how many lines are there
        let line_count = markdown_with_thousands_of_depths.lines().count();
        println!("line count: {}", line_count);

        let reader = MarkdownReader::default();

        // measure the time it takes to generate the outline

        let start = std::time::Instant::now();
        let outline = Freeout::new(markdown_with_thousands_of_depths, Some(FreeoutOptions {
            include_content: true,
            ..Default::default()
        }))
            .outline(&reader)
            .unwrap();

        let end = std::time::Instant::now();
        info!(
            "time to generate outline with thousands of levels: {:?}",
            end - start
        );

        // write out
        let json_file = get_temp_folder().join("thousands_of_depths_outline.json");

        let json_content = serde_json::to_string_pretty(&outline).unwrap();

        std::fs::write(json_file, json_content).unwrap();

    }
}
