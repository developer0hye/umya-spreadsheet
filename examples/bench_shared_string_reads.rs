use std::alloc::{GlobalAlloc, Layout, System};
use std::env;
use std::hint::black_box;
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

struct CountingAllocator;

static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);
static ALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATION_COUNT.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(new_size, Ordering::Relaxed);
        unsafe { System.realloc(pointer, layout, new_size) }
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: CountingAllocator = CountingAllocator;

fn main() {
    let mut arguments = env::args_os();
    let program_name = arguments.next().unwrap_or_default();
    let Some(input_path) = arguments.next().map(PathBuf::from) else {
        eprintln!(
            "usage: {} <workbook.xlsx> [iterations]",
            program_name.to_string_lossy()
        );
        process::exit(2);
    };
    let iteration_count: usize = arguments
        .next()
        .map(|value| {
            value
                .to_string_lossy()
                .parse()
                .expect("iterations must be an integer")
        })
        .unwrap_or(5);

    assert!(iteration_count > 0, "iterations must be greater than zero");

    let warmup_workbook =
        umya_spreadsheet::reader::xlsx::read(&input_path).expect("warmup workbook read failed");
    black_box(&warmup_workbook);
    drop(warmup_workbook);

    for iteration in 1..=iteration_count {
        ALLOCATION_COUNT.store(0, Ordering::Relaxed);
        ALLOCATED_BYTES.store(0, Ordering::Relaxed);

        let start = Instant::now();
        let workbook = umya_spreadsheet::reader::xlsx::read(&input_path)
            .expect("measured workbook read failed");
        let elapsed = start.elapsed();
        black_box(&workbook);

        println!(
            "iteration={iteration} elapsed_ms={} allocations={} allocated_bytes={}",
            elapsed.as_millis(),
            ALLOCATION_COUNT.load(Ordering::Relaxed),
            ALLOCATED_BYTES.load(Ordering::Relaxed),
        );
        drop(workbook);
    }
}
