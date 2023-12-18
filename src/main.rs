
use std::io::{self, Read, Write};

static SPACES_PER_TAB: usize = 8;

fn read_all() -> Result<String, Box<dyn std::error::Error>> {
    let mut buf: Vec<u8> = vec!();

    let _ = std::io::stdin().read_to_end(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}

fn expand(str_in: &String) -> String {
    str_in.replace("\t", " ".repeat(SPACES_PER_TAB).as_str())
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
    let cleaned_input = expand(input);
    let max_line_len: usize = get_max_line_len(&cleaned_input);

    let split = match (&cleaned_input).strip_suffix("\n") {
        Some(prefix) => prefix.split("\n"),
        None => cleaned_input.split("\n")
    };

    let mut out: Vec<String> = vec!();

    out.push(build_top_str(max_line_len));
    split.for_each(|substr: &str| out.push(build_body_str(substr, max_line_len)));
    out.push(build_bottom_str(max_line_len));


    out.join("\n") + "\n"
}

#[cfg(test)]
mod tests {
    use crate::{*};

    #[test]
    fn test_build_top_str() {
        assert_eq!(build_top_str(0), "╭╮");

        assert_eq!(build_top_str(5), "╭―――――╮");
    }

    #[test]
    fn test_build_bottom_str() {
        assert_eq!(build_bottom_str(0), "╰╯");

        assert_eq!(build_bottom_str(5), "╰―――――╯");
    }

    #[test]
    fn test_build_body_str() {
        assert_eq!(build_body_str("", 0), "││");

        assert_eq!(build_body_str("test", 10), "│test      │");

        assert_eq!(build_body_str("test", 4), "│test│");

    }

    #[test]
    fn test_expand() {
        let test_str = String::from("\ttest");

        assert_eq!(expand(&test_str).len(), 4 + SPACES_PER_TAB);
    }
}

fn main() -> io::Result<()>{
    let input = match read_all() {
        Ok(input) => input,
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
    };

    let mut writer = io::BufWriter::new(io::stdout());
    let _ = writer.write_all(boxify(&input).as_bytes());

    Ok(())
}
