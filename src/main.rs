use regex::Regex;
use std::collections::{HashMap, HashSet};

const DEBUG: bool = false;
macro_rules! debugln {
    ($($args:expr),*) => ( if DEBUG {println!($( $args ),* )});
}

fn main() {
    let data = String::from_utf8_lossy(include_bytes!("data.txt"));
    let data: Vec<(HashSet<&str>, HashSet<&str>)> = data
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(
            |x| match Regex::new(r"(.*) \(contains (.*)\)").unwrap().captures(x) {
                Some(x) => {
                    let ingredients = x.get(1).unwrap().as_str();

                    let allergens = x.get(2).unwrap().as_str();
                    (
                        ingredients.split(" ").collect(),
                        allergens.split(", ").collect(),
                    )
                }
                None => unreachable!(),
            },
        )
        .collect();
    let mut allergens: HashSet<&str> = HashSet::new();
    for (_, i) in &data {
        allergens.extend(i);
    }
    debugln!("Data: {:#?}", data);
    debugln!("Allergens: {:#?}", allergens);

    // Discover which ingredients contains allergens by isolating allergens to a single ingredient.
    // As this ingredient can only have one allergen it will fall off the rest of the allergen lists,
    // leaving some of those list with only one ingredient and so on
    let mut ingredients_by_allergen: HashMap<&str, &str> = HashMap::new();
    while allergens.len() > 0 {
        allergens.retain(|allergen| {
            let mut result: Option<HashSet<&str>> = None;
            for (a, b) in &data {
                if b.contains(allergen) {
                    if result.is_none() {
                        result = Some(a.clone());
                    } else {
                        result = Some(result.unwrap().intersection(&a).copied().collect());
                    }
                }
            }
            let mut result = result.unwrap();
            for (_, v) in &ingredients_by_allergen {
                result.remove(v);
            }
            if result.len() == 1 {
                let selected = result.iter().next().unwrap();
                ingredients_by_allergen.insert(allergen, selected);
                return false;
            }
            true
        });
    }

    // Get ingredients_wo_allergen by substracting the ingredients_by_allergen from the list of ingredients
    let mut ingredients_wo_allergen: Vec<&str> = data.iter().fold(Vec::new(), |mut acc, x| {
        acc.extend(x.0.iter());
        acc
    });
    ingredients_wo_allergen.retain(|v| !ingredients_by_allergen.values().any(|w| w == v));
    println!("Answer to part 1: {:#?}", ingredients_wo_allergen.len());

    // Fold the sorted vector that comes from ingredients_by_allergen map
    let mut v: Vec<_> = ingredients_by_allergen.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    println!(
        "Answer to part 2: {:#?}",
        v.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(x.1);
            acc.push_str(",");
            acc
        })
    );
}
