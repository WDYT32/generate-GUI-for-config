use crate::parse_format::*;
use std::{collections::HashMap, fs};
use unicode_segmentation::UnicodeSegmentation;

pub fn gen(parts: Vec<Parts>, src: String, format: String) -> String {
    let mut fragments: HashMap<String, String> = HashMap::new();
    fs::read_dir(src).unwrap().for_each(|el| {
        let e = el.unwrap();
        if e.file_type().unwrap().is_file() {
            let (name, path) = (e.file_name().into_string().unwrap(), e.path());
            fragments.insert(name, fs::read_to_string(path).unwrap());
        }
    });
    let base = fragments.get("base.html").unwrap().to_string();
    let mut script = fragments.get("scripts.html").unwrap().to_string();
    let (mut innumerable, mut numarable) = (
        Vec::with_capacity(parts.len()),
        Vec::with_capacity(parts.len()),
    );
    let mut i = 0;
    parts.iter().for_each(|part| match part {
        Parts::Innumerable(e) => {
            innumerable.push(
                fragments
                    .get("innumerable.html")
                    .unwrap()
                    .replace("$n", format!("{i}").as_str()),
            );
            script = script.replace(
                "//<!--html-fragment-->",
                &format!(
                    "case {i}: hf='{}';break;//<!--html-fragment-->",
                    gen_html(e)
                ),
            );
            i += 1;
        }
        Parts::Numerable(e) => {
            numarable.push(format!("<div class=\"numerable\">{}</div>", gen_html(e)))
        }
    });
    let skp_char = match get_rule(format.clone(), "space_char".to_string()) {
        Rules::SetSymbol(c) => c,
        Rules::None => ' ',
    }
    .to_string();
    base.replace("<!--innumerable-->", &innumerable.concat())
        .replace("<!--numerable-->", &numarable.concat())
        .replace(
            "<!--code-->",
            &script
                .replace("<!--format-->", &format_to_script(format, skp_char.clone()))
                .replace(
                    "<!--space_char-->",
                    &skp_char
                        .to_string()
                        .replace("\n", "\\n")
                        .replace("\t", "\\t"),
                ),
        )
}
fn format_to_script(format: String, skip_char: String) -> String {
    let mut res = String::with_capacity(format.len());
    let mut innumerable = 0;
    let mut mode = 0;
    let mut pervious_mode = 0;
    let mut word_skiped = false;
    let mut specless_mode = false;
    let mut i = 0;
    UnicodeSegmentation::graphemes(format.as_str(), true).for_each(|c| match c {
        ch if specless_mode => {
            if ch.eq("@") {
                mode = 4;
            } else if ch.eq("\"") {
                if mode == 4 {
                    specless_mode = false;
                    res.pop();
                    return;
                }
            } else if mode == 4 {
                mode = 0;
            }
            res.push_str(ch);
        }
        ch if mode == 4 => {
            if c.eq("\"") {
                specless_mode = true;
            } else {
                res.push_str(ch);
            }
            mode = 0;
        }
        "[" => {
            if mode == 0 {
                mode = 3;
            }
        }
        "@" => mode = 4,
        "]" => {
            if word_skiped {
                if mode == 2 {
                    res.push_str(&format!("+{innumerable}"));
                    innumerable += 1;
                    res.push_str(")~@%");
                } else if mode == 3 {
                    res.push_str(&format!("{i}"));
                    i += 1;
                }
                word_skiped = false;
            }
            mode = 0;
        }
        "!" => mode = 1,
        ")" if pervious_mode == 2 || pervious_mode == 3 => mode = pervious_mode,
        "}" if pervious_mode == 2 || pervious_mode == 3 => {
            mode = pervious_mode;
            word_skiped = true;
        }
        "+" => {
            res.push_str("%@~(");
            mode = 2;
        }
        c => match mode {
            0 => {
                res.push_str(c);
                word_skiped = false
            }
            2 | 3 => match c {
                "(" | "{" => {
                    pervious_mode = mode;
                    mode = 1;
                }
                ch => {
                    if ch.eq(&skip_char) || ch.eq("\"") {
                        if word_skiped {
                            if mode == 2 {
                                res.push_str(&format!("+{innumerable}"));
                                innumerable += 1;
                            } else {
                                res.push_str(&format!("{i}"));
                                i += 1;
                            }
                            word_skiped = false;
                        }
                        res.push_str(ch);
                    } else {
                        word_skiped = true;
                    }
                }
            },
            _ => (),
        },
    });
    res.replace("\n", "\\n")
        .replace("\t", "\\t")
        .replace("'", "\'")
}
fn get_rule(format: String, _type: String) -> Rules {
    let lines = parse_segment(vec![], &format);
    lines
        .iter()
        .filter_map(|x| if x.contains(&_type) { None } else { Some(x) })
        .filter_map(|x| match x.chars().nth(0).unwrap() {
            '!' => Some(Rules::parse(x.clone())),
            _ => None,
        })
        .collect::<Vec<Rules>>()
        .pop()
        .unwrap_or(Rules::None)
}
fn gen_html(el: &Vec<Vec<(Variable, Vec<Propertise>)>>) -> String {
    el.iter()
        .map(move |frag| {
            if frag.len() == 1 {
                single_prop_to_html(frag.get(0).unwrap())
            } else {
                format!(
                    "<select class=\"selection\">{}</select>",
                    frag.iter()
                        .map(|(var, props)| variant_to_html(var, props))
                        .collect::<Vec<String>>()
                        .concat()
                )
            }
        })
        .collect::<Vec<String>>()
        .concat()
}
fn single_prop_to_html(el: &(Variable, Vec<Propertise>)) -> String {
    let mut name = String::new();
    match &el.0 {
        Variable::Val(s) => format!(
            "<label class=\"{s}\" {}>{}</label>",
            el.1.iter()
                .map(|prop| match prop {
                    Propertise::Name(s) => {
                        name = s.to_string();
                        "".to_string()
                    }
                    Propertise::IsDefault => "".to_string(),
                })
                .collect::<Vec<String>>()
                .concat(),
            name.is_empty().then_some(s).unwrap_or(&name)
        ),
        Variable::String => format!(
            "<input type=\"text\" {}>",
            el.1.iter()
                .map(|prop| match prop {
                    Propertise::Name(s) => format!("value=\"{s}\" "),
                    Propertise::IsDefault => "".to_string(),
                })
                .collect::<Vec<String>>()
                .concat()
        ),
    }
}
fn variant_to_html(var: &Variable, props: &Vec<Propertise>) -> String {
    let mut name = String::new();
    let prop = props
        .iter()
        .filter_map(|prop| match prop {
            Propertise::Name(s) => {
                name = s.to_string();
                None
            }
            Propertise::IsDefault => Some("selected "),
        })
        .collect::<Vec<&str>>()
        .concat();
    match var {
        Variable::Val(s) => {
            format!(
                "<option value=\"{s}\" {prop}>{}</option>",
                name.is_empty().then_some(s).unwrap_or(&name)
            )
        }
        Variable::String => "".to_string(),
    }
}
