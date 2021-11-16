use std::fmt;

use num_traits::Num;

use crate::formatter::{ArrayFormatter, VerboseFormatter};
use crate::storage::Storage;
use crate::views;
use crate::views::ArrayView;

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

    // TODO: fn stride(&self) -> &[usize];

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.storage.storage_get() {
            // Difference between &[..] and as_slice? Why as_slice not working?
            Ok(arr) => VerboseFormatter::<'_, T, ViewType>::new(&arr[..], &self.view).format(f),
            Err(err) => write!(f, "error while formatting array: {}", err),
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
