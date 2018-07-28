use std::collections::HashMap;

pub struct Protein {
    codon_protein_map: HashMap<char, HashMap<char, HashMap<char, &'static str>>>,
}

pub fn parse(pairs: Vec<(&'static str, &'static str)>) -> Protein {
    let mut c0_hash: HashMap<char, HashMap<char, HashMap<char, &'static str>>> = HashMap::new();

    for pair in pairs {
        let mut c_keys = pair.0.chars();

        let c0: char = c_keys.next().unwrap();
        let c1: char = c_keys.next().unwrap();
        let c2: char = c_keys.next().unwrap();

        let mut c1_hash: HashMap<char, HashMap<char, &'static str>> = HashMap::new();
        let mut c2_hash: HashMap<char, &'static str> = HashMap::new();

        let mut c1_hash = c0_hash.entry(c0).or_insert(c1_hash);
        let mut c2_hash = c1_hash.entry(c1).or_insert(c2_hash);
        c2_hash.insert(c2, pair.1);
    }

    Protein {
        codon_protein_map: c0_hash,
    }
}

static STOP_CODON: &'static str = "stop codon";
impl Protein {
    pub fn name_for(&self, codon: &str) -> Result<&str, &str> {
        if codon.len() != 3 {
            return Err("Invalid codon");
        }

        let mut c_keys = codon.chars();

        let c0: char = c_keys.next().unwrap();
        let c1: char = c_keys.next().unwrap();
        let c2: char = c_keys.next().unwrap();

        match self.codon_protein_map.get(&c0) {
            None => Err("Not a codon"),
            Some(hm) => match hm.get(&c1) {
                None => Err("Not a codon"),
                Some(hm) => match hm.get(&c2) {
                    None => Err("Not a codon"),
                    Some(protein) => Ok(protein),
                },
            },
        }
    }

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
