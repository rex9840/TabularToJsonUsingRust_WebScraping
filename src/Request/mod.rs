use crate::{FileOp ,FileOp::FileInfo};
use reqwest;


pub fn get_html(url:&str)->Result<String,reqwest::Error>
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



pub fn doc_create(raw_data:&str,file_info:FileInfo)-> std::io::Result<FileOp::FileInfo>
{ 
    file_info.doc(raw_data).expect("Error creating file"); 
    return Ok(file_info);
}

fn doc_raw_data(file:FileOp::FileInfo)->std::io::Result<String>
{
    println!("\n\n{0:#?}\n\n",file);
    let data:String =file.return_raw_data()?;
    return Ok(data.to_owned());
} 
