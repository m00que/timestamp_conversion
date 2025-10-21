use chrono::{Local, NaiveDateTime, TimeZone};
use std::io::{self, Write};

// Apple时间戳和Unix时间戳的差值（秒）
// 从 1970-01-01 00:00:00 到 2001-01-01 00:00:00 的秒数
const APPLE_UNIX_OFFSET: i64 = 978307200;

fn main() {
    loop {
        println!("\n==== 时间戳转换工具 ====");
        println!("1. Unix时间戳转标准时间");
        println!("2. 标准时间转Unix时间戳");
        println!("3. Apple时间戳转标准时间");
        println!("4. 标准时间转Apple时间戳");
        println!("5. Unix时间戳转Apple时间戳");
        println!("6. Apple时间戳转Unix时间戳");
        println!("7. 退出");
        print!("请选择功能 (1-7): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("读取输入失败");

        match choice.trim() {
            "1" => unix_timestamp_to_datetime(),
            "2" => datetime_to_unix_timestamp(),
            "3" => apple_timestamp_to_datetime(),
            "4" => datetime_to_apple_timestamp(),
            "5" => unix_to_apple_timestamp(),
            "6" => apple_to_unix_timestamp(),
            "7" => {
                println!("再见！");
                break;
            }
            _ => println!("无效选择，请输入 1-7"),
        }
    }
}

fn unix_timestamp_to_datetime() {
    print!("请输入Unix时间戳（秒，支持小数）: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    match input.trim().parse::<f64>() {
        Ok(timestamp) => {
            display_timestamp_as_datetime(timestamp);
        }
        Err(_) => println!("错误：请输入有效的数字（支持整数或小数）"),
    }
}

fn apple_timestamp_to_datetime() {
    print!("请输入Apple时间戳（秒，支持小数）: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    match input.trim().parse::<f64>() {
        Ok(apple_timestamp) => {
            // 转换为Unix时间戳
            let unix_timestamp = apple_timestamp + APPLE_UNIX_OFFSET as f64;
            println!("对应Unix时间戳: {}", unix_timestamp);
            display_timestamp_as_datetime(unix_timestamp);
        }
        Err(_) => println!("错误：请输入有效的数字（支持整数或小数）"),
    }
}

fn display_timestamp_as_datetime(timestamp: f64) {
    // 分离整数部分（秒）和小数部分（纳秒）
    let secs = timestamp.trunc() as i64;
    let nsecs = ((timestamp.fract().abs() * 1_000_000_000.0).round() as u32).min(999_999_999);
    
    match Local.timestamp_opt(secs, nsecs) {
        chrono::LocalResult::Single(dt) => {
            println!("\n转换结果：");
            println!("本地时间 (UTC+8): {}", dt.format("%Y-%m-%d %H:%M:%S%.3f"));
            println!("UTC时间: {}", dt.with_timezone(&chrono::Utc).format("%Y-%m-%d %H:%M:%S%.3f"));
        }
        _ => println!("错误：无效的时间戳"),
    }
}

fn datetime_to_unix_timestamp() {
    print!("请输入标准时间 (格式: YYYY-MM-DD HH:MM:SS): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    let datetime_str = input.trim();

    match NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
        Ok(naive_dt) => {
            let local_dt = Local.from_local_datetime(&naive_dt).unwrap();
            let timestamp = local_dt.timestamp();
            
            println!("\n转换结果：");
            println!("Unix时间戳（秒）: {}", timestamp);
            println!("Unix时间戳（毫秒）: {}", timestamp * 1000);
        }
        Err(_) => println!("错误：时间格式不正确，请使用 YYYY-MM-DD HH:MM:SS 格式"),
    }
}

fn datetime_to_apple_timestamp() {
    print!("请输入标准时间 (格式: YYYY-MM-DD HH:MM:SS): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    let datetime_str = input.trim();

    match NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
        Ok(naive_dt) => {
            let local_dt = Local.from_local_datetime(&naive_dt).unwrap();
            let unix_timestamp = local_dt.timestamp();
            let apple_timestamp = unix_timestamp - APPLE_UNIX_OFFSET;
            
            println!("\n转换结果：");
            println!("Apple时间戳（秒）: {}", apple_timestamp);
            println!("Apple时间戳（毫秒）: {}", apple_timestamp * 1000);
        }
        Err(_) => println!("错误：时间格式不正确，请使用 YYYY-MM-DD HH:MM:SS 格式"),
    }
}

fn unix_to_apple_timestamp() {
    print!("请输入Unix时间戳（秒，支持小数）: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    match input.trim().parse::<f64>() {
        Ok(unix_timestamp) => {
            let apple_timestamp = unix_timestamp - APPLE_UNIX_OFFSET as f64;
            println!("\n转换结果：");
            println!("Apple时间戳: {}", apple_timestamp);
        }
        Err(_) => println!("错误：请输入有效的数字（支持整数或小数）"),
    }
}

fn apple_to_unix_timestamp() {
    print!("请输入Apple时间戳（秒，支持小数）: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("读取输入失败");

    match input.trim().parse::<f64>() {
        Ok(apple_timestamp) => {
            let unix_timestamp = apple_timestamp + APPLE_UNIX_OFFSET as f64;
            println!("\n转换结果：");
            println!("Unix时间戳: {}", unix_timestamp);
        }
        Err(_) => println!("错误：请输入有效的数字（支持整数或小数）"),
    }
}
