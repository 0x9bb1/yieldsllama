use crate::args::{Args, Exposure, Sort};
use crate::request::Pool;
use std::cmp::Ordering;
use std::str::FromStr;

pub async fn handle(args: &Args, data: &mut [Pool]) -> anyhow::Result<Vec<Pool>> {
    let limit = args.limit as usize;
    let exposure = Exposure::from_str(args.exposure.as_str()).unwrap_or(Exposure::Single);
    let sort = Sort::from_str(args.sort.as_str()).unwrap_or(Sort::Apy);
    let tvl = args.tvl;
    let chain = args.chain.clone();
    let token = args.asset.clone();

    data.sort_by(|p, p2| match sort {
        Sort::Tvl => p.tvl_usd.cmp(&p2.tvl_usd),
        Sort::Apy => p
            .apy
            .unwrap_or(0f32)
            .partial_cmp(&p2.apy.unwrap_or(0f32))
            .unwrap_or(Ordering::Equal),
        Sort::Other => p
            .apy
            .unwrap_or(0f32)
            .partial_cmp(&p2.apy.unwrap_or(0f32))
            .unwrap_or(Ordering::Equal),
    });
    data.reverse();

    let target_data = data
        .iter()
        .filter(|pool| {
            let pool_exposure =
                Exposure::from_str(pool.exposure.as_str()).unwrap_or(Exposure::Other);
            pool_exposure == exposure
        })
        .filter(|pool| {
            if let Some(tvl) = tvl {
                pool.tvl_usd >= tvl
            } else {
                true
            }
        })
        .filter(|pool| {
            if let Some(chain) = &chain {
                pool.chain.eq_ignore_ascii_case(chain)
            } else {
                true
            }
        })
        .filter(|pool| {
            if let Some(token) = &token {
                pool.symbol.eq_ignore_ascii_case(token)
            } else {
                true
            }
        })
        .take(limit)
        .cloned()
        .collect::<Vec<_>>();

    Ok(target_data)
}
