use rutorch::*;
use std::ops::{Index, IndexMut};
use std::convert::From;
use std::cmp::max;
use std::hash::{Hash, Hasher};

use storage::*;
use rand;
use {Ixs, RcMut};

#[derive(Hash)]
pub enum TensorKind {
    FloatTensor(Tensor<f32>),
    LongTensor(Tensor<i64>),
}

pub type TensorList<T> = Vec<Tensor<T>>;
pub type TensorKindList = Vec<TensorKind>;
pub type RefTensorList<'a, T> = Vec<&'a mut Tensor<T>>;
pub type RefTensorKindList<'a> = Vec<&'a TensorKind>;
pub type TensorId = i32;


impl PartialEq for TensorKind {
    fn eq(&self, other: &TensorKind) -> bool {
        use self::TensorKind::{FloatTensor, LongTensor};
        match (self, other) {
            (&FloatTensor(ref t1), &FloatTensor(ref t2)) => t1.id == t2.id,
            (&LongTensor(ref t1), &LongTensor(ref t2)) => t1.id == t2.id,
            _ => false,
        }
    }
}
impl Eq for TensorKind {}
impl Clone for TensorKind {
    fn clone(&self) -> Self {
        use self::TensorKind::{FloatTensor, LongTensor};
        match *self {
            FloatTensor(ref t) => FloatTensor(t.clone()),
            LongTensor(ref t) => LongTensor(t.clone()),
        }
    }
}

impl<T> From<Tensor<T>> for TensorKind {
    default fn from(input: Tensor<T>) -> Self {
        unreachable!()
    }
}

impl From<Tensor<f32>> for TensorKind {
    fn from(input: Tensor<f32>) -> TensorKind {
        TensorKind::FloatTensor(input)
    }
}

impl From<Tensor<i64>> for TensorKind {
    fn from(input: Tensor<i64>) -> TensorKind {
        TensorKind::LongTensor(input)
    }
}

impl<T> From<TensorKind> for Tensor<T> {
    default fn from(input: TensorKind) -> Tensor<T> {
        panic!("bad cast")
    }
}
impl From<TensorKind> for Tensor<f32> {
    fn from(input: TensorKind) -> Self {
        match input {
            TensorKind::FloatTensor(v) => v,
            _ => unimplemented!(),
        }
    }
}

impl From<TensorKind> for Tensor<i64> {
    fn from(input: TensorKind) -> Self {
        match input {
            TensorKind::LongTensor(v) => v,
            _ => unimplemented!(),
        }
    }
}

pub struct Tensor<T> {
    pub id: i32,
    value: RcMut<TensorImpl<T, Output = T>>,
}
impl<T> Hash for Tensor<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<T: Copy> Index<usize> for Tensor<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {

        //        self.value.borrow_mut().index(idx as isize)
        unimplemented!()
    }
}

impl<T: Copy> Index<i32> for Tensor<T> {
    type Output = T;
    fn index(&self, idx: i32) -> &Self::Output {

        //        self.value.borrow_mut().index(idx as isize)
        unimplemented!()
    }
}


pub trait New<D, T> {
    fn new(args: D) -> T;
    fn new_(&self, args: D) -> T;
}

impl<T> New<usize, Tensor<T>> for Tensor<T> {
    fn new(args: usize) -> Self {
        unimplemented!()
    }
    fn new_(&self, args: usize) -> Self {
        unimplemented!()
    }
}
impl<T> New<Vec<usize>, Tensor<T>> for Tensor<T> {
    fn new(args: Vec<usize>) -> Self {
        unimplemented!()
    }
    fn new_(&self, args: Vec<usize>) -> Self {
        unimplemented!()
    }
}

