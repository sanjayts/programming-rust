use crossbeam_utils::sync::Parker;
use futures_lite::{pin, FutureExt};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

fn main() {
    let fut = spawn_blocking(|| {
        thread::sleep(Duration::from_secs(5));
        1200
    });
    let x = block_on(fut);
    println!("Answer is {}", x);
}

pub fn spawn_blocking<R, F>(closure: F) -> SpawnBlocking<R>
where
    F: FnOnce() -> R,
    F: Send + 'static,
    R: Send + 'static,
{
    let inner = Arc::new(Mutex::new(Shared {
        value: None,
        waker: None,
    }));
    thread::spawn({
        let inner = inner.clone();
        move || {
            let output = closure();
            let waker_maybe = {
                let mut guard = inner.lock().expect("Failed to lock");
                guard.value = Some(output);
                guard.waker.take()
            };
            if let Some(waker) = waker_maybe {
                waker.wake();
            }
        }
    });
    SpawnBlocking(inner)
}

pub struct SpawnBlocking<T>(Arc<Mutex<Shared<T>>>);

impl<R> Future for SpawnBlocking<R> {
    type Output = R;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.0.lock().expect("Failed to lock");
        if let Some(value) = guard.value.take() {
            return Poll::Ready(value);
        }
        let waker = cx.waker();
        guard.waker = Some(waker.clone());
        Poll::Pending
    }
}

struct Shared<T> {
    value: Option<T>,
    waker: Option<Waker>,
}

fn block_on<F: Future>(future: F) -> F::Output {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    let waker = waker_fn::waker_fn(move || unparker.unpark());
    let mut context = Context::from_waker(&waker);

    pin!(future);

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => {
                println!("Value ready -- returning");
                return value
            },
            Poll::Pending => {
                println!("Value not ready -- parking the thread");
                parker.park()
            },
        }
    }
}
