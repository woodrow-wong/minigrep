use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main(){
    // let args:Vec<String>=env::args().collect();


    let config=Config::from(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments:{err}");
        // Config { query: String::from("fdsdf"), file_path: "gfdgdsf".to_string() }
        process::exit(1);
    });
    
    // println!("{:?}",config);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error:{e}");
        process::exit(1);
    }

    // dbg!(args);
}

// #[derive(Debug)]
// struct Config{
//     query:String,
//     file_path:String,
// }

// impl Config {
//     fn from(args:&[String])->Result<Config,&str>{
//         if args.len()<3{
//             // panic!("not enough arguments!");
//             return Err("not enough arguments!");
//         }
//         let query=args[1].clone();
//         let file_path=args[2].clone();

//         Ok(Config{query,file_path})
//     }
// }

// fn run(config:Config)->Result<(),Box<dyn Error>>{
//     let contents=fs::read_to_string(config.file_path)?;
//         // .expect("Should have been able to read the file");
    
//     println!("With text:\n{contents}");
//     Ok(())
// }

// fn parse_config(args:&[String])->Config{
    
//     let query=args[1].clone();
//     let file_path=args[2].clone();

//     Config{query,file_path}
// }

// fn main(){
//     let a=1;
//     let b=2;
//     let _c=sum(&a,&b);
//     println!("{a}");
//     println!("{b}");
// }

// fn sum<'d>( a:&'d i32,b:&'d i32)-> &'d i32{
//     let mut c=&(a+b);
//     println!("{c}");
//     // // &4
//     let mut e="hope";
//     e="wish";
//     println!("{e}");
//     // &e
//     &5
// }