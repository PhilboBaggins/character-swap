//
// https://en.m.wikipedia.org/wiki/Gothic_alphabet
//

use crate::language_map::LanguageMap;

pub fn add_gothic(mapping: &mut LanguageMap) {
    // 𐌰 not used
    mapping.add('𐌱', "B");
    mapping.add('𐌲', "r");
    mapping.add('𐌳', "Jj");
    mapping.add('𐌴', "E");
    mapping.add('𐌵', "Uu");
    mapping.add('𐌶', "Zz");
    mapping.add('𐌷', "hn");
    // 𐌸 not used
    mapping.add('𐌹', "l");
    mapping.add('𐌺', "R");
    mapping.add('𐌻', "A");
    mapping.add('𐌼', "M");
    mapping.add('𐌽', "N");
    mapping.add('𐌾', "G");
    mapping.add('𐌿', "n");
    mapping.add('𐍀', "Nn");
    mapping.add('𐍁', "Yy");
    mapping.add('𐍂', "R");
    mapping.add('𐍃', "Ss");
    mapping.add('𐍄', "Tt");
    mapping.add('𐍅', "Y");
    mapping.add('𐍆', "F");
    mapping.add('𐍇', "Xx");
    mapping.add('𐍈', "Oo");
    mapping.add('𐍉', "Q");
    mapping.add('𐍊', "T");
}
