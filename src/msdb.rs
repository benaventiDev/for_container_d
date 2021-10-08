use mysql::*;
use mysql::prelude::*;
#[path = "db/tweet.rs"]
pub mod tweet;
#[path = "db/pubsub.rs"]
pub mod pubsub;
use dotenv::dotenv;


pub async fn  sql_db_writer(comment:tweet::Tweet, hasHashs:bool  ) -> Result<()>{
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    
    let url: &str = &*format!( "mysql://{}:{}@{}/MYSQLDB", config.sql_server.user, config.sql_server.password, config.sql_server.host );
    let opts = Opts::from_url(url)?;
    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;
    let ins = vec![comment];
    //Code to write on Comments Table
    conn.exec_batch(r"insert into COMENTARIO (username, content, upvotes_count, downvotes_count, fecha, avatar) 
    values (:username, :content, :upvotes_count, :downvotes_count, :fecha, :avatar)", ins.iter().map(|p |params!{
        "username" => p.username.clone(),
        "content" => p.content.clone(),
        "upvotes_count" => p.upvotes, 
        "downvotes_count" => p.downvotes,
        "fecha" => p.fecha.clone(),
        "avatar" => p.avatar.clone()
    }))?;
    // Code to put Hashs table
    if hasHashs {
        let  result = conn.query_map("SELECT LAST_INSERT_ID()", 
                        | added_id| {
                        tweet::Row{added_id}
                        }, 
                    )?;
        if result.len() == 1{
            let ret_id = result[0].added_id;
            let a = &ins[0];
            let arr = a.hashtags.as_ref().unwrap();

        conn.exec_batch(r"insert into HASHTAG (tag, ID_comentario) 
            values (:tag, :comment_id )", arr.iter().map(|p |params!{
            "tag" => p.clone(),
            "comment_id" => ret_id
            }))?;
        }  
    } 

    
    /*
    let mut start = SystemTime::now();
    let start_time = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    let url: &str = &*format!( "mysql://{}:123456789@34.122.20.143:3306/MYSQLDB", config.sql_server.user );

    let opts = Opts::from_url(url)?;
    let pool = Pool::new(opts)?;
    let mut conn = pool.get_conn()?;

    
    for comment in comments{
        let has_hashs: bool = comment.hashtags != None;
        let ins: Vec<tweet::Tweet> = vec![comment];
        conn.exec_batch(r"insert into COMENTARIO (username, content, upvotes_count, downvotes_count, fecha, avatar) 
        values (:username, :content, :upvotes_count, :downvotes_count, :fecha, :avatar)", ins.iter().map(|p |params!{
          "username" => p.username.clone(),
          "content" => p.content.clone(),
          "upvotes_count" => p.upvotes, 
          "downvotes_count" => p.downvotes,
          "fecha" => p.fecha.clone(),
          "avatar" => p.avatar.clone()
      }))?;

      if has_hashs {
        let  result = conn.query_map("SELECT LAST_INSERT_ID()", 
                        | added_id| {
                        tweet::Row{added_id}
                        }, 
                    )?;

        if result.len() == 1{
            let ret_id = result[0].added_id;
            let a = &ins[0];
            let arr = a.hashtags.as_ref().unwrap();

        conn.exec_batch(r"insert into HASHTAG (tag, ID_comentario) 
            values (:tag, :comment_id )", arr.iter().map(|p |params!{
            "tag" => p.clone(),
            "comment_id" => ret_id
            }))?;
        }  
    } 
      
    }

    start = SystemTime::now();
    let time_milis = start.duration_since(UNIX_EPOCH).expect("Time went backwards") -  start_time;
    */

                Ok(())
}








