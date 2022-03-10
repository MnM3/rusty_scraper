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
    //chaining .await will yield our query result
    let resp = reqwest::get("https://www.bundestag.de/parlament/plenum/sitzungskalender/bt2022-837528").await.unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().await.unwrap();
    //println!("{:?}", body);
    let fragment = Html::parse_document(&body);

    let stories = Selector::parse(".table").unwrap();

    let mut dates =  Vec::<&str>::new();

    for story in fragment.select(&stories) {
        let story_txt = story.text().collect::<Vec<_>>();
        
        //println!("{:?}", story_txt);
        //type_of(&story_txt);
        dates = story_txt;
    }

    //println!("{:?}", dates);

    let current_date = chrono::Utc::now().date();
    //println!("{}", current_date.month());
    //let test: str = ({if current_date.month() < 10 {"0"} else {""}},current_date.month(),".",current_date.year() ); 
    let mut test = String::from("");
    test.push_str(if current_date.month() < 10 {"0"} else {""});
    test.push_str(&current_date.month().to_string());
    test.push_str(".");
    test.push_str(&current_date.year().to_string());
    //println!("{}",test);

    //println!("{:?}", resp.status())
    //println!("{:?}", result);
    println!("In den folgenden Zeiträumen finden Bundestagssitzungen statt: ");
    for s in dates {
        if s.contains(&test) {
            println!("{}", s)
        }
    }
}

fn type_of<T>(_: &T) {
    println!("{}",type_name::<T>())
}




/*   
    fn hacker_news(url: &str) {
    let mut resp = reqwest::get(url).await.unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));


        */

    
    