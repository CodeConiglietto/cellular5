use std::{
    fmt::Debug,
    sync::{
        mpsc::{self, Receiver, TryRecvError},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use log::{debug, info, trace};

pub struct Preloader<T>
where
    T: Debug + Send + 'static,
{
    child_thread: Option<JoinHandle<()>>,
    running: Arc<Mutex<bool>>,
    receiver: Receiver<T>,
}

impl<T> Preloader<T>
where
    T: Debug + Send + 'static,
{
    pub fn new<G>(pool_size: usize, mut generator: G) -> Self
    where
        G: Generator<Output = T> + Send + 'static,
    {
        let (sender, receiver) = mpsc::sync_channel(pool_size);
        let running = Arc::new(Mutex::new(true));
        let running_child = Arc::clone(&running);

        let child_thread = thread::spawn(move || {
            loop {
                debug!(
                    "Preloader child thread {:?} starting up",
                    thread::current().id()
                );

                if sender.send(generator.generate()).is_err() {
                    break;
                }

                if !*running_child.lock().unwrap() {
                    break;
                }

                trace!(
                    "Preloader child thread {:?} looping",
                    thread::current().id()
                );
            }
            debug!(
                "Preloader child thread {:?} shutting down",
                thread::current().id()
            );
        });

        debug!(
            "Parent thread {:?} spawned child preloader thread {:?}",
            thread::current().id(),
            child_thread.thread().id()
        );

        Self {
            child_thread: Some(child_thread),
            running,
            receiver,
        }
    }

    pub fn _get_next(&self) -> T {
        self.receiver.recv().unwrap()
    }

    pub fn try_get_next(&self) -> Option<T> {
        match self.receiver.try_recv() {
            Ok(item) => Some(item),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Child thread disconnected"),
        }
    }
}

impl<T> Drop for Preloader<T>
where
    T: Debug + Send + 'static,
{
    fn drop(&mut self) {
        info!("Shutting down preloader thread");
        let mut running = self.running.lock().unwrap();
        if *running {
            let child_thread = self.child_thread.take().unwrap();
            debug!(
                "Parent thread {:?} shutting down child preloader thread {:?}",
                thread::current().id(),
                child_thread.thread().id()
            );

            *running = false;

            loop {
                if self.receiver.try_recv().is_err() {
                    break;
                }
            }

            child_thread.join().unwrap();
        }
    }
}

pub trait Generator {
    type Output: Sized;

    fn generate(&mut self) -> Self::Output;
}
