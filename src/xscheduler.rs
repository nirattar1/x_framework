
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

pub struct JobParams {
    // Delay before starting job.
    pub delay_secs: usize,
    // The job function (given as a function pointer)
    // TODO: support closures as well.
    pub job_func: fn(i32) -> i32,

    time_for_next_run: Instant,
}

impl JobParams {
    pub fn new(delay_secs: usize, job_func: fn(i32) -> i32) -> JobParams {
        return JobParams {
            delay_secs: delay_secs,
            job_func: job_func,
            time_for_next_run: Instant::now(),
        };
    }
}
pub struct XScheduler {
    m_queued_jobs: Vec<JobParams>,
    m_stop_running: bool,
}

impl XScheduler {
    pub fn new() -> XScheduler {
        return XScheduler { m_queued_jobs: Vec::new(), m_stop_running: false };
    }

    pub fn queue_job(&mut self, mut job: JobParams) -> bool {
        // define time for next run of job (based on now + delay)
        job.time_for_next_run = Instant::now()
            .checked_add(Duration::new(job.delay_secs as u64, 0))
            .unwrap();
        self.m_queued_jobs.push(job);
        true
    }

    pub fn run_job(&self) -> i32 {
        let handler = spawn(|| {
            Command::new("/bin/sleep").arg("5").status().expect("failed to execute process");
        });

        return 0;
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
                    //let res = (job.job_func)(3);
                    let res: i32 = self.run_job();
                    println!(
                        "{:?}: Job ran, result: {}",
                        Instant::now().duration_since(start_timestamp),
                        res
                    );

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
