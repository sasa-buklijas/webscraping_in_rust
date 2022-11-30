use reqwest;
use scraper::{Html, Selector};
use std::process;
use std::thread;
use std::time::Duration;

// TODO, maybe add price as separate structure
/*struct Price {
    old: f32,
    discounted: f32,
}*/

#[derive(Debug)]
struct Item {   
    category: String,
    name: String,
    url: String,
    discount: u8,
    price_old: f32,
    price_discounted: f32,
    available: bool,
}

struct TzhParser {

}

impl TzhParser {
    const URL: &'static str = "https://www.tvornicazdravehrane.com/proizvodi/popusti-i-akcije/?discount__gt=0&sort=new";

    fn number_of_items_on_page(document: &Html) -> usize {

        let item_selector = Selector::parse("#items_catalog > div.cp.cp-grid").unwrap();
        document.select(&item_selector).count()
    }

    fn get_document_with_all_items(&self)-> Html {
        // idea was to use CSS selector for load more button, but it is allways display: none;
            // only when you scrole it is changed("", then display: inline-flex), this aproach is not possible
        // let item_selector = Selector::parse("a[style=\"display: none;\"]").unwrap(); 
        // I will use other aproach, counting number of items on page "number_of_items_on_page"
        // there are few strategies for it
            // option_1: is to start from 1, and increment by one until you found it
        
        let mut page_number:u8 = 6;
        println!("0:::page_number={page_number}");
        let mut found = false;
        let mut last_page_number = page_number + 5; 
        loop {
            let url:&str = &format!("{}&to_page={}", TzhParser::URL, page_number);
            let response = reqwest::blocking::get(url).expect(&format!("Could not load url {}", url));
            thread::sleep(Duration::from_millis(2000));   // without sleep, website usually(90%) block after 2 attempts, // it can happend eve on first attemp
            let body = response.text().unwrap(); //println!("{}", body);
            let document = Html::parse_document(&body);

            let number_of_items = TzhParser::number_of_items_on_page(&document);
            // default per page is 16 on 29/11/2022
            let max_number_of_items_for_page_number: usize = usize::from(page_number) * 16;
            let min_number_of_items_for_page_number = max_number_of_items_for_page_number - 16;

            if number_of_items == max_number_of_items_for_page_number {
                if last_page_number == page_number + 1 {
                    println!("1:::special_case.");
                    found = true;
                }
                else {
                    println!("1:::page {page_number} there is more than {number_of_items} items on sale.");
                    last_page_number = page_number;
                    page_number = page_number + 1;
                }
            }
            // between check
            else if number_of_items > min_number_of_items_for_page_number && number_of_items < max_number_of_items_for_page_number {
                println!("2:::page is {} with {} items", page_number, number_of_items);
                found = true;
            }
            else if number_of_items == 16 {
                if page_number == 1 {
                    last_page_number = page_number;
                    page_number = page_number + 1;
                    println!("3:::page_number +1 to page_number={page_number}");
                }
                else {
                    last_page_number = page_number;
                    page_number = page_number - 1;
                    println!("3:::page_number -1 to page_number={page_number}");
                }

            }
            else {
                eprintln!("ERROR number_of_items={number_of_items}, this is UNEXPECTED, CHECK !!!");
                process::exit(1);

            }

            if found {
                return document
            }
        }

    }

    fn get_items(&self) -> Vec<Item> {
        let document_with_all_items = self.get_document_with_all_items();

        // must be #items_catalog > .....
        let item_selector = Selector::parse("#items_catalog > div.cp.cp-grid").unwrap();
        let item_category_selector = Selector::parse("div.cp-cnt a").unwrap();
        let item_name_selector = Selector::parse("div div.cp-title a").unwrap();
        let item_discount_selector = Selector::parse("div div div.cp-badges span").unwrap();
        // price only in Euro, HRK calculated in JS
        let item_old_price_selector = Selector::parse("div.cp-old-price > span").unwrap();
        let item_discount_price_selector = Selector::parse("div.cp-discount-price").unwrap();
        let item_unavailable_selector = Selector::parse("span.cp-unavailable-label").unwrap();

        let mut items = Vec::new();
        for element in document_with_all_items.select(&item_selector) {
            let item_category_element = element.select(&item_category_selector).next().expect("Could not select item category.");
            let item_category = item_category_element.text().collect::<String>();
    
            let item_name_element = element.select(&item_name_selector).next().expect("Could not select item name.");
            let item_name = item_name_element.text().collect::<String>();
            let item_url = item_name_element.value().attr("href").expect("Could not find item href attribute.");
            let item_url = item_url.to_string();
        
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
            item_old_price = item_old_price.replace(",", ".");
            let item_old_price = item_old_price.parse::<f32>().unwrap();
            //println!("      {:?}", item_old_price); panic!();
    
            let item_discount_price_element =  element.select(&item_discount_price_selector).next().expect("Could not select item discount price.");
            let item_discount_price = item_discount_price_element.text().collect::<String>();
            //println!("      {:?}", item_discount_price);    // "\n\t\t\t\t\t\t\t1,99 €\t\t\t\t\t\t\t\t\t\t\t\t\t"
            let item_discount_price = item_discount_price.trim();
            let item_discount_price = &item_discount_price[0..item_discount_price.len() - 4]; // because it take 2x per char // https://stackoverflow.com/a/65976485/2006674
            //println!("      {:?}", item_discount_price); panic!();
            let item_discount_price = item_discount_price.replace(",", ".");
            let item_discount_price = item_discount_price.parse::<f32>().unwrap();
    
            let item_unavailable_element = element.select(&item_unavailable_selector).next();
            let item_available = match item_unavailable_element {
                Some(_) => false,
                None => true,
            };
    
            let item = Item {
                category: item_category,
                name: item_name,
                url: item_url,
                discount: item_discount,
                price_old: item_old_price,
                price_discounted: item_discount_price,
                available: item_available,
            };
            items.push(item);
            //print!("{:#?}", item);
        }

        let items = items;
        items
    }
}


fn main() {
    let p = TzhParser{};

    let items = p.get_items();
    //println!("{:#?}", items);
    //println!("Items on sale: {:#?}", items.len());


}
