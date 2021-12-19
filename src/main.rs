use std::{
    net::TcpStream,
    time::{Duration, Instant},
};

type ErrType = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T, E = ErrType> = std::result::Result<T, E>;

mod cmd_args;
use cmd_args::CmdArgs;
use log::{trace, info, error};

fn main() -> Result<()> {
    let cmd_args = CmdArgs::args()?;
    cmd_args.init_log()?;
    info!("{}", cmd_args);
    trace!("{:?}", cmd_args);
    loop {
        match ping(&*cmd_args.destination, cmd_args.interval) {
            Ok(ping) => {
                if ping > cmd_args.limit {
                    error!("Ping is above {}ms - {}ms", cmd_args.limit, ping)
                }
            }
            Err(e) => log::error!("Error: {:?}", e),
        }
    }
}

fn ping<T>(ping: T, secs: f32) -> Result<u128>
where
    T: std::net::ToSocketAddrs + std::fmt::Debug,
{
    std::thread::sleep(Duration::from_secs_f32(secs));
    let now = Instant::now();
    trace!("Openning connection to {:?}", ping);
    let _ = TcpStream::connect(ping)?;
    Ok(now.elapsed().as_millis())
}
