use std::fmt;

use crate::views::ArrayView;
use num_traits::Num;

pub trait ArrayFormatter {
    fn format(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub struct VerboseFormatter<'a, T, ViewType> {
    storage: &'a [T],
    view: &'a ViewType,
}

impl<'a, T: Num, ViewType: ArrayView> VerboseFormatter<'a, T, ViewType> {
    // How to do this with AsRef?
    pub fn new(storage: &'a [T], view: &'a ViewType) -> Self {
        VerboseFormatter::<'a, T, ViewType> { storage, view }
    }
}

impl<'a, T: Num + fmt::Display, ViewType: ArrayView> ArrayFormatter
    for VerboseFormatter<'a, T, ViewType>
{
    // TODO: Rewrite this when more helper methods are available
    // Try to match numpy's behavior for ndarray formatting
    fn format(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.view.ndims() {
            2 => {
                let mut s = String::new();
                let nrows = self.view.shape()[0];
                let ncols = self.view.shape()[1];
                for row in 0usize..nrows {
                    let start = self.view.translate([row, 0]);
                    let end = self.view.translate([row + 1, 0]);
                    s.push(match row {
                        0 => '[',
                        _ => ' ',
                    });
                    s.push_str(
                        format!(
                            "[{}]",
                            &self.storage[start..end]
                                .iter()
                                .map(|x| x.to_string())
                                .collect::<Vec<String>>()
                                .join(",")
                        )
                        .as_str(),
                    );
                    s.push(if row == nrows - 1 { ']' } else { '\n' });
                }
                write!(f, "{}", s)
            }
            _ => {
                write!(
                    f,
                    "[{}]",
                    self.storage
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }
}

// TODO: ADD TESTS
