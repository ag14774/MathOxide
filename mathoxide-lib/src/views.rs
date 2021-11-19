pub trait ArrayView {
    fn translate<ListType: AsRef<[usize]>>(&self, idx: ListType) -> usize;
    fn offset(&self) -> usize;
    fn shape(&self) -> &[usize];
    fn stride(&self) -> &[usize];
    fn ndim(&self) -> usize {
        self.shape().len()
    }
    fn numel(&self) -> usize {
        self.shape().iter().product()
    }
    fn is_contiguous(&self) -> bool;
}

pub struct ContiguousView {
    shape: Vec<usize>,
    offset: usize,
    stride: Vec<usize>,
}

impl ContiguousView {
    pub fn new<ListType: AsRef<[usize]>>(shape: ListType) -> Self {
        Self::new_with_offset(shape, 0)
    }

    pub fn new_with_offset<ListType: AsRef<[usize]>>(shape: ListType, offset: usize) -> Self {
        Self {
            shape: shape.as_ref().to_vec(),
            offset,
            stride: Self::compute_stride(shape.as_ref()),
        }
    }

    fn compute_stride<ListType: AsRef<[usize]>>(shape: ListType) -> Vec<usize> {
        let mut res = shape
            .as_ref()
            .iter()
            .rev()
            .scan(1, |state, &x| {
                let tmp = *state;
                *state *= x;
                Some(tmp)
            })
            .collect::<Vec<usize>>();
        res.reverse();
        res
    }
}

impl ArrayView for ContiguousView {
    fn translate<ListType: AsRef<[usize]>>(&self, idx: ListType) -> usize {
        self.offset()
            + idx
                .as_ref()
                .iter()
                .zip(self.stride().iter())
                .map(|(x, y)| x * y)
                .sum::<usize>()
    }

    fn offset(&self) -> usize {
        self.offset
    }

    fn shape(&self) -> &[usize] {
        self.shape.as_slice()
    }

    fn stride(&self) -> &[usize] {
        self.stride.as_slice()
    }

    fn is_contiguous(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn contiguous_view_check() {
        assert_eq!(ContiguousView::new([2, 3]).is_contiguous(), true);
    }

    #[test]
    fn contiguous_view_check_last_item() {
        let view = ContiguousView::new([2, 3, 4]);
        assert_eq!(view.translate([1, 2, 3]), view.numel() - 1);
    }

    #[test]
    fn contiguous_view_offset_check_last_item() {
        let offset: usize = 5;
        let view = ContiguousView::new_with_offset([2, 3, 4], offset);
        assert_eq!(view.translate([1, 2, 3]), view.numel() - 1 + offset);
    }

    #[test]
    fn contiguous_view_translate() {
        let view = ContiguousView::new([2, 3, 4]);
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
    fn contiguous_view_ndims() {
        assert_eq!(ContiguousView::new([2]).ndim(), 1);
        assert_eq!(ContiguousView::new([2, 3]).ndim(), 2);
        assert_eq!(ContiguousView::new([2, 3, 4]).ndim(), 3);
    }
}
