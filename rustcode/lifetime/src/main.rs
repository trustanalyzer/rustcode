#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel
    .split('.')
    .next()
    .expect("could find splitter");

    let important = ImportantExcerpt{
        part: first_sentence,
    };

    println!("{:?}", important);
}