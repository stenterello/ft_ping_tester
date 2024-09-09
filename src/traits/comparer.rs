use itertools::{
    EitherOrBoth::{self, *},
    Itertools, ZipLongest,
};
use ratatui::text::Line;
use std::slice::{Iter, IterMut};

pub trait Comparer {
    fn compare_output<'a>(
        ft_ping_output: &mut Vec<String>,
        ping_output: &Vec<String>,
    ) -> (bool, Option<Vec<Line<'a>>>) {
        if ft_ping_output.is_empty() && ping_output.is_empty() {
            return (false, None);
        }

        let mut v = ft_ping_output.iter_mut().zip_longest(ping_output.iter());
        let ret = v.all(|t: EitherOrBoth<&mut String, &String>| match t {
            Both(s1, s2) => s1.replace("ft_ping", "ping").eq(s2),
            _ => false,
        });
        (ret, Some(Self::format_lines(v)))
    }

    fn format_lines<'a>(v: ZipLongest<IterMut<'_, String>, Iter<'_, String>>) -> Vec<Line<'a>> {
        Vec::default()
    }
}
