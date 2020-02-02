fn main() {
    accumulated_altitude(
        vec![Lift {
            start: 0,
            end: 1000,
            id: 1,
        }],
        vec![],
    );
}

impl Arbitrary for Lift {
    fn arbitrary<G: Gen>(g: &mut G) -> Lift {
        Lift {
            start: i32::arbitrary(g),
            end: i32::arbitrary(g),
            id: u32::arbitrary(g)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Lift {
    start: i32,
    end: i32,
    id: u32,
}

fn accumulated_altitude(lifts: Vec<Lift>, rides: Vec<u32>) -> i32 {
    rides.iter().fold(0, |acc, cur| {
        let lift = lifts.iter().find(|&l| l.id == *cur);
        if let Some(l) = lift {
            return acc + l.end - l.start;
        }
        return acc;
    })
}

fn hidden14141413731_accumulated_altitude(lifts: Vec<Lift>, rides: Vec<u32>) -> i32 {
    rides.iter().fold(0, |acc, cur| {
        let lift = lifts.iter().find(|&l| l.id == *cur);
        if let Some(l) = lift {
            return acc + l.end - l.start;
        }
        return acc;
    })
}

#[cfg(test)]
extern crate quickcheck;
use quickcheck::{quickcheck, Arbitrary, Gen};

mod tests {
    use super::*;

    #[test]
    fn when_no_lifts_ridden() {
        assert_eq!(
            accumulated_altitude(
                vec![Lift {
                    start: 0,
                    end: 0,
                    id: 1
                }],
                vec![]
            ),
            0
        );
    }
    #[test]
    fn when_one_lifts_ridden() {
        assert_eq!(
            accumulated_altitude(
                vec![Lift {
                    start: 0,
                    end: 1000,
                    id: 1
                }],
                vec![1]
            ),
            1000
        );
    }

    #[test]
    fn when_two_lifts_ridden() {
        assert_eq!(
            accumulated_altitude(
                vec![
                    Lift {
                        start: 0,
                        end: 1000,
                        id: 1
                    },
                    Lift {
                        start: 1000,
                        end: 2000,
                        id: 2
                    }
                ],
                vec![1, 2]
            ),
            2000
        );
    }

    #[test]
    fn when_two_lifts_overlap() {
        assert_eq!(
            accumulated_altitude(
                vec![
                    Lift {
                        start: 0,
                        end: 1000,
                        id: 1
                    },
                    Lift {
                        start: 500,
                        end: 1000,
                        id: 2
                    }
                ],
                vec![1, 2]
            ),
            1500
        );
    }

    #[test]
    fn when_two_lifts_ridden_multiple_times() {
        assert_eq!(
            accumulated_altitude(
                vec![
                    Lift {
                        start: 1000,
                        end: 1200,
                        id: 1
                    },
                    Lift {
                        start: 500,
                        end: 1000,
                        id: 2
                    }
                ],
                vec![1, 1, 1, 1, 1, 2]
            ),
            1500
        );
    }

    quickcheck! {
        fn random_quick_check(lifts: Vec<Lift>, rides: Vec<u32>) -> bool {
            let nl = lifts.clone();
            let nr = rides.clone();
            accumulated_altitude(lifts, rides) == hidden14141413731_accumulated_altitude(nl, nr)
        }
    }
}
