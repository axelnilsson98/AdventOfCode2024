use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn part1(lines: Vec<String>){
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines{
        let ids: Vec<String> = line.split('-').map(|x|x.to_string()).collect();
        connections.entry(ids[0].clone()).or_insert(HashSet::new()).insert(ids[1].clone());
        connections.entry(ids[1].clone()).or_insert(HashSet::new()).insert(ids[0].clone());
    }
    // println!("{:?}", connections);

    let mut threes: HashSet<Vec<&String>> = HashSet::new();

    for (main, set) in &connections{
        for sub in set{
            let sub_set = connections.get(sub).unwrap();
            let intersect: Vec<&String> = set.intersection(sub_set).collect();
            if !intersect.is_empty(){
                for third in intersect{
                    let mut tmp: Vec<&String> = Vec::from([main, sub, third]);
                    tmp.sort();
                    threes.insert(tmp);
                }
            }
        }
    }

    // println!("{:?}", threes);
    
    let t_threes: Vec<Vec<&String>> = threes.into_iter().filter(|xs| xs[0].starts_with("t") || xs[1].starts_with("t") || xs[2].starts_with("t")).collect();

    println!("{:?}", t_threes.len());
}


pub fn part2(lines: Vec<String>){
    let mut connections: HashMap<String, HashSet<String>> = HashMap::new();
    for line in lines{
        let ids: Vec<String> = line.split('-').map(|x|x.to_string()).collect();
        connections.entry(ids[0].clone()).or_insert(HashSet::new()).insert(ids[1].clone());
        connections.entry(ids[1].clone()).or_insert(HashSet::new()).insert(ids[0].clone());
    }

    let mut cliques: Vec<HashSet<String>> = Vec::new();
    let p = connections.keys().map(|x| x.clone()).collect();
    let x = HashSet::new();
    let r = HashSet::new();
    cliques = bron_kerbosh(r, p, x, cliques, &connections);

    
    let biggest_clique = cliques.iter().max_by(|a,b| a.len().cmp(&b.len())).unwrap();
    let biggest_clique_vec: Vec<&String> = biggest_clique.into_iter().sorted_by(|a,b| (*a).cmp(*b)).collect();
    println!("{}", biggest_clique_vec.into_iter().join(","));

}


fn bron_kerbosh(r: HashSet<String>,mut p: HashSet<String>,mut x: HashSet<String>, mut cliques: Vec<HashSet<String>>, edges: &HashMap<String, HashSet<String>>) -> Vec<HashSet<String>>{
    if p.is_empty() && x.is_empty(){
        cliques.push(r);
        return cliques;
    }
    let backup = p.clone();
    for n in p.clone() {
        let mut tmp_r = r.clone();
        let mut tmp_x = x.clone();
        tmp_r.insert(n.clone());
        p = p.intersection(edges.get(&n).unwrap()).map(|x| x.clone()).collect();
        tmp_x = tmp_x.intersection(edges.get(&n).unwrap()).map(|x| x.clone()).collect();
        cliques = bron_kerbosh(tmp_r, p.clone(), tmp_x, cliques, edges);
        p = backup.clone();
        p.remove(&n);
        x.insert(n);
    }
    cliques
}