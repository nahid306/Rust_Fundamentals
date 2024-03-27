fn main(){
    let s= String::from("nahid");
    println!("print the value of given index:{}",match s.chars().nth(2){
        Some(c)=>c.to_string(),
        None =>" no value fund at this index". to_string()

    })

}