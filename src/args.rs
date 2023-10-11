use std::str::FromStr;

use clap::Parser;

#[derive(Clone, Debug, PartialEq)]
pub enum Exposure {
    Multi,
    Single,
    Other,
}

impl FromStr for Exposure {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Exposure::Single),
            "multi" => Ok(Exposure::Multi),
            _ => Ok(Exposure::Other),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Sort {
    Tvl,
    Apy,
    Other,
}

impl FromStr for Sort {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tvl" => Ok(Sort::Tvl),
            "multi" => Ok(Sort::Apy),
            _ => Ok(Sort::Other),
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 获取前limit条数据
    #[arg(short, long, default_value_t = 10)]
    pub limit: u8,

    /// 类型, 可选: single / multi
    #[arg(short, long, default_value = "single")]
    pub exposure: String,

    /// 倒序, 可选: tvl / apy
    #[arg(short, long, default_value = "apy")]
    pub sort: String,

    /// TVL下限
    #[arg(short, long)]
    pub tvl: Option<u128>,

    /// 筛选特定链
    #[arg(short, long)]
    pub chain: Option<String>,
}
