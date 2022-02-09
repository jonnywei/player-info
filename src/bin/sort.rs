use std::fs;
use std::env;
use std::env::Args;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args();
    let content = fs::read_to_string(args.skip(1).next().unwrap())?;
    let c = content.split("\n");
    let mut v= vec![];
    c.for_each(|p| {
        v.push(p.to_string());
    });
    v.sort_by(|a,b|{ a.cmp(b)});
    for x in v {
        println!("{}",x);
    }
    Ok(())
}