impl<T> Tensor<T> {
    pub fn len(&self) -> usize {
        unimplemented!()
    }
    pub fn size(&self) -> Vec<usize> {
        unimplemented!()
    }
    pub fn zero_(&mut self) -> Self {
        unimplemented!()
    }
    pub fn add_(&mut self, rhs: &Tensor<T>) {
        unimplemented!()
    }
    pub fn cuda(&self) -> Self {
        unimplemented!()
    }
    pub fn cpu(&self) -> Self {
        unimplemented!()
    }
    pub fn reduce_max(&self, axis: usize) -> (Self, Tensor<i64>) {
        unimplemented!()
    }
    pub fn tensor_eq(&self, rhs: &Self) -> Tensor<i64> {
        unimplemented!()
    }
    pub fn sum(&self) -> u32 {
        unimplemented!()
    }
    pub fn s(&self, dim: usize) -> Self {
        unimplemented!()
    }
}

impl<T> Default for Tensor<T> {
    fn default() -> Self {
        unimplemented!()
    }
}

impl<T> Clone for Tensor<T> {
    fn clone(&self) -> Self {
        Tensor {
            id: self.id,
            value: self.value.clone(),
        }
    }
}

impl<T: Copy> Index<isize> for Tensor<T> {
    type Output = T;

    fn index(&self, idx: isize) -> &Self::Output {
        unimplemented!()
    }
}
pub trait TensorImpl<T>: Index<Ixs, Output = T> {
    //fn view<'a>(&self, dims: &[i32]) -> Tensor<'a>;
}


pub struct FloatTensor {
    t: *mut THFloatTensor,
    storage: FloatStorage,
    dims: Vec<isize>,
}

impl Default for FloatTensor {
    fn default() -> Self {
        FloatTensor::new()
    }
}


impl FloatTensor {
    pub fn new() -> Self {
        unsafe {
            FloatTensor {
                t: THFloatTensor_new(),
                storage: FloatStorage::new(),
                dims: Vec::new(),
            }
        }
    }
    pub fn with_capacity(dims: &[isize]) -> Self {
        let size = dims.iter().product();
        let storage = FloatStorage::with_capacity(size);
        let strides = vec![1; dims.len()];
        let mut t = THFloatTensor {
            size: dims.clone().as_ptr() as *mut ::std::os::raw::c_long,
            stride: strides.as_ptr() as *mut ::std::os::raw::c_long,
            nDimension: dims.len() as i32,
            storage: storage.t,
            storageOffset: 0,
            refcount: 1,
            flag: TH_TENSOR_REFCOUNTED as i8,
        };
        FloatTensor {
            t: &mut t,
            storage: storage,
            dims: Vec::from(dims),
        }
    }
    pub fn randn(dims: &[isize]) -> Self {
        /* XXX */
        let mut t = FloatTensor::with_capacity(dims);
        for x in t.storage.iter_mut() {
            *x = rand::random::<f32>()
        }
        t
    }
}

impl<'a> Index<&'a [isize]> for FloatTensor {
    type Output = f32;

    fn index(&self, idx: &'a [isize]) -> &Self::Output {
        let mut index: isize = 0;
        let lastidx = max(0, idx.len() as isize - 1) as usize;
        if idx.len() != self.dims.len() {
            panic!("bad dimlen")
        }
        for i in 0..lastidx {
            if idx[i] >= self.dims[i] {
                panic!("bad dimlen")
            }
            index += idx[i] * self.dims[i]
        }
        if idx[lastidx] >= self.dims[lastidx] {
            panic!("bad dimlen")
        }
        index += idx[lastidx];
        &self.storage[index]
    }
}

impl<'a> IndexMut<&'a [isize]> for FloatTensor {
    fn index_mut(&mut self, idx: &'a [isize]) -> &mut Self::Output {
        let mut index: isize = 0;
        let lastidx = max(0, idx.len() as isize - 1) as usize;
        if idx.len() != self.dims.len() {
            panic!("bad dimlen")
        }
        for i in 0..lastidx {
            if idx[i] >= self.dims[i] {
                panic!("bad dimlen")
            }
            index += idx[i] * self.dims[i]
        }
        if idx[lastidx] >= self.dims[lastidx] {
            panic!("bad dimlen")
        }
        index += idx[lastidx];
        &mut self.storage[index]
    }
}
