// while a WIP
#![allow(unused_variables)]
use rutorch::*;
use std::ops::{Index, IndexMut};
use std::convert::From;
use std::cmp::max;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use num::NumCast;

use storage::*;
use ::*;
pub use tensor::tensor_ops::*;
use rand;
use {Ixs, RcMut};

pub struct THVec<T> {
    pub data: Vec<T>,
    pub dims: Vec<usize>,
}
pub struct THVecGeneric {
    pub data: Vec<i64>,
    pub dims: Vec<usize>,
}
pub struct THDims {
    pub dims: Vec<usize>,
}

pub enum TensorType {
    Float,
    Double,
    Byte,
    Char,
    Short,
    Int,
    Long,
}

pub trait NumLimits
    : Copy + Default + ::num::Num + ::num::NumCast + serde::Serialize {
}
impl NumLimits for f32 {}
impl NumLimits for f64 {}
impl NumLimits for i32 {}
impl NumLimits for i64 {}
impl NumLimits for u8 {}

#[derive(Hash, Serialize, Deserialize)]
pub enum TensorKind {
    FloatTensor(Tensor<f32>),
    LongTensor(Tensor<i64>),
    ByteTensor(Tensor<u8>),
}

pub type TensorList<T> = Vec<Tensor<T>>;
pub type TensorKindList = Vec<TensorKind>;
pub type OptTensorKindList = Vec<Option<TensorKind>>;
pub type RefTensorList<'a, T> = Vec<&'a mut Tensor<T>>;
pub type RefTensorKindList<'a> = Vec<&'a TensorKind>;
pub type TensorId = usize;


impl TensorKind {
    pub fn new<S>(&self, args: S) -> Self
        where S: Into<THVecGeneric>
    {
        match *self {
            TensorKind::FloatTensor(ref t) => {
                let tv: THVecGeneric = args.into();
                let tv: THVec<f32> = tv.into();
                let mut newt: Tensor<f32> = t.new(tv.dims);
                newt.set(tv.data);
                newt.into()
            }
            TensorKind::LongTensor(ref t) => {
                let tv: THVecGeneric = args.into();
                let tv: THVec<i64> = tv.into();
                let mut newt: Tensor<i64> = t.new(tv.dims);
                newt.set(tv.data);
                newt.into()
            }
            TensorKind::ByteTensor(ref t) => {
                let tv: THVecGeneric = args.into();
                let tv: THVec<u8> = tv.into();
                let mut newt: Tensor<u8> = t.new(tv.dims);
                newt.set(tv.data);
                newt.into()
            }

        }
    }

    pub fn backend(&self) -> Box<nn::BackendIntf> {
        unimplemented!()
    }
    pub fn inner(&self) -> *mut ::std::os::raw::c_void {
        unimplemented!()
    }
    pub fn len(&self) -> usize {
        unimplemented!()
    }
    pub fn s(&self, dim: usize) -> Self {
        unimplemented!()
    }
}

impl PartialEq for TensorKind {
    fn eq(&self, other: &Self) -> bool {
        use self::TensorKind::{FloatTensor, LongTensor, ByteTensor};
        match (self, other) {
            (&FloatTensor(ref t1), &FloatTensor(ref t2)) => t1.id() == t2.id(),
            (&LongTensor(ref t1), &LongTensor(ref t2)) => t1.id() == t2.id(),
            (&ByteTensor(ref t1), &ByteTensor(ref t2)) => t1.id() == t2.id(),
            _ => false,
        }
    }
}
impl Eq for TensorKind {}
impl Clone for TensorKind {
    fn clone(&self) -> Self {
        use self::TensorKind::{FloatTensor, LongTensor, ByteTensor};
        match *self {
            FloatTensor(ref t) => FloatTensor(t.clone()),
            LongTensor(ref t) => LongTensor(t.clone()),
            ByteTensor(ref t) => ByteTensor(t.clone()),
        }
    }
}

impl<T: NumLimits> From<Tensor<T>> for TensorKind {
    #[allow(unused_variables)]
    default fn from(input: Tensor<T>) -> Self {
        unreachable!()
    }
}

