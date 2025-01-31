use crate::{BLUE_COMPRA_SELECTOR, BLUE_LABEL, BLUE_VENTA_SELECTOR};
use scraper::{Html, Selector};

pub async fn get_html(url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?.text().await?;
    Ok(response)
}

pub async fn parse_document(url: &str) -> Result<Html, reqwest::Error> {
    let html = get_html(url).await?;
    let document = Html::parse_document(&html);
    Ok(document)
}

pub fn get_values(document: &Html) -> Result<(), Box<dyn std::error::Error>> {
    let options = vec![(BLUE_LABEL, BLUE_COMPRA_SELECTOR, BLUE_VENTA_SELECTOR)];

    for (label, compra_selector, venta_selector) in &options {
        let compra_sel = Selector::parse(compra_selector).unwrap();
        let venta_sel = Selector::parse(venta_selector).unwrap();

        let compra = document
            .select(&compra_sel)
            .next()
            .map(|n| n.text().collect::<String>())
            .unwrap_or("N/A".to_string());
        let venta = document
            .select(&venta_sel)
            .next()
            .map(|n| n.text().collect::<String>())
            .unwrap_or("N/A".to_string());

        println!("{}: Compra: {}, Venta: {}", label, compra, venta);
    }

    Ok(())
}
