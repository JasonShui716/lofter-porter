use scraper::{Html, Selector};

fn get_total_pages(html: &Html) -> u32 {
    let selector = Selector::parse(".num").unwrap();
    let content = html.select(&selector).next().unwrap().inner_html();
    let page_vec: Vec<&str> = content.as_str().split(" / ").collect();
    page_vec[1].parse().unwrap()
}

fn main() {
    let body = reqwest::get("http://watergun716.lofter.com").unwrap()
        .text().unwrap();
    let document = Html::parse_document(body.as_str());
    let pages = get_total_pages(&document);
    println!("{:?}", pages);
    let client = reqwest::Client::new();
    for page in 0..pages {
        let res = client.get("http://watergun716.lofter.com/")
            .query(&[("page", page)])
            .send().unwrap().text().unwrap();
        let document = Html::parse_document(res.as_str());
        let selector = Selector::parse(".hoverlyr").unwrap();
        for element in document.select(&selector) {
            println!("{:?}", element.value().attr("href"));
        }
    }
}