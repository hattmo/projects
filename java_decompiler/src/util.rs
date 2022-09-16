use colored::*;

pub trait Pretty {
    fn pretty(&self) -> String;
}
impl Pretty for &[u8] {
    fn pretty(&self) -> String {
        let content = self
            .iter()
            .map(|x| {
                if x.is_ascii_alphanumeric() {
                    format!("{}", *x as char).blue().to_string()
                } else {
                    format!("{{{x:02X}}}").normal().to_string()
                }
            })
            .collect::<String>();
        format!("[{content}]")
    }
}
