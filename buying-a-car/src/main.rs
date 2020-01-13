fn main() {
    println!("Hello, world!");
}
fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut current_balance: f64 = 0.0;
    let mut months: i32 = 0;
    let mut current_depreciation_rate: f64 = perc;
    let mut current_value_old: f64 = old as f64;
    let mut current_value_new: f64 = new as f64;

    fn depreciate_asset(value: f64, depreciation_rate: f64) -> f64 {
        let to_percent: f64 = 100.0;
        (value - (value * (depreciation_rate/to_percent)))
    }

    fn difference(new: f64, old: f64, bal: f64) -> f64 {
        let assets = old + bal;
        let liabilities = new;
        let diff = assets - liabilities;
        diff
    }

    while difference(current_value_new, current_value_old, current_balance) < 0.0 {
        current_balance += saving as f64;
        current_value_new = depreciate_asset(current_value_new.into(), current_depreciation_rate);
        current_value_old = depreciate_asset(current_value_old.into(), current_depreciation_rate);
        if months % 2 == 0 {
            current_depreciation_rate += 0.5;
        }
        months +=1;
    }
    return (months, difference(current_value_new, current_value_old, current_balance).round() as i32)
}

fn testing(old: i32, new: i32, saving: i32, perc: f64, exp: (i32, i32)) -> () {
    assert_eq!(nb_months(old, new, saving, perc), exp)
}

#[test]
fn basics_nb_months() {
    testing(2000, 8000, 1000, 1.5, (6, 766));
    testing(12000, 8000, 1000, 1.5 , (0, 4000));
    testing(8000, 12000, 500, 1.0, (8, 597));
    testing(18000, 32000, 1500, 1.25, (8, 332));
    testing(7500, 32000, 300, 1.55, (25, 122));
}