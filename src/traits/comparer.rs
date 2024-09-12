use itertools::{EitherOrBoth::*, Itertools};

pub trait Comparer {
    fn compare_output(
        ft_ping_output: &mut Vec<String>,
        ping_output: &Vec<String>,
    ) -> Vec<Vec<(bool, char)>> {
        if ft_ping_output.is_empty() && ping_output.is_empty() {
            return vec![vec![(true, char::default())]];
        }
        let mut translated: Vec<String> = vec![];
        for string in ft_ping_output.iter() {
            translated.push(string.replace("ft_ping", "ping"));
        }
        let v = translated.iter().zip_longest(ping_output.iter());
        let mut ft_ping_ret: Vec<Vec<(bool, char)>> = Vec::default();
        let mut index: usize = 0;

        for string_couple in v {
            match string_couple {
                Both(l, r) => {
                    let v2 = l.chars().zip_longest(r.chars());
                    for char_couple in v2 {
                        match char_couple {
                            Both(c1, c2) => match ft_ping_ret.get_mut(index) {
                                Some(s) => s.push(((c1 == c2), c1)),
                                None => {
                                    ft_ping_ret.push(Vec::default());
                                    ft_ping_ret[index].push(((c1 == c2), c1));
                                }
                            },
                            Left(c1) => match ft_ping_ret.get_mut(index) {
                                Some(s) => s.push((false, c1)),
                                None => {
                                    ft_ping_ret.push(Vec::default());
                                    ft_ping_ret[index].push((false, c1));
                                }
                            },
                            Right(_) => {}
                        }
                    }
                }
                Left(l) => {
                    for c in l.chars() {
                        match ft_ping_ret.get_mut(index) {
                            Some(s) => s.push((false, c)),
                            None => {
                                ft_ping_ret.push(Vec::default());
                                ft_ping_ret[index].push((false, c));
                            }
                        }
                    }
                }
                Right(_) => {}
            }
            index += 1;
        }
        ft_ping_ret
    }
}
