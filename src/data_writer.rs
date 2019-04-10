use serde_json;
use std::fs::File;
use std::io::Write;

/// Trait that describes
/// the mechanism of writing data in a file
pub trait DataWrite: Send + Sync {
    fn open(filename: &str) -> Self
    where
        Self: Sized;
    fn write(&mut self, record: Box<dyn erased_serde::Serialize>);
}

/// Writer that writes data in a csv format in a specified file
///
///
pub struct CsvWriter {
    csv_write: csv::Writer<File>,
}
impl DataWrite for CsvWriter {
    fn open(filename: &str) -> Self {
        let filename = [filename, ".csv"].concat();

        let file = File::create(filename.to_string()).unwrap();
        Self {
            csv_write: csv::Writer::from_writer(file),
        }
    }

    fn write(&mut self, record: Box<dyn erased_serde::Serialize>) {
        self.csv_write.serialize(record).unwrap();
    }
}

/// Logger that writes data in a json format in a specified file
///
///
pub struct NdJsonWriter {
    file_writer: File,
}

impl DataWrite for NdJsonWriter {
    fn open(filename: &str) -> Self {
        let filename = [filename, ".ndjson"].concat();
        Self {
            file_writer: File::create(filename.to_string()).unwrap(),
        }
    }

    fn write(&mut self, record: Box<dyn erased_serde::Serialize>) {
        let mut json = serde_json::to_string(&record).unwrap();
        json.push('\n');
        self.file_writer.write_all(json.as_bytes()).unwrap();
    }
}
