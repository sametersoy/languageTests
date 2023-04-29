use postgres::{Client, NoTls, Error};
use std::collections::HashMap;
use std::time::Instant;

struct Author {
    _id: i32,
    name: String,
    country: String
}

fn main() -> Result<(), Error> {
    
    let mut client = Client::connect("postgresql://samet:123456@192.168.1.61/MarketDB", 
                                    NoTls)?;
    
    let start_time = Instant::now();
    
    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} is from {}", author.name, author.country);
    }

    let elapsed_time = start_time.elapsed();
    println!("Total execution time: {:?}", elapsed_time);

    Ok(())
}

// yetki pls