#![feature(allocator_api)]
#![feature(ptr_internals)]
#![feature(generic_associated_types)]
use std::{alloc::{Allocator, Global, Layout}, ptr::Unique, ops::{Index, IndexMut}, time::Instant};

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


    // A simple benchmark:

    // array dimensions
    const IMAX: usize = 2002;
    const JMAX: usize = 2002;
    
    // zero intialize 2D arrays
    let mut x: Vec2D<f64> = Vec2D::new(JMAX, IMAX);
    let mut xnew: Vec2D<f64> = Vec2D::new(JMAX, IMAX);
    let mut flush = vec![0u32;JMAX * IMAX * 10];

    // I assume I have to implement the RangeIndex trait to make this work
    // for el in &x[JMAX/2-5..JMAX/2+5][IMAX/2 - 5..IMAX/2+5] {
    //     el = 400.0;
    // } ;

    // set center block of memory to a larger value
    for j in JMAX / 2 - 5..JMAX / 2 + 5 {
        for i in IMAX/2 - 5 .. IMAX/2 + 5 {
            x[j][i] = 400.0;
        }
    }
    
    let it = Instant::now();

    // ITERATION
    for iter in 0..10000 {
               
        // Flushing the cache
        for el in &mut flush[..] {
            *el = 1;
        }

        for j in 1..JMAX-1 {
            for i in 1 .. IMAX-1 {
                // Calculation kernel
                xnew[j][i] = ( x[j][i] + x[j][i-1] + x[j][i+1] + x[j-1][i] + x[j+1][i] ) / 5.0;
            }
        }

        let xtmp = x.ptr.clone();
        x.ptr = xnew.ptr;
        xnew.ptr = xtmp;

        // cycles_run += rdtsc() - it;
        
        if iter % 1000 == 0 {
            println!("Iter {iter}");
        }

    }

    let final_time = it.elapsed().as_secs();
    println!("Total elapsed time (s): {final_time}"); 
}


// Rust optimization Guide:
// https://gist.github.com/jFransham/369a86eff00e5f280ed25121454acec1.js

// Cache size and other CPU info crates:
// https://docs.rs/cache-size/latest/cache_size/index.html#
// https://docs.rs/raw-cpuid/latest/raw_cpuid/index.html#
