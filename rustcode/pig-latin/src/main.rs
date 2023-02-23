
fn main() {
    let slice = "ay";
    let string = String::from("apple");

    if let Some(chr) = string.chars().nth(0) {
        match chr {
            'a' | 'e' |'i'|'o'|'u' => println!("{:?}", string+"-h"+&slice),
            _ => println!("{}", "other"),
        }
    } else {
        println!("{:?}", &slice);
    }
}
