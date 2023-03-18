pub struct Defaults;

impl Defaults {
    pub fn unknown() -> String {
        "Unknown".to_owned()
    }

    pub fn unspecified() -> String {
        "Unspecified".to_owned()
    }

    pub fn vec_unspecified() -> Vec<String> {
        vec!["Unspecified".to_owned()]
    }

    pub fn xmlns_xsi() -> String {
        "http://www.w3.org/2001/XMLSchema-instance".to_owned()
    }

    pub fn xmlns() -> String {
        "http://www.clarin.eu/cmd/".to_owned()
    }

    pub fn xmlns_cmd() -> String {
        "http://www.clarin.eu/cmd/".to_owned()
    }

    pub fn xmlns_imdi() -> String {
        "http://www.mpi.nl/IMDI/Schema/IMDI".to_owned()
    }

    pub fn xmlns_lat() -> String {
        "http://lat.mpi.nl/".to_owned()
    }

    pub fn xmlns_iso() -> String {
        "http://www.iso.org/".to_owned()
    }

    pub fn xmlns_sil() -> String {
        "http://www.sil.org/".to_owned()
    }

    pub fn xmlns_xs() -> String {
        "http://www.w3.org/2001/XMLSchema".to_owned()
    }

    pub fn xmlns_functx() -> String {
        "http://www.functx.com".to_owned()
    }

    pub fn xsi_schemalocation() -> String {
        "http://www.clarin.eu/cmd/ http://catalog.clarin.eu/ds/ComponentRegistry/rest/registry/profiles/clarin.eu:cr1:p_1407745712035/xsd".to_owned()
    }
}