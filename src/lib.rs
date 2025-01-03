use clap::Parser;
use rdkitcffi::{read_sdfile, Molecule};
use std::error::Error;
use std::fs;
use std::process;

/// Compute molecule properties
pub fn generate_molecule(smiles: &str) -> Result<Molecule, Box<dyn Error>> {
    match Molecule::new(smiles) {
        Some(mol) => Ok(mol),
        None => Err("Failed to create molecule".into()),
    }
}

pub fn generate_q_molecule(smarts: &str) -> Result<Molecule, Box<dyn Error>> {
    match Molecule::get_qmol(smarts, "") {
        Some(mol) => Ok(mol),
        None => Err("Failed to create molecule".into()),
    }
}

/// load molecules
fn search_in_txt<'a>(query: &Molecule, contents: &'a str) -> Vec<String> {
    let mut results = Vec::new();

    for line in contents.lines() {
        let temp_mol = generate_molecule(line.trim());
        if temp_mol.is_err() {
            continue;
        }

        let mol = temp_mol.expect("No failure expected. Tested before for error");
        if mol.get_substruct_match(query, "").len() > 2 {
            results.push(String::from(line).clone());
        }
    }
    results
}

fn search_in_sdf<'a>(query: &Molecule, contents: &'a Vec<Molecule>) -> Vec<String> {
    let mut results = Vec::new();

    for mol in contents {
        if mol.get_substruct_match(query, "").len() > 2 {
            results.push(mol.get_smiles(""));
        }
    }
    results
}

/// entry point
pub fn find_mol(args: Args) -> Result<(), Box<dyn Error>> {
    let query: String = args.smarts;

    // load query molecule
    let query_mol = generate_q_molecule(&query);
    if let Err(_e) = query_mol {
        eprintln!("ERROR: smarts query is not valid {query}");
        process::exit(1);
    }

    println!("Searching for molecules ...");

    let result = if args.file_type == String::from("txt") {
        let contents = fs::read_to_string(args.path)?;
        search_in_txt(
            &query_mol.expect("No Failure expected. Tested before"),
            &contents,
        )
    } else {
        let mol_opt_list: Vec<Option<Molecule>> = read_sdfile(&args.path);
        let mut mol_list: Vec<Molecule> = mol_opt_list.into_iter().filter_map(|m| m).collect();
        mol_list.iter_mut().for_each(|m| m.remove_all_hs());

        if mol_list.len() == 0 {
            print!("No molecules in file!");
            Vec::<String>::new()
        } else {
            search_in_sdf(
                &query_mol.expect("No Failure expected. Tested before"),
                &mol_list,
            )
        }
    };

    if result.len() > 0 {
        for line in result {
            println!("{line}");
        }
    } else {
        println!("No matches found for {query}");
    }

    Ok(())
}

/// Parse smiles query
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// smarts substructure to search for
    #[arg(short, long)]
    smarts: String,

    /// File to search in
    #[arg(short, long)]
    path: String,

    /// File type txt (one SMILES per line), or sdf
    #[arg(short, long, default_value_t = String::from("txt"))]
    file_type: String,
}

/// tests
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    /// smiles to molecules tests
    #[test]
    fn smiles_to_mol() {
        let smiles: &str = "C1CCCCC1";

        assert!(!generate_molecule(smiles).is_err());
    }

    #[test]
    fn invalid_smiles_to_mol() {
        let smiles: &str = "%&&&1CCCCQ1";

        assert!(generate_molecule(smiles).is_err());
    }

    /// TXT route test
    #[test]
    fn has_true_subs_txt() {
        let query = generate_molecule("CO").expect("");
        let contents = "\
CCCCC
c1cccc1
CCOCC
";

        assert_eq!(vec!["CCOCC"], search_in_txt(&query, contents));
    }

    #[test]
    fn has_false_subs_txt() {
        let query = generate_molecule("CO").expect("");
        let contents = "\
CCCCC
c1cccc1
CCNCC
";

        assert_eq!(Vec::<&str>::new(), search_in_txt(&query, contents));
    }

    /// SDFF route tests
    #[test]
    fn has_true_subs_sdf() {
        let query = generate_molecule("C=O").expect("");
        // molecule is acetone
        let mol_opt_list: Vec<Option<Molecule>> = read_sdfile("./test_data/smiles.sdf");
        let mut mol_list: Vec<Molecule> = mol_opt_list.into_iter().filter_map(|m| m).collect();
        mol_list.iter_mut().for_each(|m| m.remove_all_hs());

        assert_eq!(vec!["CC(C)=O"], search_in_sdf(&query, &mol_list));
    }

    #[test]
    fn has_false_subs_sdf() {
        let query = generate_molecule("CN").expect("");
        // molecule is acetone
        let mol_opt_list: Vec<Option<Molecule>> = read_sdfile("./test_data/smiles.sdf");
        let mut mol_list: Vec<Molecule> = mol_opt_list.into_iter().filter_map(|m| m).collect();
        mol_list.iter_mut().for_each(|m| m.remove_all_hs());

        assert_eq!(Vec::<&str>::new(), search_in_sdf(&query, &mol_list));
    }
}
