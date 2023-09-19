use reqwest;
use serde;
use serde_json; 


#[allow(dead_code)]

fn get_html(url:&str)->Result<String,reqwest::Error>
{ 
    let response  = reqwest::blocking::get(url)?; 
    println!("STATUS:{0:?}",response.status()); 
    let html = response.text()?; 
    return Ok(html);
}

fn table_scraping(html_content:String)->Result<String,String>
{ 
     return Ok("".to_string()); 
}




fn main()
{ 
    let url:&str = r#"https://www.worldometers.info/coronavirus/#countries"#;
    let html =get_html(url).unwrap(); 
    println!("{0:#?}", html);

    let table = table_scraping(html).unwrap();

    

}

