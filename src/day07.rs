use std::collections::HashMap;

type BagMap<'a> = HashMap<&'a str, Vec<(i32, &'a str)>>;

fn parse(input: &str) -> BagMap {
    input
        .lines()
        .filter_map(|line| {
            let mut colour_and_inner_bags = line.split(" bags contain ");

            let colour = colour_and_inner_bags.next()?;

            let inner_bags = colour_and_inner_bags
                .next()?
                .split(", ")
                .filter_map(|inner_bag| {
                    let (inner_bag, _bag) = inner_bag.rsplit_once(' ')?;

                    let (quantity, colour) = inner_bag.split_once(' ')?;
                    let quantity = quantity.parse::<i32>().ok()?;

                    Some((quantity, colour))
                })
                .collect::<Vec<_>>();

            Some((colour, inner_bags))
        })
        .collect()
}

fn contains_gold<'a>(cache: &mut HashMap<&'a str, bool>, map: &BagMap<'a>, bag: &'a str) -> bool {
    match map.get(bag) {
        Some(bags) => match cache.get(bag) {
            Some(cached) => *cached,
            None => {
                let contains_gold = bags.iter().any(|(_, bag)| contains_gold(cache, map, bag));
                cache.insert(bag, contains_gold);

                contains_gold
            }
        },
        None => false,
    }
}

pub fn part1(input: String) -> usize {
    let bags = parse(&input);

    let mut cache = HashMap::new();
    cache.insert("shiny gold", true);

    let count = bags
        .keys()
        .filter(|k| contains_gold(&mut cache, &bags, k))
        .count();

    count - 1
}

fn total_bags(map: &BagMap, bag: &str) -> i32 {
    match map.get(bag) {
        Some(bags) => {
            bags.iter()
                .map(|(colour, bags)| colour * total_bags(map, bags))
                .sum::<i32>()
                + 1
        }
        None => 0,
    }
}

pub fn part2(input: String) -> usize {
    let bags = parse(&input);

    let total = total_bags(&bags, "shiny gold");

    (total as usize) - 1
}
