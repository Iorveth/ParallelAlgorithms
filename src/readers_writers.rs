use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::thread::spawn;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, RwLock};

pub enum Priority {
    Readers,
    Writers,
    Equal
}

pub struct ReadersWriters {
    readers_count: AtomicUsize,
    writers_count: AtomicUsize,
    priority: Priority,
    read_lock: RwLock<AtomicBool>,
    write_lock: RwLock<AtomicBool>
}

impl ReadersWriters {
    fn new(readers_count: usize, writers_count: usize, priority: Priority) -> Arc<ReadersWriters> {
        Arc::new(ReadersWriters {
            readers_count: AtomicUsize::new(readers_count),
            writers_count: AtomicUsize::new(writers_count),
            priority,
            read_lock: RwLock::new(AtomicBool::new(false)),
            write_lock: RwLock::new(AtomicBool::new(false))
        })
    }

    fn get_readers_count(&self) -> &AtomicUsize {
        &self.readers_count
    }

    fn get_writers_count(&self) -> &AtomicUsize {
        &self.writers_count
    }

    fn read(&self, i: usize) {
        if self.write_lock.read().unwrap().load(Ordering::SeqCst) == true {}
        self.read_lock.read().unwrap().store(true, Ordering::SeqCst);
        println!("{} Started reading.", i);
        thread::sleep(Duration::from_millis(1500));
        println!("{} Finished reading.", i);

        match self.priority {
            Priority::Readers => {
                self.get_readers_count().fetch_sub(1, Ordering::SeqCst);
                if self.get_readers_count().load(Ordering::SeqCst) <= 0 {
                    self.read_lock.read().unwrap().store(false, Ordering::SeqCst);
                }
            }
            Priority::Writers | Priority::Equal => {
                self.read_lock.read().unwrap().store(false, Ordering::SeqCst);
            }
            _ => unreachable!()
        }
    }

    fn write(&self, i: usize) {
        match self.priority {
            Priority::Readers | Priority::Equal => {
                while self.write_lock.write().unwrap().load(Ordering::SeqCst) == true || self.read_lock.read().unwrap().load(Ordering::SeqCst) == true  {}
            }
            Priority::Writers => {
                while self.read_lock.read().unwrap().load(Ordering::SeqCst) == true  {}
            }
            _ => unreachable!()
        }
        let mut write = self.write_lock.write().unwrap();
        write.store(true, Ordering::SeqCst);
        println!("{} Started writing.", i);
        thread::sleep(Duration::from_millis(2500));
        println!("{} Finished writing.", i);
        match self.priority {
            Priority::Readers | Priority::Equal => {
                write.store(false, Ordering::SeqCst);
            }
            Priority::Writers => {
                if self.get_writers_count().load(Ordering::SeqCst) <= 0 {
                    write.store(false, Ordering::SeqCst);
                }
            }
            _ => unreachable!()
        }
    }
}
pub fn readers_writers_test(readers_count: usize, writers_count: usize, priority: Priority){
    let mut readers_writers= ReadersWriters::new(readers_count, writers_count, priority);
    let mut readers = Vec::new();
    let mut writers = Vec::new();
    let readers_count = readers_writers.get_readers_count().load(Ordering::SeqCst);
    let writers_count = readers_writers.get_writers_count().load(Ordering::SeqCst);

    for i in 1..=writers_count {
        let readers_writers_cloned = readers_writers.clone();
        writers.push(spawn(move || {
            readers_writers_cloned.write(i as usize);
        }));
    }
    for i in 1..=readers_count {
        let readers_writers_cloned = readers_writers.clone();
        readers.push(spawn(move || {
            readers_writers_cloned.read(i as usize);
        }));
    }
    for r in readers {
        r.join().unwrap();
    }
    for w in writers {
        w.join().unwrap();
    }
}