use std::thread;
use std::sync::{ Arc, Mutex };
use std::sync::atomic::{ AtomicBool, Ordering };
use std::io;
use std;
use std::error::Error;

pub struct SpawnableTaskHandle {
    pub handle: thread::JoinHandle<()>,
    pub stop_signal: Arc<AtomicBool>
}

impl SpawnableTaskHandle {
    pub fn stop(&self) {
        self.stop_signal.store(true, Ordering::Relaxed);
    }

    pub fn join(self) -> Result<(),Box<std::any::Any + Send + 'static>> {
        match self.handle.join() {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(err))
        }
    }
}

pub trait SpawnableTaskTrait {
    fn werk(&mut self, &mut [u8]) -> io::Result<()> {
        unimplemented!();
    }

    fn spawn(syncy_self: Arc<Mutex<Self>>) -> SpawnableTaskHandle where Self: std::marker::Send, Self: 'static {
        let thread_this = syncy_self.clone();

        let signal = Arc::new(AtomicBool::new(false));
        let signal_clone = signal.clone();

        let handle = thread::spawn(move || {
            // put it on the stack! default on linux is like 2MB
            let mut buf = [0; 1024*64];

            let this = thread_this;

            loop {
                let mut this = this.lock().unwrap();

                let res = Self::werk(&mut this, &mut buf[..]);

                // TODO: we need to be sure there aren't packets in limbo going either in or out
                if signal_clone.load(Ordering::Relaxed) {
                    break
                }

                match res {
                    Ok(_) => continue,
                    Err(err) => {
                        if err.description() == "Resource temporarily unavailable" || err.description() == "operation would block" {
                            continue
                        } else {
                            panic!("omg: {:?}", err)
                        }
                    }
                }


                // Could put exit check here but could
                // cancel thread when socket still has data... does it matter?
            }
        });

        SpawnableTaskHandle {
            handle: handle,
            stop_signal: signal
        }
    }

    fn stop(&mut self);
    fn join(self) -> thread::Result<()>;
}