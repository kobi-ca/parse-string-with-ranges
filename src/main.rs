// e.g:
// NumCollection c1("-17, -10--5, 20-25, 50");
// assert(c1.contains(-10));
// assert(!c1.contains(-3));

// NumCollection c2("-1000000-5, 1000000");
// assert(c2.contains(1000000));
// assert(!c2.contains(6));

// NumCollection c3("0-5, 10-15, 20, 30-35, 40"); // it can be a much longer sequence
// assert(c3.contains(34));
// assert(!c3.contains(36));

// limits:
// min = std::numeric_limits<long>::lowest()
// max = std::numeric_limits<long>::max()

#[cfg(test)]
mod tests{

    use super::*;

    #[test]
    fn basic() {
        let result = parse("-17, -10--5, 20-25, 50");
        assert_eq!(result, Pair(-17, -17));
    }

}

#[derive(Debug, PartialEq)]
struct Pair (i64, i64);

fn parse(string: &str) -> Pair {
    let result = string.split(',')
        .map(|s| s.trim())
        .map(|s|
            {
                let v = s.split('-').collect::<Vec<&str>>();
                println!("{:?}", v);
                match v.len() {
                    1 => { (v[0], v[0]) },
                    2 => match v[0].len() {
                       0 => (v[1], v[1]),
                       _ => (v[0], v[1]),
                    },
                    _ => { panic!("error") },
                }
            }
        )
        .map(|(v1, v2)| Pair(v1.parse::<i64>().unwrap(), v2.parse::<i64>().unwrap()))
        .collect::<Vec<Pair>>();
    for s in result {
        println!("{:?}", s);
    }
    Pair(-17, -17)
}


fn main() {
}
