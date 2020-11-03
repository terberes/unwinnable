pub fn parse_number_selection(select: &String)
                              -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut include_range = vec![];
    for word in select.split_whitespace() {
        if word.contains("-") {
            let bounds: Vec<&str> = word.split("-").collect::<Vec<&str>>();
            if bounds.len() != 2 {
                return Err(format!("Invalid range: {}", word).into());
            }

            let lower: u32 = bounds[0].parse()?;
            let upper: u32 = bounds[1].parse()?;

            if lower == upper {
                include_range.push(word.parse()?);
                continue;
            }

            if lower > upper {
                return Err("Reverse range is not allowed".into());
            }

            include_range.extend(lower..upper);
        } else {
            include_range.push(word.parse()?);
        }
    };
    include_range.sort();
    include_range.dedup();
    Ok(include_range)
}
