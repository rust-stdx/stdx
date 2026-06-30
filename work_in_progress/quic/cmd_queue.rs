#[cfg(feature = "tokio")]
mod imp {
    use tokio::sync::mpsc;

    pub(crate) struct CmdSender<T> {
        tx: mpsc::UnboundedSender<T>,
    }

    impl<T> CmdSender<T> {
        pub fn push(&self, item: T) {
            let _ = self.tx.send(item);
        }
    }

    impl<T> Clone for CmdSender<T> {
        fn clone(&self) -> Self {
            Self {
                tx: self.tx.clone(),
            }
        }
    }

    pub(crate) struct CmdReceiver<T> {
        rx: mpsc::UnboundedReceiver<T>,
    }

    impl<T> CmdReceiver<T> {
        pub fn try_recv(&mut self) -> Option<T> {
            self.rx.try_recv().ok()
        }

        pub async fn recv(&mut self) -> Option<T> {
            self.rx.recv().await
        }
    }

    pub(crate) fn cmd_queue<T>() -> (CmdSender<T>, CmdReceiver<T>) {
        let (tx, rx) = mpsc::unbounded_channel();
        (
            CmdSender {
                tx,
            },
            CmdReceiver {
                rx,
            },
        )
    }
}

#[cfg(not(feature = "tokio"))]
mod imp {
    use alloc::{collections::VecDeque, sync::Arc};
    use core::{
        cell::UnsafeCell,
        future::Future,
        pin::Pin,
        sync::atomic::{AtomicBool, AtomicU32, Ordering},
        task::{Context, Poll, Waker},
    };

    struct Inner<T> {
        locked: AtomicBool,
        queue: UnsafeCell<VecDeque<T>>,
        waker: UnsafeCell<Option<Waker>>,
        senders: AtomicU32,
    }

    unsafe impl<T: Send> Sync for Inner<T> {}
    unsafe impl<T: Send> Send for Inner<T> {}

    impl<T> Inner<T> {
        fn new() -> Self {
            Self {
                locked: AtomicBool::new(false),
                queue: UnsafeCell::new(VecDeque::new()),
                waker: UnsafeCell::new(None),
                senders: AtomicU32::new(1),
            }
        }

        fn with_lock<R>(&self, f: impl FnOnce(&mut VecDeque<T>, &mut Option<Waker>, u32) -> R) -> R {
            while self.locked.swap(true, Ordering::Acquire) {
                core::hint::spin_loop();
            }
            let result = f(
                unsafe { &mut *self.queue.get() },
                unsafe { &mut *self.waker.get() },
                self.senders.load(Ordering::Relaxed),
            );
            self.locked.store(false, Ordering::Release);
            result
        }
    }

    pub(crate) struct CmdSender<T> {
        inner: Arc<Inner<T>>,
    }

    impl<T> CmdSender<T> {
        pub fn push(&self, item: T) {
            let waker = self.inner.with_lock(|queue, waker, _| {
                queue.push_back(item);
                waker.take()
            });
            if let Some(w) = waker {
                w.wake();
            }
        }
    }

    impl<T> Clone for CmdSender<T> {
        fn clone(&self) -> Self {
            self.inner.senders.fetch_add(1, Ordering::Relaxed);
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    impl<T> Drop for CmdSender<T> {
        fn drop(&mut self) {
            let waker = if self.inner.senders.fetch_sub(1, Ordering::Relaxed) == 1 {
                self.inner.with_lock(|_queue, waker, _| waker.take())
            } else {
                None
            };
            if let Some(w) = waker {
                w.wake();
            }
        }
    }

    pub(crate) struct CmdReceiver<T> {
        inner: Arc<Inner<T>>,
    }

    impl<T> CmdReceiver<T> {
        pub fn try_recv(&mut self) -> Option<T> {
            self.inner.with_lock(|queue, _, _| queue.pop_front())
        }

        pub async fn recv(&mut self) -> Option<T> {
            RecvFuture {
                receiver: self,
            }
            .await
        }
    }

    struct RecvFuture<'a, T> {
        receiver: &'a mut CmdReceiver<T>,
    }

    impl<'a, T> Future for RecvFuture<'a, T> {
        type Output = Option<T>;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
            let inner = &*self.receiver.inner;
            inner.with_lock(|queue, waker, senders| {
                if let Some(item) = queue.pop_front() {
                    return Poll::Ready(Some(item));
                }
                *waker = Some(cx.waker().clone());
                if let Some(item) = queue.pop_front() {
                    return Poll::Ready(Some(item));
                }
                if senders == 0 {
                    return Poll::Ready(None);
                }
                Poll::Pending
            })
        }
    }

    pub(crate) fn cmd_queue<T>() -> (CmdSender<T>, CmdReceiver<T>) {
        let inner = Arc::new(Inner::new());
        (
            CmdSender {
                inner: inner.clone(),
            },
            CmdReceiver {
                inner,
            },
        )
    }
}

pub(crate) use imp::{CmdReceiver, CmdSender, cmd_queue};
