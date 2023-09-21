use std::vec;

use scraper::{Html,Selector}; 
mod FileOp;
mod Request;
use json;



fn json_dump_file(json_obj:json::JsonValue)->std::io::Result<FileOp::FileInfo>
{
    let mut file_info = FileOp::FileInfo::default(); 
    file_info.change_extension(String::from("json"));
    file_info.change_name(String::from("ShareShansarData"));

    file_info.doc(&json_obj.dump()).expect("Error creating file"); 
    return Ok(file_info);
}

fn main()
{ 
    let url = r#"https://merolagani.com/LatestMarket.aspx"#;
    let html =Request::get_html(url).unwrap(); 
//html doc  parser
    let document  = Html::parse_document(&html);
    let table_selector = Selector::parse(r#"table"#).unwrap();
    let input = document.select(&table_selector).next().unwrap(); //gets  thead
//table parser 
    let table_feild_select  = Selector::parse(r#"thead>tr"#).unwrap();
    let table_feilds = input.select(&table_feild_select);
//json conversion    
    let mut json_obj = json::object::Object::new();
    let mut feild_len = 0 ;
    let mut counter  = 0;
    let mut keys = Vec::new();
    for feild in table_feilds
    {
        for data in feild.text().into_iter()
        {
        keys.push(data);
        json_obj.insert(data,json::JsonValue::Null);        
        feild_len+=1;   
    }
    }
    let  mut json_list = json::array![];
//table row parser
    let table_body_select = Selector::parse(r#"tbody>tr"#).unwrap();
    let table_body = input.select(&table_body_select);


    println!("\n{0:#?}\n",json_list.dump());
    println!("\n{0:#?}\n",keys); 
    for rows in table_body
    {
        for data in rows.text().into_iter()
        {
            println!("{0:?}:{1:?}->{2:?}",counter,keys[counter],data);
        
        json_obj.insert(keys[counter],json::JsonValue::String(data.to_string()));
        counter+=1;
        
        if counter == feild_len
        {
            let _ = json_list.push(json_obj.to_owned()); 
            counter = 0;
        }    
        
        }
    
    }
    println!("{0:#?}",json_list.dump());
    let _ = json_dump_file(json_list); 

}