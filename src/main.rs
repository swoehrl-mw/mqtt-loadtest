mod cli;
mod mqtt;
mod publisher;
mod subscriber;
mod util;

#[tokio::main(worker_threads = 4, flavor = "multi_thread")]
async fn main() {
    let mainargs = cli::args();

    match mainargs.command {
        cli::Command::Publisher(args) => {
            publisher::run(args).await;
        }
        cli::Command::Subscriber(args) => {
            subscriber::run(args).await;
        }
    };
}
