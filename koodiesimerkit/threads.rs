fn main() {
    // Create atomic reference counting that has mutex with value 0 inside
    let counter = Arc::new(Mutex::new(0));
    // Create vector for thread handles
    let mut handles = vec![];

    for _ in 0..10 {
        // Clone Atomic reference counting
        let counter = Arc::clone(&counter);
        // Spawn thread and move clone of Arc to it
        let handle = thread::spawn(move || {
            // Get mutex via Arc and lock it
            let mut num = counter.lock().unwrap();
            //Increment counter
            *num += 1;
        });
        // Add thread handle to vector
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
