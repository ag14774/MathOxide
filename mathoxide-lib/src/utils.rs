pub struct ByRowIterator {
    shape: Vec<usize>,
    current: usize,
    numel: usize,
}

impl Iterator for ByRowIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.numel {
            None
        } else {
            let v = inverse_translate(self.current, &self.shape);
            self.current += 1;
            Some(v)
        }
    }
}

fn inverse_translate<Shape: AsRef<[usize]>>(idx: usize, shape: Shape) -> Vec<usize> {
    shape.as_ref().iter().rev().scan((1, 0), |(divisor, remainder), e| {
        let prev_div = *divisor;
        *divisor *= e;
        let res = ((idx - *remainder) % *divisor) / prev_div;
        *remainder += res;
        Some(res)
    }).collect()
}


pub fn iterate_by_column<Shape: AsRef<[usize]>>(shape: &Shape) -> ByRowIterator {
    ByRowIterator {
        numel: shape.as_ref().iter().product(),
        shape: shape.as_ref().clone().into(),
        current: 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let mut count = 0;
        for indices in iterate_by_column(&[1,1]) {
            assert_eq!(indices.as_slice(), &[0,0]);
            count += 1;
        }

        assert_eq!(count, 1);
    }

    #[test]
    fn returns_correct_indices() {
        let mut iterator = iterate_by_column(&[2, 3, 4, 17]);

        for l in 0..2 {
            for k in 0..3 {
                for j in 0..4 {
                    for i in 0..17 {
                        let res = iterator.next().unwrap();
                        assert_eq!(res.as_slice(), &[i, j, k, l]);
                    }
                }
            }
        }

        assert!(iterator.next().is_none());
    }

    #[test]
    fn accepts_run_time_indices() {
        let _iterator = iterate_by_column(&vec![2,3,4,17]);
    }
}
