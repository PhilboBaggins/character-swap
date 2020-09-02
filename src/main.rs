mod language_map;
use language_map::LanguageMap;

mod languages;
use languages::add_gothic;

fn main() {
    let mut mapping = LanguageMap::new();
    add_gothic(&mut mapping);

    for (k, v) in &mapping.map {
        println!("{}: {:?}", k, v);
    }

    let test_str = "hello world, parts of this string is going to be Replaced";
    println!("{:?}", mapping.replace(test_str));
    println!("{:?}", mapping.replace(test_str));
    println!("{:?}", mapping.replace(test_str));
    println!("{:?}", mapping.replace(test_str));
    println!("{:?}", mapping.replace(test_str));

    println!("{:?}", mapping.replace("nnnnnnnnnnnnnnn"));
    println!("{:?}", mapping.replace("nnnnnnnnnnnnnnn"));
    println!("{:?}", mapping.replace("nnnnnnnnnnnnnnn"));
    println!("{:?}", mapping.replace("nnnnnnnnnnnnnnn"));
    println!("{:?}", mapping.replace("nnnnnnnnnnnnnnn"));
}
