use argh::FromArgs;

#[derive(FromArgs, PartialEq, Eq, Debug)]
#[argh(subcommand)]
pub enum Command {
    Publisher(PublisherArgs),
    Subscriber(SubscriberArgs),
}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// Publisher
#[argh(subcommand, name = "publisher")]
pub struct PublisherArgs {}

#[derive(FromArgs, PartialEq, Eq, Debug)]
/// Receiver
#[argh(subcommand, name = "subscriber")]
pub struct SubscriberArgs {}

#[derive(FromArgs)]
/// device simulator
pub struct Args {
    #[argh(subcommand)]
    pub command: Command,

    /// enable metrics endpoint
    #[argh(switch, short = 'm')]
    pub metrics: bool,
}

pub fn args() -> Args {
    argh::from_env()
}
