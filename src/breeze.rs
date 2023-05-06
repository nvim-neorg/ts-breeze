// TODO: Generalize to work with any language

use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use threadpool::Builder;
use tree_sitter::{Language, Parser, Tree};

/// Parses a file and returns its [`Tree`].
///
/// * `filepath`: The path of the file to read.
fn parse_file(filepath: &std::path::PathBuf, parser: &mut Parser) -> Result<Tree> {
    let mut file = File::open(filepath)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    drop(file);

    parser.parse(content, None).ok_or_else(|| {
        anyhow!(format!(
            "Parsing for file '{}' timed out!",
            filepath.display()
        ))
    })
}

pub fn parse_files(
    files: Vec<PathBuf>,
    language: Language,
    num_jobs: Option<usize>,
    callback: &'static (dyn Fn(Tree) + Send + Sync),
) {
    let threadpool = Builder::new()
        .thread_name("neorg".into())
        .num_threads(num_jobs.unwrap_or(4))
        .build();

    for file in files {
        threadpool.execute(move || {
            let mut parser = Parser::new();
            parser.set_language(language).unwrap();

            let tree = parse_file(&file, &mut parser).unwrap();
            callback(tree);
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
        let tree = parse_file(&filepath, &mut parser).unwrap();

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
            &|tree: Tree| assert!(tree.root_node().kind() == "document"),
        );
    }
}
