mod task;
mod delay;

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::Pin;
    use std::thread::sleep;
    use std::time::Duration;
    use crate::task::Task;
    use super::*;

    #[test]
    fn spawn_test() {
        //let mut tasks: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![];
        let mut task = Task::new();

        task.spawn(async {
            for n in 1..15 {
                println!("1 test: {}", n);
                Task::delay_for(Duration::from_millis(500)).await;
            }
        });
        task.spawn(async move {
            for n in 1..15 {
                println!("2 test: {}", n);
                Task::delay_for(Duration::from_millis(500)).await;
            }
        });

        task.run();

        //loop {}
    }
}
