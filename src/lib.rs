use tree_sitter::Tree;
use rusty_pool::Builder;
use std::io::Read;
use std::fs::File;
use anyhow::{Result, anyhow};

pub mod c_functions;

fn parse_file(filepath: &std::path::PathBuf) -> Result<Tree> {
    let mut file = File::open(filepath)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(tree_sitter_norg::language())?;

    parser.parse(content, None).ok_or_else(|| anyhow!(format!("Parsing for file {} timed out!", filepath.display())))
}

// TODO: Change Vec<Option<Tree>> to Result<Vec<Tree>, (Err, output.filter(|x| x.is_some()))>
//
pub fn parse_files(workspace: neorg_dirman::workspace::Workspace) -> Vec<Option<Tree>> {
    let files = workspace.files();

    let threadpool = Builder::new().name("neorg".into()).build();

    let mut output: Vec<Option<Tree>> = vec![None; files.len()];
    let file_count = files.len();

    let (tx, rx) = crossbeam_channel::bounded(file_count);

    for (i, file) in files.into_iter().enumerate() {
        let tx_clone = tx.clone();

        threadpool.execute(move || {
            let parsed = parse_file(&file).unwrap();
            tx_clone.send((i, parsed)).unwrap();
        });
    }

    for _ in 0..file_count {
        let (i, tree) = rx.recv().unwrap();
        output[i] = Some(tree);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use neorg_dirman::workspace::Workspace;

    #[test]
    fn test_parse_file() {
        let filepath = PathBuf::from("test/example_workspace/file1.norg");
        let tree = parse_file(&filepath).unwrap();

        assert!(tree.root_node().kind() == "document");
    }

    #[test]
    fn test_parse_files() {
        let workspace = Workspace {
            name: "example workspace".into(),
            path: "test/example_workspace".into(),
        };
        let trees = parse_files(workspace);

        assert!(trees[0].is_some());
    }
}
