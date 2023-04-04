use neorg_dirman::c_functions::FileList;
use std::ffi::CStr;
use std::mem::ManuallyDrop;

use crate::breeze;
use tree_sitter::ffi::TSTree;

pub unsafe extern "C" fn parse_files(files: *const FileList) -> *const TSTree {
    let vec = unsafe { std::slice::from_raw_parts((*files).data, (*files).length) }.to_vec();
    let paths = vec
        .into_iter()
        .map(|str| CStr::from_ptr(str).to_string_lossy().into_owned().into())
        .collect();

    let tree_vec = ManuallyDrop::new(
        breeze::parse_files(paths)
            .expect("Give me better error messages!")
            .into_iter()
            .map(|tree| *tree.into_raw())
            .collect::<Vec<_>>(),
    );

    tree_vec.as_ptr()
}

#[cfg(test)]
mod test {
    use super::*;
    use neorg_dirman::c_functions::*;
    use neorg_dirman::workspace::Workspace;

    #[test]
    fn test_parse_files() {
        unsafe {
            let workspace = Workspace {
                name: "test".into(),
                path: "test/example_workspace".into(),
            };

            let files = workspace_files(&workspace);

            let tree = parse_files(files);
            assert!(!tree.is_null());

            destroy_files(files);
        }
    }
}

// pub extern "C" fn parse_workspace(workspace: *mut Workspace) {
//     let vec = unsafe { std::slice::from_raw_parts((*files).data, (*files).length) }.to_vec();
//
//     let tree_vec = breeze::parse_files();
// }
