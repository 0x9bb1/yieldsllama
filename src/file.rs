use std::time::Duration;

use tokio::io::AsyncWriteExt;

use crate::request::Pool;

pub async fn write(data: &Vec<Pool>) -> anyhow::Result<()> {
    let mut file = tokio::fs::File::create("data.json").await?;

    let vec = serde_json::to_vec_pretty(data)?;
    file.write_all(&vec).await?;
    Ok(())
}

pub async fn read() -> anyhow::Result<Option<Vec<Pool>>> {
    let file = match std::fs::File::open("data.json") {
        Ok(file) => file,
        Err(e) => {
            println!("读取本地data.json文件异常: {}", e.to_string());
            return Ok(None);
        }
    };

    let twelve_hour = Duration::from_secs(60 * 60 * 12);
    let metadata = file.metadata()?;
    if metadata.modified()?.elapsed()? > twelve_hour {
        println!("本地文件过期");
        return Ok(None);
    }

    let reader = std::io::BufReader::new(file);
    let data = serde_json::from_reader(reader)?;

    Ok(Some(data))
}
