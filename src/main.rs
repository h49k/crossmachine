use scraper::{Html, Selector};
// use std::collections::HashMap;
// use cdb;
use std::fs;
use std::io::{BufWriter, Write};
const KINTOKU_TXT: &str = "金特.txt";

// urls = ("https://pawasoccer.gamewith.jp/article/show/37116", "https://pawasoccer.gamewith.jp/article/show/36772")
// a_lists.extend(soup.select("table.sorttable > tr > td:first-child > a"))

// CREATE DATABASE pawasaka TEMPLATE = template0 ENCODING = 'UTF8' LC_COLLATE = 'C' LC_CTYPE = 'C';
// CREATE TABLE url_title ( url varchar(50), titile varchar(128) not null, CONSTRAINT PRIMARY KEY (url));
// #![windows_subsystem="windows"]
struct Kintoku {
    name: String,
    exclusive: String,
    url: String,
    cf: String,
    st: String,
    wg: String,
    omf: String,
    cmf: String,
    smf: String,
    dmf: String,
    cb: String,
    sb: String,
    gk: String,
}

impl Kintoku {
    fn init(url: &str) -> Kintoku {
        Kintoku {
            name: String::from(""),
            exclusive: String::from(""),
            url: String::from(url),
            cf: String::from(""),
            st: String::from(""),
            wg: String::from(""),
            omf: String::from(""),
            cmf: String::from(""),
            smf: String::from(""),
            dmf: String::from(""),
            cb: String::from(""),
            sb: String::from(""),
            gk: String::from(""),
        }
    }

    fn tsv(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.name,
            self.url,
            self.exclusive,
            self.cf,
            self.st,
            self.wg,
            self.omf,
            self.cmf,
            self.smf,
            self.dmf,
            self.cb,
            self.sb,
            self.gk,
        )
    }
}

// const HTML: &str = r#"<!DOCTYPE html>
// <html>
//     <body>
//         Hello world from Rust!
//     </body>
// </html>"#;

fn main() {

    println!("■■金特ツール■■");

    let mut kintoku_all: Vec<Kintoku> = Vec::new();
    let url = "https://pawasoccer.gamewith.jp/article/show/45385";

    let mut html_text = String::new();

    let _result = get_page(url, &mut html_text).unwrap();

    // println!("scrapを呼びます。");
    scrap(&html_text as &str, &mut kintoku_all);

    file_save(&kintoku_all);
}

fn file_save(kintoku_all: &Vec<Kintoku>) {

    println!("{}を保存します。", KINTOKU_TXT);

    let mut f = BufWriter::new(fs::File::create(KINTOKU_TXT).unwrap());
    writeln!(f, "name\turl\tEXC\tCF\tST\tWG\tOMF\tCMF\tSMF\tDMF\tCB\tSB\tGK").unwrap();

    for v in kintoku_all {
        writeln!(f, "{}", v.tsv()).unwrap();
    }

    f.flush().unwrap();
}

#[tokio::main]
async fn get_page(url: &str, text: &mut String) -> Result<(), reqwest::Error> {
    // println!("get_page!");
    let res_body = reqwest::get(url).await?.text().await?;
    text.push_str(&res_body);

    Ok(())
}

fn scrap(html: &str, kintoku_all: &mut Vec<Kintoku>) {
    let fragment = Html::parse_document(html);

    let selector_anc = Selector::parse("table.sorttable a").unwrap();
    let selector_h1 = Selector::parse("h1 span._main").unwrap();

    if let Some(h1) = fragment.select(&selector_h1).next() {
        println!("「{}」から一覧をもってきます。", h1.inner_html());
    }
    for node in fragment.select(&selector_anc) {
        let url = node.value().attr("href").unwrap_or("");
        let nam = node.inner_html();

        if url != "----https://pawasoccer.gamewith.jp/article/show/96954" {
            let mut u = scrap_sub(url);
            u.name = nam;
            kintoku_all.push(u);
        }
    }
}

fn scrap_sub(url: &str) -> Kintoku {
    // println!("scrap_subを実行します");
    let mut html_text = String::new();

    let _result = get_page(url, &mut html_text).unwrap();
    let fragment = Html::parse_document(&html_text as &str);

    // h2 id=pwsk_anchor01の直後にテーブルがあって、その最後が同時取得不可
    let selector_h1 = Selector::parse("h1 span._main").unwrap();
    let selector_xcv = Selector::parse("#pwsk_anchor01 + table tr:last-child a").unwrap();
    let selector_cf = Selector::parse("#article-body > h3 + table tr:nth-child(2) td").unwrap();
    let selector_st = Selector::parse("#article-body > h3 + table tr:nth-child(3) td").unwrap();
    let selector_wg = Selector::parse("#article-body > h3 + table tr:nth-child(4) td").unwrap();
    let selector_omf = Selector::parse("#article-body > h3 + table tr:nth-child(5) td").unwrap();
    let selector_cmf = Selector::parse("#article-body > h3 + table tr:nth-child(6) td").unwrap();
    let selector_smf = Selector::parse("#article-body > h3 + table tr:nth-child(7) td").unwrap();
    let selector_dmf = Selector::parse("#article-body > h3 + table tr:nth-child(8) td").unwrap();
    let selector_cb = Selector::parse("#article-body > h3 + table tr:nth-child(9) td").unwrap();
    let selector_sb = Selector::parse("#article-body > h3 + table tr:nth-child(10) td").unwrap();
    let selector_gk = Selector::parse("#article-body > h3 + table tr:nth-child(11) td").unwrap();

    if let Some(h1) = fragment.select(&selector_h1).next() {
        println!("Get Information from: {}", h1.inner_html());
    }

    let mut k = Kintoku::init(url);

    k.cf = if let Some(pos) = fragment.select(&selector_cf).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.st = if let Some(pos) = fragment.select(&selector_st).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.wg = if let Some(pos) = fragment.select(&selector_wg).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.omf = if let Some(pos) = fragment.select(&selector_omf).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.cmf = if let Some(pos) = fragment.select(&selector_cmf).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.smf = if let Some(pos) = fragment.select(&selector_smf).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.dmf = if let Some(pos) = fragment.select(&selector_dmf).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.cb = if let Some(pos) = fragment.select(&selector_cb).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.sb = if let Some(pos) = fragment.select(&selector_sb).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.gk = if let Some(pos) = fragment.select(&selector_gk).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k.exclusive = if let Some(pos) = fragment.select(&selector_xcv).next() {
        String::from(pos.inner_html())
    } else {
        String::from("")
    };

    k
}
