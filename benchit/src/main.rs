use structopt::StructOpt;
use tokio::process::ChildStdout;
use tokio::process::Command;
use std::process::Stdio;
use tokio::time::{delay_for, Duration};
use psutil::process::processes;

/// Automation for running load tests and gathering stats
/// It uses wrk under the hood, make sure to have it in the PATH
#[derive(StructOpt)]
struct Opt {
    /// IP address of target hast
    #[structopt(short="h", long="host", default_value="127.0.0.1")]
    host: std::net::IpAddr,
    /// Concurrency limit
    #[structopt(short="c", long="mc", default_value="128")]
    max_concurrency: u16,
    /// Node port
    #[structopt(short="n", long="np", default_value="3000")]
    node_port: u16,
    /// Actix port
    #[structopt(short="a", long="ap", default_value="3002")]
    actix_port: u16,
    /// Monitor local processes: node, actix and postgres
    #[structopt(short="m", long="monitor")]
    monitor: bool,
}

fn wrk(concurrency: u16, url: &String) -> Command {
    let mut wrk = Command::new("wrk");
    wrk .arg(format!("-t {}", concurrency/10+1))
        .arg(format!("-c {}", concurrency))
        .arg("-d60s")
        .arg(url)
        .kill_on_drop(true)
        .stdout(Stdio::piped());
    wrk
}

#[derive(Default)]
struct ProcessesReport {
    postgres: ProcessReport,
    node: ProcessReport,
    actix: ProcessReport,
}

#[derive(Default)]
struct ProcessReport {
    cpu: f32,
    mem: u64,
}

async fn monitor_processes() -> anyhow::Result<ProcessesReport> {
    let procs: Vec<_> = processes()?
        .into_iter()
        .filter_map(|p| p.ok())
        .collect();

    delay_for(Duration::from_secs(5)).await;

    let proc_stats = |name| procs.iter()
            .filter(|p| p.name().is_ok() && p.name().unwrap().contains(name))
            .cloned()
            .fold(
                ProcessReport::default(), 
                |mut acc, mut p| {
                    acc.cpu += p.cpu_percent().unwrap();
                    acc.mem += p.memory_info().unwrap().rss();
                    acc
                }
            );

    Ok(ProcessesReport {
        postgres: proc_stats("postgres"),
        node: proc_stats("node"),
        actix: proc_stats("actix"),
    })
}

#[tokio::main(core_threads = 1)]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let node_url = format!("http://{}:{}/tasks", opt.host, opt.node_port);
    let actix_url = format!("http://{}:{}/tasks", opt.host, opt.actix_port);

    let mut c = 1u16;
    while c < opt.max_concurrency {
        println!("node");
        let wrk = wrk(c, &node_url).output();
        let proc_stats = if opt.monitor {
                delay_for(Duration::from_secs(10)).await;
                monitor_processes().await?
            } else {
                ProcessesReport::default()
            };
        let stdout = String::from_utf8(wrk.await?.stdout)?;
        for line in stdout.lines() {
            if line.contains("Latency") {}
        }
        c *= 2;
    }
    Ok(())

}
