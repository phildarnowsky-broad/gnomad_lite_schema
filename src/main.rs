use std::fs::{canonicalize, read_to_string};

pub fn build_validator_from_file_path(
    path: &str,
) -> Result<jsonschema::Validator, jsonschema::error::ValidationError<'_>> {
    let schema_path = canonicalize(path).unwrap();
    let schema_text = read_to_string(schema_path).unwrap();
    build_validator_from_string(&schema_text)
}

pub fn build_validator_from_string(
    schema_text: &str,
) -> Result<jsonschema::Validator, jsonschema::error::ValidationError<'static>> {
    let parsed_schema = serde_json::from_str(&schema_text).unwrap();
    jsonschema::Validator::new(&parsed_schema)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use factori::*;

    #[derive(serde::Serialize)]
    struct SequencingType {
        ac: u32,
        an: u32,
        af: f64,
        flags: Vec<String>,
        filters: Vec<String>,
    }

    #[derive(serde::Serialize)]
    struct Variant {
        id: String,
        exome: Option<SequencingType>,
        genome: Option<SequencingType>,
    }

    #[derive(serde::Serialize)]
    struct Gene {
        symbol: String,
        ensembl_id: String,
        chrom: String,
        start: u32,
        stop: u32,
        flags: Vec<String>,
        filters: Vec<String>,
        variant_ids: Vec<String>,
    }

    factori!(SequencingType, {
        default {
            ac: u32 = 123,
            an: u32 = 456,
            af: f64 = 0.2697,
            flags: Vec<String> = Vec::new(),
            filters: Vec<String> = Vec::new()
        }
    });

    factori!(Variant, {
        default {
            id: String = "1-234-A-C".to_string(),
            exome: Option<SequencingType> = None,
            genome: Option<SequencingType> = None
        }
    });

    factori!(Gene, {
        default {
            symbol: String = "BRCA1".to_string(),
            ensembl_id: String = "ENSG00000012048".to_string(),
            chrom: String = "17".to_string(),
            start: u32 = 43044295,
            stop: u32 = 43170245,
            flags: Vec<String> = Vec::new(),
            filters: Vec<String> = Vec::new(),
            variant_ids: Vec<String> = Vec::new(),
        }
    });

    #[test]
    fn variant_ids() {
        let validator = build_validator_from_file_path("./gnomad-lite-schema.json").unwrap();
        let valid_chroms = vec!["1", "10", "11", "20", "21", "X"];
        let valid_poses = vec!["1", "10", "19", "20", "100", "555"];
        let valid_refs = vec!["A", "C", "G", "T", "AA", "AC", "CA", "CC"];
        let valid_alts = vec!["A", "C", "G", "T", "AA", "AC", "CA", "CC"];
        let n_combinations =
            valid_chroms.len() * valid_poses.len() * valid_refs.len() * valid_alts.len();
        let mut validation_failures = Vec::with_capacity(n_combinations);
        let mut serde_values = Vec::with_capacity(n_combinations);

        for valid_chrom in &valid_chroms {
            for valid_pos in &valid_poses {
                for valid_ref in &valid_refs {
                    for valid_alt in &valid_alts {
                        let valid_id =
                            format!("{}-{}-{}-{}", valid_chrom, valid_pos, valid_ref, valid_alt);
                        serde_values.push(serde_json::json!({
                            "variants": [
                                create!(Variant, id: valid_id)
                            ],
                            "genes": []
                        }));
                    }
                }
            }
        }
        for serde_value in &serde_values {
            dbg!("{:?}", serde_value);
            let validation_result = validator.validate(serde_value);
            if let Err(validation_error) = validation_result {
                validation_failures.push(validation_error);
            }
        }
        if validation_failures.len() > 0 {
            dbg!("VALIDATION FAILURES: {:?}", &validation_failures);
        }
        assert_eq!(validation_failures.len(), 0);
    }

    #[test]
    fn ensembl_ids() {
        let good_document = serde_json::json!({
            "variants": [
            ],
            "genes": [create!(Gene)]
        });

        let bad_document = serde_json::json!({
            "variants": [
            ],
            "genes": [create!(Gene, ensembl_id: "ENSQ0101010101".to_string())]
        });

        let validator = build_validator_from_file_path("./gnomad-lite-schema.json").unwrap();
        assert_eq!(validator.is_valid(&good_document), true);
        assert_eq!(validator.is_valid(&bad_document), false);
    }
}

fn main() -> () {
    ()
}
