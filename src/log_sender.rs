use crate::log_message_senders::{LogMessage, Loggable, LOG_MESSAGE_SENDERS};
use std::sync::mpsc::Sender;

/// entry point to send logs records to an AsyncLogWriter.
///
/// # Fields
///
/// * log_input : Sender Channel that sends to the receiver in the AsyncLogWriter
///
/// TODO :: make the error handling configurable?
pub struct LogSender {
    log_input: Sender<LogMessage>,
}
impl LogSender {
    /// Create and fetch the proper sender to send future records
    ///
    pub fn new(logger_name: String) -> Self {
        let log_input = LOG_MESSAGE_SENDERS
            .get_sender(logger_name)
            .expect("The current AsyncLogWriter shut down unexpectedly");
        Self { log_input }
    }

    ///Send a serializable record to the receiver specific AsyncLogWriter
    ///
    pub fn log(&self, record: Loggable) {
        self.log_input
            .send(LogMessage::Log(record))
            .expect("The current LogWriter shut down unexpectedly")
    }
}
