// Fields to keep:
// Variant:
//  gnomad ID
//  Filters (exome/genome)
//  Flags (exome/genome)
//  AC (exome/genome)
//  AN (exome/genome)
//  AF (exome/genome)
//
//  divide by ancestry group:
//  AC
//  AN
//  AF
//  VEP annotation
//  LoF curation
//  Germline classification
//
// Variant:
//  Symbol
//  Ensembl ID
//  Region
//  Flags
//  Filters
//  fafmax95
//  gnomAD variant IDs

// Validation:
// same deal as https://github.com/dathere/qsv/blob/6b6985065a1270f767d881b13aa2a27fae1958fb/src/cmd/validate.rs#L938
// parse JSON schema, map IR types to JSON types, off to the races

//use jsonschema::types::JsonType;
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
    struct Variant {
        id: String,
    }

    factori!(Variant, {
        default {
            id: String = "1-234-A-C".to_string(),
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
                            ]
                        }));
                    }
                }
            }
        }
        for serde_value in &serde_values {
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
}

fn main() -> () {
    ()
}
