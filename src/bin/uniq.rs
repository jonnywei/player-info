use std::fs;
use std::env;
use std::env::Args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {

    let args: Args = env::args();
    let content = fs::read_to_string(args.skip(1).next().unwrap())?;
    let c = content.split("\n");
    let mut v= vec![];
    let mut last_player ="".to_string();
    c.for_each(|p| {
        if last_player != p {
            last_player = p.to_string();
            v.push(p);
        }
    });
    for x in v {
        println!("{}",x);
    }
    Ok(())
}
