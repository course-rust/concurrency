use std::{thread, time::Duration};

use anyhow::{Ok, Result};
use rand::Rng;

use concurrency::AMapMetrics;

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = AMapMetrics::new(&[
        "call.thread.worker.0",
        "call.thread.worker.1",
        "req.page.1",
        "req.page.2",
        "req.page.3",
        "req.page.4",
    ]);

    println!("{:?}", metrics);

    // Start N workers and M requests
    for idx in 0..N {
        task_worker(idx, metrics.clone())?; // Arc::clone(&metrics)
    }

    for _ in 0..M {
        request_worker(metrics.clone())?; // // Arc::clone(&metrics)
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        println!("{}", metrics);
    }

    #[allow(unreachable_code)]
    Ok(())
}

fn task_worker(idx: usize, mut metrics: AMapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            // do long term stuff
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));

            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(())
}

fn request_worker(mut metrics: AMapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = rand::thread_rng();
            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));

            let page: u32 = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok(())
    });

    Ok(())
}
