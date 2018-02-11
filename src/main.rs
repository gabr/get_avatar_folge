extern crate regex;
extern crate reqwest;

use std::process;
use std::collections::HashMap;
use std::borrow::ToOwned;
use regex::Regex;
use reqwest::Client;
use std::{thread, time};

fn main() {
    let url = "http://serien-streams.my1.ru/index/avatar_der_herr_der_elemente/0-5";

    let client = Client::new();
    let mut resp = client.get(url).send()
        .expect(&format!("Failed to perform get request for url: {}", url));

    if false == resp.status().is_success() {
        eprintln!("Failed to fetch main url: {}", url);
        process::exit(1);
    }

    let body = resp.text().expect("Reponse get failed");

    let find_links_regex = Regex::new(r#"(?i)Staffel (?P<staffel_nr>\d+)|<a href="(?P<href>.+?Avatar.+?)".*?>Folge (?P<folge_nr>\d+?)</a>"#)
        .expect("Find links regex compilation failed");

    let mut staffel = "";
    for capture in find_links_regex.captures_iter(&body) {
        let staffel_option = capture.name("staffel_nr");
        if staffel_option.is_some() {
            staffel = staffel_option.unwrap().as_str();
            continue;
        }

        println!("Staffel {}  Folge {}  {}",
            staffel,
            &capture["folge_nr"],
            &capture["href"]);
    }

    // TODO: perform for selected url only
    let folge_url = "http://streamcloud.eu/qz3u0j7y4796/Avatar_-_Folge_018.mp4.html";
    let mut folge_resp = client.get(folge_url).send()
        .expect("Failed to get folge page");
    assert!(folge_resp.status().is_success());
    let folge_body = folge_resp.text().expect("Folge reponse get fail");

    let find_form_regex = Regex::new(r#"(?m)(?i)<form .*?class="proform".+?(\n.*)*?\n*</form>"#)
        .expect("Find form regex compilation failed");
    //println!("find form regex: {}", find_form_regex.find(&folge_body).unwrap().as_str());
    let form_string = match find_form_regex.find(&folge_body) {
        None => {
            eprintln!("Failed to find form data on page: {}", folge_url);
            process::exit(1);
        },
        Some(t) => t.as_str(),
    };

    let find_form_input_regex = Regex::new(r#"<input type=".+?" name="(?P<name>.+?)" .*?value="(?P<value>.*?)""#)
        .expect("Form input regex compilation failed");
    let mut form_params = HashMap::new();
    for capture in find_form_input_regex.captures_iter(&form_string) {
        form_params.insert(capture["name"].to_owned(), capture["value"].to_owned());
    }
    //println!("form values: {:?}", form_params);

    thread::sleep(time::Duration::from_millis(1000*15)); // <-- may vary
    let mut form_res = client.post(folge_url).form(&form_params).send()
        .expect("Failed to post form");
    let find_video_url_regex = Regex::new(r#"file: "(?P<url>.+?)""#)
        .expect("Failed to compile find video url regex");
    let form_res_text = form_res.text().expect("Failed to get form response");
    let video_url = match find_video_url_regex.captures(&form_res_text) {
        None => {
            eprintln!("Failed to find video url");
            process::exit(1);
        },
        Some(t) => t.name("url").expect("Failed to get url param from regex result").as_str(),
    };
    println!("Video url: {}", video_url);
}
