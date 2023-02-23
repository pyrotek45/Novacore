use colored::Colorize;

use super::state::read_lines;

pub fn _read_lines<P>(
    filename: P,
) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}
pub fn trim_newline(s: &mut String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    s.to_string()
}

pub fn is_string_number(data: &str) -> bool {
    let mut deci = false;
    let mut start_index = 0;
    if data.is_empty() {
        return false;
    }
    if data.starts_with('-') {
        start_index = 1;
        // Check that there is at least one numeric or '.' character after the '-' symbol
        if data.len() == 1
            || (data.len() > 1 && !data.chars().skip(1).any(|c| c.is_numeric() || c == '.'))
        {
            return false;
        }
    }
    if data[start_index..].starts_with('.') {
        return false;
    }
    for (i, c) in data.chars().enumerate().skip(start_index) {
        //Checks to see if there is more than one period
        if c == '.' && deci {
            return false;
        }
        //Checks to see if it is a number, and makes sure it skips first period
        if !c.is_numeric() && c != '.' {
            return false;
        }
        //Changes deci to true after finding first period
        if c == '.' {
            deci = true
        }
        // Allows '-' symbol only at the beginning of the string
        if c == '-' && i != start_index {
            return false;
        }
    }
    true
}

pub fn print_line(line: usize, file: &str) {
    if let Ok(lines) = read_lines(file) {
        // Consumes the iterator, returns an (Optional) String
        let mut linenumber = 0;
        for l in lines {
            linenumber += 1;
            if linenumber == line {
                if let Ok(ip) = l {
                    println!("Line: {}: {} ", line, ip.white());
                }
            }
        }
    }
}

// pub fn print_error(err: &str) {
//     println!("{}: {}", "ERROR".red(), &err.bright_yellow());
// }
