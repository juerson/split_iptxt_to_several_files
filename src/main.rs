use std::collections::HashSet;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead, Write};
use std::net::Ipv4Addr;

fn main() {
	// 删除以ip_开头的TXT文件
	delete_existing_files();
    // 删除重复行并获取唯一的 IP 地址
    let unique_ips = read_and_remove_duplicates("ip.txt");
	if unique_ips.is_empty() {
        println!("未找到 ip.txt 文件或文件内容为空");
		// 等待用户输入
		println!("按 Enter 键退出程序！");
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		return;
    }
    print!("正在对ip.txt文件进行文件分割；\n请输入每个TXT文件的IP数量上限：");
    io::stdout().flush().expect("刷新 stdout 失败");

    let mut limit_input = String::new();
    io::stdin().read_line(&mut limit_input).expect("读取输入失败");

    if let Ok(limit) = limit_input.trim().parse::<usize>() {
        split_ips_to_files(unique_ips, limit);
        println!("IP 地址已根据上限数量写入多个TXT文件");
    } else {
        println!("无效的上限数量");
    }
	// 等待用户输入
	println!("按 Enter 键退出程序！");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

fn read_and_remove_duplicates(filename: &str) -> Vec<Ipv4Addr> {
    let mut unique_ips = HashSet::new();
    let mut result = Vec::new();
	


    if let Ok(file) = File::open(filename) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(ip_str) = line {
                if let Ok(ip) = ip_str.parse::<Ipv4Addr>() {
                    unique_ips.insert(ip);
                }
            }
        }

        result.extend(unique_ips);
		result.sort();
    } else {
        println!("无法打开文件：{}", filename);
    }
    result
}

fn split_ips_to_files(ips: Vec<Ipv4Addr>, limit: usize) {
    let mut file_counter = 1;
    let mut ip_counter = 0;
    let digits = calculate_digits(ips.len(), limit);
    let mut file = create_file(file_counter, digits);

    for ip in ips {
        writeln!(file, "{}", ip).expect("写入文件失败");
        ip_counter += 1;

        if ip_counter >= limit {
            ip_counter = 0;
            file_counter += 1;
            file = create_file(file_counter, digits);
        }
    }
}

// 其他辅助函数的定义 ...

fn calculate_digits(total_ips: usize, limit: usize) -> usize {
    let total_files = (total_ips + limit - 1) / limit;
    let max_digits = (total_files as f64).log(10.0).ceil() as usize;
    max_digits
}

fn create_file(counter: usize, digits: usize) -> File {
    let filename = format!("ip_{:0width$}.txt", counter, width = digits);
    let file = File::create(&filename).expect("创建文件失败");
    file
}

fn delete_existing_files() {
    let files = fs::read_dir(".").expect("读取目录失败");

    for file in files {
        if let Ok(file) = file {
            let filename = file.file_name().into_string().unwrap();
            if filename.starts_with("ip_") && filename.ends_with(".txt") {
                if let Err(err) = fs::remove_file(&filename) {
                    println!("删除文件 {} 失败: {:?}", filename, err);
                }
            }
        }
    }
}