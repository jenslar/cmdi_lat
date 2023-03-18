use std::{path::{Path, PathBuf}, collections::HashSet};

use crate::{Cmd, errors::CmdError};

pub struct Tree {}

impl Tree {
    /// Recursively traverse a linked CMDI-tree
    /// from specified top-node and down.
    // pub fn traverse(path: &Path, base_dir: Option<&Path>, errors: HashSet<PathBuf>) -> Result<HashSet<String>, CmdError> {
    // pub fn traverse(cmdi_path: &Path, base_dir: Option<&Path>) -> Result<HashSet<PathBuf>, CmdError> {
    // pub fn traverse(cmdi_path: &Path, base_dir: Option<&Path>) -> Result<Vec<(PathBuf, String)>, CmdError> {
    pub fn traverse(cmdi_path: &Path, base_dir: Option<&Path>) -> Result<(), CmdError> {
    // pub fn traverse(path: &Path, base_dir: Option<&Path>) -> Result<(), CmdError> {
        // println!("INPATH: {}", cmdi_path.display());
        
        let node = match &base_dir {
            Some(d) => d.join(cmdi_path).canonicalize()?,
            None => cmdi_path.to_owned()
        };
        
        // if !node.exists() {
        //     println!("(!) {} does not exist. Aborting.", node.display());
        //     std::process::exit(1)
        // }

        let base_dir = node.parent();

        // let mut errors: HashSet<String> = HashSet::new();
        // let mut errors = errors;
        
        let mut headers = vec!["CMD\tDATE\tDESCRIPTION".to_owned()];
        // println!("PATH: {}", node.canonicalize()?.display());
        // println!("  DIR:  {:?}", base_dir);
        
        if !node.exists() {
            return Err(CmdError::InvalidPath(node))
        }
        let cmd = Cmd::deserialize(&node)?;

        let mut actors = Vec::new();
        for (i, actor) in cmd.actors().into_iter().enumerate() {
            let age = actor.age.exact_age.map(|a| format!("{}", a.years())).unwrap_or("Unspecified".to_owned());
            // print!("{}\t{}\t{}\t", actor.sex, age, actor.role)
            headers.push(format!("GENDER_{0}\tAGE_{0}\tROLE_{0}", i+1));
            actors.push(format!("{}\t{}\t{}", actor.sex, age, actor.role))
        }
        headers.push("RESOURCES".to_owned());

        // let mut missing: HashSet<PathBuf> = HashSet::new();
        // let mut errors: Vec<(PathBuf, String)> = Vec::new();
        
        let mut files: Vec<String> = Vec::new();
        for resource in cmd.resources() {
            match resource.resource_type.value.as_str() {
                "Resource" => {
                    // println!("  SESSION LINK: {}", resource.resource_ref.lat_local_uri.display());
                    // println!("  SESSION LINK: {:?}", resource.resource_ref.lat_local_uri);
                    // print!("{}", resource.resource_ref.lat_local_uri.as_deref().unwrap_or("None"));

                    let r = resource.resource_ref.lat_local_uri.as_deref().unwrap_or("None");
                    files.push(r.to_owned());
                }
                "Metadata" => {
                    if let Some(lat_local_uri) = &resource.resource_ref.lat_local_uri {
                        // Print next CMD to parse
                        // println!("  NEXT: {}", lat_local_uri);
                        match Self::traverse(&Path::new(lat_local_uri), base_dir.as_deref()) {
                            Ok(_) => (),
                            Err(err) => {
                                println!("ERROR FOR '{}': {err}", node.display());
                                // return Err(err)
                                // errors.push((node.to_owned(), err.to_string()))
                            }
                        };
                    }
                }
                _ => ()
            }
        }
        println!("{}", headers.join("\t"));
        println!("{}\t{}\t{}\t{}\t{}",
            node.display(),
            cmd.date().as_deref().unwrap_or("Unspecified"),
            cmd.description().first().map(|d| d.value.as_str()).unwrap_or("Unspecified"),
            actors.join("\t"),
            files.join(";")
        );

        // Ok(errors)
        // Ok(missing)
        Ok(())
    }
}
