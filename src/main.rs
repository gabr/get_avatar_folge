extern crate regex;
extern crate reqwest;

use std::env;
use std::io;
use std::io::Write; // required or 'flush()' will not be found
use std::{thread, time, process};
use std::borrow::ToOwned;
use std::collections::HashMap;
use regex::Regex;
use reqwest::Client;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    let args_len = args.len();
    if (args_len < 3) || (args_len > 4) {
        eprintln!(r#"! wrong arguments

    usage:
        {} <Staffel> <Folge> [<delay_time> - min value is 10]

"#, args[0]);
        process::exit(1);
    }

    let staffel: i32 = args[1].parse().expect("Failed to parse Staffel number");
    let folge: i32 = args[2].parse().expect("Failed to parse folge number");
    let mut sleep_sec: u64 = 20;
    if args_len == 4 {
        sleep_sec = args[3].parse().expect("Failed to parse seconds");
    }

    if (staffel < 1) || (staffel > 3) {
        eprintln!("Wrong staffel number - <1; 3> allowed only, given: {}", staffel);
        process::exit(1);
    }

    if folge < 1 {
        eprintln!("Wrong folge number - cannot be negative or 0, given: {}", folge);
        process::exit(1);
    }

    if sleep_sec < 10 {
        eprintln!("Min sleep time is 10, given: {}", sleep_sec);
        process::exit(1);
    }

    let episodes_urls = [
        "http://streamcloud.eu/y2nbibtcvkky/Avatar_-_Folge_001.flv.html",
        "http://streamcloud.eu/g2aacpjlf4hv/Avatar_-_Folge_002.flv.html",
        "http://streamcloud.eu/eidf216foh7r/Avatar_-_Folge_003.flv.html",
        "http://streamcloud.eu/hx0nm565idjq/Avatar_-_Folge_004.flv.html",
        "http://streamcloud.eu/0ufynp47k1rq/Avatar_-_Folge_005.flv.html",
        "http://streamcloud.eu/mdy0zlcph9oc/Avatar_-_Folge_006.flv.html",
        "http://streamcloud.eu/mwb1du0skxat/Avatar_-_Folge_007.flv.html",
        "http://streamcloud.eu/2svzzj8wuh8t/Avatar_-_Folge_008.flv.html",
        "http://streamcloud.eu/5nnkrjvbhxrp/Avatar_-_Folge_009.flv.html",
        "http://streamcloud.eu/df78ddrfzigk/Avatar_-_Folge_010.flv.html",
        "http://streamcloud.eu/b5oxv02d24b1/Avatar_-_Folge_011.flv.html",
        "http://streamcloud.eu/prz1xcqdk6vx/Avatar_-_Folge_012.flv.html",
        "http://streamcloud.eu/ab63il4r5r8j/Avatar_-_Folge_013.flv.html",
        "http://streamcloud.eu/yuqgza2z78vc/Avatar_-_Folge_014.mp4.html",
        "http://streamcloud.eu/uul1p7mfq9ea/Avatar_-_Folge_015.flv.html",
        "http://streamcloud.eu/99xhh95ecfru/Avatar_-_Folge_016.flv.html",
        "http://streamcloud.eu/sbcgro2c19f9/Avatar_-_Folge_017.flv.html",
        "http://streamcloud.eu/qz3u0j7y4796/Avatar_-_Folge_018.mp4.html",
        "http://streamcloud.eu/kjv3ehw7ssvs/Avatar_-_Folge_019.flv.html",
        "http://streamcloud.eu/ut290aalvdl9/Avatar_-_Folge_020.flv.html",
        "http://streamcloud.eu/36br3srweof7/Avatar_-_Folge_021.mp4.html",
        "http://streamcloud.eu/hm2g7oq2m2c0/Avatar_-_Folge_022.mp4.html",
        "http://streamcloud.eu/et7dryr4vo37/Avatar_-_Folge_023.flv.html",
        "http://streamcloud.eu/y0616hwidagx/Avatar_-_Folge_024.flv.html",
        "http://streamcloud.eu/cbcx7hp4ninx/Avatar_-_Folge_025.mp4.html",
        "http://streamcloud.eu/gw1ibhnoud3h/Avatar_-_Folge_026.flv.html",
        "http://streamcloud.eu/9a1n1zsmgksh/Avatar_-_Folge_027.mp4.html",
        "http://streamcloud.eu/3dsg95axcrp8/Avatar_-_Folge_028.mp4.html",
        "http://streamcloud.eu/fd8iy9ed6h3u/Avatar_-_Folge_029.flv.html",
        "http://streamcloud.eu/trpuuhmzh37u/Avatar_-_Folge_030.flv.html",
        "http://streamcloud.eu/xb9e23z666g4/Avatar_-_Folge_031.flv.html",
        "http://streamcloud.eu/pndcldba4hzw/Avatar_-_Folge_032.flv.html",
        "http://streamcloud.eu/ljdvykc6ol01/Avatar_-_Folge_033.flv.html",
        "http://streamcloud.eu/8f3dlzv6xa6y/Avatar_-_Folge_034.mp4.html",
        "http://streamcloud.eu/e0ni94k7cad0/Avatar_-_Folge_035.flv.html",
        "http://streamcloud.eu/mshfzsuce69k/Avatar_-_Folge_036.flv.html",
        "http://streamcloud.eu/q7rk6y3wisuv/Avatar_-_Folge_037.mp4.html",
        "http://streamcloud.eu/u4pqgn46fiqf/Avatar_-_Folge_038.flv.html",
        "http://streamcloud.eu/0gy3yqgdvm3x/Avatar_-_Folge_039.flv.html",
        "http://streamcloud.eu/ae8bvafs4koa/Avatar_-_Folge_040.flv.html",
        "http://streamcloud.eu/ypte99aul5rn/Avatar_-_Folge_041.mp4.html",
        "http://streamcloud.eu/5g4y30vshxa9/Avatar_-_Folge_042.flv.html",
        "http://streamcloud.eu/hitad4zkq3ql/Avatar_-_Folge_043.mp4.html",
        "http://streamcloud.eu/cuajy22mmji0/Avatar_-_Folge_044.flv.html",
        "http://streamcloud.eu/oapg8fcwtmju/Avatar_-_Folge_045.flv.html",
        "http://streamcloud.eu/senjm49vafyk/Avatar_-_Folge_046.mp4.html",
        "http://streamcloud.eu/8u89m9p2b6lx/Avatar_-_Folge_047.mp4.html",
        "http://streamcloud.eu/291z7tm0ikox/Avatar_-_Folge_048.mp4.html",
        "http://streamcloud.eu/0c04j8kkjr7c/Avatar_-_Folge_049.mp4.html",
        "http://streamcloud.eu/hfq700xex34a/Avatar_-_Folge_050.mp4.html",
        "http://streamcloud.eu/ujo9leqvsabt/Avatar_-_Folge_051.mp4.html",
        "http://streamcloud.eu/8frx3clb1ai1/Avatar_-_Folge_052.mp4.html",
        "http://streamcloud.eu/xybd5fetyp0d/Avatar_-_Folge_053.mp4.html",
        "http://streamcloud.eu/opiskvdc4jvs/Avatar_-_Folge_054.mp4.html",
        "http://streamcloud.eu/xoyguarc5azn/Avatar_-_Folge_055.mp4.html",
        "http://streamcloud.eu/zzmrafilcksa/Avatar_-_Folge_056.mp4.html",
        "http://streamcloud.eu/ed1zxv2vdbqj/Avatar_-_Folge_057.mp4.html",
        "http://streamcloud.eu/ao0a2pd9y168/Avatar_-_Folge_058.mp4.html",
        "http://streamcloud.eu/16usew0rzoxr/Avatar_-_Folge_059.mp4.html",
        "http://streamcloud.eu/21a2uv0yno80/Avatar_-_Folge_060.mp4.html",
        "http://streamcloud.eu/0j9kfq3gwx5h/Avatar_-_Folge_061.mp4.html"
    ];

    let episode_nr: usize = ((staffel - 1) * 20 + folge) as usize;
    if episode_nr > episodes_urls.len() {
        eprintln!("Calculated episode number {} is to hight. Max value: {}", episode_nr, episodes_urls.len());
        process::exit(1);
    }

    let client = Client::new();

    /* Getting episodes urls:
    let url = "http://serien-streams.my1.ru/index/avatar_der_herr_der_elemente/0-5";

    let mut resp = client.get(url).send()
        .expect(&format!("Failed to perform get request for url: {}", url));

    if false == resp.status().is_success() {
        eprintln!("Failed to fetch main url: {}", url);
        process::exit(1);
    }

    let body = resp.text().expect("Reponse get failed");

    let find_links_regex = Regex::new(r#"(?i)Staffel (?P<staffel_nr>\d+)|<a.*?href="(?P<href>.+?Avatar.+?)".*?>Folge (?P<folge_nr>\d+?)</a>"#)
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
    */

    // TODO: perform for selected url only
    let folge_url = episodes_urls[episode_nr];
    println!("{}: {}", episode_nr, folge_url);

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

    println!("Waiting {}sec...", sleep_sec);
    io::stdout().flush().expect("Could not flush stdout");
    for i in 0..sleep_sec {
        print!("{}\r", i+1);
        io::stdout().flush().expect("Could not flush stdout");
        thread::sleep(time::Duration::from_millis(1000));
    }

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

    /*
        https://www.reddit.com/r/rust/comments/6ymhkw/how_can_i_download_a_file_with_hyper/
        https://docs.rs/reqwest/0.7.3/reqwest/

            use std::io::Read;

            let mut resp = reqwest::get("https://www.rust-lang.org")?;
            assert!(resp.status().is_success());

          > let mut content = String::new();
          > resp.read_to_string(&mut content);

        https://docs.rs/reqwest/0.7.3/reqwest/struct.Response.html
    */
}
