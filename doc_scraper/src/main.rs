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

    let re_operator = Regex::new(r"(?:[\-\+\*\/]| \.\. )").unwrap();
    let re_brackets = Regex::new(r"[\[\]]").unwrap();
    let re_function = Regex::new(r"^(?:[\w][\w\d]*\.)*(?:[\w][\w\d]*)(?:[:.][\w][\w\d]*)").unwrap();
    // let re_class = Regex::new(r"^((?:[\w][\w\d]*\.)*(?:[\w][\w\d]*))(?:[:][\w][\w\d]*)").unwrap();

    let mut _poop = 0;
    let mut _last_class: String = "".to_string();
    for element in document.select(&outer) {
        for d2 in element.select(&selector1).chain(element.select(&selector2)) {
        // for d2 in element.select(&selector2) {
            _poop = _poop + 1;

            let anchor = d2.value().attr("id").unwrap_or("");

            let mut title = d2.select(&sel_title).next().unwrap().text().collect::<String>();
            let mut text: Vec<String> = Vec::new();
            for c in d2.select(&sel_content) {
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
                        print!("WARN: Extra HTML tag in admonition: {}", an.to_string());
                    }
                    text.push(an.to_string());
                }
            }
            // This gets rid of the brackets (optional functional parameters) in the title
            title = re_brackets.replace_all(&title, "").to_string();

            if re_function.is_match(&title) {
                if title.contains("-") {
                    // println!("WARN: Function with dash in title: {}. Fixed as {}", title, title.replace("-", "_"));
                    title = title.replace("-", "_");
                }
                if title.contains(":") {
                    _last_class = title.split(":").next().unwrap_or("").to_string();
                }
            }
            // TODO: This does not show multiple overloaded functions with different type params
            if re_operator.is_match(&title) {
                text.push("# {title}".to_string());
                if title.starts_with("-") {
                    title = format!("{_last_class}:__unm()").to_string();
                } else if title.contains("-") {
                    title = format!("{_last_class}:__sub(other)").to_string();
                } else if title.contains("*") {
                    title = format!("{_last_class}:__mul(other)").to_string();
                } else if title.contains("/") {
                    title = format!("{_last_class}:__div(other)").to_string();
                } else if title.contains("+") {
                    title = format!("{_last_class}:__add(other)").to_string();
                } else if title.contains("..") {
                    title = format!("{_last_class}:__concat(other)").to_string();
                } else {
                    panic!("Unknown operator: {}", title);
                }
            }
            if anchor == "" {
                // println!("WARN: No id for: {}", title);
            }
            // println!("-- {url}", url="https://sdk.play.date/2.0.1/Inside%20Playdate.html#".to_string() + &anchor);
            println!("function {title} end")
            // println!("  {}\n", text.join("\n  "));

            // if _poop > 3 { panic!(); }

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
