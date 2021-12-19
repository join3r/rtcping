use std::{
    io::ErrorKind,
    net::{SocketAddr, ToSocketAddrs},
    ops::Deref,
};

use super::Result;
use structopt::StructOpt;

#[rustfmt::skip]
#[derive(StructOpt, Debug)]
pub struct CmdArgs {
    #[structopt(short,long,help="Destination",value_name="HOSTNAME:PORT")]
    pub destination: Destination,
    #[structopt(short,long,help="Log results over this time in ms as errors",default_value="50")]
    pub limit: u128,
    #[structopt(short,long,help="Time in ms between sending connection requests",default_value="1.0")]
    pub interval: f32,
    #[structopt(short,help="Show verbose messages",takes_value=false,multiple=true,parse(from_occurrences))]
    pub v: usize,
    #[structopt(short,help="Quiet mode")]
    quiet: bool,
    #[structopt(short,long,help="Show timestamps",possible_values=&["none","sec","ms","ns"])]
    timestamps: Option<stderrlog::Timestamp>,
}

impl CmdArgs {
    pub fn args() -> Result<Self> {
        let args = Self::from_args();
        Ok(args)
    }
    pub fn init_log(&self) -> Result<()> {
        stderrlog::new()
            .verbosity(self.v)
            .quiet(self.quiet)
            .timestamp(self.timestamps.unwrap_or(stderrlog::Timestamp::Off))
            .init()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct Destination {
    inner: SocketAddr,
}

impl std::str::FromStr for Destination {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = s
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| std::io::Error::new(ErrorKind::NotFound, "error"))?;
        Ok(Destination { inner })
    }
}

impl Deref for Destination {
    type Target = SocketAddr;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::fmt::Display for CmdArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Interval between connections = {}\nLimit latency when to log error message = {}",
            self.interval, self.limit
        )
    }
}
