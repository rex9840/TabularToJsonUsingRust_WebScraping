use scraper::{Html,Selector}; 
mod FileOp;
mod Request;
use json;
use Vec;



fn main()
{ 
    // let url = r#"https://meta.wikimedia.org/wiki/List_of_Wikipedias/Table"#;
    let url = r#"https://en.wikipedia.org/wiki/List_of_in-memory_databases"#;
    let html =Request::get_html(&url)
                                .unwrap(); 
//html doc  parser
    let document  = Html::parse_document(&html);
    let table_selector = Selector::parse(r#"table"#).unwrap();
    let input = document.select(&table_selector)
                                        .next()
                                        .unwrap(); //gets  thead

//json conversion    
    let mut json_obj = json::object::Object::new(); 
    let mut feild_len = 0 ;
    let mut counter  = 0;
    let mut keys= Vec::new();
    let  mut json_list = json::array![];
//table row parser
    let table_body_select = Selector::parse(r#"tr"#).unwrap();
    let mut table_body = input.select(&table_body_select);
    let table_header = table_body.next().unwrap().text().into_iter();
    for data in table_header
    {   
        let data_= data.trim().trim_end().replace("\n","").to_string();
        if data_ == "" {continue;}
        keys.push(data_);
        feild_len+=1;
    }

    println!("\n{0:?}\n",keys);
    println!("\nfeild_len:{0}\n",feild_len);
    println!("\n----------------------------------------------\n");
    for rows in table_body
    {
        let slector = Selector::parse(r#"td"#).unwrap();
        let table_td = rows.select(&slector);
        let mut data_ = String::new(); 
        for td_data in table_td
        {
            for data in td_data.text().into_iter(){
            data_= data_.clone()+&data.trim().trim_end().replace("\n","").to_string();  
            if data_ == "" {continue;}  
          
        }
        println!("{0:?}:{1:?}->{2:?}",counter,keys[counter],data_);
        
        json_obj.insert(&keys[counter],json::JsonValue::String(data_.to_string()));

        data_ ="".to_string();
        counter+=1;
        
        if counter == feild_len
        {
            let _ = json_list.push(json_obj.to_owned()); 
            counter = 0;
            println!("\n----------------------------------------------\n");
        }    
    }
}

    println!("{0}", json_list[0].dump());
    let _ = Request::json_dump_file(json_list); 


}