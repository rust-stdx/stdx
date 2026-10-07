use core::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum SchedulerError {
    ScheduleDefinitionError {
        message: String,
    },
    JobLockError {
        message: String,
    },
    JobExecutionStateError {
        message: String,
    },
    JobExecutionError {
        source: Box<dyn core::error::Error + Send + Sync>,
    },
    JobExecutionPanic {
        cause: String,
    },
}

impl Display for SchedulerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            SchedulerError::ScheduleDefinitionError {
                message,
            } => {
                write!(f, "ScheduleDefinitionError: [{message}]")
            }
            SchedulerError::JobLockError {
                message,
            } => write!(f, "JobLockError: [{message}]"),
            SchedulerError::JobExecutionStateError {
                message,
            } => {
                write!(f, "JobExecutionStateError: [{message}]")
            }
            SchedulerError::JobExecutionError {
                ..
            } => write!(f, "JobExecutionError"),
            SchedulerError::JobExecutionPanic {
                cause,
            } => {
                write!(f, "JobExecutionPanic: [{cause}]")
            }
        }
    }
}

impl Error for SchedulerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SchedulerError::ScheduleDefinitionError {
                ..
            }
            | SchedulerError::JobLockError {
                ..
            }
            | SchedulerError::JobExecutionStateError {
                ..
            }
            | SchedulerError::JobExecutionPanic {
                ..
            } => None,

            SchedulerError::JobExecutionError {
                source,
            } => Some(source.as_ref()),
        }
    }
}
