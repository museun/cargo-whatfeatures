use std::{
    borrow::Cow,
    fmt::Display,
    io::{self, Write},
};

use super::{Style, Theme};

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

    pub fn add_child(&mut self, child: impl Into<Self>) {
        self.children.push(child.into());
    }

    pub fn empty<S: ToString>(data: S) -> Self {
        Self {
            text: data.to_string(),
            children: vec![],
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

impl<T: ToString> From<T> for Node {
    fn from(data: T) -> Self {
        Self {
            text: data.to_string(),
            children: Vec::new(),
        }
    }
}

pub trait Printer {
    fn print<W: Write + ?Sized>(self, writer: &mut W, theme: &Theme) -> io::Result<()>;
}

impl<T: Item> Printer for T {
    fn print<W: Write + ?Sized>(self, writer: &mut W, theme: &Theme) -> io::Result<()> {
        print(self, writer, theme)
    }
}

pub fn print(item: impl Item, writer: &mut (impl Write + ?Sized), theme: &Theme) -> io::Result<()> {
    let appearance = Appearance {
        style: &Style::default(),
        theme,
    };

    return print(&item, writer, "", "", &appearance, 0);

    // impl below
    struct Appearance<'a, 'b> {
        theme: &'a Theme,
        style: &'b Style,
    }

    fn print(
        item: &impl Item,
        writer: &mut (impl Write + ?Sized),
        left: impl Display,
        child: impl Display,
        appearance: &Appearance,
        depth: usize,
    ) -> std::io::Result<()> {
        let Appearance { style, theme } = appearance;

        write!(writer, "{}", theme.tree.paint(left))?;
        item.write(writer)?;
        writeln!(writer)?;

        if let Some((last, children)) = item.children().split_last() {
            let left_prefix = format!("{}{}", child, style.branch);
            let right_prefix = format!("{}{}", child, style.pipe);

            for child in children {
                print(
                    child,
                    writer,
                    &left_prefix,
                    &right_prefix,
                    appearance,
                    depth + 1,
                )?;
            }

            print(
                last,
                writer,
                format!("{}{}", child, style.edge),
                format!("{}{}", child, style.last),
                appearance,
                depth + 1,
            )?;
        }

        Ok(())
    }
}
