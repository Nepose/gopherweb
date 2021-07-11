use std::{
    io::{
        self, Read, Write
    },
    net::TcpStream,
};

mod help;

macro_rules! connect {
    ($uri_host:expr, $uri_path:expr, $sign:expr, $stack:expr) => {
        match connect($uri_host, $uri_path, $sign) {
            Ok(mut page) => {
                $stack.push(GopherStream::parse_gopherpage(&mut page));
                GopherStream::print($stack.last().unwrap().to_vec());
            }
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
                "i" => println!("{}", line.title),
                "h" => println!("[HTTP] {}\n     {} -> {}", line.title, line.counter, line.path),
                _ => continue,
            }
        }
    }
}

fn connect(uri_host: &str, uri_path: &str, sign: &mut String) -> Result<String, String> {
    match TcpStream::connect(format!("{}:70", uri_host)) {
        Ok(mut stream) => {
            let mut content = String::new();
            stream.write(format!("{}\r\n", uri_path).as_bytes()).unwrap();
            stream.flush().unwrap();
            stream.read_to_string(&mut content).unwrap();
            if uri_path == "/" || sign == "gopherweb" {
                *sign = uri_host.to_string();
            }
            Ok(content)
        },
        Err(e) => Err(format!("Couldn't connect due to: {}", e))
    }
}

fn main() {
    let mut sign = "welcome to gopherweb".to_string();
    let mut stack: Vec<Vec<GopherStream>> = Vec::new();

    loop {
        let mut prompt = String::new();
        print!("\x1b[1m[{}]$\x1b[0m ", sign);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut prompt).unwrap();

        let vec: Vec<&str> = prompt.trim().split(" ").collect();
        match vec[0] {
            "home" => connect!("gopher.floodgap.com", "/", &mut sign, &mut stack),
            "visit" if vec.len() == 2 => connect!(vec[1], "/", &mut sign, &mut stack),
            "cd" if sign != "welcome to gopherweb".to_string() && vec.len() == 2 => {
                match vec[1].parse::<usize>() {
                    Ok(num) => {
                        if "1" != stack.last().unwrap().to_vec()[num].gopher_type {
                            println!("Given number doesn't point to directory.");
                        }
                        else {
                            connect!(sign.clone().as_str(), stack.last().unwrap().to_vec()[num].path.as_str(), &mut sign, &mut stack);
                        }
                    },
                    Err(_e) => {
                        if vec[1] != ".." { println!("Not a number given."); }
                        else {
                            stack.pop();
                            GopherStream::print(stack.last().unwrap().to_vec());
                        }
                    }
                }
            },
            "bye" => break,
            "help" => println!("{}", help::help()),
            "changelog" => println!("{}", help::changelog()),
            "history" => {
                println!("{:?}", stack);
            }
            _ => continue
        }
    }
}
