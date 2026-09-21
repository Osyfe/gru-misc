extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree, Ident};
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
    let name_core = root.file_name().unwrap().to_str().unwrap();
    let name_id = quote::format_ident!("{}_id", name_core);
    let name_enum = quote::format_ident!("{}_enum", name_core);

    let mut paths = Vec::new();
    let mut names = Vec::new();
    let tree = if root.is_file() { panic!("choose a directory") }
    else if root.is_dir() { walk_dir(&prefix, &root, root.clone(), &name_id, &mut paths, &mut names) }
    else { unreachable!("unknown path type") };
    let n_paths = paths.len();

    let code = quote::quote!
    (
        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct #name_id(usize);

        impl #name_id
        {
            const ALL: &[&str] = &[#(#paths),*];
            pub fn enumerate() -> impl Iterator<Item = Self> { (0..#n_paths).map(|i| Self(i)) }
            pub const fn path(self) -> &'static str { Self::ALL[self.0] }
            pub const fn idx(self) -> usize { self.0 }
        }

        #tree

        #[derive(Clone, Copy, PartialEq, Eq, Hash)]
        #[repr(usize)]
        pub enum #name_enum
        {
            #(#names),*
        }

        impl From<#name_enum> for #name_id
        {
            fn from(e: #name_enum) -> #name_id
            {
                #name_id(e as usize)
            }
        }
    );

    proc_macro::TokenStream::from(code)
}

fn file(prefix: &str, root: &PathBuf, path: PathBuf, ty: &Ident, paths: &mut Vec<String>, names: &mut Vec<Ident>) -> TokenStream
{
    let path_stripped = path.strip_prefix(root).expect(&format!("{} prefix of {}", root.display(), path.display()));
    let file_path = format!("{prefix}/{}", path_stripped.to_str().unwrap().replace("\\", "/"));
    let file_name = quote::format_ident!("{}", path.file_name().unwrap().to_str().unwrap().replace(".", "__"));
    let enum_name = quote::format_ident!("{}", path_stripped.to_str().unwrap().replace(['\\', '/', '.'], "__"));

    let id = paths.len();
    paths.push(file_path);
    names.push(enum_name);
    quote::quote!(pub const #file_name: #ty = #ty(#id);)
}

fn walk_dir(prefix: &str, root: &PathBuf, path: PathBuf, ty: &Ident, paths: &mut Vec<String>, names: &mut Vec<Ident>) -> TokenStream
{
    //let path_stripped = path.strip_prefix(root).expect(&format!("{} prefix of {}", root.display(), path.display())).to_str().unwrap();
    //let dir_path = if path_stripped.is_empty() { format!("{prefix}") } else { format!("{prefix}/{}", path_stripped) };
    let dir_name = quote::format_ident!("{}", path.file_name().unwrap().to_str().unwrap());

    let children = path.read_dir().unwrap().map(|child|
    {
        let child = child.unwrap();
        let path = child.path();
        if path.is_file() { file(prefix, root, path, ty, paths, names) }
        else if path.is_dir() { walk_dir(prefix, root, path, ty, paths, names) }
        else { unreachable!("unknown path type") }
    });

    quote::quote!
    (
        pub mod #dir_name
        {
            use super::#ty;

            #(#children)*
        }
    )
}
