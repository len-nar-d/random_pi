use rand::distributions::{Distribution, Uniform};
use std::io::{stdout, Write};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;


fn get_random() -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let random = Uniform::from(0.0..1.0);
    
    return (random.sample(&mut rng), random.sample(&mut rng));
}


fn get_position() -> bool {
    let (mut a, mut b) = get_random();

    if a.powi(2) + b.powi(2) <= 1.0 {
        return true;
    }
    return false;
}


fn create_working_threads(transmitter: Sender<u32>, available_cores: usize) {
    for i in 0..available_cores {
        let transmitter: Sender<u32> = transmitter.clone();

        thread::spawn(move || {
            loop {
                let mut a = 0;

                for _ in 0..1000000 {
                    if get_position() {
                        a = a + 1;
                    }
                }

                transmitter.send(a).expect(&format!("Error: Can't send data to Main-Thread [Working-Thread {}]", i));
            }  
        });
    }
}


fn main() {
    let (transmitter, receiver): (Sender<u32>, Receiver<u32>) = channel();
    let available_cores = thread::available_parallelism()
                                 .expect("Error: Can't get number of cores for parallelism")
                                 .get();

    println!("calculating pi with {} threads...\n", available_cores);
        
    create_working_threads(transmitter, available_cores);


    let mut a = 0.0;
    let mut i = 0.0;
    
    let mut stdout = stdout();
        
    loop {
        let message = receiver.recv().expect("Error: Main-Thread didn't received data from working threads");
        i += 1000000.0;
        a = a + message as f64;
        
        let pi = a / i * 4.0;
        print!("\r{:.64}        |{} random trys|", pi, i);
        stdout.flush().expect("Error: Can't flush the console");
    }
}

