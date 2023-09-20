use reqwest::{self, Response, dns::Resolving};
use scraper::{Html,Selector}; 
use std::{fs::File, default};

use crate::FileOp::FileInfo; 
mod FileOp;

fn get_html(url:&str)->Result<String,reqwest::Error>
{ 
    
    if reqwest::blocking::get(url).is_ok()
    {
        let response = reqwest::blocking::get(url)?;
        println!("STATUS:{0:?}",response.status());

    if  response.status().is_success()
    {
        let html = response.text()?; 
        let raw_data = doc_raw_data(doc_create(&html,FileOp::FileInfo::default()).expect("CreationERROR"));
        return Ok(raw_data.unwrap())
    }
    else
    {
        let raw_data = doc_raw_data(FileOp::FileInfo::default());
        return Ok(raw_data.unwrap()); 
    }
    }
    
    else
    {
        let raw_data = doc_raw_data(FileOp::FileInfo::default());
        return Ok(raw_data.unwrap());     
    }
    
}



fn doc_create(raw_data:&str,file_info:FileInfo)-> std::io::Result<FileOp::FileInfo>
{ 
    println!("{0:#?}",file_info);
    file_info.doc(raw_data).expect("Error creating file"); 
    return Ok(file_info);
}

fn doc_raw_data(file:FileOp::FileInfo)->std::io::Result<String>
{
    let data:String =file.return_raw_data()?;
    return Ok(data.to_owned());
} 


fn main()
{ 
    let url = r#"https://www.worldometers.info/coronavirus/#countries"#;
    let html =get_html(url).unwrap(); 

    let document  = Html::parse_document(&html);
    let selector = Selector::parse(r#"div[class="maincounter-number"]"#).unwrap();


    let input = document.select(&selector).next().unwrap(); 

    println!("{0:?}",input.text().collect::<Vec<_>>());


}







