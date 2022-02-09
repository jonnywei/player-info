use std::fs;
use std::env;
use std::env::Args;
use std::error::Error;
use std::ops::Index;

fn main() -> Result<(), Box<dyn Error>> {

    let args: Args = env::args();
    let content = fs::read_to_string(args.skip(1).next().unwrap())?;
    let c = content.split("\n");
    let mut v= vec![];
    c.for_each(  |p| {
        let body = reqwest::blocking::get(p).unwrap()
            .text().unwrap();
        let  index = body.find("<div class=\"baike-desc c-color c-line-clamp4\" style=\"text-align:left;\" data-a-5db495cf>");
        if index.is_none(){
            println!("index is none");
            return
        }
        let content = &body[(index.unwrap()+87)..];
        // println!("content = {:?}",content);
        let index = content.find("</div>");
        if index.is_none(){
            return
        }
        println!("body = {:?}",&content[..index.unwrap()] );
        v.push((&content[..index.unwrap()]).to_string());

    });
    for x in v {
        println!("{}",x);
    }
    Ok(())
}