impl From<Tensor<f32>> for TensorKind {
    fn from(input: Tensor<f32>) -> Self {
        TensorKind::FloatTensor(input)
    }
}
impl From<Tensor<u8>> for TensorKind {
    fn from(input: Tensor<u8>) -> Self {
        TensorKind::ByteTensor(input)
    }
}

impl From<Tensor<i64>> for TensorKind {
    fn from(input: Tensor<i64>) -> Self {
        TensorKind::LongTensor(input)
    }
}

impl<T: NumLimits> From<TensorKind> for Tensor<T> {
    #[allow(unused_variables)]
    default fn from(input: TensorKind) -> Self {
        panic!("bad cast")
    }
}

impl<'a, T: NumLimits> From<&'a TensorKind> for &'a Tensor<T> {
    #[allow(unused_variables)]
    default fn from(input: &'a TensorKind) -> Self {
        panic!("bad cast")
    }
}

impl<'a> From<&'a TensorKind> for &'a Tensor<f32> {
    fn from(input: &'a TensorKind) -> Self {
        match *input {
            TensorKind::FloatTensor(ref t) => t,
            _ => unreachable!(),
        }
    }
}

impl<'a> From<&'a TensorKind> for &'a Tensor<i64> {
    fn from(input: &'a TensorKind) -> Self {
        match *input {
            TensorKind::LongTensor(ref t) => t,
            _ => unreachable!(),
        }
    }
}

impl<'a, T: NumLimits> From<&'a mut TensorKind> for &'a mut Tensor<T> {
    #[allow(unused_variables)]
    default fn from(input: &'a mut TensorKind) -> Self {
        panic!("bad cast")
    }
}

impl<'a> From<&'a mut TensorKind> for &'a mut Tensor<f32> {
    fn from(input: &'a mut TensorKind) -> Self {
        match *input {
            TensorKind::FloatTensor(ref mut t) => t,
            _ => unreachable!(),
        }
    }
}

impl<'a> From<&'a mut TensorKind> for &'a mut Tensor<i64> {
    fn from(input: &'a mut TensorKind) -> Self {
        match *input {
            TensorKind::LongTensor(ref mut t) => t,
            _ => unreachable!(),
        }
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
/*
impl From<TensorKind> for Tensor<f64> {
    fn from(input: TensorKind) -> Self {
        match input {
            TensorKind::DoubleTensor(v) => v,
            _ => unimplemented!(),
        }
    }
}
*/
impl From<TensorKind> for Tensor<i64> {
    fn from(input: TensorKind) -> Self {
        match input {
            TensorKind::LongTensor(v) => v,
            _ => unimplemented!(),
        }
    }
}
impl From<TensorKind> for Tensor<u8> {
    fn from(input: TensorKind) -> Self {
        match input {
            TensorKind::ByteTensor(v) => v,
            _ => unimplemented!(),
        }
    }
}

pub struct Tensor<T: NumLimits> {
    pub value: RcMut<TensorImpl<T, Output = T>>,
}

impl<T: NumLimits> Serialize for Tensor<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let rt = self.value.borrow().to_rust_tensor();
        let result = rt.serialize(serializer)?;
        Ok(result)
    }
}
impl<'de, T: NumLimits + Deserialize<'de>> Deserialize<'de> for Tensor<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let rt: RustTensor<T> = RustTensor::deserialize(deserializer)?;
        Ok(rt.into())
    }
}

impl<T: NumLimits> Hash for Tensor<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl<T: NumLimits> Index<usize> for Tensor<T> {
    type Output = T;
    fn index(&self, idx: usize) -> &Self::Output {

        //        self.value.borrow_mut().index(idx as isize)
        unimplemented!()
    }
}

impl<T: NumLimits> Index<i32> for Tensor<T> {
    type Output = T;
    fn index(&self, idx: i32) -> &Self::Output {

        //        self.value.borrow_mut().index(idx as isize)
        unimplemented!()
    }
}



impl<T: NumLimits> Tensor<T> {
    pub fn new<S>(&self, args: S) -> Self
        where S: Into<THVec<T>>
    {
        ::torch::tensor::<S, T>(args)
    }
    pub fn set(&mut self, args: Vec<T>) {
        unimplemented!()
    }
    pub fn len(&self) -> usize {
        unimplemented!()
    }
    pub fn s(&self, dim: usize) -> Self {
        unimplemented!()
    }
    pub fn cast<D>(&self) -> Tensor<D>
        where D: NumLimits
    {
        unimplemented!()
    }
}

