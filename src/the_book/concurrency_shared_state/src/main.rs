use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc<T>はRc<T>のような並行な状況で安全に使用できる型
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // ロックすることでデータを更新できる
            let mut num = counter.lock().unwrap();
            *num += 1;
        }); // スコープ終わり 自動的にロックが解除される
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
