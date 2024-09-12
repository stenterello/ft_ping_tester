use itertools::{EitherOrBoth::*, Itertools};

pub trait Comparer {
    fn compare_output(
        ft_ping_output: &mut Vec<String>,
        ping_output: &Vec<String>,
    ) -> Vec<Vec<(bool, u8)>> {
        if ft_ping_output.is_empty() && ping_output.is_empty() {
            return vec![vec![(true, u8::default())]];
        }

        let mut translated: Vec<String> = Vec::default();
        for string in ft_ping_output.iter() {
            translated.push(string.replace("ft_ping", "ping"));
        }

        let mut ret: Vec<Vec<(bool, u8)>> = Vec::default();
        let mut ret_index: usize = 0;

        let v = translated.iter().zip_longest(ping_output.iter());
        for string_couple in v {
            match string_couple {
                Both(l, r) => {
                    let mut ping_iter = r.as_bytes().iter();
                    l.as_bytes().iter().for_each(|c| {
                        match ret.get(ret_index) {
                            Some(_) => {}
                            None => ret.push(Vec::default()),
                        }

                        let nc = ping_iter.next();
                        match nc {
                            Some(nc) => {
                                ret[ret_index].push((*c == *nc, *c));
                            }
                            None => ret[ret_index].push((false, *c)),
                        }
                    });
                }
                Left(l) => {
                    l.as_bytes().iter().for_each(|c| {
                        match ret.get(ret_index) {
                            Some(_) => {}
                            None => ret.push(Vec::default()),
                        }

                        ret[ret_index].push((false, *c));
                    });
                }
                Right(_) => {}
            }
            ret_index += 1;
        }

        ret
    }
}
