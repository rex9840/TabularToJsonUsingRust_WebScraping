use std::fs::File; 
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
pub struct FileInfo
{ 
    pub path:String, 
    name:String, 
    extension:String
}


impl Default for FileInfo
{
    fn default()->Self
     { 
        return FileInfo{
            path:String::from(".//src//Store//"),
            name:String::from("index"), 
            extension:String::from("html") 
        };
     }
}


impl FileInfo
{

    pub  fn doc(&self,content:&str)-> std::io::Result<&str>
    {
        let file_string = format!("./{0}/{1}.{2}",self.path,self.name,self.extension);
        let  file_info:&str = &file_string;
        let mut file =  File::create(file_info).expect("Error creating file");
        file.write_all(content.as_bytes()).expect("Error writing file"); 
        return Ok("SucessCreation"); 
    }
    
    pub fn return_raw_data(&self)->std::io::Result<String>
    {
        let file_info = &format!("./{0}/{1}.{2}",self.path,self.name,self.extension);
        
        let mut buf_reader = BufReader::new(File::open(file_info).expect("cound not open file"));
        let mut content  = String::new(); 
        buf_reader.read_to_string(& mut content)?;  
        return Ok(content); 
    }

}

