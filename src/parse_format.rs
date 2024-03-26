use std::{char, collections::HashSet};

use unicode_segmentation::UnicodeSegmentation;
#[derive(Debug)]
pub enum Parts {
    Innumerable(Vec<Vec<(Variable, Vec<Propertise>)>>),
    Numerable(Vec<Vec<(Variable, Vec<Propertise>)>>),
}
#[derive(Clone, Copy, Debug)]
pub enum Rules {
    SetSymbol(char),
    None,
}
#[derive(Debug)]
pub enum Propertise {
    Name(String),
    IsDefault,
}
#[derive(Debug)]
pub enum Variable {
    Val(String),
    String,
}
#[derive(Clone, Copy, Debug)]
struct RulesOfParse {
    space_sym: char,
}
impl Rules {
    pub fn parse(mut rule: String) -> Self {
        let mut mode = 0;
        let mut res = Rules::None;
        rule.remove(0);
        rule.remove(0);
        rule.remove(rule.len() - 1);
        for arg in rule.split(' ') {
            match arg {
                "space_symbol" => mode = 1,
                s => {
                    res = match mode {
                        1 => Self::SetSymbol(s.parse().unwrap()),
                        _ => Self::None,
                    }
                }
            }
        }
        res
    }
}
impl RulesOfParse {
    fn new() -> Self {
        RulesOfParse { space_sym: ' ' }
    }
    fn update(self, rule: String) -> Self {
        let mut new = self;
        match Rules::parse(rule) {
            Rules::SetSymbol(s) => new.space_sym = s,
            Rules::None => (),
        }
        new
    }
}
pub fn parse_segment(mut acc: Vec<String>, str: &str) -> Vec<String> {
    match (str.find('['), str.find(']')) {
        (Some(start), Some(end)) => {
            acc.push(str[start - 1..=end].to_string());
            parse_segment(acc, &str[end + 1..])
        }
        (_, _) => acc,
    }
}
trait SkipSplit {
    fn skip_split(self, split: char) -> Vec<String>;
}
impl SkipSplit for String {
    fn skip_split(self, split: char) -> Vec<String> {
        let mut exc_chars = HashSet::from(['\'', '"']);
        exc_chars.remove(&split);
        let mut last = String::new();
        let mut res = vec![];
        for s in self.split(split) {
            let len = exc_chars
                .iter()
                .fold(0, |acc, x| s.matches(x.clone()).count() + acc);
            if last.is_empty() {
                if len % 2 == 0 {
                    res.push(s.to_string());
                } else {
                    last = s.to_string();
                }
            } else {
                last.push_str(format!(" {s}").as_str());
                if len % 2 != 0 {
                    res.push(last.clone());
                    last.clear();
                }
            }
        }
        res
    }
}
pub fn comments_cleaning(mut format: String) -> String {
    let mut start = 0;
    let mut is_comment = false;
    for (i, c) in format.clone().char_indices() {
        if c == '#' {
            start = i;
            is_comment = true;
        }
        if c == '\n' && is_comment {
            format = format.replace(&format.as_str()[start..=i], "");
            is_comment = false;
        }
    }
    format
}
impl Parts {
    pub fn parse(mut format: String) -> Vec<Parts> {
        format = comments_cleaning(format);
        let lines = parse_segment(vec![], format.as_str());
        let mut rop = RulesOfParse::new();
        lines
            .iter()
            .filter_map(|val| match val.chars().nth(0).unwrap() {
                '!' => {
                    rop = rop.update(val.to_string());
                    None
                }
                '+' | '[' => Some(parse_value(val.to_string(), rop)),
                _ => {
                    let mut s = val.clone();
                    s.remove(0);
                    Some(parse_value(s.to_string(), rop))
                }
            })
            .collect::<Vec<Parts>>()
    }
}
fn parse_value(mut val: String, rules: RulesOfParse) -> Parts {
    let mut _type: u8 = match val.chars().nth(0).unwrap() {
        '+' => {
            val.remove(0);
            0
        }
        _ => 1,
    };
    val.remove(0);
    val.remove(val.len() - 1);
    let segment: Vec<Vec<(Variable, Vec<Propertise>)>> = val
        .skip_split(rules.space_sym)
        .iter()
        .map(|seg| {
            seg.split('|')
                .map(|variant| {
                    let mut prop = vec![];
                    let mut var = variant;
                    match (variant.find('('), variant.find(')')) {
                        (Some(start), Some(end)) => {
                            let mut chars =
                                UnicodeSegmentation::graphemes(&variant[start..=end], true)
                                    .collect::<Vec<&str>>();
                            chars.remove(0);
                            let mut word: Vec<&str> = Vec::with_capacity(chars.len());
                            for c in chars {
                                if c.eq(",") || c.eq(")") {
                                    prop.push(match word.concat().as_str() {
                                        "default" => Propertise::IsDefault,
                                        v => Propertise::Name(v.replace("'", "")),
                                    });
                                    word.clear();
                                } else {
                                    word.push(c)
                                }
                            }
                            var = &variant[..start];
                        }
                        (_, _) => (),
                    }
                    (
                        match var.replace("\"", "").as_str() {
                            "{}" => Variable::String,
                            v => Variable::Val(v.to_string()),
                        },
                        prop,
                    )
                })
                .collect()
        })
        .collect();
    match _type {
        0 => Parts::Innumerable(segment),
        _ => Parts::Numerable(segment),
    }
}
