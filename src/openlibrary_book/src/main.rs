pub mod book;


fn main() {
    println!("========> run start <===========");

    book::init_book("斗破苍穹".to_string(), "天蚕土豆".to_string(), "玄幻小说".to_string());

    println!( "书名: {}",book::get_book_name());
    println!( "作者: {}",book::get_author_name());
    println!( "简介: {}",book::get_book_desc());

    book::add_section("第一章 莫欺少年穷".to_string(), "未完待续...".to_string());
    println!("{:?}", book::get_section(0));

    println!("========> run done. <===========");
}
