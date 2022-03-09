use convert_case::{Case, Casing};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

const DMSP: &'static str = include_str!("dmsp.txt");

fn main() {
    // println!("{}", DMSP);

    // for header in get_headers(DMSP) {
    //     println!("{header}");
    // }
    // for subheader in get_subheaders() {
    //     println!("{subheader}");
    // }

    // let re = Regex::new(r"(?m)^-+\r\n(^.+\w)\r\n^-+").unwrap();
    // let mut iter = re.split(DMSP);

    // println!("{:?}\n", iter.next().unwrap());
    // println!("{:?}\n", iter.next().unwrap());

    // let re_header_text = Regex::new(r"^\w.+\w$").unwrap();
    // let re_header_below = Regex::new(r"^-+-$").unwrap();
    // let mut prev_line = "";
    // for line in DMSP.lines() {
    //     if re_header_text.is_match(prev_line) && re_header_below.is_match(line) {
    //         println!("{prev_line}");
    //     }

    //     prev_line = line;
    // }

    // parse tree
    let headers = get_headers(DMSP);
    let bodies = get_bodies(DMSP);

    let mut pages: Vec<Page> = vec![];
    let mut appendices = Page {
        header: "Appendices".to_string(),
        body: "".to_string(),
        children: vec![],
    };
    for i in 0..headers.len() {
        let header = &headers[i];
        if header == "CONTENTS" {
            continue;
        }

        let subheaders = get_subheaders(&bodies[i + 1]);
        let subbodies = get_subbodies(&bodies[i + 1]);

        let has_body = subheaders.len() != subbodies.len();
        let body = if has_body { &subbodies[0] } else { "" }.to_string();

        let mut children: Vec<Page> = vec![];
        for j in 0..subheaders.len() {
            children.push(Page {
                header: subheaders[j].clone(),
                body: subbodies[j + 1].clone(),
                children: vec![],
            });
        }

        if headers[i].starts_with("APPENDIX") {
            appendices.children.push(Page {
                header: header.clone(),
                body,
                children,
            });
        } else {
            pages.push(Page {
                header: header.clone(),
                body,
                children,
            });
        }
    }
    pages.push(appendices);

    // write pages
    for page in &pages {
        page.write_markdown(Path::new("test"));
    }

    // write summary
    for page in &pages {
        // TODO: generate summary
    }
}

#[derive(Debug)]
struct Page {
    header: String,
    body: String,
    children: Vec<Page>,
}

impl Page {
    fn write_markdown(&self, dir: &Path) {
        let header = parse_header(&self.header);

        let dir = if self.children.len() > 0 {
            dir.join(header.to_case(Case::Kebab))
        } else {
            dir.to_path_buf()
        };
        create_dir_all(dir.clone()).unwrap();

        let path = if self.children.len() > 0 {
            dir.join("README.md")
        } else {
            dir.join(format!("{}.md", header.to_case(Case::Kebab)))
        };
        let mut file = File::create(path).unwrap();
        writeln!(&mut file, "# {}\n", header).unwrap();
        writeln!(&mut file, "{}", self.body.trim()).unwrap();

        for child in &self.children {
            child.write_markdown(&dir);
        }
    }
}

fn get_headers(text: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^-+\r\n(^.+\w)\r\n^-+").unwrap();
    }
    RE.captures_iter(text)
        .map(|cap| cap[1].to_string())
        .collect()
}

fn get_bodies(text: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)^-+\r\n(^.+\w)\r\n^-+").unwrap();
    }
    RE.split(text).map(|body| body.to_string()).collect()
}

fn get_subheaders(text: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)(^\[.+\w)\r\n^=+").unwrap();
    }
    RE.captures_iter(text)
        .map(|cap| cap[1].to_string())
        .collect()
}

fn get_subbodies(text: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?m)(^\[.+\w)\r\n^=+").unwrap();
    }
    RE.split(text).map(|body| body.to_string()).collect()
}

fn parse_header(text: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[.+\]: (.+)").unwrap();
    }
    RE.captures(text)
        .map(|cap| cap.get(1).unwrap().as_str().to_string())
        .unwrap_or(text.to_case(Case::Title))
}
