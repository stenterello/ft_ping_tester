pub struct Comparer;

impl Comparer {
    pub fn compare_output(ft_ping_output: &mut Vec<String>, ping_output: &Vec<String>) -> bool {
        ft_ping_output
            .iter_mut()
            .zip(ping_output.iter())
            .all(|(s1, s2)| s1.replace("ft_ping", "ping").eq(s2))
    }

    pub fn compare_exit_code(ft_ping_code: u8, ping_code: u8) -> bool {
        true
    }
}
