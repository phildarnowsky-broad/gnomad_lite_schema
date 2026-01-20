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
// Gene:
//  Ensembl ID
//  Region
//  Flags
//  Filters
//  fafmax95
//  gnomAD variant IDs

// Validation:
// same deal as https://github.com/dathere/qsv/blob/6b6985065a1270f767d881b13aa2a27fae1958fb/src/cmd/validate.rs#L938
// parse JSON schema, map IR types to JSON types, off to the races

use std::fs::{canonicalize, read_to_string};

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let schema_path = canonicalize("./gnomad-lite-schema.json")?;
    let schema_text = read_to_string(schema_path)?;
    let schema = serde_json::from_str(&schema_text)?;
    match jsonschema::meta::validate(&schema) {
        Err(err) => {
            print!("error: {}\n", err);
        }
        Ok(_) => {
            print!("schema: {}\n", schema);
        }
    }
    Ok(())
}
