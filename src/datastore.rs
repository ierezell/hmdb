use crate::metabolites::parse_metabolites;

use std::path::Path;
use surrealdb::Datastore;
pub fn populate_datastore(datastore: &Datastore, metabolite_path: &Path) {
    parse_metabolites(metabolite_path);
}
