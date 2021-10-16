use clap::{App, Arg};
use regex::Regex;
use reqwest::header::USER_AGENT;
use serde_json::json;
use std::io;
use std::time::SystemTime;
use std::process::Command;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AnilistResult {
    data: Data,
}
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Data {
    #[serde(rename = "Page")]
    page: Page,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Page {
    airing_schedules: Vec<AiringSchedule>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiringSchedule {
    media: Media,
    episode: u64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Media {
    title: Title,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Title {
    romaji: String,
}

struct Category {
    name: String,
    id: String,
    is_nsfw: bool
}

impl Category{
    pub fn new(name: String, id: String, is_nsfw: bool) -> Self {
        Self {
            name,
            id,
            is_nsfw
        }
    }
}

struct Item {
    title: String,
    link: String,
    seeders: String,
    size: String,
    trusted: String,
    remake: String,
}

impl Item {
    pub fn new() -> Self {
        Self {
            title: String::new(),
            link: String::new(),
            seeders: String::new(),
            size: String::new(),
            trusted: String::new(),
            remake: String::new(),
        }
    }
}

const ANILIST_API_URL: &str = "https://graphql.anilist.co/";
const ANILIST_QUERY: &str = "
query ($airingAt_greater: Int, $airingAt_lesser: Int) {
    Page(perPage: 50) {
        airingSchedules(airingAt_greater: $airingAt_greater, airingAt_lesser: $airingAt_lesser, sort: TIME) {
            media {
                title {
                    romaji
                }
            }
            airingAt
            episode
        }
    }
}  
";


fn main() {
    let categories = vec![
        Category::new(String::from("English-Translated"), String::from("1_2"), false),
        Category::new(String::from("Non-English-Translated"), String::from("1_3"), false),
        Category::new(String::from("Raw"), String::from("1_4"), false),
        //nsfw
        Category::new(String::from("Anime[NSFW]"), String::from("1_1"), true),
        Category::new(String::from("Videos[NSFW]"), String::from("2_2") , true)
    ];

    let matches = App::new("anitor-stream")
        .version("1.0")
        .about("Watch anime/hentai from cli in your favourite video player by torrenting the source file")
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use"))
        .arg(Arg::with_name("player")
            .help("Choose which player you like(default=mpv): ")
            .short("p")
            .long("player")
            .takes_value(true)            
            .default_value("mpv")
            .possible_values(&["airplay", "chromecast", "mplayer", "mpv", "vlc", "xbmc"]))
        .get_matches();

    let mut search_query = String::from(matches.value_of("INPUT").unwrap_or(""));
    // if nothing supplied display shows of the last 24hrs (supplied by anilist)
    let client = reqwest::blocking::Client::new();
    if search_query.is_empty() {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let json = json!({"query": ANILIST_QUERY, "variables": { "airingAt_greater": now-60*60*24, "airingAt_lesser": now }});

        let res = client.post(ANILIST_API_URL)
                    .header("Content-Type", "application/json")
                    .header("Accept", "application/json")
                    .body(json.to_string())
                    .send()
                    .unwrap()
                    .text();
        // Get json    
        let result: AnilistResult = serde_json::from_str(&res.unwrap()).unwrap();
        print!("\x1B[2J");
        println!("Shows that aired in the last 24hrs:\n");
        for (i, schedule) in result.data.page.airing_schedules.iter().enumerate() {
            println!("{} {} Ep.{}", i, schedule.media.title.romaji, schedule.episode);
        } 
       
        let input = ask_input();
        //println!("{}", input);

        search_query = result.data.page.airing_schedules.get(input.trim().parse::<usize>().unwrap()).unwrap().media.title.romaji.clone();
    }
    let search_query = search_query.trim().replace(" ", "+");
    //println!("{}", search_query);

    print!("\x1B[2J");
    println!("Please choose a category by entering it's number:\n");
    for (i, c) in categories.iter().enumerate() {
        println!("{} {}", i, c.name);
    }

    let input = ask_input();
    print!("\x1B[2J");
    let category = categories.get(input.trim().parse::<usize>().unwrap()).expect("Invaild Input! That's not a category number!");

    // set up nyaa.si request
    let query_url = match category.is_nsfw {
        true=>{
            format!(
            "https://sukebei.nyaa.si/?page=rss&q={}&c={}&f=0",
            search_query, category.id)},
        false=>{
            format!(
            "https://nyaa.si/?page=rss&q={}&c={}&f=0",
            search_query, category.id)
        }
    };
    //println!("{}", query_url);

    
    let res = client.get(query_url)
        .header(USER_AGENT, "user-agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Safari/537.36")
        .send().unwrap();
    if res.status() != 200 {
        panic!(
            "Nyaa request wasn't succesfull http code: {:?}",
            res.status()
        );
    }

    let re_item = Regex::new(r"<item>[\s\S]*?</item>").unwrap();
    let body = res.text().unwrap();
    //println!("{}", body);

    let mut items: Vec<Item> = Vec::new();
    let mut terminal = term::stdout().unwrap();
    for caps in re_item.captures_iter(&body) {
        let mut item = Item::new();

        let lines = &caps.get(0).unwrap().as_str();
        for line in lines.split('\n') {
            let line = line.trim();
            if line.trim() == "" {
                continue;
            }
            //println!("{}", line);

            match line.split_inclusive(">").next().unwrap() {
                "<title>" => {
                    item.title = remove_xml_tag(line);
                },
                "<link>" => {
                    item.link = remove_xml_tag(line);
                },
                "<nyaa:seeders>" => {
                    item.seeders = remove_xml_tag(line);
                },
                "<nyaa:size>" => {
                    item.size = remove_xml_tag(line);
                },
                "<nyaa:trusted>" => {
                    item.trusted = remove_xml_tag(line);
                },
                "<nyaa:remake>" => {
                    item.remake = remove_xml_tag(line);
                },
                "</item>" => {
                    if item.trusted == *"Yes" && item.remake == *"No" {
                        terminal.fg(term::color::BRIGHT_GREEN).unwrap();
                    }
                    if item.trusted == *"No" && item.remake == *"Yes" {
                        terminal.fg(term::color::RED).unwrap();
                    }
                    println!(
                        "[{}] {} - Size {} - Seeders {}",
                        items.len() + 1,
                        item.title,
                        item.size,
                        item.seeders
                    );
                    terminal.reset().unwrap();
                    items.push(item);
                    break;
                },                
                &_=>{
                    continue;
                }
            };
        }
    }
    terminal.reset().unwrap();

    if items.is_empty() {
        panic!("no search results");
    };

    println!("Enter a number:\n");
    let input = ask_input().trim().parse::<usize>().unwrap();
    if input < 1 || input > items.len() {
        panic!("Number out of range: 1-{}", input)
    }

    let torrent = &items.get(input-1).unwrap();
    println!("{}: {}", torrent.title, torrent.link);

    Command::new("webtorrent")
        .arg(format!("--{}", matches.value_of("player").unwrap()))
        .arg(torrent.link.clone())
        .output()
        .expect("failed to execute process");
    
}

fn remove_xml_tag(s: &str) -> String {
    let mut out = s.split('>').nth(1).unwrap();
    out = out.split('<').next().unwrap();
    String::from(out)
}

fn ask_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_remove_xml_tags() {
        assert_eq!("ThisIsTextWithNoTags", remove_xml_tag("<prefixOf:someTag>ThisIsTextWithNoTags</prefixOf:someTag>"));
    }
}
