use crossbeam_channel::*;
use std::convert::TryInto;
use std::thread;
use std::time::{Duration, Instant};

/// Setup to test the latency of a crossbeam channel
/// should probably run as root, panic otherwise most likely
fn main() {
    let iters: u32 = 100;
    let pin_prio = true;
    if pin_prio {
        set_thread_priority(20);
        pin_thread_to_cpu(0).unwrap();
    }
    let mut v = Vec::with_capacity(iters.try_into().unwrap());
    for i in 0..iters {
        v.push(bench(1000, pin_prio));
        println!(
            "iter {} of {} finished with {:?}",
            i + 1,
            iters,
            v.last().unwrap()
        );
        thread::sleep(Duration::from_millis(1));
    }
    let mean = v
        .iter()
        .fold(Duration::from_nanos(0), |sum, val| sum + *val)
        / iters;
    let stddev = v.iter().fold(0u128, |sum, val| {
        sum + (mean.as_nanos() - val.as_nanos()).pow(2)
    });
    let stddev = (stddev as f64 / iters as f64).sqrt();
    println!("mean: {:?} stddev: {:0.3}us", mean, stddev as f64 / 1000.);
    print_rusage();
}

fn print_rusage() {
    println!("Thread stats");
    unsafe {
        let mut r: libc::rusage = std::mem::uninitialized();
        let ret = libc::getrusage(libc::RUSAGE_THREAD, &mut r);
        if ret != 0 {
            panic!();
        }
        println!(
            r#"rusage {{
    ru_minflt: {},
    ru_majflt: {},
    ru_inblock: {},
    ru_oublock: {},
    ru_nvcsw: {},
    ru_nivcsw: {},
}}"#,
            r.ru_minflt, r.ru_majflt, r.ru_inblock, r.ru_oublock, r.ru_nvcsw, r.ru_nvcsw
        )
    }
}

fn set_thread_priority(prio: i32) {
    unsafe {
        let mut sched: libc::sched_param = std::mem::uninitialized();
        sched.sched_priority = 30 + prio;
        let ret = libc::sched_setscheduler(0, libc::SCHED_FIFO, &sched);
        if ret != 0 {
            panic!()
        }
    }
}

fn pin_thread_to_cpu(cpu: usize) -> Result<(), libc::c_int> {
    let res;
    unsafe {
        let mut cpuset: libc::cpu_set_t = std::mem::uninitialized();
        libc::CPU_ZERO(&mut cpuset);
        libc::CPU_SET(cpu, &mut cpuset);
        res = libc::pthread_setaffinity_np(
            libc::pthread_self(),
            std::mem::size_of::<libc::cpu_set_t>(),
            &cpuset,
        );
    }
    if res == 0 {
        Ok(())
    } else {
        Err(res)
    }
}

fn bench(iters: usize, pin_prio: bool) -> Duration {
    let (s, r) = unbounded();
    let mut v = Vec::with_capacity(iters);
    let handle = thread::spawn(move || {
        if pin_prio {
            set_thread_priority(10);
            pin_thread_to_cpu(1).unwrap();
        }
        std::thread::sleep(Duration::from_millis(10));
        send(s, iters);
    });
    for _ in 0..iters {
        v.push(r.recv().unwrap().elapsed())
    }

    let mean = v
        .iter()
        .fold(Duration::from_nanos(0), |sum, val| sum + *val)
        / iters as u32;
    handle.join().unwrap();
    // println!("{:?}", mean);
    mean
}

fn send(s: Sender<Instant>, iters: usize) {
    for _ in 0..iters {
        s.send(Instant::now()).unwrap();
        std::thread::sleep(Duration::from_micros(10));
    }
}
