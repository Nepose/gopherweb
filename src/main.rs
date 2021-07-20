use std::{
    fs,
    io::{
        self, Read, Write
    },
    net::TcpStream,
};

use serde::Deserialize;

static DEFAULT_URL_SIGN: &str = "welcome to gopherweb";
static UPDATE_HOST: &str = "157.90.164.160";
static UPDATE_URL: &str = "/gopherweb/update_list.txt";

#[cfg(windows)]
static GOPHERWEB_DIR: &str = "C:\\gopherweb";

#[cfg(unix)]
static GOPHERWEB_DIR: &str = "/var/cache/gopherweb";

mod help;

macro_rules! connect {
    ($uri_host:expr, $uri_path:expr, $sign:expr, $stack:expr, $pwd:expr) => {
        match connect($uri_host, $uri_path, $sign, $pwd) {
            Ok(mut page) => $stack.push(GopherStream::parse_gopherpage(&mut page)),
            Err(e) => println!("{}", e)
        }
    };
}

#[derive(Clone, Debug)]
struct GopherStream {
    gopher_type: String,
    title: String,
    path: String,
    host: String,
    port: String,
    counter: u32
}

#[derive(Deserialize)]
struct UpdateList {
    name: String,
    path: String,
    r#type: String,
    date: String
}

impl GopherStream {
    fn parse_gopherpage(content: &mut String) -> Vec<GopherStream> {
        let mut list_data: Vec<GopherStream> = Vec::new();
        let mut counter: u32 = 0;
        for line in content.split("\r\n") {
            if line == "" { continue; }
            match &line[..1] {
                "." => break,
                _ => {
                    let line_parts: Vec<&str> = line[1..].split("\t").collect();
                    list_data.push(GopherStream {
                        gopher_type: line[..1].to_string(),
                        title: line_parts[0].to_string(),
                        path: line_parts[1].to_string(),
                        host: line_parts[2].to_string(),
                        port: line_parts[3].to_string(),
                        counter
                    })
                }
            }
            counter += 1;
        }
        list_data
    }

    fn print(data: Vec<GopherStream>) {
        for line in data {
            match line.gopher_type.as_str() {
                "0" => println!("[File] {}\n     {} -> //{}{}", line.title, line.counter, line.host, line.path),
                "1" => println!("[Directory] {}\n     {} -> //{}{}", line.title, line.counter, line.host, line.path),
/*                "2" => println!("[CSO phonebook] {}\n     {} -> //{}{}", line.title, line.counter, line.host, line.path),
                "5" => println!("[DOS binary archive] {}\n     {} -> //{}{}", line.title, line.counter, line.host, line.path),
                "6" => println!("[Unix uuencoded file] {}\n     {} -> //{}{}", line.title, line.counter, line.host, line.path),
                "7" => println!("[Search] {}\n     {} -> //{}{}", line.title, line.counter, line.host, line.path),
                "9" => println!("[Binary file] {}\n     {} -> //{}{}", line.title, line.counter, line.host, line.path),*/
                "i" => println!("{}", line.title),
                "h" => println!("[HTTP] {}\n     {} -> {}", line.title, line.counter, line.path),
                _ => continue,
            }
        }
    }
}

fn connect(uri_host: &str, uri_path: &str, sign: &mut String, pwd: &mut Vec<String>) -> Result<String, String> {
    match TcpStream::connect(format!("{}:70", uri_host)) {
        Ok(mut stream) => {
            let mut content: Vec<u8> = vec![];
            stream.write(format!("{}\r\n", uri_path).as_bytes()).unwrap();
            stream.flush().unwrap();
            stream.read_to_end(&mut content).unwrap();
            if uri_host != sign {
                *sign = uri_host.to_string();
            }
            pwd.push(format!("{}{}", uri_host, uri_path));
            Ok(String::from_utf8_lossy(&content).to_string())
        },
        Err(e) => Err(format!("Couldn't connect due to: {}", e))
    }
}

