use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};

use crate::thread_safe_storage::{
    ThreadSafeStorage, ThreadSafeStorageGuard, ThreadSafeStorageGuardMut,
};
use crate::thread_unsafe_storage::ThreadUnsafeStorage;

pub trait Storage: From<Vec<Self::Stored>> {
    type Stored;
    type Guard<'a>: Deref<Target = [Self::Stored]>
    where
        Self: 'a;
    type GuardMut<'a>: DerefMut<Target = [Self::Stored]>
    where
        Self: 'a;

    fn storage_get<'a>(&'a self) -> Result<Self::Guard<'a>, &str>;
    fn storage_get_mut<'a>(&'a mut self) -> Result<Self::GuardMut<'a>, &str>;
    fn storage_len(&self) -> Result<usize, &str>;
}

impl<T> Storage for ThreadSafeStorage<T> {
    type Stored = T;
    type Guard<'a>
    where
        Self: 'a,
    = ThreadSafeStorageGuard<'a, T>;
    type GuardMut<'a>
    where
        Self: 'a,
    = ThreadSafeStorageGuardMut<'a, T>;

    fn storage_get<'a>(&'a self) -> Result<Self::Guard<'a>, &str> {
        self.get()
    }

    fn storage_get_mut<'a>(&'a mut self) -> Result<Self::GuardMut<'a>, &str> {
        self.get_mut()
    }

    fn storage_len(&self) -> Result<usize, &str> {
        self.len()
    }
}

impl<T> Storage for ThreadUnsafeStorage<T> {
    type Stored = T;
    type Guard<'a>
    where
        Self: 'a,
    = Ref<'a, [T]>;
    type GuardMut<'a>
    where
        Self: 'a,
    = RefMut<'a, [T]>;

    fn storage_get<'a>(&'a self) -> Result<Self::Guard<'a>, &str> {
        Ok(Ref::map(self.get()?, |r| r.as_slice()))
    }

    fn storage_get_mut<'a>(&'a mut self) -> Result<Self::GuardMut<'a>, &str> {
        Ok(RefMut::map(self.get_mut()?, |r| r.as_mut_slice()))
    }

    fn storage_len(&self) -> Result<usize, &str> {
        self.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut storage = ThreadSafeStorage::from(vec![1, 2, 3, 4, 5]);

        assert_eq!(storage.storage_get().unwrap()[0], 1);

        storage.storage_get_mut().unwrap()[0] = 100;

        assert_eq!(storage.storage_get().unwrap()[0], 100);
    }

    fn test_thread_safe_generics<'a, S: 'static + Storage<Stored = u32> + Send>(mut storage: S) {
        std::thread::spawn(move || {
            println!("{}", storage.storage_get_mut().unwrap()[0]);
            storage.storage_get_mut().unwrap()[0] = 101;
        });
    }

    fn test_thread_unsafe_generics<'a, S: Storage<Stored = u32>>(storage: &mut S) {
        storage.storage_get_mut().unwrap()[0] = 100;
    }

    #[test]
    fn thread_safe_generics() {
        let mut storage = ThreadSafeStorage::from(vec![1, 2, 3, 4, 5]);
        test_thread_unsafe_generics(&mut storage);
        assert_eq!(storage.storage_get().unwrap()[0], 100);

        test_thread_safe_generics(storage.clone());
        std::thread::sleep(std::time::Duration::from_secs(1));
        assert_eq!(storage.storage_get().unwrap()[0], 101);
    }

    #[test]
    fn thread_unsafe_generics() {
        let mut storage = ThreadUnsafeStorage::from(vec![1, 2, 3, 4, 5]);
        test_thread_unsafe_generics(&mut storage);
        assert_eq!(storage.storage_get().unwrap()[0], 100);
    }
}
