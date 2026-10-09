use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    time::Duration,
};

use time::{DateTime, TimeZone};
use tokio::{sync::RwLock, task::JoinHandle};
#[cfg(feature = "tracing")]
use tracing::*;

use crate::{
    error::SchedulerError,
    job::{Job, JobScheduler},
    scheduler::{Scheduler, TryToScheduler},
};

pub mod error;
pub mod job;
pub mod scheduler;

#[derive(Clone)]
pub struct JobExecutor {
    executor: Arc<JobExecutorInternal>,
}

struct JobExecutorInternal {
    sleep_between_checks_ms: AtomicU64,
    running: AtomicBool,
    jobs: RwLock<Vec<Arc<JobScheduler>>>,
}

impl JobExecutorInternal {
    /*
        /// Returns true if the JobExecutor contains no jobs.
        pub async fn is_empty(&self) -> bool {
            let jobs = self.jobs.read().await;
            jobs.is_empty()
        }

        /// Returns the number of jobs in the JobExecutor.
        pub async fn len(&self) -> usize {
            let jobs = self.jobs.read().await;
            jobs.len()
        }

        /// Clear the JobExecutor, removing all jobs.
        pub async fn clear(&mut self) {
            let mut jobs = self.jobs.write().await;
            jobs.clear()
        }

        /// Returns true if there is at least one job pending.
        pub async fn is_pending_job(&self) -> bool {
            let jobs = self.jobs.read().await;
            for job_scheduler in jobs.iter() {
                if job_scheduler.is_pending().await {
                    return true;
                }
            }
            false
        }
    */
    /// Returns true if the Job Executor is running
    fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Returns true if there is at least one job running.
    async fn is_running_job(&self) -> bool {
        let jobs = self.jobs.read().await;
        for job_scheduler in jobs.iter() {
            if job_scheduler.job.is_running().await {
                return true;
            }
        }
        false
    }

    /// Run pending jobs in the JobExecutor.
    async fn run_pending_jobs(&self) {
        #[cfg(feature = "tracing")]
        trace!("Check pending jobs");
        let jobs = self.jobs.read().await;
        for job_scheduler in jobs.iter() {
            //println!("check JOB IS PENDING: {}", job.is_pending());
            if job_scheduler.is_pending().await {
                //println!("JOB IS RUNNING? {}", is_running);
                if !job_scheduler.job.is_running().await {
                    let job_clone = job_scheduler.clone();

                    let timestamp = DateTime::now_in(TimeZone::UTC).unix_seconds();
                    let group = job_clone.job.group().to_owned();
                    let name = job_clone.job.name().to_owned();

                    let fut = instrument(timestamp, group.clone(), name.clone(), async move {
                        #[cfg(feature = "tracing")]
                        info!("scheduler: starting task {name}");
                        let start = std::time::Instant::now();
                        let result = job_clone.run().await;

                        let duration = start.elapsed();

                        match result {
                            Ok(()) => {
                                #[cfg(feature = "tracing")]
                                info!("scheduler: task {name} completed successfully in {duration:?}")
                            }
                            Err(err) => {
                                #[cfg(feature = "tracing")]
                                error!("scheduler: task {name} failed in {duration:?} with errors: Err: {err:?}")
                            }
                        }
                    });

                    tokio::spawn(fut);
                } else {
                    #[cfg(feature = "tracing")]
                    debug!(
                        "Job [{}/{}] is pending but already running. It will not be executed.",
                        job_scheduler.job.group(),
                        job_scheduler.job.name()
                    )
                }
            }
        }
    }

    /// Adds a job to the JobExecutor.
    async fn add_job_with_scheduler<S: Into<Scheduler>>(&self, schedule: S, job: Job) {
        #[cfg(feature = "tracing")]
        debug!("Add job to scheduler. Group [{}] - Name [{}]", job.group(), job.name());
        let mut jobs = self.jobs.write().await;
        jobs.push(Arc::new(JobScheduler::new(schedule.into(), job)));
    }
}

impl JobExecutor {
    /// Creates a new Executor that uses the Local time zone for the execution times evaluation.
    /// For example, the cron expressions will refer to the Local time zone.
    pub fn new_with_local_tz() -> JobExecutor {
        Self::new_with_tz()
    }

    /// Creates a new Executor that uses the UTC time zone for the execution times evaluation.
    /// For example, the cron expressions will refer to the UTC time zone.
    pub fn new_with_utc_tz() -> JobExecutor {
        Self::new_with_tz()
    }

    /// Creates a new Executor that uses a custom time zone for the execution times evaluation.
    /// For example, the cron expressions will refer to the specified time zone.
    pub fn new_with_tz() -> JobExecutor {
        JobExecutor {
            executor: Arc::new(JobExecutorInternal {
                sleep_between_checks_ms: AtomicU64::new(1_000),
                running: AtomicBool::new(false),
                jobs: RwLock::new(vec![]),
            }),
        }
    }

