
use std::io::{self, Read, Write};

fn read_all() -> Result<String, Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = vec!();

    let _ = std::io::stdin().read_to_end(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}

fn get_max_line_len(input: &String) -> usize {
    let split = input.split("\n");

    let size_iter = split.map(|substr: &str| -> usize {substr.len()});
    let max_size = size_iter.fold(usize::MIN, |acc, num| -> usize {acc.max(num)} );

    return max_size
}

fn build_top_str(max_line_len: usize) -> String {
    ["╭", "―".repeat(max_line_len).as_str(), "╮"].concat()
}

fn build_bottom_str(max_line_len: usize) -> String {
    ["╰", "―".repeat(max_line_len).as_str(), "╯"].concat()
}

fn build_body_str(line: &str, max_line_len: usize) -> String {
    let line_size: usize = line.len();
    ["│", line, " ".repeat(max_line_len - line_size).as_str(), "│"].concat()
}

fn boxify(input: &String) -> String {
    let max_line_len: usize = get_max_line_len(input);
    let split = match input.strip_suffix("\n") {
        Some(prefix) => prefix.split("\n"),
        None => input.split("\n")
    };

    let mut out: Vec<String> = vec!();

    out.push(build_top_str(max_line_len));
    split.for_each(|substr: &str| out.push(build_body_str(substr, max_line_len)));
    out.push(build_bottom_str(max_line_len));


    out.join("\n") + "\n"
}

fn main() -> io::Result<()>{
    let input = match read_all() {
        Ok(input) => input,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
    };

    let mut writer = io::BufWriter::new(io::stdout());
    let _ = writer.write_all(boxify(&input).as_bytes());
    // println!("max line len is: {}", get_max_line_len(&input));

    Ok(())
}
