// pub fn parse_number_selection(select: &String)
//                               -> Result<(Vec<u32>, Vec<u32>), Box<dyn std::error::Error>> {
//     let mut include_range = vec![];
//     let mut exclude_range = vec![];
//     for mut word in select.split_whitespace() {
//         let mut exclude = word.starts_with("^");
//         if exclude {
//             word = word.strip_prefix("^").unwrap();
//         }
//         if word.contains("-") {
//             let bounds = word.split("-").collect::<Vec<&str>>();
//             if bounds.len() != 2 {
//                 return Err(format!("Invalid range: {}", word).into());
//             }
//             let mut effective_range =
//                 ((bounds[0].parse()?)..=(bounds[1].parse()?)).collect();
//             if exclude {
//                exclude_range.append(&mut effective_range);
//             } else {
//                 include_range.append(&mut effective_range)
//             }
//         } else if exclude {
//             exclude_range.push(word.parse()?)
//         } else {
//             include_range.push(word.parse()?)
//         }
//     };
//     Ok((include_range, exclude_range))
// }
//
// use std::collections::HashSet;
//
// pub fn parse_number_selection(select: &String)
//                               -> Result<HashSet<u32>, Box<dyn std::error::Error>> {
//     let mut include_range: HashSet<u32> = HashSet::new();
//     for word in select.split_whitespace() {
//         if word.contains("-") {
//             let bounds: Vec<&str> = word.split("-").collect::<Vec<&str>>();
//             if bounds.len() != 2 {
//                 return Err(format!("Invalid range: {}", word).into());
//             }
//             let effective_range =
//                 (bounds[0].parse()?)..=(bounds[1].parse()?);
//
//             effective_range.for_each(|x| { include_range.insert(x); });
//             include_range.insert(word.parse()?);
//         }
//     };
//     Ok(include_range)
// }

use std::collections::HashSet;

pub fn parse_number_selection(select: &String)
                              -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut include_range = vec![];
    for word in select.split_whitespace() {
        if word.contains("-") {
            let bounds: Vec<&str> = word.split("-").collect::<Vec<&str>>();
            if bounds.len() != 2 {
                return Err(format!("Invalid range: {}", word).into());
            }
            let effective_range =
                (bounds[0].parse()?)..=(bounds[1].parse()?);

            effective_range.for_each(|x| { include_range.push(x); });
        } else {
            include_range.push(word.parse()?);
        }
    };
    include_range.sort();
    include_range.dedup();
    Ok(include_range)
}
