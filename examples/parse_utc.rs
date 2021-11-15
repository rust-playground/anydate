fn main() -> Result<(), Box<dyn std::error::Error>> {
    let parsed = anydate::parse_utc("2021-11-10T03:25:06.533447000Z");
    println!("{:#?}", parsed);
    Ok(())
}
