use webscraping_in_rust::TzhParser;

fn main() {
    let p = TzhParser{};

    //TzhParser::save(505);
    //println!("{}", TzhParser::load_or(999));

    let items = p.get_items();
    //println!("{:#?}", items);
    println!("Items on sale: {:#?}", items.len());


}
