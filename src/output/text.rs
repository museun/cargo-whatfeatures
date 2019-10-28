use crate::output;

use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::{Result, Write};

use yansi::{Color, Paint};

// TODO nake this configurable
const MAX_SIZE: usize = 80;

macro_rules! simple_colors {
    ($($ident:ident => $color:ident),* $(,)?) => {
        $( #[allow(dead_code)] #[inline]
            fn $ident(data: impl Display) -> impl Display { Paint::new(data).fg(Color::$color) }
        )*
    };
}

simple_colors! {
    red => Red,
    yellow => Yellow,
    green => Green,
    magenta => Magenta,
    cyan => Cyan,
    blue => Blue,
}

pub trait RenderAsText {
    fn render<W: Write>(&self, output: &mut W) -> Result<()>;
}

impl<T> RenderAsText for T
where
    T: TextRender,
{
    fn render<W: Write>(&self, output: &mut W) -> Result<()> {
        self.render(output, 0, &mut Default::default())
    }
}

pub trait TextRender {
    fn render<W: Write>(&self, output: &mut W, depth: usize, next: &mut State) -> Result<()>;
}

impl TextRender for crate::error::UserError {
    fn render<W: Write>(&self, output: &mut W, _depth: usize, _next: &mut State) -> Result<()> {
        use crate::error::UserError::*;

        match self {
            CannotLookup {
                name,
                version: Some(version),
                ..
            } => writeln!(
                output,
                "{}: cannot lookup '{}/{}'",
                red("error"),
                name,
                version
            ),
            CannotLookup { name, .. } => writeln!(
                output,
                "{}: cannot lookup '{}'. no version found",
                red("error"),
                green(&name)
            ),
            NoVersions { name } => writeln!(
                output,
                "{}: no versions published for '{}'",
                red("error"),
                green(&name),
            ),
            InvalidVersion { name, version } => writeln!(
                output,
                "{}: invalid version '{}' for '{}'",
                red("error"),
                yellow(&version),
                green(&name),
            ),
        }
    }
}

impl<'a> TextRender for output::SimpleModel<'a> {
    fn render<W: Write>(&self, output: &mut W, _: usize, _: &mut State) -> Result<()> {
        if self.yanked {
            write!(output, "{}: ", red("yanked"))?;
        }
        writeln!(output, "{}/{}", self.name, yellow(&self.version))
    }
}

impl<'a> TextRender for output::FeaturesModel<'a> {
    fn render<W: Write>(&self, output: &mut W, depth: usize, next: &mut State) -> Result<()> {
        if let State::First = next {
            <_ as TextRender>::render(&self.simple, output, depth, &mut next.advance())?;
        }

        let map: BTreeMap<_, _> = self
            .version
            .features
            .iter()
            .map(|(k, v)| (k.as_str(), v))
            .collect();

        let pad = " ".repeat(depth + 2);

        if let Some(default) = map.get("default") {
            if !default.is_empty() {
                write!(output, "{}{}: ", pad, cyan("default"))?;
                let len = pad.len() + "default: ".len();
                let (mut max, list) =
                    BoundingBox::new(default.as_slice()).display(MAX_SIZE.saturating_sub(len));
                for (i, line) in list.iter().enumerate() {
                    if i > 0 {
                        write!(output, "{: <len$}", " ", len = len)?;
                    }
                    for word in line {
                        max -= 1;
                        write!(output, "{}", word)?;
                        if max > 0 {
                            write!(output, ", ")?;
                        }
                    }
                    writeln!(output)?;
                }
            } else {
                writeln!(output, "{}{}", pad, magenta("no default features"))?
            }
        } else {
            writeln!(output, "{}{}", pad, magenta("no default features"))?
        }

        for (&k, v) in &map {
            if k == "default" {
                continue;
            }

            write!(output, "{}{}", pad, cyan(k))?;
            if v.is_empty() {
                writeln!(output)?;
                continue;
            }

            write!(output, ": ")?;
            let len = pad.len() + k.len() + 2;
            let (mut max, list) =
                BoundingBox::new(v.as_slice()).display(MAX_SIZE.saturating_sub(len));
            for (i, line) in list.iter().enumerate() {
                if i > 0 {
                    write!(output, "{: <len$}", " ", len = len)?;
                }
                for word in line {
                    max -= 1;
                    write!(output, "{}", word)?;
                    if max > 0 {
                        write!(output, ", ")?;
                    }
                }
                writeln!(output)?;
            }
        }

        Ok(())
    }
}

