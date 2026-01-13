pub fn reverse(input: &str) -> String {

    let mut reversed_string:String = String::new();
    let chars:Vec<char> = input.chars().collect();
    let length_string = chars.len();
    //println!("{}",length_string);
    for i in 0..length_string {
    let ch =  chars[length_string-1-i] ;
    reversed_string.push(ch);
    }
    reversed_string
}
