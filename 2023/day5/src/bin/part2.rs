use std::{fs, collections::BTreeMap, usize, ptr::null};
use nom::{IResult, character::complete::{line_ending, space1, digit1, alpha1}, sequence::{preceded, terminated, separated_pair, tuple, pair}, multi::separated_list1, bytes::complete::tag};

#[derive(Debug, Clone)]
struct SeedMapping {
    source: usize,
    length: usize,
    dest: usize   
}

#[derive(Debug, Clone)]
struct Seed {
    num: usize,
    length: usize,
}

#[derive(Debug, Clone)]
struct Mapping {
    origin: String,
    end: String,
    seed_mappings: Vec<SeedMapping>,
}

#[derive(Debug, Clone)]
struct Map {
    start_seeds: Vec<Seed>,
    mappings: BTreeMap<String, Mapping>,
}

impl SeedMapping {
    fn overlap(&self, val: &Seed) -> bool {
        return  (self.source <= val.num && val.num < self.source + self.length) ||
            (self.source < val.num + val.length && val.num + val.length < self.source + self.length) 
        }

    fn map_overlap(&self, val: &Seed) -> (Seed, Vec<Seed>) {
        let start = self.source.max(val.num);
        let end = (self.source + self.length).min(val.num + val.length);

        let mut ret: Vec<Seed> = Vec::new();
        if val.num < start {
            ret.push(Seed { num: val.num, length: start - val.num})
        }
        if end < val.num + val.length {
            ret.push(Seed { num: end, length: val.num + val.length - end + 1})
        }

        let overlap = Seed{num: start - self.source + self.dest, length: end - start};

        //println!("{:?} -> {:?} + {:?} | {:?} -> {:?}\n", (val.num, val.num + val.length - 1) ,(start, end), &ret, (self.source, self.source + self.length - 1), overlap);

        return (overlap, ret);
    }
}

impl Seed {
    fn min(&self) -> usize {
        return self.num;
    }
}

fn to_usize(val: &str) -> usize {
    val.parse::<usize>().unwrap()
} 

fn parse_heading(input: &str) -> IResult<&str,Vec<Seed>> {
    let (input, seed_string) = preceded(tag("seeds: "), separated_list1(space1, separated_pair(digit1, tag(" "), digit1)))(input)?;
    let mut seeds: Vec<Seed> = Vec::new();
    for (start, len) in seed_string {
        seeds.push(Seed { num: to_usize(start), length: to_usize(len) });
    }
    Ok((input, seeds))
}

fn parse_mapping_heading(input: &str) -> IResult<&str,(String, String)> {
    let (input, (origin, end)) = terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:"))(input)?;
    Ok((input, (origin.to_owned(), end.to_owned())))
}

fn parse_mapping(input: &str) -> IResult<&str, Mapping> {
    let (input, (origin, end)) = terminated(parse_mapping_heading, line_ending)(input)?;
    let (input, seed_mappings) = separated_list1(line_ending, parse_seed)(input)?;
    Ok((input, Mapping{ origin: origin, end: end, seed_mappings: seed_mappings}))
}

fn parse_seed(input: &str) -> IResult<&str, SeedMapping> {
    let (input, (seed, (from, length))) = tuple((terminated(digit1, tag(" ")), separated_pair(digit1, tag(" "), digit1)))(input)?;
    Ok((input, SeedMapping{source: to_usize(from), length: to_usize(length), dest: to_usize(seed)}))
}

fn parse_body(input: &str) -> IResult<&str,BTreeMap<String, Mapping>> {
    let (input, res) = separated_list1(pair(line_ending,line_ending), parse_mapping)(input)?;
    let mut tree = BTreeMap::new();
    res.iter().for_each(|mapping| {tree.insert(mapping.origin.to_owned(), mapping.clone());});
    Ok((input, tree))
}

fn parse_map(input: &str) -> IResult<&str,Map> {
    let (input, heading) = terminated(parse_heading, pair(line_ending,line_ending))(input)?;
    let (input, body) = parse_body(input)?;
    Ok((input, Map{ start_seeds: heading, mappings: body}))
}

fn solve_rec(map: &mut Map, key: String) -> usize {
    let mapping = map.mappings.get(&key).unwrap();
    let mut current_seeds: Vec<Seed> = Vec::from(map.start_seeds.clone());
    let mut new_seeds: Vec<Seed> = Vec::new();
    
    while !current_seeds.is_empty() {
        let seed = current_seeds.pop().unwrap();
        let mut used = false;
        for seed_map in mapping.seed_mappings.iter() {
            if seed_map.overlap(&seed) {
                let (inside, outside) = seed_map.map_overlap(&seed);
                new_seeds.push(inside);
                current_seeds.append(&mut outside.clone());
                //dbg!(&new_seeds);
                used = true;
            }
        }
        if !used {new_seeds.push(seed);};
    }

    map.start_seeds = new_seeds.clone();

    if mapping.end == "location" {
        let mut min = usize::MAX;
        for seed in map.start_seeds.clone() {
            min = min.min(seed.min())
        }

        return min;
    } else {
        return solve_rec(map, mapping.end.clone());
    }
}

fn solution(file: &str) -> usize{
    let (_, mut map) = parse_map(file).expect("should parse");
    solve_rec(&mut map, "seed".to_string())
}

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();
    println!("{:?}", solution(&file));
}