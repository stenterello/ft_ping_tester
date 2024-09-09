use itertools::{
    EitherOrBoth::{self, *},
    Itertools,
};
use ratatui::{style::Color, text::Line};

fn format_lines<'a>(
    ft_ping_output: &Vec<String>,
    ping_output: &Vec<String>,
) -> (Vec<Line<'a>>, Vec<Line<'a>>) {
    let i = ft_ping_output.iter().zip_longest(ping_output.iter());
    let mut ft_ping_vec: Vec<Line<'a>> = vec![];
    let ping_vec: Vec<Line<'a>> = vec![];
    for couple in i {
        match couple {
            Both(l, r) => {}
            Left(l) => {
                // ft_ping_vec.push();
            }
            Right(r) => {}
        }
    }
    (Vec::default(), Vec::default())
}

pub trait Comparer {
    fn compare_output<'a>(
        ft_ping_output: &mut Vec<String>,
        ping_output: &Vec<String>,
    ) -> (bool, Option<(Vec<Line<'a>>, Vec<Line<'a>>)>) {
        if ft_ping_output.is_empty() && ping_output.is_empty() {
            return (false, None);
        }

        let mut v = ft_ping_output.iter_mut().zip_longest(ping_output.iter());
        let ret = v.all(|t: EitherOrBoth<&mut String, &String>| match t {
            Both(s1, s2) => s1.replace("ft_ping", "ping").eq(s2),
            _ => false,
        });
        (ret, Some(format_lines(ft_ping_output, ping_output)))
    }
}
