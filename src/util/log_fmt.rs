pub struct LogFormatter {}
impl LogFormatter {
    fn chuck_message(message: &str, base_prefix: &str) -> Vec<String> {
        let (tem_w, _tem_h) = crossterm::terminal::size().expect("Could not get terminal size");
        if message.len() + base_prefix.len() <= tem_w as usize {
            return vec![format!("{}{}", base_prefix, message)];
        }
        let mut output = Vec::new();
        let mut chucked_len = 0;
        let mut start = 0;
        while start < message.len() {
            let prefix = format!("{}#{}] ", base_prefix, chucked_len);
            let chunk_size = tem_w as usize - prefix.len();
            let end = std::cmp::min(start + chunk_size, message.len());
            let part = &message[start..end];
            output.push(format!("{}{}", prefix, part));
            start += chunk_size;
            chucked_len += 1;
        }
        output
    }
    pub fn error(message: &str) -> Vec<String> {
        Self::chuck_message(message, "[Error] ")
    }
    pub fn info(message: &str) -> Vec<String> {
        Self::chuck_message(message, "[Info] ")
    }
}