impl<T: NumLimits> Default for Tensor<T> {
    fn default() -> Self {
        unimplemented!()
    }
}

impl<T: NumLimits> Clone for Tensor<T> {
    fn clone(&self) -> Self {
        Tensor { value: self.value.clone() }
    }
}

impl<T: NumLimits> Index<isize> for Tensor<T> {
    type Output = T;

    fn index(&self, idx: isize) -> &Self::Output {
        unimplemented!()
    }
}

type RefTI<T> = RcMut<TensorImpl<T, Output = T>>;
pub type TIArg<T> = TensorImpl<T, Output = T>;
pub trait TensorImpl<T: NumLimits>: Index<Ixs, Output = T> {
    fn new(&self) -> RefTI<T>;
    fn add(&self, value: T, output: &TIArg<T>);
    fn addt(&self, value: T, rhs: &TIArg<T>, output: &TIArg<T>);
    fn inner(&self) -> *mut ::std::os::raw::c_void;
    fn view(&self, dims: &[isize]) -> RefTI<T>;
    fn to_rust_tensor(&self) -> RustTensor<T>;
}

impl<T: NumLimits> From<RustTensor<T>> for Tensor<T> {
    #[allow(unused_variables)]
    default fn from(input: RustTensor<T>) -> Self {
        unreachable!()
    }
}
impl From<RustTensor<u8>> for Tensor<u8> {
    #[allow(unused_variables)]
    default fn from(input: RustTensor<u8>) -> Self {
        ByteTensor::from_rust_tensor(input)
    }
}
impl From<RustTensor<f32>> for Tensor<f32> {
    #[allow(unused_variables)]
    default fn from(input: RustTensor<f32>) -> Self {
        FloatTensor::from_rust_tensor(input)
    }
}
impl From<RustTensor<f64>> for Tensor<f64> {
    #[allow(unused_variables)]
    default fn from(input: RustTensor<f64>) -> Self {
        DoubleTensor::from_rust_tensor(input)
    }
}
impl From<RustTensor<i64>> for Tensor<i64> {
    #[allow(unused_variables)]
    default fn from(input: RustTensor<i64>) -> Self {
        LongTensor::from_rust_tensor(input)
    }
}


#[derive(Serialize, Deserialize)]
pub struct RustTensor<T> {
    size: Vec<i64>,
    stride: Vec<i64>,
    storage: Vec<T>,
    storage_offset: isize,
}

