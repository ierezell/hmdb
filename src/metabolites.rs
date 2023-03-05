use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Accession {
    accession: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Synonym {
    synonym: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _AlternativeParent {
    alternative_parent: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Substituent {
    substituent: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _ExternalDescriptor {
    external_descriptor: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Taxonomy {
    description: String,
    direct_parent: String,
    kingdom: String,
    super_class: String,
    class: String,
    sub_class: String,
    molecular_framework: String,
    alternative_parents: Vec<_AlternativeParent>,
    substituents: Vec<_Substituent>,
    external_descriptors: Vec<_ExternalDescriptor>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Protein {
    protein_accession: String,
    name: String,
    uniprot_id: String,
    genre_name: String,
    protein_type: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Disease {
    name: String,
    omim_id: f32,
    references: Vec<_Reference>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Reference {
    reference_text: String,
    pubmed_id: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _AbnormalConcentration {
    biospecimen: String,
    concentration_value: String,
    concentration_units: String,
    patient_age: String,
    patient_sex: String,
    patient_information: String,
    references: Vec<_Reference>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _NormalConcentration {
    biospecimen: String,
    concentration_value: String,
    concentration_units: String,
    subject_age: String,
    subject_sex: String,
    subject_condition: String,
    patient_information: String,
    references: Vec<_Reference>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Pathway {
    name: String,
    smpdb_id: String,
    // kegg_map_id: ?,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _BiologicalProperties {
    pathways: Vec<_Pathway>,
    // tissue_locations: ?,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Spectrum {
    r#type: String,
    spectrum_id: f32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Property {
    kind: String,
    value: f32,
    source: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct _Ontology {
    term: String,
    definition: String,
    // parent_id: ?,
    level: i32,
    r#type: String,
    synonyms: Vec<_Synonym>,
    descendants: Vec<_Ontology>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Metabolite {
    version: f32,
    creation_date: String,
    update_date: String,
    accession: String,
    status: String,
    secondary_accessions: Vec<_Accession>,
    name: String,
    description: String,
    synonyms: Vec<_Synonym>,
    chemical_formula: String,
    average_molecular_weight: f32,
    monisotopic_molecular_weight: f32,
    iupac_name: String,
    traditional_iupac: String,
    cas_registry_number: String,
    smiles: String,
    inchi: String,
    inchikey: String,
    taxonomy: _Taxonomy,
    kegg_id: String,
    drugbank_id: String,
    foodb_id: String,
    chemspider_id: f32,
    pubchem_compound_id: f32,
    chebi_id: f32,
    knapsack_id: String,
    state: String,
    biocyc_id: String,
    wikipedia_id: String,
    bigg_id: f32,
    vmh_id: String,
    synthesis_reference: String,
    spectra: Vec<_Spectrum>,
    ontology: Vec<_Ontology>,
    biological_properties: _BiologicalProperties,
    experimental_properties: Vec<_Property>,
    predicted_properties: Vec<_Property>,
    abnormal_concentrations: Vec<_AbnormalConcentration>,
    normal_concentrations: Vec<_NormalConcentration>,
    diseases: Vec<_Disease>,
    general_references: Vec<_Reference>,
    protein_associations: Vec<_Protein>,
    // phenol_explorer_compound_id : ?
    // metlin_id : ?
    // pdb_id : ? ,
    // fbonto_id : ?
}

pub fn parse_metabolites(path: &Path) {
    let mut reader = Reader::from_file(path).expect("Cannot create XML reader");

    let mut buf = Vec::new();
    let mut unsupported = false;
    loop {
        buf.clear();
        match reader.read_event_into(&mut buf).unwrap() {
            Event::Eof => break,
            _ => {}
        }
    }
}
