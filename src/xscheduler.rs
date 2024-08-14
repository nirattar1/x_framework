/**
 * In-process job scheduler library.
 *
 * The client app will use this scheduler to schedule jobs.
 * Jobs run in a separate (single) job thread.
 */

use std::thread::spawn;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;
use std::process::Command;
use std::string::String;
use std::vec::Vec;
use std::process::Child;
use std::io::Result;

// A struct representing a system job (task) to be be run.
pub struct JobParams {
    // Delay before starting job.
    pub delay_secs: usize,
    // The job command (given as a string)
    job_cmd: String,
    job_args: Vec<String>,

    time_for_next_run: Instant,
}

impl JobParams {
    pub fn new(delay_secs: usize, job_cmd: String, job_args: Vec<String>) -> JobParams {
        return JobParams {
            delay_secs: delay_secs,
            job_cmd: job_cmd,
            job_args: job_args,
            time_for_next_run: Instant::now(),
        };
    }
}
pub struct XScheduler {
    m_queued_jobs: Vec<JobParams>,
    m_stop_running: bool,
    m_runnning_jobs: Vec<Child>,
}

impl XScheduler {
    pub fn new() -> XScheduler {
        return XScheduler {
            m_queued_jobs: Vec::new(),
            m_stop_running: false,
            m_runnning_jobs: vec![],
        };
    }

    pub fn queue_job(&mut self, mut job: JobParams) -> bool {
        // define time for next run of job (based on now + delay)
        job.time_for_next_run = Instant::now()
            .checked_add(Duration::new(job.delay_secs as u64, 0))
            .unwrap();
        self.m_queued_jobs.push(job);
        true
    }

    pub fn run_job(&self, job: &JobParams) -> Result<Child> {
        let mut cmd = Command::new(job.job_cmd.clone());
        cmd.args(job.job_args.clone());
        cmd.spawn()
    }

    pub fn run_event_loop(&mut self) {
        let start_timestamp = Instant::now();
        loop {
            // sleep interval between cycles
            sleep(Duration::from_millis(100));

            // evaluate ready jobs
            let mut i: usize = 0;
            while i < self.m_queued_jobs.len() {
                let job: &JobParams = &self.m_queued_jobs[i];

                println!("iterating job");
                let now = Instant::now();

                // Job is ready for run based on delay.
                if job.time_for_next_run <= now {
                    println!("{:?}: Starting job", now.duration_since(start_timestamp));
                    let res: Result<Child> = self.run_job(job);
                    let child = res.expect("command : failed to start!");
                    println!(
                        "{:?}: Job started, pid: {}",
                        Instant::now().duration_since(start_timestamp),
                        child.id()
                    );
                    self.m_runnning_jobs.push(child);

                    // job finished, TODO remove it from queue.
                    self.m_queued_jobs.remove(i);
                    // TODO retry failed jobs.
                } else {
                    i += 1;
                }
            }
            if self.m_stop_running == true {
                break;
            }
        }
    }
}
