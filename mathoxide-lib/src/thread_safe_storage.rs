use std::sync::{Arc, Mutex, MutexGuard};

pub(crate) struct ThreadSafeStorageGuard<'a, T> {
    _guard: MutexGuard<'a, Vec<T>>,
    r: *const [T],
}

impl<'a, T> ThreadSafeStorageGuard<'a, T> {
    fn new(guard: MutexGuard<'a, Vec<T>>) -> Self {
        let r = guard.as_slice() as *const [T];
        Self { _guard: guard, r }
    }
}

impl<'a, T> std::ops::Deref for ThreadSafeStorageGuard<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // Safety: If Self is alive, then _guard is alive, and thus we hold
        // a lock to the underlying storage. Additionally r was taken through
        // as_slice, and thus points to a valid memory region.
        unsafe { self.r.as_ref().unwrap() }
    }
}

pub(crate) struct ThreadSafeStorageGuardMut<'a, T> {
    _guard: MutexGuard<'a, Vec<T>>,
    r: *mut [T],
}

impl<'a, T> ThreadSafeStorageGuardMut<'a, T> {
    fn new(mut guard: MutexGuard<'a, Vec<T>>) -> Self {
        let r = guard.as_mut_slice() as *mut [T];
        Self { _guard: guard, r }
    }
}

impl<'a, T> std::ops::Deref for ThreadSafeStorageGuardMut<'a, T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        // Safety: See safety comment for Deref implementation of ThreadSafeStorageGuard
        unsafe { self.r.as_ref().unwrap() }
    }
}

impl<'a, T> std::ops::DerefMut for ThreadSafeStorageGuardMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: See safety comment for Deref implementation of ThreadSafeStorageGuard
        // Additionally, since we hold a lock to the underlying storage, it is safe to
        // give a mutable reference.
        unsafe { self.r.as_mut().unwrap() }
    }
}

#[derive(Clone)]
pub(crate) struct ThreadSafeStorage<T> {
    data: Arc<Mutex<Vec<T>>>,
}

impl<T> ThreadSafeStorage<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self {
            data: Arc::new(Mutex::new(v)),
        }
    }

    pub fn get(&self) -> Result<ThreadSafeStorageGuard<T>, &str> {
        Ok(ThreadSafeStorageGuard::new(
            self.data.lock().map_err(|_| "Mutex was poisoned")?,
        ))
    }

    pub fn get_mut(&mut self) -> Result<ThreadSafeStorageGuardMut<T>, &str> {
        Ok(ThreadSafeStorageGuardMut::new(
            self.data.lock().map_err(|_| "Mutex was poisoned")?,
        ))
    }

    pub fn len(&self) -> Result<usize, &str> {
        self.data
            .lock()
            .map_err(|_| "Mutex was poisoned")
            .map(|v| v.len())
    }
}

impl<T> From<Vec<T>> for ThreadSafeStorage<T> {
    fn from(val: Vec<T>) -> Self {
        ThreadSafeStorage::new(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![1, 2, 3, 4];
        let v1 = ThreadSafeStorage::new(v);
        let v2 = v1.clone();

        assert_eq!(v1.get().unwrap()[2], 3);
        assert_eq!(v2.get().unwrap()[2], 3);
    }

    #[test]
    fn it_works_with_mut() {
        let v = vec![1, 2, 3, 4];
        let mut v1 = ThreadSafeStorage::new(v);
        let mut v2 = v1.clone();

        v2.get_mut().unwrap()[3] = 20;
        v1.get_mut().unwrap()[3] = 13;

        let r = v1.get_mut().unwrap();
        assert_eq!(r[3], 13);
    }

    #[test]
    fn size_works() {
        let v = vec![1, 2, 3, 4];
        let v = ThreadSafeStorage::new(v);

        assert_eq!(v.len().unwrap(), 4);
    }
}
