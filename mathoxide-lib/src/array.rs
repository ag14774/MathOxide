use std::fmt;

use num_traits::Num;

use crate::formatter::{ArrayFormatter, VerboseFormatter};
use crate::shape_utils::{infer_shape, ShapeDim};
use crate::storage::Storage;
use crate::views::{ArrayView, ContiguousView};

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
    pub fn storage_offset(&self) -> usize {
        self.view.offset()
    }

    pub fn shape(&self) -> &[usize] {
        self.view.shape()
    }

    pub fn stride(&self) -> &[usize] {
        self.view.stride()
    }

    pub fn ndim(&self) -> usize {
        self.view.ndim()
    }

    pub fn numel(&self) -> usize {
        self.view.numel()
    }

    pub fn is_contiguous(&self) -> bool {
        self.view.is_contiguous()
    }

    pub fn storage_size(&self) -> usize {
        self.storage.storage_len().expect("Cannot get array length")
    }

    pub fn get<ListType: AsRef<[usize]>>(
        &self,
        idx: ListType,
    ) -> Array<StorageType, ContiguousView> {
        let offset = match self.view.try_translate(&idx) {
            Ok(u) => u,
            Err(_) => panic!(
                "Index {:?} is not valid for a view with shape {:?}",
                idx.as_ref(),
                self.shape()
            ),
        };
        let view = ContiguousView::new_with_offset([1], offset);
        Array {
            storage: self.storage.clone(),
            view,
        }
    }
}

impl<T, StorageType, ViewType> Array<StorageType, ViewType>
where
    T: Num + Clone,
    StorageType: Storage<Stored = T>,
    ViewType: ArrayView,
{
    pub fn item(&self) -> StorageType::Stored {
        if self.numel() > 1 {
            panic!("'item()' can only be run on arrays of size 1");
        }
        let offset = self.storage_offset();
        self.storage
            .storage_get()
            .expect("error while getting storage")[offset]
            .clone()
    }
}

impl<T, StorageType> Array<StorageType, ContiguousView>
where
    T: Num,
    StorageType: Storage<Stored = T>,
{
    pub fn zeros<ListType: AsRef<[usize]>>(shape: ListType) -> Self {
        let view = ContiguousView::new(shape);
        let mut v = Vec::new();
        v.resize_with(view.numel(), T::zero);
        let storage = StorageType::from(v);
        Array { storage, view }
    }

    pub fn reshape<SizeType, ListType>(&self, shape: ListType) -> Self
    where
        SizeType: Copy,
        ShapeDim: From<SizeType>,
        ListType: AsRef<[SizeType]>,
    {
        let new_shape = infer_shape(shape, self.numel()).expect("error while inferring shape");
        if new_shape.iter().product::<usize>() != self.numel() {
            panic!(
                "{:?} not a valid shape for array of size {}",
                new_shape,
                self.numel()
            );
        }
        let view = ContiguousView::new_with_offset(new_shape, self.storage_offset());
        Array {
            storage: self.storage.clone(),
            view,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::thread_safe_storage::ThreadSafeStorage;

    #[test]
    fn check_all_zeros() {
        let array = Array::<ThreadSafeStorage<u32>, ContiguousView>::zeros(&[4, 5]);
        let array = array.reshape([-1isize]);
        for i in 0..20 {
            assert_eq!(array.get([i]).item(), 0);
        }
    }

    #[test]
    fn item_panic() {
        let array = Array::<ThreadSafeStorage<u32>, ContiguousView>::zeros(&[4, 5]);
        let result = std::panic::catch_unwind(|| array.item());
        assert!(result.is_err());
    }

    #[test]
    fn check_2d_nrows_format() {
        let array = Array::<ThreadSafeStorage<u32>, ContiguousView>::zeros(&[4, 5]);
        let formatted = array.to_string();
        assert_eq!(formatted.split('\n').count(), array.shape()[0]);
    }

    #[test]
    fn reshape_2d_to_1d() {
        let array = Array::<ThreadSafeStorage<u32>, ContiguousView>::zeros(&[2, 3]);
        let row_vec = array.reshape([6usize]);
        assert_eq!(row_vec.shape(), &[6]);
    }

    #[test]
    fn reshape_2d_to_1d_inferred() {
        let array = Array::<ThreadSafeStorage<u32>, ContiguousView>::zeros(&[2, 3]);
        let row_vec = array.reshape([-1isize]);
        assert_eq!(row_vec.shape(), &[6]);
    }
}