impl<'a> TextRender for output::DependencyModel<'a> {
    fn render<W: Write>(&self, output: &mut W, depth: usize, next: &mut State) -> Result<()> {
        if let State::First = next {
            <_ as TextRender>::render(&self.simple, output, depth, &mut next.advance())?;
        }

        let pad = " ".repeat(depth + 2);
        let pad2 = " ".repeat(depth + 4);
        let pad3 = " ".repeat(depth + 6);

        if self.dependencies.is_empty() {
            return writeln!(output, "{}{}", pad, red("no dependencies"));
        }

        let mut sorted: BTreeMap<_, _> = self.dependencies.clone().into_iter().collect();
        let widths = columns(
            3,
            sorted
                .iter_mut()
                .map(|(kind, list)| {
                    list.sort_unstable_by(|l, r| l.crate_id.cmp(&r.crate_id));
                    (kind, list)
                })
                .flat_map(|(_, list)| list.iter())
                .map(|dep| {
                    vec![
                        dep.crate_id.as_str(),
                        if dep.optional { "  " } else { "" },
                        dep.req.as_str(),
                    ]
                }),
        );

        struct Widths {
            name: usize,
            opt: usize,
            req: usize,
        }

        let widths = Widths {
            name: widths[0],
            opt: widths[1],
            req: widths[2],
        };

        use crate::crates::DependencyKind::*;
        for (kind, deps) in sorted {
            match kind {
                Normal => writeln!(output, "{}{}", pad, magenta("normal"))?,
                Dev => writeln!(output, "{}{}", pad, blue(&"dev"))?,
                Build => writeln!(output, "{}{}", pad, red(&"build"))?,
            };

            let (ok, opt): (Vec<_>, Vec<_>) = deps.into_iter().partition(|k| !k.optional);

            for dep in ok {
                let target = dep
                    .target
                    .as_ref()
                    .map(|target| format!("if {}", cyan(target)))
                    .unwrap_or_default();

                writeln!(
                    output,
                    "{}{: <name_width$} = {: <req_width$} {}",
                    pad2, // TODO better naming
                    &dep.crate_id,
                    yellow(&dep.req),
                    target,
                    name_width = widths.name + widths.opt,
                    req_width = widths.req,
                )?;

                if dep.default_features && !dep.features.is_empty() {
                    write!(output, "{}- {}: ", pad3, blue("features"))?;
                    let len = pad3.len() + 2 + "features: ".len();
                    let (mut max, list) =
                        BoundingBox::new(&dep.features).display(MAX_SIZE.saturating_sub(len));

                    for (i, line) in list.iter().enumerate() {
                        if i > 0 {
                            write!(output, "{: <len$}", " ", len = len)?;
                        }
                        for word in line {
                            max -= 1;
                            write!(output, "{}", word)?;
                            if max > 0 {
                                write!(output, ", ")?;
                            }
                        }
                        writeln!(output)?;
                    }
                }
            }

            if opt.is_empty() {
                continue;
            }

            writeln!(output, "{}{}", pad2, cyan("optional"))?;
            for dep in opt {
                let target = dep
                    .target
                    .as_ref()
                    .map(|target| format!("if {}", cyan(target)))
                    .unwrap_or_default();

                writeln!(
                    output,
                    "{}{: <name_width$} = {: <req_width$} {}",
                    pad3, // TODO better naming
                    &dep.crate_id,
                    yellow(&dep.req),
                    target,
                    name_width = widths.name,
                    req_width = widths.req,
                )?;

                if dep.default_features && !dep.features.is_empty() {
                    write!(output, "{}{}- {}: ", pad, pad2, blue("features"))?;
                    let len = pad.len() + pad2.len() + 2 + "features: ".len();
                    let (mut max, list) =
                        BoundingBox::new(&dep.features).display(MAX_SIZE.saturating_sub(len));

                    for (i, line) in list.iter().enumerate() {
                        if i > 0 {
                            write!(output, "{: <len$}", " ", len = len)?;
                        }
                        for word in line {
                            max -= 1;
                            write!(output, "{}", word)?;
                            if max > 0 {
                                write!(output, ", ")?;
                            }
                        }
                        writeln!(output)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl<'a> TextRender for output::CompositeModel<'a> {
    fn render<W: Write>(&self, output: &mut W, depth: usize, next: &mut State) -> Result<()> {
        if let State::First = next {
            <_ as TextRender>::render(&self.simple, output, depth, &mut next.advance())?;
        }
        // TODO don't do this by 2/4 but rather by ceil(d * 1.5) and ceil(d * 2.0)
        let depth = depth + 2;
        let pad = " ".repeat(depth);

        writeln!(output, "{}{}", pad, green("features"))?;
        <_ as TextRender>::render(&self.features, output, depth, &mut next.advance())?;

        if self.dependencies.dependencies.is_empty() {
            writeln!(output, "{}{}", pad, red("no dependencies"))
        } else {
            writeln!(output, "{}{}", pad, green("dependencies"))?;
            <_ as TextRender>::render(&self.dependencies, output, depth, &mut next.advance())
        }
    }
}

impl<'a> TextRender for output::SimpleListModel<'a> {
    fn render<W: Write>(&self, output: &mut W, depth: usize, next: &mut State) -> Result<()> {
        for model in &self.simple_list {
            <_ as TextRender>::render(model, output, depth, &mut next.reset())?;
        }
        Ok(())
    }
}

impl<'a> TextRender for output::FeaturesListModel<'a> {
    fn render<W: Write>(&self, output: &mut W, depth: usize, next: &mut State) -> Result<()> {
        for model in &self.features_list {
            <_ as TextRender>::render(model, output, depth, &mut next.reset())?;
        }
        Ok(())
    }
}

trait Len {
    fn length(&self) -> usize;
}

macro_rules! impl_len_for {
    ($($ty:ty),* $(,)?) => {
        $(impl Len for $ty {
            #[inline]
            fn length(&self) -> usize {
                self.len()
            }
        })*
    };
}

impl_len_for!(str, &str, String,);

struct BoundingBox<'a, T> {
    buf: &'a [T],
}

impl<'a, T: Len> BoundingBox<'a, T> {
    pub fn new(buf: &'a [T]) -> Self {
        Self { buf }
    }

    pub fn display(self, width: usize) -> (usize, Vec<Vec<&'a T>>) {
        let mut vec = vec![];
        let mut temp = vec![];
        let mut count = 0;

        let mut budget = width;
        for n in self.buf {
            if n.length() > budget {
                if !temp.is_empty() {
                    vec.push(std::mem::replace(&mut temp, vec![]));
                }
                budget = width;
            }
            budget = budget.saturating_sub(n.length());
            temp.push(n);
            count += 1;
        }
        if !temp.is_empty() {
            vec.push(temp)
        }
        (count, vec)
    }
}

/// Output state
#[derive(Copy, Clone, Debug)]
pub enum State {
    /// First time this is being outputted
    First,
    /// The next time its being outputted
    Next,
}

impl Default for State {
    #[inline]
    fn default() -> Self {
        State::First
    }
}

impl State {
    #[inline]
    fn reset(&mut self) -> State {
        std::mem::replace(self, Default::default());
        *self
    }
    #[inline]
    fn advance(&mut self) -> State {
        std::mem::replace(self, State::Next);
        *self
    }
}

#[inline]
fn width(old: usize, s: &str) -> usize {
    std::cmp::max(old, s.chars().map(|c| c.len_utf8()).sum())
}

fn columns<'a, I, S>(expected: usize, iter: I) -> Vec<usize>
where
    I: Iterator<Item = S>,
    S: IntoIterator<Item = &'a str>,
{
    let mut result = vec![0; expected];
    for element in iter {
        for (size, res) in result
            .iter_mut()
            .zip(element)
            .map(|(l, r)| (width(*l, &r), l))
        {
            *res = std::cmp::max(*res, size);
        }
    }
    debug_assert!(result.len() == expected);
    result
}
