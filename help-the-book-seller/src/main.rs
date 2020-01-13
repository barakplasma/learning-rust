fn main() {
    println!("Hello, world!");
    stock_list(
        vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"],
        vec!["A", "B"],
    );
}

use std::collections::HashMap;

fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.len() <= 0 {
        return String::new();
    }

    let mut stock: HashMap<&str, i32> = HashMap::new();

    for art in list_art {
        let mut art_data = art.split_whitespace();
        let code: &str = art_data.next().unwrap_or_default();
        let category = code.get(0..1).unwrap_or_default();
        let qty: i32 = art_data
            .next()
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        match stock.contains_key(category) {
            true => stock.insert(category, qty + stock[category]),
            false => stock.insert(category, qty),
        };
    }

    for cat in &list_cat {
        if !stock.contains_key(cat) {
            stock.insert(cat, 0);
        }
    }

    let mut list = Vec::new();

    for sc in list_cat.into_iter() {
        list.push(format!("({} : {})", sc, stock[sc]));
    }

    list.join(" - ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests1() {
        let b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");
    }

    #[test]
    fn basic_tests2() {
        let b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");
    }

    #[test]
    fn basic_tests3() {
        let b = vec![
            "ROXANNE 102",
            "RHODODE 123",
            "BKWRKAA 125",
            "BTSQZFG 239",
            "DRTYMKH 060",
        ];
        let c = vec!["B", "R", "D", "X"];
        dotest(b, c, "(B : 364) - (R : 225) - (D : 60) - (X : 0)");
    }

    #[test]
    fn empty_test() {
        let b = vec![];
        let c = vec!["B", "R", "D", "X"];
        dotest(b, c, "");
    }
}
