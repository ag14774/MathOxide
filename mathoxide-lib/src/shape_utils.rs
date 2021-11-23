pub enum ShapeDim {
    Known(usize),
    Dyn,
}

impl From<isize> for ShapeDim {
    fn from(val: isize) -> Self {
        if val >= 0 {
            ShapeDim::Known(val as usize)
        } else {
            ShapeDim::Dyn
        }
    }
}

impl From<usize> for ShapeDim {
    fn from(val: usize) -> Self {
        ShapeDim::Known(val)
    }
}

fn parse<T, ListType>(shape: ListType) -> Vec<ShapeDim>
where
    T: Copy,
    ListType: AsRef<[T]>,
    ShapeDim: From<T>,
{
    shape.as_ref().iter().map(|x| ShapeDim::from(*x)).collect()
}

pub fn infer_shape<T, ListType>(shape: ListType, numel: usize) -> Vec<usize>
where
    T: Copy,
    ListType: AsRef<[T]>,
    ShapeDim: From<T>,
{
    let shape = parse(shape);
    let n_dyn = shape.iter().filter(|x| matches!(x, ShapeDim::Dyn)).count();
    if n_dyn > 1 {
        panic!("only one dimension can be inferred");
    }
    let product = shape
        .iter()
        .map(|x| match *x {
            ShapeDim::Known(u) => u,
            ShapeDim::Dyn => 1,
        })
        .product::<usize>();
    let inferred = numel / product;

    shape
        .iter()
        .map(|x| match *x {
            ShapeDim::Known(u) => u,
            ShapeDim::Dyn => inferred,
        })
        .collect::<Vec<usize>>()
}
