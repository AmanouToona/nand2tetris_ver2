pub fn parse_filename(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }
    let filename = args[1].clone();
    Ok(filename)
}
