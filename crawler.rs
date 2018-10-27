use http::HTTP;
use http::Protocol;
use http::Method;
use regex::Regex;
use select::document::Document;
use select::predicate::*;
use rusqlite::Connection;

//Stores http page & its depth from source
#[allow(dead_code)]
pub struct Resource {
    http: HTTP,
    depth: i32,
}

//Constructs Resource object
impl Resource {
    pub fn new(http: HTTP, depth: i32) -> Resource {
        Resource {
            http: http,
            depth: depth,
        }
    }
}

//Represents crawl items in database
pub struct CrawlItem {
    pub id: i32,
    pub url: String,
}

//Stores information from crawler
pub struct Agent {
    depth: i32,
}

impl Agent {
    //Constructs Agent object
    pub fn new(
    ) -> Agent {
        Agent {
            depth: 0,
        }
    }

    //Analyzes the http responses for crawling
    pub fn parse(address: String, href: String) -> String {
        // "http://www.um.ac.ir/"
        if Regex::new(r"\w*://.+$").unwrap().is_match(&href) {
            //Extract the domain: www.um.ac.ir
            let domain = href.split("://").nth(1).unwrap().split("/").nth(0).unwrap();
            //If source address is equal to href
            if domain == address {
                let resource = href.split("://").nth(1).unwrap();
                //If there is a resource identifier
                if resource.find('/') != None {
                    let (_first, last) = resource.split_at(resource.find('/').unwrap());
                    return last.to_string();
                }
                //No URI exists in reference
                return "URI_ERROR".to_string();
            }
            //URL is different from source
            return "URL_ERROR".to_string();
        }
        // "mailto:khademi@um.ac.ir"
        else if Regex::new(r"\w+:.*$").unwrap().is_match(&href) {
            //Extract the method & its value
            let (_method, _value) = href.split_at(href.find(':').unwrap());
        }
        // "/Students.html"
        else if Regex::new(r"\w+$").unwrap().is_match(&href) {
            //If the first character is "/"
            if (href.find('/') != None) && (href.find('/').unwrap() == 0) {
                let (_first, last) = href.split_at(href.find('/').unwrap());
                return last.to_string();
            } else {
                return "/".to_string() + &href;
            }
        }
        return "UNKNOWN_ERROR".to_string();
    }

    //Crawlers over the input http object
    pub fn crawl(mut self, url: String, depth: i32) -> i32 {
        let mut hseed = HTTP::new(
            url.to_string(),
            80,
            Protocol::HTTP,
            Method::Get,
            "/".to_string(),
        );

        hseed.set_body("Ok".to_string());
        hseed.do_request();

        let mut rseed: Vec<Resource> = Vec::new();
        rseed.push(Resource::new(hseed, self.depth));


        let mut resources_list: Vec<Vec<Resource>> = Vec::new();
        resources_list.push(rseed);

        let mut url_counter = 0;
        while self.depth <= depth {
            self.depth = self.depth + 1;
            let past_resources = resources_list.remove(0);
            let mut future_resources: Vec<Resource> = Vec::new();
            for resource in past_resources {
                let mut http = resource.http.clone();
                //If the response is invalid
                if !http.is_sent() {
                        http.do_request();
                }
                let document = Document::from_str(&String::from_utf8(http.get_response().body()).unwrap());


                // println!("\n##########################################################################################################\n");
                // for tag in document.find(Name("a")).iter() {
                //     if let Some(href) = tag.attr("href") {
                //         println!("{}‍", href);
                //     }
                // }
                // println!("\n##########################################################################################################\n");

                //Crawls through relevant http response tags
                for tag in document.find(Name("a")).iter() {
                    if let Some(href) = tag.attr("href") {

                        let mut _http = http.clone();
                        let uri = Agent::parse(http.address.to_string(), href.to_string());

                        //Check for duplicate URIs in the same depth
                        let mut check = false;
                        for resource in future_resources.iter() {
                            let old = resource.http.address.to_string() + &resource.http.request.header.uri;
                            let new = resource.http.address.to_string() + &uri;
                            if old == new {
                                check = true;
                            }
                        }
                        //If there is no duplicate URI
                        if !check {
                            //If there is no error in analysis
                            if (uri != "UNKNOWN_ERROR") & (uri != "URL_ERROR") & (uri != "URI_ERROR") {
                                //Update the URI & response of http page
                                _http.request.header.uri = uri;
                                //_http.clear_response();
                                url_counter = url_counter + 1;
                                let url_string = resource.http.address.to_string() + &_http.request.header.uri;
                                println!("[{}]->{}", url_counter, url_string);
                                //Push the new resource into resources vector
                                future_resources.push(Resource::new(_http, self.depth));
                            }
                        }
                    }
                }
            }
            resources_list.push(future_resources);
        }

        println!("\n==> Storing all URL addresses into database.");

        let path = "./".to_string() + &url;
        let conn = Connection::open(path).unwrap();

        conn.execute("CREATE TABLE crawl_item (
                      id              INTEGER PRIMARY KEY,
                      url            TEXT NOT NULL
                      )", &[]).unwrap();

        let mut id = 0;
        for resources in resources_list.iter() {
            for resource in resources.iter() {
                let url = resource.http.address.to_string() + &resource.http.request.header.uri;
                let query = "INSERT INTO crawl_item (id, url)
                          VALUES (?1, ?2)".to_string();
                conn.execute(
                    &query,
                    &[
                        &id,
                        &url,
                    ],
                ).unwrap();
                id = id + 1;
            }
        }

        println!("\n==> Data insertion successfull.");

        resources_list.clear();

        println!("\n==> Reading all URL addresses from database:\n");

        let query = "SELECT id, url FROM crawl_item";

        //Outputs the query into stmt
        let mut stmt = conn.prepare(query).unwrap();

        //Creates an iterator for the query
        let crawl_item_iter = stmt.query_map(&[], |row| CrawlItem {
            id: row.get(0),
            url: row.get(1),
        }).unwrap();

        //Finds user in user_iter
        for item in crawl_item_iter {
            let value = item.ok().unwrap();
            println!("[{}]:{}", value.id, value.url);
        }
        return 1;
    }
}

// use crawler::Agent;
// let mut _depth: i32 = 0;
// let mut _resources: Vec<crawler::Resource> = Vec::new();
// let mut _agent = Agent::new(_depth, _resources);
// _agent.crawler("www.um.ac.ir".to_string(), 0);
