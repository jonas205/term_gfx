#[cfg(feature = "profiling")]
use std::{
    fs::File,
    io::{BufWriter, Error, Write},
    sync::{mpsc, Mutex},
    thread,
    time::{Duration, Instant},
};

#[cfg(not(feature = "profiling"))]
use std::io::Error;

#[cfg(feature = "profiling")]
#[macro_export]
macro_rules! profile {
    () => {
        let name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            name.strip_suffix("::f").unwrap()
        };
        let _timer = $crate::profiler::Timer::new(name.to_string());
    };
    ($name:expr) => {
        let name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            let mut name = name.strip_suffix("::f").unwrap().to_string();
            name.push(' ');
            name.push_str($name);
            name
        };
        let _timer = $crate::profiler::Timer::new(name);
    };
}

#[cfg(not(feature = "profiling"))]
#[macro_export]
macro_rules! profile {
    () => {};
    ($name:expr) => {};
}

pub struct Timer {
#[cfg(feature = "profiling")]
    name: Option<String>,
#[cfg(feature = "profiling")]
    start: Instant,
}

impl Timer {
    #[cfg(not(feature = "profiling"))]
    pub fn new(_name: String) -> Timer { Timer {} }
    #[cfg(feature = "profiling")]
    pub fn new(name: String) -> Timer {
        Timer {
            name: Some(name),
            start: Instant::now(),
        }
    }
}

#[cfg(feature = "profiling")]
impl Drop for Timer {
    fn drop(&mut self) {
        let stop = Instant::now();
        if let Some(p) = PROFILER.lock().unwrap().as_mut() {
            p.send((self.name.take().unwrap(), self.start, stop - self.start))
                .unwrap();
        }
    }
}

#[cfg(feature = "profiling")]
lazy_static! {
    static ref PROFILER: Mutex<Option<mpsc::Sender<(String, Instant, Duration)>>> =
        Mutex::new(None);
}

pub(crate) struct Profiler {
    #[cfg(feature = "profiling")]
    thread: Option<thread::JoinHandle<()>>,
}

impl Profiler {
    #[cfg(not(feature = "profiling"))]
    pub fn new() -> Result<Profiler, Error> { Ok(Profiler {}) }

    #[cfg(feature = "profiling")]
    pub fn new() -> Result<Profiler, Error> {
        let (sender, receiver) = mpsc::channel();

        let file = match File::create("term_gfx.json") {
            Ok(f) => f,
            Err(e) => return Err(e),
        };
        let mut file = BufWriter::new(file);
        file.write(b"{\"otherData\": {}, \"traceEvents\":[")
            .unwrap();
        let profile_start = Instant::now();

        *PROFILER.lock().unwrap() = Some(sender);
        let handle = thread::spawn(move || {
            let mut first = true;

            loop {
                match receiver.recv() {
                    Ok((name, start, time)) => {
                        let name = name.replace('\"', "\\\"");
                        let start = start.duration_since(profile_start).as_nanos() as f64 / 1000.0;
                        let time = time.as_nanos() as f64 / 1000.0;

                        if first {
                            first = false;
                        } else {
                            file.write(b",").unwrap();
                        }

                        file.write(format!("{{\"cat\":\"function\",\"dur\":{},\"name\":\"{}\",\"ph\":\"X\",\"pid\":0,\"tid\":0,\"ts\":{}}}", time, name, start).as_bytes()).unwrap();
                    }
                    Err(_) => {
                        break;
                    }
                }
            }

            file.write(b"]}").unwrap();
            file.flush().unwrap();
        });

        Ok(Profiler {
            thread: Some(handle),
        })
    }
}

#[cfg(feature = "profiling")]
impl Drop for Profiler {
    fn drop(&mut self) {
        drop(PROFILER.lock().unwrap().take());
        self.thread.take().unwrap().join().unwrap();
    }
}
