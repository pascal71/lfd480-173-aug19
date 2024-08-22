use std::time::Instant;

fn main() {
    // Measure time for stack allocation
    let start = Instant::now();
    let stack_array = [0; 10_000]; // Large array on the stack
    let stack_duration = start.elapsed();
    println!(
        "Stack allocation and initialization took: {:?}",
        stack_duration
    );

    // Measure time for heap allocation using Box
    let start = Instant::now();
    let heap_array = Box::new([0; 10_000]); // Large array on the heap
    let heap_duration = start.elapsed();
    println!(
        "Heap allocation and initialization took: {:?}",
        heap_duration
    );
}