fn main() {
    let mut sign = DEFAULT_URL_SIGN.to_string();
    let mut stack: Vec<Vec<GopherStream>> = Vec::new();
    let mut pwd: Vec<String> = Vec::new();
    
    match help::check_gopherweb_dir(&GOPHERWEB_DIR) {
        Ok(()) => (),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    }

    loop {
        let mut prompt = String::new();
        print!("[{}] $ ", sign);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut prompt).unwrap();

        let vec: Vec<&str> = prompt.trim().split(" ").collect();
        match vec[0] {
            "home" => connect!("gopher.floodgap.com", "/", &mut sign, &mut stack, &mut pwd),
            "visit" if vec.len() == 2 => connect!(vec[1], "/", &mut sign, &mut stack, &mut pwd),
            "cd" if sign != DEFAULT_URL_SIGN.to_string() && vec.len() == 2 => {
                match vec[1].parse::<usize>() {
                    Ok(num) => {
                        if "1" != stack.last().unwrap().to_vec()[num].gopher_type {
                            println!("Given number doesn't point to directory.");
                        }
                        else {
                            connect!(stack.last().unwrap().to_vec()[num].host.as_str(), stack.last().unwrap().to_vec()[num].path.as_str(), &mut sign, &mut stack, &mut pwd);
                        }
                    },
                    Err(_e) => {
                        if vec[1] != ".." { println!("Not a number given."); }
                        else if pwd.len() > 1 {
                            stack.pop();
                            pwd.pop();
                            sign = "cd ..".to_string();
                        }
                    }
                }
            },
            "bye" => break,
            "help" => println!("{}", help::help()),
            "changelog" => println!("{}", help::changelog()),
            "pwd" => {
                for i in 0..pwd.len() {
                    println!("[{}.] {}", i + 1, pwd[i]);
                }
            },
            "revert" if vec.len() == 2 => {
                match vec[1].parse::<usize>() {
                    Ok(i) => {
                        let tmp_pwd = pwd.clone();
                        let mut url_iter = tmp_pwd[i-1].splitn(2, '/');
                        let url_host = url_iter.next().unwrap();
                        let url_path = url_iter.next().unwrap();
                        connect!(url_host, url_path, &mut sign, &mut stack, &mut pwd);
                    },
                    Err(_e) => println!("Not a number entered.")
                }
            },
            "ls" if sign != DEFAULT_URL_SIGN.to_string() => GopherStream::print(stack.last().unwrap().to_vec()),
            "show" => {
                match vec[1].parse::<usize>() {
                    Ok(num) => {
                        if "0" != stack.last().unwrap().to_vec()[num].gopher_type {
                            println!("Given number doesn't point to file.");
                        }
                        else {
                            match connect(sign.clone().as_str(), stack.last().unwrap().to_vec()[num].path.as_str(), &mut sign, &mut pwd) {
                                Ok(file) => println!("{}", file),
                                Err(e) => println!("{}", e)
                            }
                        }
                    },
                    Err(_e) => println!("Not a number given.")
                }
            },
            "update" => {
                match connect(UPDATE_HOST, UPDATE_URL, &mut sign, &mut pwd) {
                    Ok(update) => {
                        sign = DEFAULT_URL_SIGN.to_string();
                        pwd.pop();
                        let update_list: UpdateList = serde_json::from_str(update.as_str()).unwrap();
                        if update_list.name.as_str() != help::get_version() {
                            println!("There is an update available!\n    Your current version:     {}\n    Date of current version:  {}\n    New available version:    {}\n    Date of new version:      {}\n    Type of new version:      {}\nYou can download the file of update to your computer, but before doing it check whether your cache directory is writable.\n", help::get_version(), help::get_date(), update_list.name, update_list.date, update_list.r#type);
                            print!("Do you want to download new version? [yes or no] $ ");
                            let mut input = String::new();
                            io::stdout().flush().unwrap();
                            io::stdin().read_line(&mut input).unwrap();
                            if input.trim() == "yes" {
                                match connect(UPDATE_HOST, update_list.path.as_str(), &mut sign, &mut pwd) {
                                    Ok(update) => {
                                        sign = DEFAULT_URL_SIGN.to_string();
                                        pwd.pop();
                                        fs::write(format!("{}/{}.tgz", GOPHERWEB_DIR, update_list.name).as_str(), update.as_str())
                                            .expect("Couldn't save update file on your computer.");
                                        println!("Succesfully saved new version of Gopherweb in {}.", GOPHERWEB_DIR);
                                    },
                                    Err(_e) => println!("Couldn't download update file.")
                                }
                            }
                        } else {
                            println!("You are up to date.");
                        }
                    },
                    Err(e) => println!("{}", e)
                }
            },
            "custom" => {
                match vec.len() {
                    2 => connect!(sign.clone().as_str(), vec[1], &mut sign, &mut stack, &mut pwd),
                    3 => connect!(vec[1], vec[2], &mut sign, &mut stack, &mut pwd),
                    _ => ()
                }
            },
            _ => continue
        }
    }
}
