/**
 * In-process job scheduler library.
 *
 * The client app will use this scheduler to schedule jobs.
 * Jobs run in a separate (single) job thread.
 */

use std::thread;
use std::thread::sleep;
use std::thread::JoinHandle;
use std::time::Duration;
use std::time::Instant;
use std::process::Command;
use std::string::String;
use std::vec::Vec;
use std::process::Child;
use std::io::Result;
use std::sync::Mutex;
use std::sync::Arc;

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
    m_queued_jobs: Mutex<Vec<JobParams>>,
    m_stop_running: bool,
    m_runnning_jobs: Mutex<Vec<Child>>,
}

// pub fn run_event_loop() {
//     let mut s: XScheduler = XScheduler::new();

//     let handle = thread::spawn(move || { s.run_event_loop_internal() });

//     handle.join().unwrap();
// }

static REQUEST_QUEUE: Vec<i32> = vec![];

// pub fn run_event_loop() {
//     let mut queue = Arc::new(Mutex::new(REQUEST_QUEUE.clone()));
//     println!("created request queue");
//     let child_ptr = queue.clone();
//     let child_thread = thread::spawn(move || {
//         match child_ptr.lock() {
//             Ok(mut data) => {
//                 // here we have exclusive access to the data
//                 data.push(2);
//             }
//             Err(e) => {
//                 println!("Failed to get a lock: {}", e);
//             }
//         }
//     });

//     thread::sleep(Duration::from_millis(5000));

//     let mut scheduler: XScheduler = XScheduler::new();
//     scheduler.run_event_loop_internal();

//     child_thread.join().unwrap();
// }


pub fn start_scheduler_event_loop() -> (JoinHandle<()>, Arc<XScheduler>) {

    let scheduler_ptr = Arc::new(XScheduler::new());

    println!("created scheduler");
    let child_ptr = scheduler_ptr.clone();
    let child_thread = thread::spawn(move || {
        println!("started child");
        child_ptr.run_event_loop_internal();
    });

    thread::sleep(Duration::from_millis(500));

    return (child_thread, scheduler_ptr);
}


impl XScheduler {
    pub fn new() -> XScheduler {
        return XScheduler {
            m_queued_jobs: Mutex::new(Vec::new()),
            m_stop_running: false,
            m_runnning_jobs: Mutex::new(vec![]),
        };
    }

    pub fn queue_job(&self, mut job: JobParams) -> bool {
        // define time for next run of job (based on now + delay)
        job.time_for_next_run = Instant::now()
            .checked_add(Duration::new(job.delay_secs as u64, 0))
            .unwrap();

            println!("{:?}: queued job", &job.job_cmd);

            self.m_queued_jobs.lock().unwrap().push(job);

        true
    }

    pub fn run_job(&self, job: &JobParams) -> Result<Child> {
        let mut cmd = Command::new(job.job_cmd.clone());
        cmd.args(job.job_args.clone());
        cmd.spawn()
    }

    pub fn run_event_loop_internal(& self) {
        let start_timestamp = Instant::now();
        println!("started run_event_loop_internal");

        loop {
            // sleep interval between cycles
            sleep(Duration::from_millis(100));

            // evaluate ready jobs
            let mut i: usize = 0;
            let mut queue = self.m_queued_jobs.lock().unwrap();
            while i < queue.len() {
                let job: &JobParams = &queue[i];

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
                    self.m_runnning_jobs.lock().unwrap().push(child);

                    // job finished, TODO remove it from queue.
                    queue.remove(i);
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
