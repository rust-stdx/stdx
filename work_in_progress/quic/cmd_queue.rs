use tokio::sync::mpsc;

pub(crate) struct CmdSender<T> {
    tx: mpsc::UnboundedSender<T>,
}

pub(crate) struct CmdReceiver<T> {
    rx: mpsc::UnboundedReceiver<T>,
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
