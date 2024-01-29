use std::collections::{HashMap, HashSet};
use serde_derive::{Serialize,Deserialize};
use std::path::Path;
use counter::Counter;
use rayon::prelude::*;
use std;

fn _calculate(tf: f32, num_docs: f32, doc_len: usize, average_length: f32, k1: f32, b: f32, df: f32) -> f32 {
    (tf * (k1 + 1.0)) / (tf + k1 * (1.0 - b + b * (doc_len as f32 / average_length))) * (((num_docs as f32 + 1.0) / (df + 1.0)).ln() + 1.0)
}


#[derive(Serialize, Deserialize, Debug)]
pub struct BM25 {
    pub index_map: HashMap<String, HashMap<String, u32>>,
    doc_len_map: HashMap<String, usize>,
    freeze_map: HashMap<String, HashMap<String, f32>>,
    k1: f32,
    b: f32,
    average_length: f32,
}



impl BM25 {
    pub fn new() -> Self {
        BM25 { index_map: HashMap::new(), doc_len_map: HashMap::new(), freeze_map: HashMap::new(), k1: 1.5, b: 0.75, average_length: 0.0}
    }

    pub fn load(path:String) -> Self {
        let json_file = std::fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&json_file).unwrap()
    }

    pub fn save(&self, path: String) {
        let json_file = serde_json::to_string(&self).unwrap();
        std::fs::write(path, json_file).expect("Unable to write file");
    }


    pub fn add_document(&mut self, id: String, document: Vec<String>) {
        for token in document.iter() {
            if !self.index_map.contains_key(token) {
                self.index_map.insert(
                    token.to_string(),
                    HashMap::new(),
                );
            }
            let target = self.index_map.get_mut(token).unwrap();
            if !target.contains_key(id.as_str()) {
                target.insert(id.to_string(), 0);
            };

            *target.get_mut(id.as_str()).unwrap() += 1;
        }
        self.doc_len_map.insert(id.to_string(), document.len());
    }

    pub fn freeze(&mut self) {
        self.average_length = self.doc_len_map.values().sum::<usize>() as f32 / self.doc_len_map.len() as f32;
        self.freeze_map = self.index_map.iter()
            .map(|(k, doc_freq)| (k.to_string(), doc_freq.iter()
                .map(|(dk, dv)|
                    (
                        dk.to_string(),
                        _calculate(
                            *dv as f32,
                            self.doc_len_map.len() as f32,
                            self.doc_len_map.get(dk).unwrap().clone(),
                            self.average_length,
                            self.k1,
                            self.b,
                            doc_freq.len() as f32,
                        )
                    )
                ).collect::<HashMap<String, f32>>())
            ).collect::<HashMap<String, HashMap<String, f32>>>();
    }



    pub fn search(&self, query_tokens: Vec<String>, n: usize) -> Vec<(String, f32)> {
        if self.freeze_map.len() == 0 {
            panic!("Please freeze the index before searching!");
        }
        let mut scores = HashMap::new();
        for (query, _) in query_tokens.iter().collect::<Counter<_>>().iter(){
            if self.freeze_map.contains_key(query.as_str()){
                let targets = self.freeze_map.get(query.as_str()).unwrap();
                for (doc_id, score) in targets {
                    if !scores.contains_key(doc_id.as_str()) {
                        scores.insert(doc_id.to_string(), 0.0);
                    }
                    *scores.get_mut(doc_id.as_str()).unwrap() += score;
                }
            }
        }
        let mut scores = scores.iter().map(|(k, v)| (k.to_string(), v.to_owned())).collect::<Vec<(String, f32)>>();
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores.truncate(n);
        scores
    }


    pub fn batch_search(&self, tokenized_queries: Vec<Vec<String>>, n: usize) -> Vec<Vec<(String, f32)>> {
        tokenized_queries.par_iter().map(
            |tokenized_query| self.search(tokenized_query.to_vec(), n)
        ).collect()
    }

    pub fn delete_document(&mut self, id: String) {
        let tokens_to_modify: Vec<String> = self.index_map.iter()
            .filter(|(_, target)| target.contains_key(&id))
            .map(|(token, _)| token.clone())
            .collect();

        for token in tokens_to_modify {
            self.index_map.get_mut(&token).unwrap().remove(&id);
        }

        self.doc_len_map.remove(&id);

        if !self.freeze_map.is_empty() {
            let freeze_tokens_to_modify: Vec<String> = self.freeze_map.iter()
                .filter(|(_, target)| target.contains_key(&id))
                .map(|(token, _)| token.clone())
                .collect();

            for token in freeze_tokens_to_modify {
                self.freeze_map.get_mut(&token).unwrap().remove(&id);
            }
        }
    }


}