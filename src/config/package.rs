use serde::{Deserialize};

use crate::download::{ git, remote };
use crate::config::Config;
use super::github::Github;
use crate::tree;

use std::sync::Arc;
use crate::error::Error;


const DEFAULT_OUTPUT_DIR: &str = "repositories";
const DEFAULT_FILENAME: &str = "no_name_provided";



#[derive(Clone, Debug, Deserialize)]
pub struct Package {
    github: Option<Github>,
    remote: Option<String>,

    directory: Option<String>,
    filename: Option<String>,

    exec: Option<Vec<String>>,


    #[serde(skip_deserializing)]
    config: Arc<Config>,

    #[serde(skip_deserializing)]
    pub name: String
}



impl Package {
    pub fn give(&mut self, name: String, config: Arc<Config>) {
        self.name = name;
        self.config = config;
    }


    pub fn build(&self, fresh: bool) -> Result<(), Error> {
        let output_dir = &self.directory();

        if fresh {
            tree::remove_dir(output_dir);
        }
        tree::create_dir(output_dir);


        if let Some(repo) = &self.github {
            return git::clone(&repo.url(), output_dir);
        }


        let output_file = &self.filename();

        if let Some(url) = &self.remote {
            return remote::get(url, output_dir, output_file);
        }

        Ok(())
    }


    pub fn exec(&self) -> Result<(), Error> {
        let names = match self.exec.as_ref() {
            Some(vec) => vec,
            None => return Ok(())
        };

        let mut scripts = match self.config.scripts.clone() {
            Some(map) => map,
            None => return Err(Error::NoScripts)
        };

        for name in names {
            let script = &mut scripts[name];

            script.give(name);
            script.exec(self);
        }

        Ok(())
    }


    pub fn directory(&self) -> String {
        match &self.directory {
            Some(dir) => dir.clone(),
            _ => format!("{}/{}", DEFAULT_OUTPUT_DIR, self.name)
        }
    }


    pub fn filename(&self) -> String {
        let file: &str = match &self.filename {
            Some(file) => file,
            _ => DEFAULT_FILENAME
        };

        file.to_string()
    }


    pub fn full_path(&self) -> String {
        format!("{}/{}", tree::current_dir(), self.directory())
    }
}