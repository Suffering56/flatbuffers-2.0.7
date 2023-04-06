// automatically generated by the FlatBuffers compiler, do not modify
// @generated
extern crate alloc;
extern crate flatbuffers;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::mem;
use core::cmp::Ordering;
use self::flatbuffers::{EndianScalar, Follow};
use super::*;
// struct ArrayStruct, aligned to 8
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq)]
pub struct ArrayStruct(pub [u8; 160]);
impl Default for ArrayStruct { 
  fn default() -> Self { 
    Self([0; 160])
  }
}
impl core::fmt::Debug for ArrayStruct {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.debug_struct("ArrayStruct")
      .field("a", &self.a())
      .field("b", &self.b())
      .field("c", &self.c())
      .field("d", &self.d())
      .field("e", &self.e())
      .field("f", &self.f())
      .finish()
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ArrayStruct {}
impl flatbuffers::SafeSliceAccess for ArrayStruct {}
impl<'a> flatbuffers::Follow<'a> for ArrayStruct {
  type Inner = &'a ArrayStruct;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    <&'a ArrayStruct>::follow(buf, loc)
  }
}
impl<'a> flatbuffers::Follow<'a> for &'a ArrayStruct {
  type Inner = &'a ArrayStruct;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    flatbuffers::follow_cast_ref::<ArrayStruct>(buf, loc)
  }
}
impl<'b> flatbuffers::Push for ArrayStruct {
    type Output = ArrayStruct;
    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::core::slice::from_raw_parts(self as *const ArrayStruct as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}
impl<'b> flatbuffers::Push for &'b ArrayStruct {
    type Output = ArrayStruct;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        let src = unsafe {
            ::core::slice::from_raw_parts(*self as *const ArrayStruct as *const u8, Self::size())
        };
        dst.copy_from_slice(src);
    }
}

impl<'a> flatbuffers::Verifiable for ArrayStruct {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.in_buffer::<Self>(pos)
  }
}

impl<'a> ArrayStruct {
  #[allow(clippy::too_many_arguments)]
  pub fn new(
    a: f32,
    b: &[i32; 15],
    c: i8,
    d: &[NestedStruct; 2],
    e: i32,
    f: &[i64; 2],
  ) -> Self {
    let mut s = Self([0; 160]);
    s.set_a(a);
    s.set_b(b);
    s.set_c(c);
    s.set_d(d);
    s.set_e(e);
    s.set_f(f);
    s
  }

  pub const fn get_fully_qualified_name() -> &'static str {
    "MyGame.Example.ArrayStruct"
  }

  pub fn a(&self) -> f32 {
    let mut mem = core::mem::MaybeUninit::<f32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[0..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<f32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_a(&mut self, x: f32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const f32 as *const u8,
        self.0[0..].as_mut_ptr(),
        core::mem::size_of::<f32>(),
      );
    }
  }

  pub fn b(&'a self) -> flatbuffers::Array<'a, i32, 15> {
    flatbuffers::Array::follow(&self.0, 4)
  }

  pub fn set_b(&mut self, items: &[i32; 15]) {
    flatbuffers::emplace_scalar_array(&mut self.0, 4, items);
  }

  pub fn c(&self) -> i8 {
    let mut mem = core::mem::MaybeUninit::<i8>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[64..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i8>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_c(&mut self, x: i8) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i8 as *const u8,
        self.0[64..].as_mut_ptr(),
        core::mem::size_of::<i8>(),
      );
    }
  }

  pub fn d(&'a self) -> flatbuffers::Array<'a, NestedStruct, 2> {
    flatbuffers::Array::follow(&self.0, 72)
  }

  pub fn set_d(&mut self, x: &[NestedStruct; 2]) {
    unsafe {
      core::ptr::copy(
        x.as_ptr() as *const u8,
        self.0.as_mut_ptr().add(72),
        64,
      );
    }
  }

  pub fn e(&self) -> i32 {
    let mut mem = core::mem::MaybeUninit::<i32>::uninit();
    unsafe {
      core::ptr::copy_nonoverlapping(
        self.0[136..].as_ptr(),
        mem.as_mut_ptr() as *mut u8,
        core::mem::size_of::<i32>(),
      );
      mem.assume_init()
    }.from_little_endian()
  }

  pub fn set_e(&mut self, x: i32) {
    let x_le = x.to_little_endian();
    unsafe {
      core::ptr::copy_nonoverlapping(
        &x_le as *const i32 as *const u8,
        self.0[136..].as_mut_ptr(),
        core::mem::size_of::<i32>(),
      );
    }
  }

  pub fn f(&'a self) -> flatbuffers::Array<'a, i64, 2> {
    flatbuffers::Array::follow(&self.0, 144)
  }

  pub fn set_f(&mut self, items: &[i64; 2]) {
    flatbuffers::emplace_scalar_array(&mut self.0, 144, items);
  }

  pub fn unpack(&self) -> ArrayStructT {
    ArrayStructT {
      a: self.a(),
      b: self.b().into(),
      c: self.c(),
      d: { let d = self.d(); flatbuffers::array_init(|i| d.get(i).unpack()) },
      e: self.e(),
      f: self.f().into(),
    }
  }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ArrayStructT {
  pub a: f32,
  pub b: [i32; 15],
  pub c: i8,
  pub d: [NestedStructT; 2],
  pub e: i32,
  pub f: [i64; 2],
}
impl ArrayStructT {
  pub fn pack(&self) -> ArrayStruct {
    ArrayStruct::new(
      self.a,
      &self.b,
      self.c,
      &flatbuffers::array_init(|i| self.d[i].pack()),
      self.e,
      &self.f,
    )
  }
}
