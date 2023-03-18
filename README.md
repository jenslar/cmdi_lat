# cmdi-lat

Experimental (i.e. in flux, and lacking in many places) Rust crate for reading/writing CMDI metadata.
Cover two CMDI v1.1 profiles used in MPI Nijmegen's FLAT archiving software:
- FLAT Collection, a.k.a. `lat-corpus`.
    - Profile [`clarin.eu:cr1:p_1407745712064`](https://catalog.clarin.eu/ds/ComponentRegistry#/?itemId=clarin.eu%3Acr1%3Ap_1407745712064&registrySpace=public)
    - [Schema](https://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/1.1/profiles/clarin.eu:cr1:p_1407745712064/xsd)
- FLAT Bundle, a.k.a. `lat-session`:
    - Profile [`clarin.eu:cr1:p_1407745712035`](https://catalog.clarin.eu/ds/ComponentRegistry#/?itemId=clarin.eu%3Acr1%3Ap_1407745712035&registrySpace=public)
    - [Schema](https://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/1.1/profiles/clarin.eu:cr1:p_1407745712035/xsd)

Usage (not yet on crates.io):

`Cargo.toml`:
```toml
[dependencies]
cmdi-lat = {git = "https://github.com/jenslar/cmdi_lat.git"}
```

`src/main.rs`:
```rs
use std::path::Path;
use cmdi_lat::Cmd;

fn main() -> std::io::Result<()> {
    let path = Path::new("MYCMDI.cmdi");
    let cmd = Cmd::deserialize(&path)?;
    println!("{cmd:#?}");
    Ok(())
}
```