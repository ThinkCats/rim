use mysql::{Pool, prelude::Queryable};

pub fn pool() -> Result<Pool, mysql::Error> {
    let url = "mysql://root:12345678@localhost:3306/rim";
    Pool::new(url)
}

type GroupRow = (u64,String,String,u8);

pub fn query_group(id: u32) {
    let sql = format!( "select id,name,avatar,type from `groups` where id = {}",id); 
    println!("SQL:{}",sql);
    let pool = pool().unwrap();
    let mut conn = pool.get_conn().unwrap();
    let result: Vec<GroupRow> = conn.query(sql).unwrap();
    println!("result:{:?}", result);
}