pub fn reverse(input: &str) -> String {

    let mut reversedString:String = String::new();
    let chars:Vec<char> = input.chars().collect();
    let lengthString = chars.len();
    println!("{}",lengthString);
    for i in 0..lengthString {
    let ch =  chars[lengthString-1-i] ;
    reversedString.push(ch);
    }
    reversedString
}
