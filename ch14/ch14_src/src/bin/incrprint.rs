use std::thread;
use std::time::Duration;

const NUM: usize = 5;
static mut COUNTER: i32 = 0;

fn main() {
    let handle = thread::spawn(|| {
        for _ in 0..NUM {
                // unsafe {println!("count = {}", COUNTER); }
                // data race
            thread::sleep(Duration::from_secs(1));
        }
    });

    for _ in 0..NUM {
        unsafe {
            COUNTER += 1;
        }
        thread::sleep(Duration::from_secs(1));
    }

    handle.join().unwrap();
}