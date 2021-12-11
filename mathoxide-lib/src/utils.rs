enum IndexState {
    NotStarted,
    InProgress,
    Finished,
}

pub enum UpdaterResult {
    NotDone,
    Done,
}

type UpdaterType = fn(&mut Vec<usize>, &[usize]) -> UpdaterResult;

struct IndexIteration<'a>
{
    shape: &'a [usize],
    index: Vec<usize>,
    state: IndexState,
    updater: UpdaterType,
}

impl<'a> IndexIteration<'a>
{
    fn new<Shape>(shape: &'a Shape, updater: UpdaterType) -> Self
    where
        Shape: AsRef<[usize]>,
    {
        let shape = shape.as_ref();
        Self {
            shape,
            index: vec![0; shape.len()],
            state: IndexState::NotStarted,
            updater,
        }
    }

    pub fn row_major<Shape>(shape: &'a Shape) -> Self
    where
        Shape: AsRef<[usize]>,
    {
            Self::new(shape, update_index_row_major)
    }

    pub fn next<'b>(&'b mut self) -> Option<&'b [usize]>
    where
        'a: 'b
    {
        match self.state {
            IndexState::NotStarted => {
                self.state = IndexState::InProgress;
                Some(self.index.as_slice())
            }
            IndexState::InProgress => {
                if let UpdaterResult::NotDone = (self.updater)(&mut self.index, &self.shape) {
                    Some(self.index.as_slice())
                } else {
                    self.state = IndexState::Finished;
                    None
                }
            }
            IndexState::Finished => None,
        }
    }
}

pub fn update_index_row_major<Shape: AsRef<[usize]> +?Sized >(index: &mut Vec<usize>, shape: &Shape) -> UpdaterResult {
    let shape = shape.as_ref();

    let ndim = shape.len();
    let mut carry: usize = 1;
    let mut pointer: usize = 0;

    while carry == 1 && pointer < ndim {
        index[ndim - pointer - 1] += carry;
        carry = index[ndim - pointer - 1] / shape[ndim - pointer - 1];
        index[ndim - pointer - 1] %= shape[ndim - pointer - 1];
        pointer += 1;
    }
    if carry == 1 {
        UpdaterResult::Done
    }
    else {
        UpdaterResult::NotDone
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn update_by_column_simple_test() {
        let mut index = vec![0, 0];
        let mut count = 0;
        while let UpdaterResult::NotDone = update_index_row_major(&mut index, &[1,1]) {
            assert_eq!(index.as_slice(), &[0,0]);
            count += 1;
        }

        assert_eq!(count, 0);
    }

    #[test]
    fn returns_correct_indices() {
        let mut index = vec![0, 0, 0, 0];
  
        for i in 0..2 {
            for j in 0..3 {
                for k in 0..4 {
                    for l in 0..17 {
                        assert_eq!(index.as_slice(), &[i, j, k, l]);
                        if let UpdaterResult::Done = update_index_row_major(&mut index, &[2, 3, 4, 17]) {
                            break;
                        }
                    }
                }
            }
        }
  
        assert_eq!(index.as_slice(), &[0, 0, 0, 0]);
    }
  
    #[test]
    fn by_column_iteration_wrapper_exhaustive() {
        let mut index_wrapper = IndexIteration::row_major(&[2,3,4,17]);
        for i in 0..2 {
            for j in 0..3 {
                for k in 0..4 {
                    for l in 0..17 {
                        let res = index_wrapper.next().unwrap();
                        assert_eq!(res, &[i, j, k, l]);
                    }
                }
            }
        }
        assert_eq!(index_wrapper.next(), None);
        assert_eq!(index_wrapper.next(), None);
    }

    #[test]
    fn by_column_iteration_wrapper_no_more() {
        let mut index_wrapper = IndexIteration::row_major(&[2,3,4,17]);
        let mut count = 0;
        while let Some(_) = index_wrapper.next() {
            count += 1;
        }
        assert_eq!(count, 2 * 3 * 4 * 17);
    }
}
