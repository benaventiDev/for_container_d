use actix_web:: {HttpResponse,  web::{Bytes}};//App, HttpServer,   {, post}
use serde_json::Map;
use serde_json::value::Value;
#[path = "db/cosmosdb.rs"]
pub mod cosmosdb;
#[path = "./msdb.rs"]
mod msdb;
use std::time::{SystemTime, UNIX_EPOCH};

//#[path = "./main.rs"]
//mod main;
static mut COUNTER: i32 = 0;
static mut SQL_TIME_COUNTER: u128 = 0;
static mut COSMOS_TIME_COUNTER: u128 = 0;

pub async fn startLoad(bytes: Bytes) -> Result<String, HttpResponse> {
    unsafe{
        COUNTER = 0;
        SQL_TIME_COUNTER = 0;
        COSMOS_TIME_COUNTER = 0;
    }
    Ok(format!("START!"))
}

pub async fn endLoad(bytes: Bytes) -> Result<String, HttpResponse> {
    unsafe{
        let open_br = "{";
        let close_br = "}";
        let pub_sub_sql = format!("{}\"guardados\": {:?},\n\"api\": \"Rust\",\n\"tiempoDeCarga\": {},\n\"bd\": \"SQL\"{}", 
        open_br, COUNTER, SQL_TIME_COUNTER/1000, close_br);
        let pub_sub_cosmos = format!("{}\"guardados\": {:?},\n\"api\": \"Rust\",\n\"tiempoDeCarga\": {},\n\"bd\": \"COSMOS\"{}", 
        open_br, COUNTER, COSMOS_TIME_COUNTER/1000, close_br);
        println!("SQL: {}", pub_sub_sql);
        println!("COSMOS: {}", pub_sub_cosmos);
    }
    Ok(format!("END!"))
}


pub async fn create(bytes: Bytes) -> Result<String, HttpResponse> {
    let mut start = SystemTime::now();
    let mut start_time = start.duration_since(UNIX_EPOCH).expect("Time went backwards");



    match String::from_utf8(bytes.to_vec()) {
        Ok(body) => {
            let content: &str = &*body;
            let comment_sql: msdb::tweet::Tweet = serde_json::from_str(content).unwrap();  
            let comment_cosmos: cosmosdb::tweet::Tweet = serde_json::from_str(content).unwrap();  
            if comment_sql.username != ""{
                let has_hashs = comment_sql.hashtags != None;
                msdb::sql_db_writer(comment_sql, has_hashs).await;
                start = SystemTime::now();
                let mut time_milis = start.duration_since(UNIX_EPOCH).expect("Time went backwards") -  start_time;
                unsafe{
                    SQL_TIME_COUNTER = SQL_TIME_COUNTER + time_milis.as_millis();
                }
                //start_time = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
                cosmosdb::cosmos_db_writer(comment_cosmos, has_hashs).await;
                start = SystemTime::now();
                time_milis = start.duration_since(UNIX_EPOCH).expect("Time went backwards") -  start_time;
                unsafe{
                    COSMOS_TIME_COUNTER = COSMOS_TIME_COUNTER + time_milis.as_millis();
                }
                //println!("Time after COSMOS: {:?}", time_milis.as_millis());
                
                

            }


            unsafe{
                COUNTER = COUNTER + 1;
                //println!("It works! :) {:?}", COUNTER);
            }

            
            //COUNTER = COUNTER + 1;
            Ok(format!("It worked!\n"))},
        Err(_) => Err(HttpResponse::BadRequest().into())
    }
}



//Read File:
pub fn read_file(data: String) -> Map<String, Value> {
    
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state
}






//Read File:
/*
pub fn read_file(data: String) -> Map<String, Value> {
    
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state
}*/


/*
pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}


pub struct Pending {
    pub super_struct: Base
    }
    impl Pending {
    pub fn new(input_title: &str) -> Pending {
    let base: Base = Base::new(input_title,
    "pending");
    return Pending{super_struct: base}
    }
}


    pub struct Done {
        pub super_struct: Base
    }
    
    impl Done {
        pub fn new(input_title: &str) -> Done {
        let base: Base = Base::new(input_title,
        "done");
        return Done{super_struct: base}
        }
    }


        pub struct Base {
            pub title: String,
            pub status: String
        }

        impl Base {
            pub fn new(input_title: &str, input_status: &str) -> Base {
            return Base {title: input_title.to_string(),
            status: input_status.to_string()}
        }



/// This view creates a to do item and saves it in the state.json file.
///
/// # Arguments
/// * req (HttpRequest): the HTTP request passed into the view
///
/// # Returns
/// * (String): message to be sent back to the user
pub async fn create(req: HttpRequest) -> String {

    // get the title from the http request
    let title: String = req.match_info().get("title"
    ).unwrap().to_string();
    let title_reference: String = title.clone()
    
    // return a message to viewer
    return format!("{} created", "title_reference")
}*/



