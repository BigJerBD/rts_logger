use crate::log_message_senders::LogMessage;
use crate::log_writer_manager::LoggerConfiguration;
use std::{
    sync::mpsc::{Receiver, Sender},
    thread,
};

/// Log writer that listens for new data to be written to a specific file
///
pub struct LogWriter {
    queue: Sender<LogMessage>,
    worker_thread: Option<thread::JoinHandle<()>>,
}

impl LogWriter {
    pub fn new(
        log_config: LoggerConfiguration,
        queue: Sender<LogMessage>,
        receiver: Receiver<LogMessage>,
    ) -> Self {
        Self {
            queue,
            worker_thread: Some(thread::Builder::new().name(log_config.name.clone()).spawn(
                move || {
                    let mut log_type = log_config.data_writer;
                    loop {
                        let msg = receiver.recv();

                        match msg {
                            Ok(LogMessage::Log(msg)) => &log_type.write(msg),
                            Ok(LogMessage::Quit) => break,
                            Err(_) => panic!("The current LogWriter closed unexpectedly without sending a Quit first"),
                        };
                    }
                }).unwrap()),
        }
    }
}

impl Drop for LogWriter {
    fn drop(&mut self) {

        self.queue
            .send(LogMessage::Quit)
            .expect("The current LogWriter's queue was closed before joining thread");

        let join_handle = self.worker_thread
            .take()
            .expect("Cannot take worker thread when it does not exist");

        join_handle
            .join()
            .expect("Failed to join the worker thread when dropping current LogWriter");
    }
}
