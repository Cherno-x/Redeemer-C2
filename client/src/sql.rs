use sqlite::{Connection, State};
use std::path::Path;
use std::error::Error;
use chrono::Local;
use github::{Metadata, Session};



pub fn init_db() -> Result<(), Box<dyn Error>> {
    // 数据库文件路径
    let db_path = "database.db";

    // 检查数据库文件是否存在
    let db_exists = Path::new(db_path).exists();

    // 连接到 SQLite 数据库
    let connection = Connection::open(db_path)?;

    if !db_exists {
        println!("[*] 数据库不存在，开始初始化...");
        // 创建 implants 表
        connection.execute(
            "
            CREATE TABLE implants (
                id TEXT PRIMARY KEY,
                ip TEXT NOT NULL,
                username TEXT NOT NULL,
                system TEXT NOT NULL,
                time TEXT NOT NULL
            );
            "
        )?;

        // 创建 command 表
        connection.execute(
            "
            CREATE TABLE command (
                id TEXT PRIMARY KEY,
                command TEXT NOT NULL,
                response TEXT NOT NULL,
                time TEXT NOT NULL
            );
            "
        )?;

        //banner::print_info("数据库已成功初始化并创建表格");
    } else {
        //banner::print_info("数据库已存在，无需初始化");
    }

    Ok(())
}


pub fn insert_metadata(conn: &Connection,metadata: &Metadata) -> Result<(), Box<dyn std::error::Error>> {
 
    let current_time = Local::now(); // 获取当前 local 时间
    let current_time_str = current_time.format("%Y-%m-%d %H:%M:%S").to_string(); 
    // 准备 SQL 插入语句
    let mut statement = conn.prepare("INSERT INTO implants (id, ip, username, system, time) VALUES (?,?,?,?,?)")?;

    // 绑定参数（只传递值）
    statement.bind((1,metadata.id.as_str()))?;
    statement.bind((2,metadata.ip.as_str()))?;
    statement.bind((3,metadata.username.as_str()))?;
    statement.bind((4,metadata.system.as_str()))?;
    statement.bind((5,current_time_str.as_str()))?; 

    // 执行插入
    match statement.next() {
        Ok(_) => {
            // 插入成功
            Ok(())
        }
        Err(err) => {
            // 处理插入错误
            eprintln!("Error inserting metadata: {}", err);
            Err(Box::new(err))
        }
    }
}

pub fn check_metadata_exists(conn: &Connection, id: &String) -> Result<bool, Box<dyn std::error::Error>> {
    // 准备 SQL 查询语句
    let mut statement = conn.prepare("SELECT COUNT(*) FROM implants WHERE id = ?")?;

    // 绑定参数
    statement.bind((1, id.as_str()))?;

    // 执行查询
    if let State::Row = statement.next()? {
        // 获取查询结果
        let count: i64 = statement.read::<i64,_>(0)?;
        return Ok(count > 0); // 如果 count 大于 0，则记录存在
    }

    Ok(false) // 记录不存在
}

pub fn get_all_implants(conn: &Connection) -> Result<Vec<Session>, Box<dyn std::error::Error>> {
    // 准备 SQL 查询语句
    let mut statement = conn.prepare("SELECT id, ip, username, system, time FROM implants")?;
    let mut sessions = Vec::new(); // 用于存储结果的向量
    // 遍历查询结果
    while let sqlite::State::Row = statement.next()? {
        // 创建 Metadata 实例并将结果绑定到结构体字段
        let metadata = Session {
            id: statement.read::<String,_>(0)?, // 获取第一列
            ip: statement.read::<String,_>(1)?, // 获取第二列
            username: statement.read::<String,_>(2)?, // 获取第三列
            system: statement.read::<String,_>(3)?, // 获取第四列
            time: statement.read::<String,_>(4)?,
        };

        // 将 Metadata 实例推入向量
        sessions.push(metadata);
    }
    Ok(sessions)
}
