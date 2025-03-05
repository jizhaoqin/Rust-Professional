use std::collections::{HashMap, HashSet};
use std::fs;

fn parse_json_like_string(input: &str) -> Vec<(String, HashMap<String, HashSet<String>>)> {
    let mut result: Vec<(String, HashMap<String, HashSet<String>>)> = Vec::new();
    let mut current_batch: Option<String> = None;

    for line in input.lines() {
        let line = line.trim();

        if line.ends_with("{") && line.contains(":") {
            let parts: Vec<&str> = line.split(':').collect();
            if let Some(batch) = parts.first() {
                let batch = batch.trim().trim_matches('"');
                current_batch = Some(batch.to_string());
                result.push((batch.to_string(), HashMap::new()));
            }
        }
        // 解析城市 "青浦": ["嘉定", "青浦"],
        else if line.contains(":") {
            let parts: Vec<&str> = line.split(':').collect();
            if let (Some(city), Some(neighbors)) = (parts.first(), parts.get(1)) {
                let city = city.trim().trim_matches('"');
                let neighbors = neighbors
                    .trim()
                    .trim_matches(|c| c == '[' || c == ']' || c == ',')
                    .replace("\"", "")
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<HashSet<String>>();

                if let Some(batch) = &current_batch {
                    if let Some((_, batch_map)) = result.iter_mut().find(|(b, _)| b == batch) {
                        let city_neighbors = batch_map.entry(city.to_string()).or_default();
                        city_neighbors.extend(neighbors);
                    }
                }
            }
        }
    }

    result
}

fn dfs(city: &str, graph: &HashMap<String, HashSet<String>>, visited: &mut HashSet<String>) {
    visited.insert(city.to_string());

    if let Some(neighbors) = graph.get(city) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                dfs(neighbor, graph, visited);
            }
        }
    }
}

pub fn count_provinces() -> String {
    // 读取JSON文件
    let filename = "./district.json";
    let file_contents = fs::read_to_string(filename).expect("Unable to read file");
    let data = parse_json_like_string(&file_contents);

    let mut results: Vec<usize> = Vec::new();

    // 遍历每个批次
    for (_, city_map) in data {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();

        // 构建邻接表
        for (city, neighbor_set) in city_map {
            // 为城市和它的邻居建立连接
            graph.entry(city.clone()).or_default();
            for neighbor in neighbor_set {
                // 由于是无向图，确保两个方向都建立连接
                graph.entry(neighbor.clone()).or_default();
                graph.get_mut(&city).unwrap().insert(neighbor.clone());
                graph.get_mut(&neighbor).unwrap().insert(city.clone());
            }
        }

        // 计算连通分量（省份数）
        let mut visited: HashSet<String> = HashSet::new();
        let mut province_count = 0;

        // DFS遍历所有城市，统计连通分量
        for city in graph.keys() {
            if !visited.contains(city) {
                dfs(city, &graph, &mut visited);
                province_count += 1;
            }
        }

        // 记录每个批次的省份数
        results.push(province_count);
    }

    results
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}
