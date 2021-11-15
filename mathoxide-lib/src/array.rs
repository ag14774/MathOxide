use crate::storage::Storage;
use crate::views;
use crate::views::ArrayView;

use num_traits::Num;

use std::fmt;

pub struct Array<StorageType, ViewType> {
    storage: StorageType,
    view: ViewType,
}

impl<T, StorageType, ViewType> Array<StorageType, ViewType>
where
    T: Num,
    StorageType: Storage<Stored = T>,
    ViewType: ArrayView,
{
    fn offset(&self) -> usize {
        self.view.offset()
    }
    // fn stride(&self) -> &[usize];
    fn ndims(&self) -> usize {
        self.view.ndims()
    }

    fn size(&self) -> usize {
        self.view.size()
    }

    fn shape(&self) -> &[usize] {
        self.view.shape()
    }
}

impl<T, StorageType, ViewType> fmt::Display for Array<StorageType, ViewType>
where
    T: Num + fmt::Display,
    StorageType: Storage<Stored = T>,
    ViewType: ArrayView,
{
    // TODO: Rewrite this when more helper methods are available
    // Try to match numpy's behavior for ndarray formatting
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = String::new();
        match self.storage.storage_get() {
            Ok(arr) => match self.ndims() {
                2 => {
                    let mut s = String::new();
                    let nrows = self.shape()[0];
                    let ncols = self.shape()[1];
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
                                &arr[start..end]
                                    .iter()
                                    .map(|x| x.to_string())
                                    .collect::<Vec<String>>()
                                    .join(",")
                            )
                            .as_str(),
                        );
                        if row == nrows - 1 {
                            s.push(']');
                        } else {
                            s.push('\n');
                        }
                    }
                    write!(f, "{}", s)
                }
                _ => {
                    write!(
                        f,
                        "[{}]",
                        arr.iter()
                            .map(|x| x.to_string())
                            .collect::<Vec<String>>()
                            .join(",")
                    )
                }
            },
            Err(err) => {
                write!(f, "error while formatting array: {}", err)
            }
        }
    }
}

impl<T, StorageType> Array<StorageType, views::SimpleView>
where
    T: Num,
    StorageType: Storage<Stored = T>,
{
    pub fn zeros<ListType: AsRef<[usize]>>(shape: ListType) -> Self {
        let view = views::SimpleView::new(shape);
        let mut v = Vec::new();
        v.resize_with(view.size(), T::zero);
        let storage = StorageType::from(v);
        Array { storage, view }
    }

    pub fn from_vector(vec: Vec<u32>) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::thread_safe_storage::ThreadSafeStorage;

    #[test]
    fn it_works() {
        let array = Array::<ThreadSafeStorage<u32>, views::SimpleView>::zeros(&[4, 5]);
        println!("{}", array);
    }

    #[test]
    fn create_from_vector() {
        let array =
            Array::<ThreadSafeStorage<u32>, views::SimpleView>::from_vector(vec![1, 2, 3, 4]);
    }
}
