use std::error;
use std::fmt;

#[derive(Debug)]
pub enum ProfError {
    MemProfilingNotEnabled,
    CPUProfilingStartFailed,
    CPUProfilingStopFailed,
    IOError(std::io::Error),
    JemallocError(i32),
    PathError(std::ffi::OsString), // When temp files are in a non-unicode directory, OsString.into_string() will cause this error
}

pub type ProfResult<T> = std::result::Result<T, ProfError>;

impl fmt::Display for ProfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfError::MemProfilingNotEnabled => write!(f, "mem-profiling was not enabled"),
            ProfError::CPUProfilingStartFailed => write!(f, "start cpu-profiling failed"),
            ProfError::CPUProfilingStopFailed => write!(f, "stop cpu-profiling failed"),
            ProfError::IOError(e) => write!(f, "io error occurred {:?}", e),
            ProfError::JemallocError(e) => write!(f, "jemalloc error {}", e),
            ProfError::PathError(path) => write!(f, "Dump target path {:?} is non-unicode ", path),
        }
    }
}

impl From<std::io::Error> for ProfError {
    fn from(e: std::io::Error) -> Self {
        ProfError::IOError(e)
    }
}

impl error::Error for ProfError {}
