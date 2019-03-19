use std::sync::{Arc, Mutex, RwLock};
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use std::collections::VecDeque;
use std::thread::spawn;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};

struct SleepingDresser {
    clients: Mutex<VecDeque<usize>>,
    client_lock: RwLock<AtomicBool>,
}
impl SleepingDresser{
    fn new() -> Arc<SleepingDresser>  {
        Arc::new(SleepingDresser {
            clients: Mutex::new(VecDeque::new()),
            client_lock: RwLock::new(AtomicBool::new(false)),
        })
    }

    fn come(&self, i: usize, sender: SyncSender<usize>){
        let client_lock = self.client_lock.write().unwrap();
        while client_lock.load(Ordering::SeqCst) == true {}
        client_lock.store(true, Ordering::SeqCst);
        println!("client № {} comes to the dresser`s office ",i+1);
        let mut clients = self.clients.lock().unwrap();
        clients.push_back(i+1);
        let n = clients.get(clients.len()-1).unwrap().clone();
            match sender.try_send(n){
                Ok(()) => {
                    println!("client № {} added to the queue ",i+1);
                    clients.pop_front();
                    thread::sleep(Duration::from_millis(200));
                },
                Err(e) => {
                    println!("client № {} goes away ", i+1);
                    clients.pop_back();
                    thread::sleep(Duration::from_millis(200));
                }
            };
        client_lock.store(false, Ordering::SeqCst);
    }

    fn serve(&self, receiver: Receiver<usize>) {
        while let Ok(i) = receiver.try_recv() {
            println!("Dresser begins to serve client № {}", i);
            thread::sleep(Duration::from_millis(500));
            println!("Dresser ends to serve client № {}", i);
        }
    }
}

pub fn sleeping_dresser_test(capacity: usize, clients_amount: usize){
    let (sender, receiver) = sync_channel(capacity);
    let sleeping_dresser = SleepingDresser::new();
    let mut clients = Vec::new();

    for i in 0..clients_amount{
        let sleeping_dresser_cloned = sleeping_dresser.clone();
        let s = sender.clone();
        clients.push(spawn(move|| {
            sleeping_dresser_cloned.come(i, s);
        }))
    }

    let sleeping_dresser_cloned = sleeping_dresser.clone();
    spawn(move||{
        sleeping_dresser_cloned.serve(receiver);
    }).join().unwrap();

    for c in clients {
        c.join().unwrap();
    }
}