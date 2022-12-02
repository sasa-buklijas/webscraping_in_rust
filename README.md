

# Known Issues
- retry for reqwest::blocking::get
    -  one idea https://stonecharioteer.com/posts/2022/rust-reqwest-retry.html
- add persistance to page_number
    - simple file is OK, as first idea https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html#read-lines-of-strings-from-a-file
- ~~remove main get_document_with_all_items algorithm to separate function, then I can do unit test for it~~

# Future Work
- persist items to Sqlite
    - maybe https://docs.rs/rusqlite/latest/rusqlite/ or https://docs.rs/diesel/latest/diesel/
        - when SQlite is done page_number can be added to it, so that I do not have one DB file and one file just for page_number
- show diff on items on CLI
    - send diff to email
