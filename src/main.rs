#![feature(allocator_api)]
#![feature(ptr_internals)]
#![feature(generic_associated_types)]
use std::{alloc::{Allocator, Global, Layout}, ptr::Unique, ops::{Index, IndexMut}};

pub struct Vec2D<T> {
    ptr: Unique<T>,
    col_len: usize,
    row_len: usize,
} 

impl<T> Vec2D<T> {
    pub fn new(j: usize, i: usize) -> Self {
        let alloc_size = j * i;
        let layout = match Layout::array::<T>(alloc_size) {
            Ok(layout) => layout,
            Err(_) => panic!("capacity overflow"),
        };

        if usize::BITS < 64 && alloc_size > isize::MAX as usize {
            panic!("capacity overflow");
        }

        let result = Global.allocate_zeroed(layout);
        let ptr = match result {
            Ok(ptr) => ptr,
            Err(_) => panic!("allocation failed"),
        };

        Self { ptr: unsafe { Unique::new_unchecked(ptr.cast().as_ptr()) }, col_len: j, row_len: i }

    }
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.col_len);
        let offset = index * self.row_len;
        let ptr = unsafe{self.ptr.as_ptr().add(offset)};
        unsafe { std::slice::from_raw_parts(ptr, self.row_len) }
    }
}

impl<T> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.col_len);
        let offset = index * self.row_len;
        let ptr = unsafe { self.ptr.as_ptr().add(offset) };
        unsafe { std::slice::from_raw_parts_mut(ptr, self.row_len) }
    }
}

fn main() {
    println!("Hello, world!");

    let mut init: Vec2D<f32> = Vec2D::new(10, 10);
    let val = init[5][5];
    println!("Initialized to ZERO: {val}");

    init[5][5] = 5.0;
    let val = init[5][5];
    println!("Mutated Vec2D: {val}");
    assert_eq!(val, 5.0);

    // Todo, Geert: Implement a benchmark use case using Vec2D.
}


// Rust optimization Guide:
// https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1.js

// Take the size of each type and make...
// Only when the total size is below the size of a cache line (64 bytes)
// use padding to make it a divisor of the cache size
// use the number of times tyhe struct fits within the cache size = V
// this would benefit from a AoSoA layout.
// Otherwise, an SoA or AoS could be chosen based on the performance benchmark.


// Cache size and other CPU info crates:
// https://docs.rs/cache-size/latest/cache_size/index.html#
// https://docs.rs/raw-cpuid/latest/raw_cpuid/index.html#
