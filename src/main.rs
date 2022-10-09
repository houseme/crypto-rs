fn main() {
    use snowflake_me::Snowflake;

    let mut sf = Snowflake::new().unwrap();
    for _ in 0..1000 {
        let next_id = sf.next_id().unwrap();
        println!("{}", next_id);
    }

    println!("{}", "Hello, world!");
}