    /// Adds a job to the JobExecutor.
    pub async fn add_job(&self, schedule: &dyn TryToScheduler, job: Job) -> Result<(), SchedulerError> {
        self.add_job_with_scheduler(schedule.to_scheduler()?, job).await;
        Ok(())
    }

    /// Adds a job to the JobExecutor.
    pub async fn add_job_with_multi_schedule(
        &self,
        schedule: &[&dyn TryToScheduler],
        job: Job,
    ) -> Result<(), SchedulerError> {
        self.add_job_with_scheduler(schedule.to_scheduler()?, job).await;
        Ok(())
    }

    /// Adds a job to the JobExecutor.
    pub async fn add_job_with_scheduler<S: Into<Scheduler>>(&self, schedule: S, job: Job) {
        self.executor.add_job_with_scheduler(schedule, job).await
    }

    /// Starts the JobExecutor
    pub async fn run(&self) -> Result<JoinHandle<()>, SchedulerError> {
        let was_running = self.executor.running.swap(true, Ordering::SeqCst);
        if !was_running {
            let executor = self.executor.clone();
            Ok(tokio::spawn(async move {
                #[cfg(feature = "tracing")]
                debug!("Starting the job executor");
                while executor.is_running() {
                    executor.run_pending_jobs().await;
                    tokio::time::sleep(Duration::from_millis(executor.sleep_between_checks_ms.load(Ordering::SeqCst)))
                        .await;
                }
                #[cfg(feature = "tracing")]
                debug!("Job executor stopped");
            }))
        } else {
            #[cfg(feature = "tracing")]
            warn!("The JobExecutor is already running.");
            Err(SchedulerError::JobExecutionStateError {
                message: "The JobExecutor is already running.".to_owned(),
            })
        }
    }

    /// Stops the JobExecutor
    pub async fn stop(&self, graceful: bool) -> Result<(), SchedulerError> {
        let was_running = self.executor.running.swap(false, Ordering::SeqCst);
        if was_running {
            #[cfg(feature = "tracing")]
            debug!("Stopping the job executor");
            if graceful {
                #[cfg(feature = "tracing")]
                debug!("Wait for all Jobs to complete");
                while self.executor.is_running_job().await {
                    tokio::time::sleep(Duration::from_millis(
                        self.executor.sleep_between_checks_ms.load(Ordering::SeqCst),
                    ))
                    .await;
                }
                #[cfg(feature = "tracing")]
                debug!("All Jobs completed");
            }
            Ok(())
        } else {
            #[cfg(feature = "tracing")]
            warn!("The JobExecutor is not running.");
            Err(SchedulerError::JobExecutionStateError {
                message: "The JobExecutor is not running.".to_owned(),
            })
        }
    }

    /// Sets the sleep time between checks for pending Jobs.
    /// The default is 1 second.
    pub fn set_sleep_between_checks(&self, sleep_ms: u64) {
        self.executor.sleep_between_checks_ms.store(sleep_ms, Ordering::SeqCst);
    }
}

#[cfg(feature = "tracing")]
fn instrument<F: std::future::Future<Output = ()>>(
    timestamp: i64,
    group: String,
    name: String,
    fut: F,
) -> impl std::future::Future<Output = ()> {
    let span = tracing::error_span!("run_pending", group, name, timestamp);
    fut.instrument(span)
}

#[cfg(not(feature = "tracing"))]
fn instrument<F: std::future::Future<Output = ()>>(
    _timestamp: i64,
    _group: String,
    _name: String,
    fut: F,
) -> impl std::future::Future<Output = ()> {
    fut
}

