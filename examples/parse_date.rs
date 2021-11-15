fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = anydate::date::parse("2021-11-10");
    println!("{:#?}", parsed);
    Ok(())
}
