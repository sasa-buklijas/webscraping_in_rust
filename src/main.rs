use reqwest::Url;
use scraper::{Html, Selector};

// TODO, maybe add price as separate structure
/*struct Price {
    old: f32,
    discounted: f32,
}*/

#[derive(Debug)]
struct Item<'a> {   // <'a> is lifetime
    category: &'a str,   
    name: &'a str,
    url: &'a str,
    discount: u8,
    //discount: &'a str,
    price_old: f32,
    price_discounted: f32,
    //price_old: &'a str,
    //price_discounted: &'a str,
    available: bool,
}

fn main() {
    let url = "https://www.tvornicazdravehrane.com/proizvodi/popusti-i-akcije/?discount__gt=0&sort=new";
    //let url = "https://www.tvornicazdravehrane.com/proizvodi/popusti-i-akcije/?discount__gt=0&sort=new&to_page=7";
    // https://www.tvornicazdravehrane.com/proizvodi/popusti-i-akcije/?discount__gt=0&sort=new&to_page=6
        // da li mogu samo povecati broj stranice i ako ne postoji i da ce mi on vratiti sve elemente
            // ne radi tako, ako povecam broj vise nego sto ima, onda vrati samo 1 stranicu 
                // moglo bi se pogadati ++ -- koji je zadni broj, ali to je kompliciranii algoritam, moza poslje implementirati
    let response = reqwest::blocking::get(url).expect(&format!("Could not load url {}", url));
    let body = response.text().unwrap();
    //print!("{}",body); panic!();

    let document = Html::parse_document(&body);
    let item_selector = Selector::parse("#items_catalog > div.cp.cp-grid").unwrap();
    // must be #items_catalog > .....
    let item_category_selector = Selector::parse("div.cp-cnt a").unwrap();
    let item_name_selector = Selector::parse("div div.cp-title a").unwrap();
    let item_discount_selector = Selector::parse("div div div.cp-badges span").unwrap();
    // price only in Euro, HRK calculated in JS
    let item_old_price_selector = Selector::parse("div.cp-old-price > span").unwrap();
    let item_discount_price_selector = Selector::parse("div.cp-discount-price").unwrap();
    let item_unavailable_selector = Selector::parse("span.cp-unavailable-label").unwrap();
    //let item_unavailable_selector = Selector::parse("div > span.cp-unavailable-label").unwrap();
    //let item_unavailable_selector = Selector::parse("div.cp-addtocart > span").unwrap();
    //#items_catalog > div:nth-child(1) > div.cp-col.cp-col2 > div.cp-footer > div.cp-addtocart.cp-addtocart-single > span
    

    for element in document.select(&item_selector) {
        let item_category_element = element.select(&item_category_selector).next().expect("Could not select item category.");
        let item_category = item_category_element.text().collect::<String>();
        let item_category = item_category.as_str();

        let item_name_element = element.select(&item_name_selector).next().expect("Could not select item name.");
        let item_name = item_name_element.text().collect::<String>();
        let item_name = item_name.as_str();
        let item_url = item_name_element.value().attr("href").expect("Could not find item href attribute.");
    

        let item_discount_element =  element.select(&item_discount_selector).next().expect("Could not select item discount.");
        let mut item_discount = item_discount_element.text().collect::<String>();
        item_discount.pop();            // remove last char
        item_discount.remove(0);    // remove first char    // https://stackoverflow.com/a/70598494/2006674
        let item_discount = item_discount.parse::<u8>().unwrap();
        //println!("      {:?}", item_discount); panic!();

        let item_old_price_element =  element.select(&item_old_price_selector).next().expect("Could not select item old price.");
        let mut item_old_price = item_old_price_element.text().collect::<String>();
        item_old_price.pop(); 
        item_old_price.pop(); // to remove space 
        //let item_old_price = item_old_price.trim();
        item_old_price = item_old_price.replace(",", ".");
        let item_old_price = item_old_price.parse::<f32>().unwrap();
        //println!("      {:?}", item_old_price); panic!();

        let item_discount_price_element =  element.select(&item_discount_price_selector).next().expect("Could not select item discount price.");
        let item_discount_price = item_discount_price_element.text().collect::<String>();
        //println!("      {:?}", item_discount_price);    // "\n\t\t\t\t\t\t\t1,99 €\t\t\t\t\t\t\t\t\t\t\t\t\t"
        let item_discount_price = item_discount_price.trim();
        let item_discount_price = &item_discount_price[0..item_discount_price.len() - 4]; // because it take 2x per char // https://stackoverflow.com/a/65976485/2006674
        //println!("      {:?}", item_discount_price); panic!();
        //item_discount_price.pop(); 
        //item_discount_price.pop(); // to remove space
        let item_discount_price = item_discount_price.replace(",", ".");
        let item_discount_price = item_discount_price.parse::<f32>().unwrap();

        let item_unavailable_element = element.select(&item_unavailable_selector).next();        
        // let item_available = if item_unavailable_element == None {true} else {false};
        // like this syntaks
        let item_available = match item_unavailable_element {
            Some(_) => false,
            None => true,
        };
        /* 
        let mut item_available = true;
        if let Some(_) = item_unavailable_element {
            item_available = false;
        }*/

        //println!("{:?}",item_name);
        //println!("      {:?}", item_category);
        //println!("      {:?}", item_url);
        //println!("      {:?}", item_discount);
        //println!("      {:?}", item_old_price);
        //println!("      {:?}", item_discount_price);
        //println!("      {:?}", item_available);
        //break;

        let item = Item {
            category: item_category,
            name: item_name,
            url: item_url,
            discount: item_discount,
            price_old: item_old_price,
            price_discounted: item_discount_price,
            available: item_available,
        };

        print!("{:#?}", item);

    }
}