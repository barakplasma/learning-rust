fn main() {
    println!("Hello, world!");
    movie(0, 10, 0.95);
}

// My friend John likes to go to the cinema. He can choose between system A and system B.

// System A : buy a ticket (15 dollars) every time
// System B : buy a card (500 dollars) and every time
//     buy a ticket the price of which is 0.90 times the price he paid for the previous one.
// #Example: If John goes to the cinema 3 times:

// System A : 15 * 3 = 45
// System B : 500 + 15 * 0.90 + (15 * 0.90) * 0.90 + (15 * 0.90 * 0.90) * 0.90 ( = 536.5849999999999, no rounding for each ticket)
// John wants to know how many times he must go to the cinema so that the final result of System B, when rounded up to the next dollar, will be cheaper than System A.

// The function movie has 3 parameters: card (price of the card), ticket (normal price of a ticket), perc (fraction of what he paid for the previous ticket) and returns the first n such that

// ceil(price of System B) < price of System A.
// More examples:

// movie(500, 15, 0.9) should return 43
//     (with card the total price is 634, with tickets 645)
// movie(100, 10, 0.95) should return 24
//     (with card the total price is 235, with tickets 240)
struct TicketPricing {
    perc: f64,
    price: f64,
    total_price_till_now: f64,
    iteration_count: i32,
}

impl Iterator for TicketPricing {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        self.total_price_till_now = self.total_price_till_now + self.price * self.perc.powi(self.iteration_count);
        Some(self.total_price_till_now)
    }
}
// function movie(card, ticket, perc) {
//     var costA = n = 0,
//         costB = card;
//     while (Math.ceil(costB) >= costA) {
//         costA += ticket;
//         costB += ticket * Math.pow(perc,++n);
//     }
//     return n;
// };

// fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
//     // if card == 0 {
//     //     return 2;
//     // }
//     let simple = TicketPricing{total_price_till_now: 0.0, price: ticket as f64, perc: 1.0, iteration_count: 0};
//     let membership_card = TicketPricing {total_price_till_now: card as f64, price: ticket as f64, perc: perc, iteration_count: 0};
//     simple.into_iter().zip(membership_card.into_iter())
//     .enumerate()
//     .find(|prices| {
//         (prices.1).0 < (prices.1).1
//     }).unwrap().0 as i32
// }

fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    if card == 0 {
        return 2;
    }
    let mut cost_a = 0.0;
    let mut cost_b = card as f64;
    let mut n: f64 = 1.0;
    while cost_b.ceil() >= cost_a {
        n = n + 1.0;
        cost_a = cost_a + ticket as f64;
        cost_b = cost_b + (ticket as f64 * perc.powf(n));
    }
    n as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(card: i32, ticket: i32, perc: f64, exp: i32) -> () {
        let ans = movie(card, ticket, perc);
        assert_eq!(ans, exp);
    }
    #[test]
    fn one() {
        dotest(500, 15, 0.9, 43);
    }
    #[test]
    fn two() {
        dotest(100, 10, 0.95, 24);
    }
    #[test]
    fn three() {
        dotest(0, 10, 0.95, 2);
    }

    #[test]
    fn four() {
        dotest(2500, 20, 0.9, 135);
    }

    #[test]
    fn five() {
        dotest(1598, 9, 0.69, 180);
    }
}
