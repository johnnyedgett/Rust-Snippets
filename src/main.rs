use std::env;
use rand::Rng;

fn main(){
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: rust_snippets snippet_type args");
        println!("See reference table in README for available types");
        return;
    }
    match args[1].as_ref() {
        "random_strings" => {
            let num_strs: u32 = args[2].trim().parse::<u32>().unwrap();
            let str_len: u32 = args[3].trim().parse::<u32>().unwrap();

            let mut strs: Vec<String> = Vec::new();

            for _i in 0..num_strs {
                strs.push(gen_rand_string(str_len));
            }

            for string in strs {
                println!("{}", string);
            }
        },
        _ => {
            println!("Usage: rust_snippets snippet_type args");
            println!("See reference table in README for available types");
        }
    }
}

fn gen_rand_string(len: u32) -> String {
    let mut s = String::from("");
    let opts: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    
    for _i in 0..len {
        let num = rand::thread_rng().gen_range(1, opts.len());
        s.push(opts[num]);
    }

    return s;
}