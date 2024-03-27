fn main() {
    let mut  height=190;
    height = height - 20;
    let result = if height > 180{
        "tall"
    } else if height > 170 {
        "avarge"
    } else {
        "short"
    };
    println!("result: {}",result);
    
    let helth = if height <180 {"good"} else {"unknown"};
    println!("helth:{}",helth);

}