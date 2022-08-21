use std::process::exit;

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

pub fn print_error(er: &str, line: usize, r: usize, file: &str, last: &str) {
    println!(
        "{}: on line {}, {}",
        "ERROR".red(),
        line,
        &er.bright_yellow()
    );
    if let Ok(lines) = read_lines(file) {
        // Consumes the iterator, returns an (Optional) String
        let mut linenumber = 0;
        for l in lines {
            linenumber += 1;
            if linenumber == line {
                if let Ok(ip) = l {
                    println!("  {}  ", ip.bright_blue());
                    for _n in 0..r {
                        print!(" ");
                    }
                    println!("{}", "  ^".bright_yellow())
                }
            }
        }
    } else {
        println!("  {}", file.bright_blue());
        for _n in 0..r {
            print!(" ");
        }
        println!("{}", "  ^".bright_yellow());
    }

    if !last.is_empty() {
        println!(
            "  NOTE: Previous function call {}",
            last.yellow().underline()
        )
    }
    exit(1)
}
