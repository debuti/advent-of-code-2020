use std::collections::HashMap;

fn main() {
    let data = include_bytes!("data.txt");
    let mut data: Vec<_> = String::from_utf8_lossy(data)
        .split("\n")
        .filter(|x| x.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    data.push(0);
    data.push(*data.iter().max().unwrap() + 3);
    data.sort();
    //println!("{:#?}", data);

    let diffs: Vec<u32> = data.windows(2).map(|x| x[1] - x[0]).collect(); //println!("{:#?}", diffs);
    println!("The processed histogram is {:#?}", histogram(diffs));

    let mut counts: Vec<u64> = vec![0; *data.last().unwrap() as usize + 1];
    for item in data {
        counts[item as usize] = match item {
            0 => 1,
            1 => counts[(item - 1) as usize],
            2 => counts[(item - 1) as usize] + counts[(item - 2) as usize],
            _ => {
                counts[(item - 1) as usize]
                    + counts[(item - 2) as usize]
                    + counts[(item - 3) as usize]
            }
        }
    }
    println!(
        "The answer to the second question is {:#?}",
        counts.last().unwrap()
    );
}

fn histogram<I>(it: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut result = HashMap::new();

    for item in it {
        *result.entry(item).or_insert(0) += 1;
    }

    result
}
