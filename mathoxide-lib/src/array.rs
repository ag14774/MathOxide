use crate::storage::Storage;

pub struct Array<StorageType> {
    storage: StorageType,
}

impl<T: Default + Clone, StorageType: Storage<Stored = T>> Array<StorageType> {
    pub fn zeros(size: usize) -> Self {
        let mut v = Vec::new();
        v.resize(size, T::default());
        let storage = StorageType::from(v);
        Array { storage }
    }

    pub fn from_vector(vec: Vec<u32>) {}
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::thread_safe_storage::ThreadSafeStorage;

    #[test]
    fn it_works() {
        let array = Array::<ThreadSafeStorage<u32>>::zeros(10);
    }

    #[test]
    fn create_from_vector() {
        let array = Array::<ThreadSafeStorage<u32>>::from_vector(vec![1, 2, 3, 4]);
    }
}
