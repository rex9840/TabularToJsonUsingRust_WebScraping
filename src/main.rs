use scraper::{Html,Selector}; 
mod FileOp;
mod Request;

fn main()
{ 
    let url = r#"https://www.worldometers.info/coronavirus/#countries"#;
    let html =Request::get_html(url).unwrap(); 

    let document  = Html::parse_document(&html);
    let selector = Selector::parse(r#"div[class="maincounter-number"]"#).unwrap();


    let input = document.select(&selector).next().unwrap(); 

    println!("{0:?}",input.text().collect::<Vec<_>>());


}







