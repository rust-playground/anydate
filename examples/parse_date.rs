fn main() {
    let parsed = anydate::date::parse("2021-11-10");
    println!("{parsed:#?}");
}
