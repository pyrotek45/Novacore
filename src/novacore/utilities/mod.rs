use colored::Colorize;

pub fn read_lines<P>(
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
    let mut deci: bool = false;
    if data.is_empty() {
        return false;
    }
    if data.starts_with('.') {
        return false;
    }
    for c in data.chars() {
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
    }
    true
}

pub fn print_error(err: &str) {
    println!("{}: {}", "ERROR".red(), &err.bright_yellow());
}
