use itertools::{
    EitherOrBoth::{self, *},
    Itertools,
};

pub struct Comparer;

impl Comparer {
    pub fn compare_output(ft_ping_output: &mut Vec<String>, ping_output: &Vec<String>) -> bool {
        let mut v = ft_ping_output.iter_mut().zip_longest(ping_output.iter());
        v.all(|t: EitherOrBoth<&mut String, &String>| match t {
            Both(s1, s2) => s1.replace("ft_ping", "ping").eq(s2),
            _ => return false,
        })
    }

    pub fn compare_exit_code(ft_ping_code: u8, ping_code: u8) -> bool {
        true
    }
}
