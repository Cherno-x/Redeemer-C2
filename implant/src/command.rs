use get_if_addrs::get_if_addrs;
use std::{env, fs, io, net::IpAddr, path::{Path, PathBuf},process::{Command, Stdio}};

pub struct Cmd{
    pub cmd:String,
    pub args:String,
}



pub fn ls_command<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut entries = Vec::new();

    let path = path.as_ref();

    // 如果传入的路径为空字符串，使用当前目录
    let path = if path.as_os_str().is_empty() {
        PathBuf::from(".") // 当前目录
    } else {
        path.to_path_buf()
    };

    // 尝试读取指定目录下的所有条目
    match fs::read_dir(&path) {
        Ok(dir_entries) => {
            for entry_result in dir_entries {
                match entry_result {
                    Ok(entry) => {
                        let path = entry.path();
                        entries.push(path.display().to_string());
                    }
                    Err(e) => {
                        eprintln!("[-] Error reading entry: {}", e);
                        continue; // 遇到错误时，跳过该条目并继续
                    }
                }
            }
        }
        Err(e) => {
            return Err(e); // 返回错误，调用者可以决定如何处理
        }
    }

    let result = entries.join("\n");
    Ok(result)
}


pub fn shell_command(command: &String) -> Result<String, io::Error> {
    // 检测操作系统
    #[cfg(target_os = "windows")]
    let shell = "cmd"; // Windows 使用 cmd

    #[cfg(not(target_os = "windows"))]
    let shell = "sh"; // Unix 系统使用 sh

    // 创建命令
    let output = Command::new(shell)
        .arg(if cfg!(target_os = "windows") { "/C" } else { "-c" }) // 设置相应参数
        .args(command.split_whitespace()) // 根据空格解析参数
        .stdout(Stdio::piped()) // 管道标准输出
        .output()?; // 执行命令并获取输出

    // 检查命令是否成功执行
    if output.status.success() {
        // 将标准输出转为字符串
        let result = String::from_utf8_lossy(&output.stdout);
        Ok(result.to_string())
    } else {
        // 返回错误信息
        let err_msg = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, err_msg.to_string()))
    }
}



//以下为功能函数
pub fn string2cmd(command_str:&String)->Cmd{
    let mut parts = command_str.split_whitespace();
    let cmd = parts.next().unwrap_or("").to_string();
    let args = parts.collect::<Vec<&str>>().join(" ");
    Cmd { cmd, args }
}

pub fn get_private_ip() -> Option<IpAddr> {
    let if_addrs = get_if_addrs().unwrap();

    for if_addr in if_addrs {
        let ip = if_addr.ip();
        // 过滤掉回环地址，并且只返回IPv4地址
        if !ip.is_loopback() {
            if let IpAddr::V4(ipv4) = ip {
                return Some(IpAddr::V4(ipv4));
            }
        }
    }
    None
}


pub fn get_username() -> Option<String> {
    env::var("USER").or_else(|_| env::var("USERNAME")).ok()
}

