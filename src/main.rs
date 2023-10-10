use clap::Parser;

use crate::args::Args;
use crate::printer::PrintMode;
use crate::request::{Pool, Response};

mod args;
mod file;
mod handle;
mod printer;
mod request;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut data: Vec<Pool> = if let Some(local) = file::read().await? {
        local
    } else {
        let res: Response = request::request().await?;
        file::write(&res.data).await?;
        res.data
    };

    let data = handle::handle(&args, &mut data).await?;

    printer::print_result(PrintMode::Text, &data)?;

    Ok(())
}
