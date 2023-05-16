use std::{
    ffi::{c_char, CStr},
    path::PathBuf,
};

use neorg_dirman::c_functions::FileList;
use tree_sitter::{
    ffi::{TSLanguage, TSTree},
    Tree,
};

use crate::breeze;

type Callback = unsafe extern "C" fn(tree: *mut TSTree);

pub unsafe extern "C" fn parse_file_list(
    file_list: *const FileList,
    language: *mut TSLanguage,
    callback: Callback,
) {
    assert!(!file_list.is_null(), "Parameter `path` must not be `null`!");
    assert!(
        !language.is_null(),
        "Parameter `parser` must not be `null`!"
    );

    let files = std::slice::from_raw_parts(
        (*file_list).data as *const *const c_char,
        (*file_list).length,
    )
    .iter()
    .map(|str| CStr::from_ptr(*str).to_string_lossy().into_owned().into())
    .collect::<Vec<PathBuf>>();

    let language = tree_sitter::Language::from_raw(language);

    struct Wrapper(Callback);

    unsafe impl Send for Wrapper {}
    unsafe impl Sync for Wrapper {}

    impl Wrapper {
        unsafe fn call(&self, tree: *mut TSTree) {
            (self.0)(tree);
        }
    }

    let callback = Wrapper(callback);

    breeze::parse_files(files, language, Some(4), move |tree: Tree| {
        callback.call(tree.into_raw())
    });
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::*;
    use neorg_dirman::c_functions::*;

    #[test]
    fn test_parse_file_list() {
        let language = tree_sitter_norg::language();

        unsafe extern "C" fn callback(tree: *mut TSTree) {
            assert!(Tree::from_raw(tree).root_node().kind() == "document");
        }

        unsafe {
            let name = CString::new("test").unwrap();
            let path = CString::new("test/example_workspace/").unwrap();

            let workspace = create_workspace(name.as_ptr(), path.as_ptr());
            let workspace_files = workspace_files(workspace);

            parse_file_list(
                workspace_files,
                language.into_raw() as *mut TSLanguage,
                callback,
            );

            // TODO: Why is this triggering an error?
            // Is something taking ownership and then we have a double free?
            destroy_files(workspace_files);
            destroy_workspace(workspace);
        }
    }
}
