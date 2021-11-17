
extern "C" {
    fn doubler(x: i32) -> i32;
    // accepts pointer to array
    fn quicksort(arr: *const i32, first: i32, last: i32) -> ();
}

// to accept the slice, rather than the pointer
fn quicksort_wrapped(n: &mut [i32], first: i32, last: i32) {
    unsafe {
        quicksort(n.as_mut_ptr(), first, last);
    }
}

fn main() {
    let mut numbers_slice = [6, 4, 7, 2, 5, 9, 3, 1, 8];
    let first = 0;
    let last: i32 = (numbers_slice.len() as i32) - 1;
    
    unsafe {
        println!("doubler: {}", doubler(2));
        quicksort_wrapped(&mut numbers_slice, first, last);
        println!("quicksort: {:?}", numbers_slice);
    }
}