#[cfg(test)]
pub mod test {

    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };

    use time::{DateTime, TimeZone};
    use tokio::sync::mpsc::channel;

    use super::*;

    #[tokio::test]
    async fn should_not_run_an_already_running_job() {
        let executor = JobExecutor::new_with_utc_tz();

        let count = Arc::new(AtomicUsize::new(0));
        let count_clone = count.clone();

        let (tx, mut rx) = channel(1000);

        executor
            .add_job(
                &Duration::new(0, 1),
                Job::new("g", "n", None, move || {
                    let count_clone = count_clone.clone();
                    let tx = tx.clone();
                    Box::pin(async move {
                        tx.send("").await.unwrap();
                        println!("job - started");
                        count_clone.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(Duration::new(1, 0)).await;
                        Ok(())
                    })
                }),
            )
            .await
            .unwrap();

        for i in 0..100 {
            println!("run_pending {i}");
            executor.executor.run_pending_jobs().await;
            tokio::time::sleep(Duration::new(0, 2)).await;
        }

        println!("run_pending completed");
        rx.recv().await.unwrap();

        assert_eq!(count.load(Ordering::Relaxed), 1);
    }

    #[tokio::test]
    async fn a_running_job_should_not_block_the_executor() {
        let executor = JobExecutor::new_with_local_tz();

        let (tx, mut rx) = channel(959898);

        let count_1 = Arc::new(AtomicUsize::new(0));
        let count_1_clone = count_1.clone();
        let tx_1 = tx.clone();
        executor
            .add_job_with_multi_schedule(
                &[&Duration::new(0, 1)],
                Job::new("g", "n", None, move || {
                    let count_1_clone = count_1_clone.clone();
                    let tx_1 = tx_1.clone();
                    Box::pin(async move {
                        tx_1.send("").await.unwrap();
                        println!("job 1 - started");
                        count_1_clone.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(Duration::new(1, 0)).await;
                        Ok(())
                    })
                }),
            )
            .await
            .unwrap();

        let count_2 = Arc::new(AtomicUsize::new(0));
        let count_2_clone = count_2.clone();
        let tx_2 = tx.clone();
        executor
            .add_job(
                &Duration::new(0, 1),
                Job::new("g", "n", None, move || {
                    let count_2_clone = count_2_clone.clone();
                    let tx_2 = tx_2.clone();
                    Box::pin(async move {
                        tx_2.send("").await.unwrap();
                        println!("job 2 - started");
                        count_2_clone.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(Duration::new(1, 0)).await;
                        Ok(())
                    })
                }),
            )
            .await
            .unwrap();

        let count_3 = Arc::new(AtomicUsize::new(0));
        let count_3_clone = count_3.clone();
        let tx_3 = tx.clone();
        executor
            .add_job(
                &Duration::new(0, 1),
                Job::new("g", "n", None, move || {
                    let count_3_clone = count_3_clone.clone();
                    let tx_3 = tx_3.clone();
                    Box::pin(async move {
                        tx_3.send("").await.unwrap();
                        println!("job 3 - started");
                        count_3_clone.fetch_add(1, Ordering::SeqCst);
                        tokio::time::sleep(Duration::new(1, 0)).await;
                        Ok(())
                    })
                }),
            )
            .await
            .unwrap();

        let before_millis = DateTime::now_in(TimeZone::UTC).unix_nanos() / 1_000_000;
        for i in 0..100 {
            println!("run_pending {i}");
            executor.executor.run_pending_jobs().await;
            tokio::time::sleep(Duration::new(0, 1_000_000)).await;
        }
        let after_millis = DateTime::now_in(TimeZone::UTC).unix_nanos() / 1_000_000;

        assert!((after_millis - before_millis) >= 100);
        assert!((after_millis - before_millis) < 1000);

        rx.recv().await.unwrap();

        assert_eq!(count_1.load(Ordering::SeqCst), 1);
        assert_eq!(count_2.load(Ordering::SeqCst), 1);
        assert_eq!(count_3.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn should_gracefully_shutdown_the_job_executor() {
        let executor = JobExecutor::new_with_utc_tz();

        let count = Arc::new(AtomicUsize::new(0));

        let tasks = 100;

        for _i in 0..tasks {
            let count_clone = count.clone();
            executor
                .add_job(
                    &Duration::new(0, 1),
                    Job::new("g", "n", None, move || {
                        let count_clone = count_clone.clone();
                        Box::pin(async move {
                            tokio::time::sleep(Duration::new(1, 0)).await;
                            println!("job - started");
                            count_clone.fetch_add(1, Ordering::SeqCst);
                            Ok(())
                        })
                    }),
                )
                .await
                .unwrap();
        }

        executor.set_sleep_between_checks(10);

        executor.run().await.unwrap();

        loop {
            if executor.executor.is_running_job().await {
                break;
            }
            tokio::time::sleep(Duration::from_nanos(1)).await;
        }

        executor.stop(true).await.unwrap();

        assert_eq!(count.load(Ordering::Relaxed), tasks);
    }

    #[tokio::test]
    async fn start_should_fail_if_already_running() {
        let executor = JobExecutor::new_with_utc_tz();
        assert!(executor.run().await.is_ok());
        assert!(executor.run().await.is_err());
        assert!(executor.stop(false).await.is_ok());
    }

    #[tokio::test]
    async fn stop_should_fail_if_not_running() {
        let executor = JobExecutor::new_with_utc_tz();
        assert!(executor.stop(false).await.is_err());
        assert!(executor.run().await.is_ok());
        assert!(executor.stop(false).await.is_ok());
        assert!(executor.stop(false).await.is_err());
    }

    #[tokio::test]
    async fn should_add_with_explicit_scheduler() {
        let executor = JobExecutor::new_with_utc_tz();
        executor
            .add_job_with_scheduler(Scheduler::Never, Job::new("g", "n", None, move || Box::pin(async { Ok(()) })))
            .await;
    }

    #[tokio::test]
    async fn should_register_a_schedule_by_vec() {
        let executor = JobExecutor::new_with_utc_tz();
        executor
            .add_job(
                &vec!["0 1 * * * * *"],
                Job::new("g", "n", None, move || Box::pin(async { Ok(()) })),
            )
            .await
            .unwrap();
        executor
            .add_job(
                &vec!["0 1 * * * * *".to_owned(), "0 1 * * * * *".to_owned()],
                Job::new("g", "n", None, move || Box::pin(async { Ok(()) })),
            )
            .await
            .unwrap();
    }
}
