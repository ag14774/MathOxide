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

impl<T, StorageType, ViewType> fmt::Display for Array<StorageType, ViewType>
where
    T: Num + fmt::Display,
    StorageType: Storage<Stored = T>,
    ViewType: ArrayView,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.storage.storage_get() {
            Ok(arr) => VerboseFormatter::<'_, T, ViewType>::new(&arr[..], &self.view).format(f),
            Err(err) => write!(f, "error while formatting array: {}", err),
        }
    }
}

impl<T, StorageType, ViewType> Array<StorageType, ViewType>
where
    T: Num,
    StorageType: Storage<Stored = T>,
    ViewType: ArrayView,
{
    pub fn offset(&self) -> usize {
        self.view.offset()
    }

    pub fn ndims(&self) -> usize {
        self.view.ndims()
    }

    pub fn size(&self) -> usize {
        self.view.size()
    }

    pub fn shape(&self) -> &[usize] {
        self.view.shape()
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
    fn check_2d_nrows_format() {
        let array = Array::<ThreadSafeStorage<u32>, views::SimpleView>::zeros(&[4, 5]);
        let formatted = array.to_string();
        assert_eq!(formatted.split('\n').count(), array.shape()[0]);
    }
}
