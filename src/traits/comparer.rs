use itertools::{EitherOrBoth::*, Itertools};
use std::path::Path;

pub trait Comparer {
    fn set_errors(&mut self, val: bool) -> ();

    fn search_fields(output: Vec<String>) -> (String, String) {
        let mut ret: (String, String) = (String::default(), String::default());
        match output.get(0) {
            Some(line) => {
                if let Some((path, message)) = line.split_once(": ") {
                    ret.0 = String::from(message);
                    ret.1 = String::from(Path::new(path).to_str().unwrap());
                }
            }
            None => return ret,
        };
        ret
    }

    fn remove_path(output: &mut Vec<String>) -> (&mut Vec<String>, String) {
        let (message_to_save, parent_path) = Self::search_fields(output.clone());
        if parent_path.is_empty() {
            (output, "".to_string())
        } else {
            output.remove(0);
            output.insert(0, message_to_save);
            (output, String::from(parent_path))
        }
    }

    fn compare_output(
        &mut self,
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
                                let eq = *c == *nc;
                                if !eq {
                                    self.set_errors(true);
                                }
                                ret[ret_index].push((eq, *c));
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
                        self.set_errors(true);
                    });
                }
                Right(_) => {}
            }
            ret_index += 1;
        }

        ret
    }
}
