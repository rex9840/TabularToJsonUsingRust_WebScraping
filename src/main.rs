use scraper::{Html,Selector}; 
mod FileOp;
mod Request;
use json;

fn main()
{ 
    let url = r#"https://merolagani.com/LatestMarket.aspx"#;
    let html =Request::get_html(url).unwrap(); 
//html doc  parser
    let document  = Html::parse_document(&html);
    let table_selector = Selector::parse(r#"table"#).unwrap();
    let input = document.select(&table_selector).next().unwrap(); 
//table parser 
    let table_feild_select  = Selector::parse(r#"tr"#).unwrap();
    let table_feilds = input.select(&table_feild_select).next().unwrap().text().into_iter();
//json conversion    
    let mut json_obj = json::object::Object::new();
    let mut feild_len = 0 ;
    let mut counter  = 0;
    let mut keys = Vec::new();
    for feild in table_feilds
    {
        keys.push(feild);
        json_obj.insert(feild,json::JsonValue::Null);        
        feild_len+=1;   
    }
    let  mut json_list = json::array![];
//table row parser
    let table_body_select = Selector::parse(r#"tbody"#).unwrap();
    let table_body = input.select(&table_body_select).next().unwrap().text().into_iter();


    println!("\n{0:#?}\n",json_list.dump());
    println!("\n{0:#?}\n",keys); 
    for rows in table_body
    {
        json_obj.insert(keys[counter],json::JsonValue::String(rows.to_string()));
        counter+=1;
        if counter == feild_len
        {
            let _ = json_list.push(json_obj.to_owned()); 
            counter = 0;
        }    

    }


    println!("{0:#?}",json_list[46].dump());


}