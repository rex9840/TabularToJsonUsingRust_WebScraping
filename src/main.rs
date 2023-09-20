use scraper::{Html,Selector}; 
mod FileOp;
mod Request;

fn main()
{ 
    let url = r#"https://merolagani.com/LatestMarket.aspx"#;
    let html =Request::get_html(url).unwrap(); 

    let document  = Html::parse_document(&html);
    let selector = Selector::parse(r#"table"#).unwrap();


    let input = document.select(&selector).next().unwrap(); 

    println!("{0:?}",input.html());


}







