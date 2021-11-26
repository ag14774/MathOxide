pub enum ShapeDim {
    Known(usize),
    Inferred,
}

impl From<isize> for ShapeDim {
    fn from(val: isize) -> Self {
        if val >= 0 {
            ShapeDim::Known(val as usize)
        } else {
            ShapeDim::Inferred
        }
    }
}

impl From<usize> for ShapeDim {
    fn from(val: usize) -> Self {
        ShapeDim::Known(val)
    }
}

impl ShapeDim {
    fn unwrap_known(&self) -> Result<usize, &str> {
        match *self {
            ShapeDim::Known(u) => Ok(u),
            ShapeDim::Inferred => Err("cannot unwrap inferred value"),
        }
    }
}

fn normalize_shape<T, ListType>(shape: ListType) -> Vec<ShapeDim>
where
    T: Copy,
    ListType: AsRef<[T]>,
    ShapeDim: From<T>,
{
    shape.as_ref().iter().map(|x| ShapeDim::from(*x)).collect()
}

pub fn infer_shape<T, ListType>(shape: ListType, numel: usize) -> Result<Vec<usize>, String>
where
    T: Copy,
    ListType: AsRef<[T]>,
    ShapeDim: From<T>,
{
    let shape = normalize_shape(shape);

    if shape.is_empty() {
        return Err("shape cannot be empty".to_string());
    }

    let n_dyn = shape
        .iter()
        .filter(|x| matches!(x, ShapeDim::Inferred))
        .count();

    if n_dyn > 1 {
        return Err("only one dimension can be inferred".to_string());
    }
    if n_dyn == 0 {
        return Ok(shape
            .iter()
            .map(|x| x.unwrap_known().unwrap())
            .collect::<Vec<usize>>());
    }

    let product: usize = shape
        .iter()
        .map(|x| x.unwrap_known().unwrap_or(1))
        .product();

    if product == 0 {
        if numel == 0 {
            return Err("unspecified dimension can be any value for tensor of size 0".to_string());
        } else {
            return Err("shape with 0-dimensions is not valid for non-empty tensors".to_string());
        }
    }

    if numel % product != 0 {
        return Err(format!("cannot infer shape with {} elements", numel));
    }
    let inferred = numel / product;

    Ok(shape
        .iter()
        .map(|x| x.unwrap_known().unwrap_or(inferred))
        .collect::<Vec<usize>>())
}
