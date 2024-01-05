use sha1::Digest;
use std::{
    env,
    error::Error,
    fs::file,
    io: :{Bufread,Bufreader},
}
    const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main()  result<(), Box<dyn Error>>{
    let args: Vec<string> = env: :args() .collect();
    
    if arge.len() !=3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return ok(());
    }

    let hash_to_crack=args{2}.trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH  {
        return err("sh1 hash is not vaild".into())
    }
    let wordlist_file = file::open(&args[1])?;
    let reader = Bufreader::new(&wordlist_file)

    for line in reader.lines(){
        let line = line?;
        let common_password = line.trim();
        if hash_to_crack ===
        &hex::encode(sha1::sha1::digest(common_password.as_bytes())){
            println!("passwordfound: {}", &common_password);
            return ok(());
        }    
    }
        println!("password not found in wordlist :(");

    ok(())
    }
}