macro_rules! impl_tensor_impl {
    ($name:ident, $type:ident, $thname:ident, $storage_name:ident) => {
        pub struct $name {
            t: *mut $thname,
            storage: $storage_name,
            dims: Vec<isize>,
        }
        impl $name {
            pub fn new() -> Self {
                unsafe {
                    $name {
                        t: concat_idents!($thname, _new)(),
                        storage: $storage_name ::new(),
                        dims: Vec::new(),
                    }
                }
            }
            fn from_parts(t: *mut $thname, storage: $storage_name, dims: Vec<isize>) -> Self {
                $name {
                    t: t,
                    storage: storage,
                    dims: dims,
                }
            }
            fn to_rust_tensor(&self) -> RustTensor<$type> {
                let mut size: Vec<i64> = Vec::new();
                let mut stride: Vec<i64> = Vec::new();
                let mut storage = Vec::new();
                let offset = unsafe {(*self.t).storageOffset};
                let nd  = unsafe {(*self.t).nDimension};
                let need_stride = unsafe {(*self.t).stride != std::ptr::null_mut()};

                for i in 0..nd {
                    let s = unsafe { &*(*self.t).size.offset(i as isize) };
                    size.push(*s);
                }
                if need_stride {
                    for i in 0..nd {
                        let s = unsafe { &*(*self.t).stride.offset(i as isize) };
                        stride.push(*s);
                    }
                }
                for i in self.storage.iter() {
                    storage.push(*i);
                }
                RustTensor {size: size, stride: stride, storage: storage, storage_offset: offset}
            }
            fn from_rust_tensor(rt: RustTensor<$type>) -> Tensor<$type> {
                let size : Vec<usize> = rt.size.iter().map(|t| *t as usize).collect();
                let mut newt = $name ::with_capacity(size);
                unsafe {
                    (*newt.t).storageOffset = rt.storage_offset;
                }
                for (i, d) in rt.storage.iter().enumerate() {
                    newt.storage[i as isize] = *d;
                }
                Tensor {value: RcMutNew(newt)}
            }
            pub fn with_capacity<D>(dims: D) -> Self
                where D: AsRef<[usize]>
            {
                let dims_long : Vec<i64> = dims.as_ref().iter().map(|t| *t as i64).collect();
                let dims = dims.as_ref();
                let sizes = LongStorage::with_data(dims_long.as_slice());
                let size = dims.iter().product();
                let storage = $storage_name ::with_capacity(size);
                let t = unsafe {
                    concat_idents!($thname, _newWithStorage)(storage.t,
                                                             0,
                                                             sizes.t,
                                                             std::ptr::null_mut())
                };
                $name {
                    t: t,
                    storage: storage,
                    dims: dims.iter().map(|t| *t as isize).collect(),
                 }
            }
            pub fn randn<D>(dims: D) -> Self
                where D: AsRef<[usize]>
            {
                let dims = dims.as_ref();
                let mut t = $name  ::with_capacity(dims);
                for x in t.storage.iter_mut() {
                    *x = rand::random::<$type>()
                }
                t
            }
        }

        impl TensorImpl<$type> for $name {
            fn new(&self) -> RefTI<$type> {
                RcMutNew($name ::new())
            }
            fn add(&self, value: $type, output: &TIArg<$type>) {
                let out = typecast!(output.inner(), $thname);
                unsafe {
                    concat_idents!(TH, $name, _add)(out, self.t, value);
                };
            }
            fn inner(&self) -> *mut ::std::os::raw::c_void {
                self.t as *mut ::std::os::raw::c_void
            }
            fn addt(&self, value: $type, rhs: &TIArg<$type>, output: &TIArg<$type>) {
                let out = typecast!(output.inner(), $thname);
                let rhsp = typecast!(rhs.inner(), $thname);
                unsafe {
                    concat_idents!($thname, _add)(out, rhsp, value);
                };
            }
            fn view(&self, dims: &[isize]) -> RefTI<$type> {
                let dims_long : Vec<i64> = dims.iter().map(|t| *t as i64).collect();
                let size = LongStorage::with_data(dims_long.as_slice());
                let t = unsafe { concat_idents!($thname, _newView)(self.t, size.t)  };
                let t = $name :: from_parts(t, self.storage.clone(), dims.to_vec());
                RcMutNew(t)
            }
            fn to_rust_tensor(&self) -> RustTensor<$type> {
                self.to_rust_tensor()
            }
        }
        impl Default for $name {
            fn default() -> Self {
                $name ::new()
            }
        }
        impl<'a> Index<&'a [isize]> for $name {
            type Output = $type;

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

        impl<'a> IndexMut<&'a [isize]> for $name {
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
        impl Index<isize> for $name {
            type Output = $type;
            fn index(&self, idx: isize) -> &Self::Output {
                unimplemented!()
            }
        }
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe { concat_idents!($thname, _free)(self.t) }
            }
        }
        impl Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: Serializer
            {
                let rt = self.to_rust_tensor();
                let result = rt.serialize(serializer)?;
                Ok(result)
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: Deserializer<'de>
            {
                unimplemented!()
            }
        }
    }
}

impl_tensor_impl!(FloatTensor, f32, THFloatTensor, FloatStorage);
impl_tensor_impl!(DoubleTensor, f64, THDoubleTensor, DoubleStorage);
impl_tensor_impl!(LongTensor, i64, THLongTensor, LongStorage);
impl_tensor_impl!(ByteTensor, u8, THByteTensor, ByteStorage);

pub fn make_vec(val: usize, count: usize) -> Vec<isize> {
    let mut vec = Vec::new();
    for _ in 0..count {
        vec.push(val as isize)
    }
    vec
}
