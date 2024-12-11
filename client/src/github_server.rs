
use github;
use std::error::Error;

use crate::{banner::print_new_implant, sql};

//label监控，获取implant，print，写入数据库
pub async fn lable_monitor(config:&github::GithubConfig)-> Result<String, Box<dyn Error>>{
    let conn = sqlite::open("database.db")?;
    loop{
        let response = github::get_labels(config).await?;
        let implants = github::convert_label_implants(&response)?;
        //比对入库，新则通知
        for implant in &implants{
            let implant_id = &implant.name;
            if sql::check_metadata_exists(&conn, implant_id)?{
                continue;
            };
            let metadata = &implant.description;
            sql::insert_metadata(&conn,metadata)?;
            print_new_implant(implant)
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}