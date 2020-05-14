use std::{
    borrow::Cow,
    fmt::Display,
    io::{self, Write},
};
use yansi::*;

#[derive(Debug, Copy, Clone)]
struct Style {
    pipe: &'static str,
    branch: &'static str,
    edge: &'static str,
    right: &'static str,
    empty: &'static str,
}

impl Default for Style {
    #[inline]
    fn default() -> Self {
        Self {
            pipe: "│ ",
            edge: "└─ ",
            branch: "├─ ",
            right: "─ ",
            empty: "   ",
        }
    }
}

pub trait Item: Clone {
    type Child: Item;
    fn write<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<()>;
    fn children(&self) -> Cow<[Self::Child]>;
}

#[derive(Debug, Clone)]
pub struct Node {
    pub text: String,
    pub children: Vec<Self>,
}

impl Node {
    pub fn new<S, I>(data: S, children: I) -> Self
    where
        S: ToString,
        I: IntoIterator,
        I::Item: Into<Self>,
    {
        Self {
            text: data.to_string(),
            children: children.into_iter().map(Into::into).collect(),
        }
    }

    pub fn empty<S>(data: S) -> Self
    where
        S: ToString,
    {
        Self {
            text: data.to_string(),
            children: vec![],
        }
    }
}

impl<T: ToString> From<T> for Node {
    fn from(data: T) -> Self {
        Self {
            text: data.to_string(),
            children: Vec::new(),
        }
    }
}

impl Item for Node {
    type Child = Self;

    fn write<W: Write + ?Sized>(&self, writer: &mut W) -> io::Result<()> {
        write!(writer, "{}", self.text)
    }

    fn children(&self) -> Cow<[Self::Child]> {
        self.children[..].into()
    }
}

pub trait Printer {
    fn print<W: Write + ?Sized>(self, writer: &mut W) -> io::Result<()>;
}

impl<T> Printer for T
where
    T: Item,
{
    fn print<W: Write + ?Sized>(self, writer: &mut W) -> io::Result<()> {
        print(self, writer)
    }
}

pub fn print<I, W>(item: I, writer: &mut W) -> io::Result<()>
where
    I: Item,
    W: Write + ?Sized,
{
    fn print<I, W, L, C>(
        item: &I,
        writer: &mut W,
        left: L,
        child: C,
        style: &Style,
        depth: usize,
    ) -> std::io::Result<()>
    where
        I: Item,
        W: Write + ?Sized,
        L: Display,
        C: Display,
    {
        write!(writer, "{}", Paint::cyan(left))?;
        item.write(writer)?;
        writeln!(writer)?;

        if let Some((last, children)) = item.children().split_last() {
            let left_prefix = format!("{}{}", child, style.branch);
            let right_prefix = format!("{}{}", child, style.pipe);

            for child in children {
                print(child, writer, &left_prefix, &right_prefix, style, depth + 1)?;
            }

            // TODO we can get rid of these 2 allocations if we change the signature
            let left_prefix = format!("{}{}", child, style.edge);
            let right_prefix = format!("{}{}", child, style.empty);
            print(last, writer, left_prefix, right_prefix, style, depth + 1)?;
        }

        Ok(())
    }

    print(&item, writer, "", "", &Style::default(), 0)
}
