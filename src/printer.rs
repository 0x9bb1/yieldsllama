use colored::Colorize;
use prettytable::{cell, row, Row, Table};

use crate::request::Pool;

#[derive(Clone, Copy)]
pub enum PrintMode {
    Text,
}

pub fn print_result(mode: PrintMode, res: &[Pool]) -> anyhow::Result<()> {
    match mode {
        PrintMode::Text => print_text(res)?,
    }
    Ok(())
}

/// Print all summary as Text
fn print_text(res: &[Pool]) -> anyhow::Result<()> {
    let mut table = Table::new();
    table.add_row(row![
        "apy",
        "symbol",
        "chain",
        "project",
        "tvlUsd",
        "apyBase",
        "apyReward",
        "exposure"
    ]);

    res.iter().for_each(|pool| {
        table.add_row(Row::new(vec![
            cell!(pool.apy.unwrap_or_default().to_string().red()),
            cell!(pool.symbol.green()),
            cell!(pool.chain.yellow()),
            cell!(pool.project.blue()),
            cell!(pool.tvl_usd.to_string().magenta()),
            cell!(pool.apy_base.unwrap_or_default().to_string().purple()),
            cell!(pool.apy_reward.unwrap_or_default().to_string().cyan()),
            cell!(pool.exposure.red()),
        ]));
    });

    table.printstd();

    Ok(())
}
