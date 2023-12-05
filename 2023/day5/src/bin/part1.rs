use std::{fs, collections::BTreeMap, usize};
use nom::{IResult, character::complete::{line_ending, space1, digit1, alpha1}, sequence::{preceded, terminated, separated_pair, tuple, pair}, multi::separated_list1, bytes::complete::tag};

#[derive(Debug, Clone)]
struct SeedMapping {
    source: usize,
    length: usize,
    dest: usize   
}

#[derive(Debug, Clone)]
struct Mapping {
    origin: String,
    end: String,
    seed_mappings: Vec<SeedMapping>,
}

#[derive(Debug, Clone)]
struct Map {
    start_seeds: Vec<usize>,
    mappings: BTreeMap<String, Mapping>,
}

impl SeedMapping {
    fn contains(&self, val: &usize) -> bool{
        return self.source <= *val && *val < self.source + self.length
    }

    fn get_dest(&self, val: &usize) -> usize {
        let diff = val - self.source;
        return diff + self.dest;
    }
}

fn to_usize(val: &str) -> usize {
    val.parse::<usize>().unwrap()
} 

fn parse_heading(input: &str) -> IResult<&str,Vec<usize>> {
    let (input, seeds) = preceded(tag("seeds: "), separated_list1(space1, digit1))(input)?;
    Ok((input, seeds.iter().map(|val| val.parse::<usize>().unwrap()).collect::<Vec<_>>()))
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
    let mut new_seeds: Vec<usize> = Vec::from(map.start_seeds.clone()); 
    for (id, val) in map.start_seeds.iter().enumerate() {
        for seed_mapping in mapping.seed_mappings.clone() {
            if seed_mapping.contains(val) {
                new_seeds.remove(id);
                new_seeds.insert(id, seed_mapping.get_dest(val))
            }
        }
    };

    map.start_seeds = new_seeds;

    if mapping.end == "location" {
        return *map.start_seeds.iter().min().unwrap();
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