mod language_map;
use language_map::LanguageMap;

mod languages;
use languages::add_gothic;

fn main() {
    let mut mapping = LanguageMap::new();
    add_gothic(&mut mapping);

    let test_str = "hello world, parts of this string is going to be Replaced";
    let replaced = mapping.replace(test_str);
    println!("{}", replaced);

    //for (k, v) in mapping.map {
    //    println!("{}: {:?}", k, v);
    //}
}
