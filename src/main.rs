use constant::USERNAME;
use scraper::{Html, Selector};
use hyper::{Uri, Client};
use hyper_tls::HttpsConnector;
mod constant;

fn get_total_pages(username: &str) -> Result<u32, Box<dyn std::error::Error + Send + Sync>> {
    let body = reqwest::get(format!("http://{}.lofter.com/", username).as_str())?.text()?;
    let document = Html::parse_document(body.as_str());
    let selector = Selector::parse(".num").unwrap();
    let content = document.select(&selector).next().unwrap().inner_html();
    let page_vec: Vec<&str> = content.as_str().split(" / ").collect();
    Ok(page_vec[1].parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pages = get_total_pages(USERNAME)?;
    let mut article_vec = Vec::new();
    let mut request_fut_vec = Vec::new();
    let client = reqwest::Client::new();
    for page in 0..pages {
        let res = client
            .get(format!("http://{}.lofter.com/", USERNAME).as_str())
            .query(&[("page", page)])
            .send()?
            .text()?;
        let document = Html::parse_document(res.as_str());
        let selector = Selector::parse(".hoverlyr").unwrap();
        for element in document.select(&selector) {
            article_vec.push(element.value().attr("href").unwrap().to_owned().parse::<Uri>().unwrap());
        }
    }
    println!("{:?}", article_vec);
    for article in article_vec {
        let fut = async {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
            let resp = Client::new().get(article).await;
            let body = hyper::body::to_bytes(resp.unwrap().into_body()).await;
            String::from_utf8(body.unwrap().to_vec()).unwrap()
        };
        request_fut_vec.push(fut);
    }

    for fut in request_fut_vec {
        let bytes = fut.await;
    }

    Ok(())
}
