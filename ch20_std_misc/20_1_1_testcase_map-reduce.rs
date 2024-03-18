// Testcase: map-reduce

// Rust makes it easy parallelise data processing, without many headaches traditionally associated with attempt

// Standard library has great threading primitives out of the box -> combined with Rust concept of ownership and aliasing rules to prevent data races

// Aliasing rules (one writable reference XOR many readable references) automatically prevent you from manipulating state available to other threads

// When synchronization is needed, use primitives like `Mutex`es or `Channel`s

// This example: calc sum of all digits in block of nums
// - Chunk block into different threads, sum within thread, then between threads

// Though references passed across thread boundaries, Rust knows they are read-only so no race conditions can occur
// References also have 'static lifetimes, data won't be destroyed while threads still running
// `Arc` is smart pointer that can keep data alive and avoid non-static lifetimes

use std::thread;

// `main` thread
fn main() {

    // Data to process
    // Calc sum of all digits via threaded map reduce algorithm
    // Each whitespace separated chunk handled as different thread

    let data = "86967897737416471853297327050364959
                11861322575564723963297542624962850
                70856234701860851907960690014725639
                38397966707106094172783238747669219
                52380795257888236525459303330302837
                58495327135744041048897885734297812
                69920216438980873548808413720956532
                16278424637452589860345374828574668";

    // Make vector to hold child-threads which we spawn
    let mut children = vec![];


    // MAP PHASE -> divide data into initial segments, then begin processing

    // split data into segments for individual calculation
    // each chunk will be reference (&str) into actual data
    let chunked_data = data.split_whitespace();

    // Iterate over data segments
    // .enumerate() adds current loop index to whatever is iterated
    // the resulting tuple "(index, element)" is then immediately "destructured" into two variables "i" nad "data_segment" with a "destructuring assignment"
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process data segment in separate thread
        // spawn() returns handle to new thread, which must keep to access returned value

        // `move || u32` is syntax for closure that takes no args, takes ownership of captured variables (`move`) and returns unsigned 32-bit integer

        // Rust is smart enough to infer the `-> u32` from closure itself so could have left that out

        children.push(thread::spawn(move || -> u32 {

            // Calc intermediate sum of this segment
            let result = data_segment
                // iterate over chars of segment
                .chars()
                // convert text-chars to number value
                .map(|c| c.to_digit(10).expect("should be a digit"))
                // sum resulting iterator of numbers
                .sum();
            
                // println! locks stdout, no so text-interleaving occurs
            println!("processed segment {}, result = {}", i, result);

            // return not needed, because Rust is "expression language", last evaluated expression in each block is automatically its value
            result
        }));
    }

    // REDUCE PHASE -> Collect intermediate results, combine into final result

    // combine each thread's intermediate results into final sum
    // use "turbofish" ::<> to provide sum() with type hint

    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);



}