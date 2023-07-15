use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

mod t {
    use std::fmt::Debug;
    use std::pin::Pin;

    trait SayHi: Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }

    trait MutMeSomehow
    where
        Self: Default,
    {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            self.set(Self::default())
        }
    }

    impl<T: Debug> SayHi for T {}
    impl<T: Default> MutMeSomehow for T {}
}

mod other {
    use std::fmt::Debug;
    use std::pin::Pin;
    use std::rc::Rc;

    trait SayHi: Debug {
        fn say_hi(self: Pin<&Self>) {
            println!("Hi from {:?}", self)
        }
    }

    trait MutMeSomehow
    where
        Self: Default,
    {
        fn mut_me_somehow(mut self: Pin<&mut Self>) {
            self.set(Self::default())
        }
    }

    impl<T: Debug> SayHi for Box<T> {}
    impl<T: Default> MutMeSomehow for Box<T> {}

    impl<T: Debug> SayHi for Rc<T> {}
    impl<T: Default> MutMeSomehow for Rc<T> {}

    impl<T: Debug> SayHi for Vec<T> {}
    impl<T> MutMeSomehow for Vec<T> {}

    impl SayHi for String {}
    impl MutMeSomehow for String {}

    impl SayHi for &[u8] {}
    impl MutMeSomehow for &[u8] {}
}

#[pin_project::pin_project]
struct MeasurableFuture<Fut> {
    #[pin]
    inner_future: Fut,
    started_at: Option<Instant>,
}

impl<Fut: Future> Future for MeasurableFuture<Fut> {
    type Output = (Fut::Output, Duration);

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut this = self.project();

        let start = this.started_at.get_or_insert_with(Instant::now);
        let inner_poll = this.inner_future.as_mut().poll(cx);
        let elapsed = start.elapsed();

        match inner_poll {
            Poll::Pending => Poll::Pending,
            Poll::Ready(output) => Poll::Ready((output, elapsed)),
        }
    }
}

fn main() {
    println!("Implement me!");
}
