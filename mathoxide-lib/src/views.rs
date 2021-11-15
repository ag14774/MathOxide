pub trait ArrayView {
    fn translate<ListType: AsRef<[usize]>>(&self, idx: ListType) -> usize;
    fn offset(&self) -> usize;
    // fn stride(&self) -> &[usize];
    fn ndims(&self) -> usize;
    fn size(&self) -> usize;
    fn shape(&self) -> &[usize];
}

pub struct SimpleView {
    _shape: Vec<usize>,
}

impl SimpleView {
    pub fn new<ListType: AsRef<[usize]>>(shape: ListType) -> Self {
        Self {
            _shape: shape.as_ref().to_vec(),
        }
    }
}

impl ArrayView for SimpleView {
    fn translate<ListType: AsRef<[usize]>>(&self, idx: ListType) -> usize {
        let mut res: usize = 0;
        for (idx_i, shape_i) in idx.as_ref().iter().zip(self._shape.iter()) {
            res *= shape_i;
            res += idx_i;
        }
        res
    }

    fn offset(&self) -> usize {
        0
    }

    fn ndims(&self) -> usize {
        self._shape.len()
    }

    fn size(&self) -> usize {
        self._shape.iter().product()
    }

    fn shape(&self) -> &[usize] {
        self._shape.as_slice()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let view = SimpleView::new([2, 3, 4]);

        // Check last item
        assert_eq!(view.translate([1, 2, 3]), view.size() - 1);
    }
}
