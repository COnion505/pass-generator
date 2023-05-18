use rand::Rng;
use std::fs::File;
use std::io::Write;

extern crate dotenv;
use dotenv::dotenv;

extern crate dirs;

#[macro_use] extern crate log;
extern crate simplelog;
fn main(){
    init_log();
    
    dotenv().ok();
    let pass_length: String = dotenv::var("PASS_LENGTH").unwrap_or_else(|_| String::from("10"));
    let pass_length: i32 = pass_length.parse().unwrap_or_else(|e| {
        warn!("can't parse PASS_LENGTH cause {}. use default num 10.", e);
        10
    });
    info!("password length: {:?}", pass_length);
    let password: String = generate_pass(pass_length);

    let file_path: String = dotenv::var("FILE_PATH").unwrap_or_else(|e| {
        warn!("can't get env FILE_PATH cause {}. use default path Desktop.", e);
        dirs::desktop_dir().unwrap().display().to_string()
    });
    let file_name: String = dotenv::var("FILE_NAME").unwrap_or_else(|e| {
        warn!("can't get env FILE_NAME cause {}. use default name pass.txt.", e);
        String::from("pass.txt")});
    info!("file_path: {}", &file_path);
    info!("file_name: {}", &file_name);
    create_pass_file(file_path, file_name, password);
}
fn generate_pass(pass_length: i32) -> String{
    const WORDS1: [&str; 26] = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
    const WORDS2: [&str; 26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
    const WORDS3: [&str; 10] = ["0","1","2","3","4","5","6","7","8","9"];
    let mut rng = rand::thread_rng();
    let mut pass: Vec<&str> = Vec::new();
    for _ in 0..pass_length {
        let mut words1_index: usize = rng.gen();
        let mut words2_index: usize = rng.gen();
        let mut words3_index: usize = rng.gen();
        let mut selecter: usize = rng.gen();
        words1_index %= WORDS1.len();
        words2_index %= WORDS2.len();
        words3_index %= WORDS3.len();
        selecter %= 3;
        if selecter == 0 {
            pass.push(WORDS1[words1_index]);
        } else if selecter == 1{
            pass.push(WORDS2[words2_index]);
        } else if selecter == 2 {
            pass.push(WORDS3[words3_index]);
        }
    }
    let mut s = String::new();
    for w in pass.iter(){
        s.push_str(w);
    }
    s
}
fn create_pass_file(file_path: String, file_name: String, password: String){
    use std::path::Path;

    // let filepath = Path::new(&file_name);   
    let full_path: String = format!("{}{}", &file_path, &file_name);
    let full_path: &Path = Path::new(&full_path);
    let mut file = match File::create(full_path) {
        Err(why) => {
            error!("Couldn't create {}: {}", &full_path.display(), why);
            panic!("Couldn't create {}: {}", &full_path.display(), why);
        },
        Ok(file) => file,
    };

    match file.write_all(password.as_bytes()) {
        Err(why) => {
            error!("Couldn't write \"{}\" to {}: {}", password, &file_name, why);
            panic!("Couldn't write \"{}\" to {}: {}", password, &file_name, why);
        },
        Ok(_) => info!("generated: {}", &full_path.display()),
    }
}

fn init_log(){
    simplelog::CombinedLogger::init(vec![
        simplelog::WriteLogger::new(
            simplelog::LevelFilter::Info,
            simplelog::Config::default(),
            File::create("passgenerator.log").unwrap(),
        ),
    ])
    .unwrap()
}
