/// Copyright (c) 2018 Jason Graalum
///
/// CS510 Rust Programming
/// Summer 2018
///
/// HW #3
/// Translate codon series to proteins
///
use std::collections::HashMap;

// use a Hash within a Hash within a Hash to decode the three letter codon
pub struct Protein {
    codon_protein_map: HashMap<char, HashMap<char, HashMap<char, &'static str>>>,
}

//
// Parse function to build the three-deep HashTable from the Vec<str,str>
// Output is a Protein instance
//
pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Protein {
    let mut c0_hash: HashMap<char, HashMap<char, HashMap<char, &'static str>>> = HashMap::new();

    for pair in pairs {
        let mut c_keys = pair.0.chars();

        let mut c1_hash: HashMap<char, HashMap<char, &'static str>> = HashMap::new();
        let mut c2_hash: HashMap<char, &'static str> = HashMap::new();

        let mut c1_hash = c0_hash.entry(c_keys.next().unwrap()).or_insert(c1_hash);
        let mut c2_hash = c1_hash.entry(c_keys.next().unwrap()).or_insert(c2_hash);
        c2_hash.insert(c_keys.next().unwrap(), pair.1);
    }

    Protein {
        codon_protein_map: c0_hash,
    }
}

static STOP_CODON: &'static str = "stop codon";
impl Protein {
    // Given a codon, return the equivalent protein or error if invalid
    pub fn name_for(&self, codon: &str) -> Result<&str, &str> {
        if codon.len() != 3 {
            return Err("Invalid codon");
        }

        let mut c_keys = codon.chars();

        match self.codon_protein_map.get(&c_keys.next().unwrap()) {
            None => Err("Not a codon"),
            Some(hm) => match hm.get(&c_keys.next().unwrap()) {
                None => Err("Not a codon"),
                Some(hm) => match hm.get(&c_keys.next().unwrap()) {
                    None => Err("Not a codon"),
                    Some(protein) => Ok(protein),
                },
            },
        }
    }

    // From a string of chars, group into 3 chars and decode stopping if
    // a STOP_CODON code is read
    pub fn of_rna(&self, rna: &str) -> Result<Vec<&str>, &str> {
        let mut result_vec = Vec::new();
        let mut index = 0;
        while index < rna.len() {
            match rna.get(index..index + 3) {
                Some(r) => match self.name_for(r) {
                    Err(err) => return Err(err),
                    Ok(protein) => {
                        println!("{}", protein);
                        if protein == STOP_CODON {
                            println!("Found it");
                            return Ok(result_vec);
                        } else {
                            result_vec.push(protein);
                        }
                    }
                },
                None => return Err("Bad codon"),
            }
            index += 3;
        }
        Ok(result_vec)
    }
}
