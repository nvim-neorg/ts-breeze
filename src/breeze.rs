// TODO: Generalize to work with any language

use anyhow::{anyhow, Result};
use std::io::Read;
use std::path::PathBuf;
use std::{fs::File, sync::{Arc, Mutex}};
use rusty_pool::Builder;
use tree_sitter::{Language, Parser, Tree};

/// Parses a file and returns its [`Tree`].
///
/// * `filepath`: The path of the file to read.
fn parse_file(filepath: &std::path::PathBuf, parser: &mut Parser) -> Result<(Tree, String)> {
    let mut file = File::open(filepath)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    match parser.parse(&content, None) {
        Some(tree) => Ok((tree, content)),
        None => Err(anyhow!(format!(
            "Parsing for file '{}' timed out!",
            filepath.display()
        )))
    }
}

/// Parses a set of files on multiple threads given a tree-sitter language.
///
/// * `files`: The files to parse.
/// * `language`: A tree-sitter language to parse.
/// * `num_jobs`: An optional cap for the amount of threads to spawn.
/// * `callback`: A capture to invoke for each file.
pub fn parse_files<F>(files: Vec<PathBuf>, language: Language, num_jobs: Option<usize>, callback: F)
where
    F: FnMut(Tree, (PathBuf, String)) + Send + Sync + 'static,
{
    let threadpool = Builder::new()
        .name("neorg".into())
        .max_size(num_jobs.unwrap_or(4))
        .build();

    let callback = Arc::new(Mutex::new(callback));

    for file in files {
        if file.is_dir() {
            continue;
        }

        let callback = Arc::clone(&callback);

        threadpool.execute(move || {
            let mut parser = Parser::new();
            parser.set_language(language).unwrap();

            let (tree, src) = parse_file(&file, &mut parser).unwrap();
            callback.lock().unwrap()(tree, (file, src));
        });
    }

    threadpool.join();
}

#[cfg(test)]
mod tests {
    use super::*;
    use neorg_dirman::workspace::Workspace;
    use std::path::PathBuf;

    #[test]
    fn test_parse_file() {
        let filepath = PathBuf::from("test/example_workspace/file1.norg");
        let mut parser = Parser::new();
        parser.set_language(tree_sitter_norg::language()).unwrap();
        let (tree, _) = parse_file(&filepath, &mut parser).unwrap();

        assert!(tree.root_node().kind() == "document");
    }

    #[test]
    fn test_parse_files() {
        let workspace = Workspace {
            name: "example workspace".into(),
            path: "test/example_workspace".into(),
        };

        parse_files(
            workspace.files(),
            tree_sitter_norg::language(),
            None,
            |tree: Tree, _| assert!(tree.root_node().kind() == "document"),
        );
    }
}
