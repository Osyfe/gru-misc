#[derive(Clone, Copy)]
pub struct Entry
{
    pub path: &'static str,
    pub name: &'static str
}

#[derive(Clone)]
pub struct Dir
{
    pub entry: Entry,
    pub dirs: Vec<Dir>,
    pub files: Vec<Entry>
}

pub use file_tree_derive::tree;
