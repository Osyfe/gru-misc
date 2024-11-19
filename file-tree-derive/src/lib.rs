extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree};
use std::path::PathBuf;

const ERROR: &str = "expected (\"path\", \"root\")";

#[proc_macro]
pub fn tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream
{
    let mut iter = TokenStream::from(input).into_iter();

    let TokenTree::Literal(path_lit) = iter.next().expect(ERROR) else { panic!("{ERROR}") };
    let path = format!("{path_lit}").replace("\\", "").replace("\"", "");
    let TokenTree::Punct(punct) = iter.next().expect(ERROR) else { panic!("{ERROR}") };
    assert!(punct.as_char() == ',', "{ERROR}");
    let TokenTree::Literal(prefix_lit) = iter.next().expect(ERROR) else { panic!("{ERROR}") };
    let prefix = format!("{prefix_lit}").replace("\\", "").replace("\"", "");
    assert!(iter.next().is_none(), "{ERROR}");

    let root = PathBuf::from(path);
    if !root.exists() { panic!("path \"{}\" does not exist", root.display()); }
    let mut code = quote::quote!();
    if root.is_file() { file(&prefix, &root, root.clone(), &mut code); }
    else if root.is_dir() { walk_dir(&prefix, &root, root.clone(), &mut code); }

    //println!("{code}");
    proc_macro::TokenStream::from(code)
}

fn file(prefix: &str, root: &PathBuf, path: PathBuf, code: &mut TokenStream)
{
    let path_stripped = path.strip_prefix(root).expect(&format!("{} prefix of {}", root.display(), path.display()));
    let file_path = format!("{prefix}/{}", path_stripped.to_str().unwrap());
    let file_name = path.file_name().unwrap().to_str().unwrap();
    code.extend(quote::quote!(Entry { path: #file_path, name: #file_name }));
}

fn walk_dir(prefix: &str, root: &PathBuf, path: PathBuf, code: &mut TokenStream)
{
    let path_stripped = path.strip_prefix(root).expect(&format!("{} prefix of {}", root.display(), path.display())).to_str().unwrap();
    let dir_path = if path_stripped.is_empty() { format!("{prefix}") } else { format!("{prefix}/{}", path_stripped) };
    let dir_name = path.file_name().unwrap().to_str().unwrap();
    let entry = quote::quote!(Entry { path: #dir_path, name: #dir_name });
    let mut dir_paths = Vec::new();
    let mut file_paths = Vec::new();
    for child in path.read_dir().unwrap()
    {
        let child = child.unwrap();
        let path = child.path();
        if path.is_file() { file_paths.push(path); }
        else if path.is_dir() { dir_paths.push(path); }
    }
    let mut dirs = quote::quote!();
    for dir_path in dir_paths
    {
        walk_dir(prefix, root, dir_path, &mut dirs);
        dirs.extend(quote::quote!(,));
    }
    let mut files = quote::quote!();
    for file_path in file_paths
    {
        file(prefix, root, file_path, &mut files);
        files.extend(quote::quote!(,));
    }
    code.extend(quote::quote!
    (
        Dir
        {
            entry: #entry,
            dirs: vec![#dirs],
            files: vec![#files]
        }
    ));
}
