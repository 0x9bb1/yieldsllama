use crate::args::{Args, Exposure, Sort};
use crate::request::Pool;
use std::cmp::Ordering;
use std::str::FromStr;

pub async fn handle(args: &Args, data: &mut Vec<Pool>) -> anyhow::Result<Vec<Pool>> {
    let limit = args.limit as usize;
    let exposure = Exposure::from_str(args.exposure.as_str()).unwrap_or(Exposure::Single);
    let sort = Sort::from_str(args.sort.as_str()).unwrap_or(Sort::Apy);
    let tvl = args.tvl;

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
            if let Some(tvl) = tvl {
                return pool_exposure == exposure && pool.tvl_usd >= tvl;
            } else {
                return pool_exposure == exposure;
            }
        })
        .take(limit)
        .cloned()
        .collect::<Vec<_>>();

    Ok(target_data)
}
