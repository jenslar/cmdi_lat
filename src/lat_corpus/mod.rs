//! Core structure representing CMDI metadata used by MPI Nijmegen's FLAT archiving software.
//!
//! FLAT Collection, a.k.a. `lat-corpus`.
//! - CMDI v1.1 profile [`clarin.eu:cr1:p_1407745712064`](https://catalog.clarin.eu/ds/ComponentRegistry#/?itemId=clarin.eu%3Acr1%3Ap_1407745712064&registrySpace=public)
//! - [CMDI v1.1 schema](https://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/1.1/profiles/clarin.eu:cr1:p_1407745712064/xsd)

pub mod corpus_link;
pub mod lat_corpus;

pub use lat_corpus::LatCorpus;