use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::{Duration, Instant};
use std::{ptr, thread};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use crate::delay::Delay;

pub struct Task {
    //tasks: Vec<Pin<Box<dyn Future<Output = ()> + Send>>>
    tasks: Arc<Mutex<Vec<Pin<Box<dyn Future<Output = ()> + Send>>>>>,
    running: Arc<AtomicBool>
}

impl Task {

    pub fn new() -> Self {
        Self {
            //tasks: Vec::new()
            tasks: Arc::new(Mutex::new(Vec::new())),
            running: Arc::new(AtomicBool::new(false))
        }
    }

    pub fn delay_for(duration: Duration) -> Delay {
        Delay {
            when: Instant::now() + duration,
        }
    }

    // Function to simulate a task similar to `tokio::spawn`

    pub fn spawn<F>(&mut self, fut: F)
    where
        F: Future<Output = ()> + Send + 'static
    {
        //self.tasks.push(Box::pin(fut));
        self.tasks.lock().as_mut().unwrap().push(Box::pin(fut));
        self.run();
    }


    // Function to create a no-op waker
    fn noop_waker() -> Waker {
        // Define the raw waker vtable
        const NOOP_VTABLE: &RawWakerVTable = &RawWakerVTable::new(
            |data: *const ()| RawWaker::new(data, NOOP_VTABLE),
            |data: *const ()| {},
            |data: *const ()| {},
            |data: *const ()| {},
        );

        // Create a raw waker
        fn raw_waker() -> RawWaker {
            RawWaker::new(ptr::null(), NOOP_VTABLE)
        }

        // Convert raw waker to waker
        unsafe { Waker::from_raw(raw_waker()) }
    }

    fn run(&mut self) {
        /*
        let waker = Self::noop_waker();
        let mut cx = Context::from_waker(&waker);

        let mut last_check = Instant::now();
        let check_interval = Duration::from_millis(2);

        while !self.tasks.is_empty() {
            let mut i = 0;
            while i < self.tasks.len() {
                let poll = self.tasks.get_mut(i).unwrap().as_mut().poll(&mut cx);
                match poll {
                    Poll::Ready(_) => {
                        self.tasks.remove(i);
                    }
                    Poll::Pending => {
                        i += 1;
                    }
                }
            }

            thread::sleep(check_interval);

            if last_check.elapsed() > check_interval {
                last_check = Instant::now();
            }
        }
        */


        /*
        let waker = Self::noop_waker();
        let mut cx = Context::from_waker(&waker);

        while !self.tasks.is_empty() {
            let mut i = 0;
            while i < self.tasks.len() {
                let poll = self.tasks.get_mut(i).unwrap().as_mut().poll(&mut cx);
                match poll {
                    Poll::Ready(_) => {
                        self.tasks.remove(i);
                    }
                    Poll::Pending => {
                        i += 1;
                    }
                }
            }
        }
        */


        if self.running.load(Ordering::Relaxed) {
            return;
        }

        self.running.store(true, Ordering::Relaxed);
        let tasks = self.tasks.clone();
        let running = self.running.clone();

        thread::spawn(move || {
            let waker = Self::noop_waker();
            let mut cx = Context::from_waker(&waker);

            let mut last_check = Instant::now();
            let check_interval = Duration::from_millis(1);

            while !tasks.lock().unwrap().is_empty() {
                let mut i = 0;
                while i < tasks.lock().as_ref().unwrap().len() {
                    let poll = tasks.lock().as_mut().unwrap().get_mut(i).unwrap().as_mut().poll(&mut cx);
                    match poll {
                        Poll::Ready(_) => {
                            tasks.lock().as_mut().unwrap().remove(i);
                        }
                        Poll::Pending => {
                            i += 1;
                        }
                    }
                }

                thread::sleep(check_interval);

                if last_check.elapsed() > check_interval {
                    last_check = Instant::now();
                }
            }

            running.store(false, Ordering::Relaxed);
        });
    }
}
