use std::fs::File;
use std::io::prelude::*;
use http::Protocol;
use http::Method;
use http::HTTP;
use rusqlite::Connection;

//Specifies vulnerabillity level (based on OWASP)
#[allow(dead_code)]
pub enum Vulnerabillity {
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
}

//Specifies fuzzing method
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum Option {
    DIRECTORY_SEARCH,
    SQL_INJECTION,
}

//Implements Output for fuzzer
#[allow(dead_code)]
pub struct Output {
    pub original: HTTP, //Original HTTP page
    pub fuzzed: Vec<HTTP> //Fuzzed HTTP page
}

//Implements FuzzItem table structure
#[allow(dead_code)]
pub struct FuzzItem {
    id: i32,
    url: String,
}

//Constructs Output object
#[allow(dead_code)]
impl Output {
    pub fn new( original: HTTP, fuzzed: Vec<HTTP>) -> Output {
        Output {
            original,
            fuzzed,
        }
    }
}

//Includes necessary functions
#[allow(dead_code)]
pub struct Fuzzer {
    option: Option,
}

//Constructs Fuzzer object
#[allow(dead_code)]
impl Fuzzer {
    pub fn new(option: Option) -> Fuzzer {
        Fuzzer {
            option,
        }
    }

    //Implements Directory search for http pages
    pub fn dir_search(url: String) {
        let mut seed = HTTP::new(
            url.to_string(),
            80,
            Protocol::HTTP,
            Method::Get,
            "/".to_string(),
        );
        seed.set_body("Ok".to_string());
        seed.do_request();
        let mut http_list: Vec<HTTP> = Vec::new();
        let mut dir_list: Vec<String> = Vec::new();

        //Reads directories from file
        let mut file = match File::open("src/directory_list.txt") {
            Ok(value) => value,
            Err(msg) => panic!("File oppening error {}", msg),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_value) => (),
            Err(msg) => panic!("File reading error: {}", msg),
        }

        //Writes directories in a vector
        let mut lines = contents.lines();
        let mut dir = lines.next();
        while dir != None {
            let mut temp = match dir {
                Some(value) => value.to_string(),
                None => panic!("Direcotry list is empty!"),
            };
            temp.remove(0);
            dir_list.push(temp.to_string());
            dir = lines.next();
        }

        //Sends http requests accordingly
        for dir in dir_list.iter() {
            let mut _http = seed.clone();
            _http.request.header.uri = format!("{}{}",
            seed.request.header.uri.to_string(), dir.to_string());
            _http.do_request();
            http_list.push(_http);
        }

        let mut id = 0;
        for http in http_list.iter() {
            let url = format!("{}{}",
            http.address.to_string(), http.request.header.uri.to_string());
            // println!("{}", http.response.header.code);
            // println!("{}", http.response.header.status);
            if http.response.header.code == 200 {
                println!("\nALARM! VULNERABLE WEBSITE:\n[{}]->{}\n", id, url);
            }
            else {
                println!("[{}]->{}", id, url);
            }
            id = id + 1;
        }

        // let path = format!("./{}", url);
        // let conn = Connection::open(path).unwrap();
        //
        // match conn.execute("CREATE TABLE fuzz_item (
        //                   id        INTEGER PRIMARY KEY,
        //                   url       TEXT NOT NULL
        //                   )", &[]) {
        //                       Ok(_value) => (),
        //                       Err(msg) => panic!("Query execution error: {}", msg),
        //                   }
        //
        // let mut id = 0;
        // for http in http_list.iter() {
        //     let url = format!("{}{}",
        //     http.address.to_string(), http.request.header.uri.to_string());
        //     let query = "INSERT INTO fuzz_item (id, url)
        //               VALUES (?1, ?2)".to_string();
        //     match conn.execute(&query, &[&id, &url]) {
        //         Ok(_value) => (),
        //         Err(msg) => panic!("FuzzItem insertion error: {}", msg),
        //     }
        //     id = id + 1;
        // }
        //
        // let query = "SELECT id, url FROM fuzz_item";
        //
        // //Outputs the query into stmt
        // let mut stmt = conn.prepare(query).unwrap();
        //
        // //Creates an iterator for the query
        // let fuzz_item_iter = stmt.query_map(&[], |row| FuzzItem {
        //     id: row.get(0),
        //     url: row.get(1),
        // }).unwrap();
        //
        // //Finds user in user_iter
        // for item in fuzz_item_iter {
        //     let value = item.ok().unwrap();
        //     println!("[{}]->{}", value.id, value.url);
        // }
    }

    //Executes the fuzzing operation for http page
    pub fn execute(&self, url: String) {
        //Choose the execution method
        match &self.option {
            Option::DIRECTORY_SEARCH => Fuzzer::dir_search(url),
            Option::SQL_INJECTION => (),
        }
    }
}

// use fuzzer::Fuzzer;
// use fuzzer::Option;
// use fuzzer::Output;
// let mut _fuzzer = Fuzzer::new(Option::DIRECTORY_SEARCH);
// _fuzzer.execute("www.um.ac.ir".to_string());
