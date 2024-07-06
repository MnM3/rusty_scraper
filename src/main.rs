extern crate reqwest;
extern crate select;
extern crate scraper;


//use select::document::Document;
//use select::predicate::Name;
use scraper::{Html, Selector};
use std::any::type_name;
use chrono::Datelike;
use std::env;
//use chrono; //0.4.19

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    //hacker_news("https://news.ycombinator.com");
    let mut mainarg = &args[1];
    if mainarg.eq("Bundestag") || mainarg.eq("bundestag") {
        bundestag_sitzungen().await;
    }
    

fn type_of<T>(_: &T) {
    println!("{}",type_name::<T>())
}

async fn bundestag_sitzungen() {
    let resp = reqwest::get("https://www.bundestag.de/parlament/plenum/sitzungskalender/bt2024-941110").await.unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().await.unwrap();
    let fragment = Html::parse_document(&body);
    let stories = Selector::parse(".table").unwrap();
    let mut dates =  Vec::<&str>::new();

    println!("In den folgenden Zeiträumen finden Bundestagssitzungen statt: ");
    for story in fragment.select(&stories) {
        let story_txt = story.text().collect::<Vec<_>>();
        for s in &story_txt {
            if s.contains("-") {
                println!("{}", s);
                dates.push(s);
            }
        }
    
    }
}
}


    
    