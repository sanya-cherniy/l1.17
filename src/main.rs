use std::sync::{Arc, Mutex};
use std::thread;

// структура-счетчик
struct CounterStructure {
    counter: u64,
}

// конструктор
impl CounterStructure {
    fn new(counter: u64) -> CounterStructure {
        CounterStructure { counter }
    }
}

fn main() {
    let counting = Arc::new(Mutex::new(CounterStructure::new(0))); // создаем экземпляр структуры через Arc::new(Mutex::new для осуществления конкурентного доступа к полям структуры
    let mut handlers = vec![];

    for _ in 1..=10 {
        let counting = Arc::clone(&counting); // клонируем для каждого потока
        let handle = thread::spawn(move || {
            let mut counting = counting.lock().unwrap(); // блокируем состояние для других потоков
            counting.counter += 1; // инкрементируем
        });
        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }
    println!("{}", counting.lock().unwrap().counter);
}
