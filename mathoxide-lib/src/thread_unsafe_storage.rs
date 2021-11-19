use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct ThreadUnsafeStorage<T> {
    data: Rc<RefCell<Vec<T>>>,
}

impl<T> ThreadUnsafeStorage<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self {
            data: Rc::new(RefCell::new(v)),
        }
    }

    pub fn get(&self) -> Result<Ref<Vec<T>>, &str> {
        self.data
            .try_borrow()
            .map_err(|_| "Array is borrowed immutable")
    }

    pub fn get_mut(&mut self) -> Result<RefMut<Vec<T>>, &str> {
        self.data
            .try_borrow_mut()
            .map_err(|_| "Array is already borrowed")
    }

    pub fn len(&self) -> Result<usize, &str> {
        self.data
            .try_borrow()
            .map_err(|_| "Array is borrowed immutable")
            .map(|v| v.len())
    }
}

impl<T> From<Vec<T>> for ThreadUnsafeStorage<T> {
    fn from(val: Vec<T>) -> Self {
        ThreadUnsafeStorage::new(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = vec![1, 2, 3, 4];
        let v1 = ThreadUnsafeStorage::new(v);
        let v2 = v1.clone();

        assert_eq!(v1.get().unwrap()[2], 3);
        assert_eq!(v2.get().unwrap()[2], 3);
    }

    #[test]
    fn it_works_with_mut() {
        let v = vec![1, 2, 3, 4];
        let mut v1 = ThreadUnsafeStorage::new(v);
        let mut v2 = v1.clone();

        v2.get_mut().unwrap()[3] = 20;
        v1.get_mut().unwrap()[3] = 13;

        let r = v1.get().unwrap()[3];
        assert_eq!(r, 13);
    }

    #[test]
    fn size_works() {
        let v = vec![1, 2, 3, 4];
        let v = ThreadUnsafeStorage::new(v);

        assert_eq!(v.len().unwrap(), 4);
    }
}
