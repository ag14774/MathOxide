use num_traits::Num;

use crate::array::Array;
use crate::storage::Storage;
use crate::thread_unsafe_storage::ThreadUnsafeStorage;
use crate::utils::IndexIteration;
use crate::views::{ArrayView, ContiguousView};

impl<Stored, StorageType, ViewType, RhsStored, RhsStorage, RhsViewType, OutputStored>
    std::ops::Add<&Array<RhsStorage, RhsViewType>> for &Array<StorageType, ViewType>
where
    RhsStorage: Storage<Stored = RhsStored>,
    RhsStored: Num,
    RhsViewType: ArrayView,
    Stored: Num,
    StorageType: Storage<Stored = Stored>,
    ViewType: ArrayView,
    OutputStored: Num,
    for<'a> &'a Stored: std::ops::Add<&'a RhsStored, Output = OutputStored>,
{
    type Output = Array<ThreadUnsafeStorage<OutputStored>, ContiguousView>;

    fn add(self, rhs: &Array<RhsStorage, RhsViewType>) -> Self::Output {
        if self.shape() != rhs.shape() {
            panic!("can't add arrays of different dimentionality");
        }

        let mut result: Self::Output = Array::zeros(self.shape());
        let mut index_iteration = IndexIteration::row_major(self.shape());

        let lhs_storage = self.storage.storage_get().unwrap();
        let rhs_storage = rhs.storage.storage_get().unwrap();
        let mut result_storage = result.storage.storage_get_mut().unwrap();

        while let Some(dim_index) = index_iteration.next() {
            let res_index = result.view.translate(dim_index);
            let lhs_index = self.view.translate(dim_index);
            let rhs_index = rhs.view.translate(dim_index);

            let sum = &lhs_storage[lhs_index] + &rhs_storage[rhs_index];
            result_storage[res_index] = sum;
        }
        drop(result_storage);

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_addition() {
        let linear = Array::<ThreadUnsafeStorage<u64>, ContiguousView>::zeros(&[4, 5]);
        let mut reshaped = linear.reshape([-1isize]);
        for i in 0..20 {
            reshaped.storage.storage_get_mut().unwrap()[i] = i as u64;
        }

        let mut ones = Array::<ThreadUnsafeStorage<u64>, ContiguousView>::zeros(&[4, 5]);
        for i in 0..20 {
            ones.storage.storage_get_mut().unwrap()[i] = 1 as u64;
        }

        let sum = &linear + &ones;
        let array = sum.reshape([-1isize]);
        for i in 0..20 {
            assert_eq!(array.get([i]).item(), 1 + i as u64);
        }
        assert_eq!(sum.shape(), &[4, 5]);
    }

    #[test]
    fn panic_on_different_shape() {
        let result = std::panic::catch_unwind(|| {
            let array1 = Array::<ThreadUnsafeStorage<u32>, ContiguousView>::zeros(&[4, 5]);
            let array2 = Array::<ThreadUnsafeStorage<u32>, ContiguousView>::zeros(&[7, 5]);
            let _ = &array1 + &array2;
        });
        assert!(result.is_err());
    }
}
