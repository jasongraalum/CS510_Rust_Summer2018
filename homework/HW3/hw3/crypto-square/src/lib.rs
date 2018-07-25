#![allow(unused)]
pub fn encrypt(input: &str) -> String {

    //let input = "adddbc def 1 HIJ asdfjasd";
    let mut normalized : String = input.to_lowercase().chars().filter(|c| c >= &'a' && c <= &'z').map(|c| c).collect();
    
    let len = normalized.len();  
    let sq = (normalized.len() as f64).sqrt().ceil() as usize;
    println!("{} {}", len, sq);
    
    let diff = (sq*sq)-len;
    let filler : String = ['_'].iter().cycle().take(diff).collect();
    normalized.push_str(&filler);
    

    let coded : String = normalized.chars().cycle().enumerate().filter(|(i,_c)| ((i - (i / (sq*sq))) % sq) == 0).take(len).map(|(_i,c)| c).collect();
    coded
}

