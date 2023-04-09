// TODO: Generalize to work with any language

use anyhow::{anyhow, Result};
use rusty_pool::Builder;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tree_sitter::{Tree, Language};

/// Parses a file and returns its [`Tree`].
///
/// * `filepath`: The path of the file to read.
fn parse_file(filepath: &std::path::PathBuf, language: Language) -> Result<Tree> {
    let mut file = File::open(filepath)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(language)?;

    parser.parse(content, None).ok_or_else(|| {
        anyhow!(format!(
            "Parsing for file '{}' timed out!",
            filepath.display()
        ))
    })
}

pub fn parse_files(files: Vec<PathBuf>) -> Result<Vec<Tree>> {
    let threadpool = Builder::new().name("neorg".into()).build();

    let mut output: Vec<Option<Tree>> = vec![None; files.len()];
    let file_count = files.len();

    let (tx, rx) = crossbeam_channel::bounded(file_count);

    let language = tree_sitter_norg::language();

    for (i, file) in files.into_iter().enumerate() {
        let tx_clone = tx.clone();

        threadpool.execute(move || {
            let parsed = parse_file(&file, language.clone());
            tx_clone.send((i, parsed)).unwrap();
        });
    }

    threadpool.shutdown_join();

    for _ in 0..file_count {
        match rx.recv()? {
            (i, Ok(tree)) => output[i] = Some(tree),
            (_, Err(err)) => return Err(err),
        }
    }

    Ok(output.into_iter().flatten().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use neorg_dirman::workspace::Workspace;
    use std::path::PathBuf;

    #[test]
    fn test_parse_file() {
        let filepath = PathBuf::from("test/example_workspace/file1.norg");
        let tree = parse_file(&filepath, tree_sitter_norg::language()).unwrap();

        assert!(tree.root_node().kind() == "document");
    }

    #[test]
    fn test_parse_files() {
        let workspace = Workspace {
            name: "example workspace".into(),
            path: "test/example_workspace".into(),
        };

        let trees = parse_files(workspace.files())
            .expect("Unable to parse files in the current workspace!");

        assert!(trees[0].root_node().kind() == "document");
    }
}
