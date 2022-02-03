pub fn get_args(str: &String) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let current: &mut Vec<char> = &mut Vec::new();
    let mut quote = false;

    for char in str.chars() {
        if char == ' ' && !quote && current.len() > 0 {
            out.push(String::from_iter(std::mem::take(current)));
            continue;
        }
        if char == '"' {
            if current.len() > 0 {
                out.push(String::from_iter(std::mem::take(current)));
            }
            quote = !quote;
            continue;
        }

        current.push(char);
    }

    if current.len() > 0 {
        out.push(String::from_iter(std::mem::take(current)));
    }

    out
}
