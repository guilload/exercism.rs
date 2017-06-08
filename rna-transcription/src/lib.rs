#[derive(Debug, PartialEq)]
pub struct DeoxyribonucleicAcid {
    sequence: String
}


impl DeoxyribonucleicAcid {
    pub fn new(string: &str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid { sequence: string.to_string(), }
    }

    pub fn to_rna(&self) -> Result<RibonucleicAcid, ()> {
        self.sequence
            .chars()
            .map(|c|
                match c {
                    'G' => Ok('C'),
                    'C' => Ok('G'),
                    'T' => Ok('A'),
                    'A' => Ok('U'),
                    _   => Err(()),
                }
            )
            .collect::<Result<String, ()>>()
            .map(|rna| RibonucleicAcid::new(&rna))
    }
}


#[derive(Debug, PartialEq)]
pub struct RibonucleicAcid {
    sequence: String
}

impl RibonucleicAcid {
    pub fn new(string: &str) -> RibonucleicAcid {
        RibonucleicAcid { sequence: string.to_string(), }
    }
}
