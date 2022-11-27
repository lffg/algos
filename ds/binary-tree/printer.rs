use std::fmt::{Debug, Display, Formatter, Result, Write};

use crate::BinaryTree;

pub const INDENT_SIZE: usize = 4;

pub struct BinaryTreePrinter<'a, T>(pub &'a BinaryTree<T>);

impl<'a, T> Display for BinaryTreePrinter<'a, T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let BinaryTreePrinter(tree) = *self;
        if tree.is_empty() {
            writeln!(f, "(empty binary tree)")
        } else {
            go(f, tree, 0)
        }
    }
}

fn go<T>(f: &mut Formatter<'_>, tree: &BinaryTree<T>, depth: usize) -> Result
where
    T: Debug,
{
    let Some(value) = &tree.value else {
        return Ok(());
    };

    repeat(f, ' ', depth * INDENT_SIZE)?;
    writeln!(f, "{value:?}")?;

    if let Some(left) = &tree.left {
        go(f, left, depth + 1)?;
    }
    if let Some(right) = &tree.right {
        go(f, right, depth + 1)?;
    }

    Ok(())
}

fn repeat(f: &mut Formatter<'_>, c: char, count: usize) -> Result {
    for _ in 0..count {
        f.write_char(c)?;
    }
    Ok(())
}
