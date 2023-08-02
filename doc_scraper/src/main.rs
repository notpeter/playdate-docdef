use std::{path::Path, fs::File, io::Read};
use regex::Regex;

fn main() {
    // let response = reqwest::blocking::get(
    //     "https://sdk.play.date/2.0.1/Inside%20Playdate.html",
    // )
    // .unwrap()
    // .text()
    // .unwrap();

    // open the file
    let path = Path::new("inside.html");
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let mut response = String::new();
    file.read_to_string(&mut response).unwrap();


    let document = scraper::Html::parse_document(&response);
    let outer = scraper::Selector::parse("div.sect1>div.sectionbody>div.sect2").unwrap();

    // let selector1 = scraper::Selector::parse("div.sect1>div.sectionbody>div.sect2>div.sect3>div.item").unwrap();
    let selector1 = scraper::Selector::parse("div.sect3>div.item").unwrap();
    let selector2 = scraper::Selector::parse("div.item").unwrap();

    let sel_title = scraper::Selector::parse("div.title").unwrap();
    let sel_content = scraper::Selector::parse("div.content").unwrap();
    let sel_paragraph = scraper::Selector::parse("div.paragraph").unwrap();
    let sel_p = scraper::Selector::parse("p").unwrap();

    let sel_admonition = scraper::Selector::parse("div.admonitionblock>table>tbody>tr>td.content").unwrap();
    let re_code = Regex::new(r"</?code>").unwrap();
    let re_em = Regex::new(r"</?em>").unwrap();
    let re_a = Regex::new(r"</?a[^>]*>").unwrap();
    let re_strong = Regex::new(r"</?strong>").unwrap();
    let html_tag = Regex::new(r"<[^>]*>").unwrap();

    let mut _poop = 0;
    for element in document.select(&outer) {
        // for d2 in element.select(&selector1).chain(element.select(&selector2)) {
        for d2 in element.select(&selector2) {
            _poop = _poop + 1;
            let anchor = d2.value().attr("id").unwrap_or("");
            let title = d2.select(&sel_title).next().unwrap().text().collect::<String>();
            for c in d2.select(&sel_content) {
                let mut text: Vec<String> = Vec::new();
                for div_p in c.select(&sel_paragraph) {
                    for p in div_p.select(&sel_p) {
                        text.push(p.text().collect::<String>());
                    }
                }
                for td in c.select(&sel_admonition) {
                    let adm = td.inner_html();
                    let a1 = re_code.replace_all(&adm, "`");
                    let a2 = re_em.replace_all(&a1, "*");
                    let a3 = re_a.replace_all(&a2, "");
                    let a4 = re_strong.replace_all(&a3, "**");
                    let an = a4.trim().to_string();
                    if html_tag.is_match(&an) {
                        print!("WARN: HTML tag in admonition: {}", an.to_string());
                    }
                    text.push(an.to_string());
                }

                println!("{} {}", title, "");
                println!("  {}", text.join("\n  "));
                println!()
            }
            // if anchor == "" {
            //     println!("WARN: No id for: {}", title);
            // }

            // if _poop > 3 {
            //     panic!();
            // }

            // let title = d2.select(&sel_title).next().unwrap().value();
            // let content = d2.select(&sel_content).next().unwrap().text().collect::<String>();
            // let name = "";
            // println!("{} {} {}", anchor, title, content);
        }
        // println!("{}, {}", element.value().name(), element.value().attr("id").unwrap());
        // let sub = scraper::Selector::parse("div.title").unwrap();
            // println!("{}", element.select("div.title").collect::<String>());
        //     print!("{} ", divs.text().collect::<String>());
    }
    // titles
    //     .zip(1..101)
    //     .for_each(|(item, number)| println!("{}. {}", number, item));
}
