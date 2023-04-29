use postgres::{Client, NoTls, Error};
use std::time::Instant;

struct Author {
    barcode: String,
}

fn main() -> Result<(), Error> {
    
    let mut client = Client::connect("postgresql://samet:123456@192.168.1.61/MarketDB", 
                                    NoTls)?;
    
    let start_time = Instant::now();
    println!(" time: {:?}", start_time);

    
    for row in client.query("SELECT barcode FROM products_yedek", &[])? {
        let step_time = Instant::now();
        let author = Author {
            barcode: row.get(0),
        };
        //println!("Author is from  {}",  author.barcode);

    }

  
    Ok(())
} // runla sana bi 