pub trait ArrayView {
    fn translate<ListType: AsRef<[usize]>>(&self, idx: ListType) -> usize;
    fn offset(&self) -> usize;
    fn ndims(&self) -> usize;
    fn size(&self) -> usize;
    fn shape(&self) -> &[usize];
}

pub struct SimpleView {
    shape: Vec<usize>,
}

impl SimpleView {
    pub fn new<ListType: AsRef<[usize]>>(shape: ListType) -> Self {
        Self {
            shape: shape.as_ref().to_vec(),
        }
    }
}

impl ArrayView for SimpleView {
    fn translate<ListType: AsRef<[usize]>>(&self, idx: ListType) -> usize {
        let mut res: usize = 0;
        for (idx_i, shape_i) in idx.as_ref().iter().zip(self.shape.iter()) {
            res *= shape_i;
            res += idx_i;
        }
        res
    }

    fn offset(&self) -> usize {
        0
    }

    fn ndims(&self) -> usize {
        self.shape.len()
    }

    fn size(&self) -> usize {
        self.shape.iter().product()
    }

    fn shape(&self) -> &[usize] {
        self.shape.as_slice()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_view_check_last_item() {
        let view = SimpleView::new([2, 3, 4]);
        assert_eq!(view.translate([1, 2, 3]), view.size() - 1);
    }

    #[test]
    fn simple_view_translate() {
        let view = SimpleView::new([2, 3, 4]);
        let mut counter = 0;
        for i in 0..2 {
            for j in 0..3 {
                for k in 0..4 {
                    assert_eq!(view.translate(&[i, j, k]), counter);
                    counter += 1;
                }
            }
        }
    }

    #[test]
    fn simple_view_ndims() {
        assert_eq!(SimpleView::new([2]).ndims(), 1);
        assert_eq!(SimpleView::new([2, 3]).ndims(), 2);
        assert_eq!(SimpleView::new([2, 3, 4]).ndims(), 3);
    }
}
