use std::collections::HashMap;

fn backtracking(
    airport: String,
    targets: &mut HashMap<&String, Vec<&String>>,
    result: &mut Vec<String>,
) {
    while let Some(next_airport) = targets.get_mut(&airport).unwrap_or(&mut vec![]).pop() {
        backtracking(next_airport.clone(), targets, result);
    }
    println!("res push {:?}", airport.clone());
    result.push(airport.clone());
}

pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
    let mut targets: HashMap<&String, Vec<&String>> = HashMap::new();
    let mut result = Vec::new();
    for t in 0..tickets.len() {
        targets
            .entry(&tickets[t][0])
            .or_default()
            .push(&tickets[t][1]);
    }
    for (_, target) in targets.iter_mut() {
        target.sort_by(|a, b| b.cmp(a));
    }
    backtracking("JFK".to_string(), &mut targets, &mut result);
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! combination_tests {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (k, expected) = $test_case;
                    //assert_eq!(find_itinerary(k), expected);
                    println!("{:?}",find_itinerary(k));
                }
            )*
        }
    }
    combination_tests! {
        test_generate_4_2: (vec![ vec!["NRT".to_string(), "JFK".to_string()],vec!["JFK".to_string(), "ZUL".to_string()], vec!["JFK".to_string(), "NRT".to_string()],], vec![
                vec![1, 2, 2], vec![2, 1, 2], vec![2, 2, 1]
            ]),
    }
}
