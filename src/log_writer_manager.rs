use crate::{
    data_writer::DataWrite,
    log_message_senders::{LogMessage, LOG_MESSAGE_SENDERS},
    log_writer::LogWriter,
};
use std::{collections::HashMap, sync::mpsc::channel};

///Configuration used to build logWriter
///
pub struct LoggerConfiguration {
    pub name: String,
    pub data_writer: Box<DataWrite>,
}

/// This module
///
///
pub struct LogWriterManager {
    loggers: HashMap<String, LogWriter>,
}
impl LogWriterManager {
    pub fn new() -> Self {
        Self {
            loggers: HashMap::new(),
        }
    }

    pub fn from_loggers<L>(log_configs: L) -> Result<Self, String>
    where
        L: Iterator<Item = LoggerConfiguration>,
    {
        let mut manager = Self::new();
        for config in log_configs {
            manager.add_logger(config)?
        }
        Ok(manager)
    }

    pub fn add_logger(&mut self, log_config: LoggerConfiguration) -> Result<(), String> {
        let name = log_config.name.clone();
        if let Some(_) = self.loggers.get(&name) {
            Err(format!(
                "The Logger name was already found in the manager. name={}",
                name
            ))
        } else {
            self.loggers
                .insert(name, Self::spawn_log_writer(log_config));
            Ok(())
        }
    }

    pub fn remove_logger(&mut self, name: String) -> Result<(), String> {
        match self.loggers.remove(&name) {
            Some(_) => Ok(()),
            None => Err(format!("The Logger to delete was not found. name={}", name)),
        }?;
        LOG_MESSAGE_SENDERS.drop_sender(name);
        Ok(())
    }

    ///spawn a log writer
    /// TODO :: handle the other config parameters more properly
    fn spawn_log_writer(log_config: LoggerConfiguration) -> LogWriter {
        let (sender, receiver) = channel::<LogMessage>();

        LOG_MESSAGE_SENDERS.add_sender(log_config.name.clone(), sender.clone());
        LogWriter::new(log_config, sender.clone(), receiver)
    }
}
