#[macro_use]
extern crate lazy_static;

pub mod data_writer;
pub mod log_message_senders;
pub mod log_sender;
pub mod log_writer;
pub mod log_writer_manager;


pub use crate::{
    log_sender::LogSender,
    log_writer_manager::{LogWriterManager,LoggerConfiguration},
    log_message_senders::LogMessage,
};

