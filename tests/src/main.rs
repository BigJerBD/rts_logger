/*! This program is used to test the performance*/
#[macro_use]
extern crate bencher;
#[macro_use]
extern crate serde_derive;
use bencher::Bencher;

benchmark_group!(
    benches,
    benchmarks::bench_nonblocking,
    benchmarks::bench_blocking
);
benchmark_main!(benches);

mod utils {
    use std::fs;

    pub static LOG_DIRECTORY: &str = "tmp_benchmark_logs";

    #[derive(Serialize)]
    pub struct TestData {
        pub a: f64,
        pub b: f64,
    }

    pub fn setup() {
        let _ = fs::remove_dir_all(LOG_DIRECTORY);
        let _ = fs::create_dir(LOG_DIRECTORY);
    }
    pub fn teardown() {
        let _ = fs::remove_dir_all(LOG_DIRECTORY);
    }
}

/// TODO :: add tests
#[cfg(test)]
mod tests {}

mod benchmarks {
    use super::utils::*;
    use super::*;
    use rts_logger::{
        data_writer::{DataWrite, NdJsonWriter},
        LogSender, LogWriterManager, LoggerConfiguration,
    };

    /// benchmark the performances of logging
    /// by directly sending data to write
    ///
    pub fn bench_blocking(bench: &mut Bencher) {
        let file_path = format!("{}/log2", LOG_DIRECTORY);
        setup();

        let mut logger = NdJsonWriter::open(&file_path);
        bench.iter(|| logger.write(Box::new(TestData { a: 1.0, b: 2.0 })));

        teardown()
    }

    /// benchmark the performances of logging
    /// using asynchronous method with rust's channels
    ///
    pub fn bench_nonblocking(bench: &mut Bencher) {
        let file_path = format!("{}/log1", LOG_DIRECTORY);
        let logger_name = String::from("log1");
        setup();
        let _manager = LogWriterManager::from_loggers(
            vec![
                LoggerConfiguration {
                    name: logger_name.clone(),
                    data_writer: Box::new(NdJsonWriter::open(&file_path)),
                },
                //other files can be added here
            ]
            .into_iter(),
        );

        let logger = LogSender::new(logger_name.clone());
        bench.iter(|| logger.log(Box::new(TestData { a: 1.0, b: 2.0 })));

        teardown()
    }
}
