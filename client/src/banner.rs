use crate::{config::Config, github::{Implant, Session}};
use ansi_term::Colour;
use std::time::{SystemTime, UNIX_EPOCH};
use prettytable::{row, Table,format};

pub fn print_banner() {
    let banners = vec![ r#"

 ____               __                                               ____        ___     
/\  _`\            /\ \                                             /\  _`\    /'___`\   
\ \ \L\ \     __   \_\ \     __     __    ___ ___      __   _ __    \ \ \/\_\ /\_\ /\ \  
 \ \ ,  /   /'__`\ /'_` \  /'__`\ /'__`\/' __` __`\  /'__`\/\`'__\   \ \ \/_/_\/_/// /__ 
  \ \ \\ \ /\  __//\ \L\ \/\  __//\  __//\ \/\ \/\ \/\  __/\ \ \/     \ \ \L\ \  // /_\ \
   \ \_\ \_\ \____\ \___,_\ \____\ \____\ \_\ \_\ \_\ \____\\ \_\      \ \____/ /\______/
    \/_/\/ /\/____/\/__,_ /\/____/\/____/\/_/\/_/\/_/\/____/ \/_/       \/___/  \/_____/ 


    "#,
    r#"

     _____  _____  _____  _____  _____  __  __  _____  _____    _____  _____ 
    /  _  \/   __\|  _  \/   __\/   __\/  \/  \/   __\/  _  \  /     \<___  \
    |  _  <|   __||  |  ||   __||   __||  \/  ||   __||  _  <  |  |--| /  __/
    \__|\_/\_____/|_____/\_____/\_____/\__ \__/\_____/\__|\_/  \_____/<_____|
                                                                                  
                                           
    "#,
    r#"

    ______         _                                 _____  _____ 
    | ___ \       | |                               /  __ \/ __  \
    | |_/ /___  __| | ___  ___ _ __ ___   ___ _ __  | /  \/`' / /'
    |    // _ \/ _` |/ _ \/ _ \ '_ ` _ \ / _ \ '__| | |      / /  
    | |\ \  __/ (_| |  __/  __/ | | | | |  __/ |    | \__/\./ /___
    \_| \_\___|\__,_|\___|\___|_| |_| |_|\___|_|     \____/\_____/
                                                                                                                                                                                                                          

    "#,
    r#"


    ██████╗ ███████╗██████╗ ███████╗███████╗███╗   ███╗███████╗██████╗      ██████╗██████╗ 
    ██╔══██╗██╔════╝██╔══██╗██╔════╝██╔════╝████╗ ████║██╔════╝██╔══██╗    ██╔════╝╚════██╗
    ██████╔╝█████╗  ██║  ██║█████╗  █████╗  ██╔████╔██║█████╗  ██████╔╝    ██║      █████╔╝
    ██╔══██╗██╔══╝  ██║  ██║██╔══╝  ██╔══╝  ██║╚██╔╝██║██╔══╝  ██╔══██╗    ██║     ██╔═══╝ 
    ██║  ██║███████╗██████╔╝███████╗███████╗██║ ╚═╝ ██║███████╗██║  ██║    ╚██████╗███████╗
    ╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝╚══════╝╚═╝     ╚═╝╚══════╝╚═╝  ╚═╝     ╚═════╝╚══════╝
                                                                                                                                                                                     

    "#,
    ];


    let raspberry_colour = Colour::RGB(227, 11, 93); // 树莓色 RGB (227, 11, 93)

    let a: u64 = 1664525;
    let c: u64 = 1013904223;
    let m: u64 = 2u64.pow(32);
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seed: u64 = since_the_epoch.as_secs();
    let mut x = seed;
    x = (a * x + c) % m;
    let index = (x % banners.len() as u64) as usize;
    println!("{}", raspberry_colour.paint(banners[index]));

    
}


pub fn print_console_help() {
    // 定义列的宽度，确保对齐效果
    let cmd_width = 15;
    
    let header = Colour::Green.paint("\n== Console Commands Help ==");
    let separator = Colour::Yellow.paint("--------------------------------");

    let help_cmd = format!("{:<width$}{}", "help :", "get cmd help", width = cmd_width);
    let implants_cmd = format!("{:<width$}{}", "implants :", "get all implants", width = cmd_width);
    let load_cmd = format!("{:<width$}{}", "load :", "load bof file", width = cmd_width);
    let use_cmd = format!("{:<width$}{}", "use :", "use target implant, example: use 1", width = cmd_width);
    let exit_cmd = format!("{:<width$}{}", "exit :", "exit the program", width = cmd_width);

    println!("{}", header);
    println!("{}", separator);
    println!("{}", &help_cmd);
    println!("{}", &implants_cmd);
    println!("{}", &load_cmd);
    println!("{}", &use_cmd);
    println!("{}", &exit_cmd);
    println!("{}", separator);
}

pub fn print_session_help() {
    // 定义列的宽度，确保对齐效果
    let cmd_width = 15;
    
    let header = Colour::Green.paint("\n== Session Commands Help ==");
    let separator = Colour::Yellow.paint("--------------------------------");

    let help_cmd = format!("{:<width$}{}", "help :", "get cmd help", width = cmd_width);
    let implants_cmd = format!("{:<width$}{}", "ls :", "list directory", width = cmd_width);
    let shell_cmd = format!("{:<width$}{}", "shell :", "execute command , example: shell whoami", width = cmd_width);
    let bof_cmd = format!("{:<width$}{}", "bof :", "execute bof , example: bof whoami", width = cmd_width);
    let exit_cmd = format!("{:<width$}{}", "background :", "back to console", width = cmd_width);

    println!("{}", header);
    println!("{}", separator);
    println!("{}", &help_cmd);
    println!("{}", &implants_cmd);
    println!("{}", &shell_cmd);
    println!("{}", &bof_cmd);
    println!("{}", &exit_cmd);
    println!("{}", separator);
}


pub fn print_new_implant(implant:&Implant){
    println!("");
    print_info("New Session : ");
    println!("  ID : {}",Colour::Green.paint(&implant.description.id));
    println!("  IP : {}",Colour::Green.paint(&implant.description.ip));
    println!("  Username : {}",Colour::Green.paint(&implant.description.username));
    println!("  System : {}",Colour::Green.paint(&implant.description.system));
}

pub fn print_implants(implants: &Vec<Session>) {
    let mut table = Table::new();

    let table_format = format::FormatBuilder::new()
    .column_separator(' ')  // 列与列之间的分隔符
    .borders(' ')            // 表格边框
    .separator(format::LinePosition::Title, format::LineSeparator::new('=', ' ', '=', ' ')) // 表头与数据之间用 "== ==" 分隔
    .padding(1, 1) 
    .build();

    table.set_format(table_format);
    //table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.set_titles(row!["ID", "IP", "Username", "System","Time"]); // 添加表头
    for metadata in implants {
        table.add_row(row![
            metadata.id,
            metadata.ip,
            metadata.username,
            metadata.system,
            metadata.time,
        ]);
    }
    println!("");
    table.printstd();
}

pub fn print_info(string:&str) {
    let sea_colour = Colour::RGB(0, 180, 255);
    println!("{} {}",sea_colour.paint("[*]"),&string);
}

pub fn print_conf(config:&Config){
    print_info("Github Config:");
    println!("    Access Token: {}",&config.github.access_token);
    println!("    Username: {}",&config.github.username);
    println!("    Repository: {}",&config.github.repository);
}