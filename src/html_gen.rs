use crate::parse_format::*;
use std::{collections::HashMap, fs};

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
    base.replace("<!--innumerable-->", &innumerable.concat())
        .replace("<!--numerable-->", &numarable.concat())
        .replace(
            "<!--code-->",
            &script.replace("<!--format-->", &format_to_script(format.clone())),
        )
}
fn format_to_script(format: String) -> String {
    let mut res = String::new();
    let mut last = 0;
    let mut i = 0;
    for c in format.chars() {
        if c == ']' {
            last = 0;
            continue;
        } else if c == '!' {
            last = 1
        } else if c == '[' {
            last = 2;
            format!("{i}").chars().for_each(|s| res.push(s));
            i += 1
        } else if last != 2 {
            if last == 1 {
                res.push('!');
                last = 0;
            }
            res.push(c)
        }
    }
    res.replace("\n", "\\n").replace("\t", "\\t").replace(
        "<!--space_char-->",
        &match get_rule(format, "space_char".to_string()) {
            Rules::SetSymbol(c) => c.to_string(),
            Rules::None => " ".to_string(),
        }
        .replace("\n", "\\n")
        .replace("\t", "\\t"),
    )
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
