use super::{CHCECK_CONFIG, ConfigSet, MainConfig};
use crate::{
    model::{Behaviour, Composition, Sketch},
    utils::{Checker, ResolvedPath},
};
use anyhow::Result;
use std::{collections::HashMap, path::PathBuf};
use thiserror::Error;

//TODO: We should own everything in this map,
//convert back to normal configset when saving
//everything should be private
#[derive(Debug)]
pub struct ConfigMap<'a> {
    pub path_map: Vec<ResolvedPath>,
    pub main_config: MainConfig,
    pub sketch_map: HashMap<&'a str, (usize, &'a Sketch)>,
    pub comp_map: HashMap<&'a str, &'a Composition>,
}
impl<'a> ConfigMap<'a> {
    pub fn new(base_path: ResolvedPath, config_sets: &'a [ConfigSet]) -> Result<Self> {
        let main_config = MainConfig::new(&config_sets[0].1);

        //---------Check Configs---------
        let check_config = &main_config.check_config;
        CHCECK_CONFIG
            .set(*check_config)
            .map_err(|_| ConfigError::BugCheckConfig)?;
        Checker::working_dir_is_symlink(config_sets[0].0.get_raw())?;
        Checker::is_git_workspace(base_path.get())?;

        //---------Build Config Map---------
        let mut path_map = Vec::new();
        let mut comp_map = HashMap::new();
        let mut sketch_map = HashMap::new();

        for (idx, ConfigSet(config_path, config)) in config_sets.iter().enumerate() {
            //-------create config Path-----------------
            //NOTE: path in the config_sets is didm.toml,so we have to convert it
            let config_path = config_path.to_parent()?;
            path_map.push(config_path);

            for (name, comp) in &config.composition {
                if comp_map.contains_key(name.as_str()) {
                    return Err(ConfigError::DuplicatedComp(name.to_string()).into());
                }
                comp_map.insert(name.as_str(), comp);
            }

            for (name, sketch) in &config.sketch {
                if sketch_map.contains_key(name.as_str()) {
                    return Err(ConfigError::DuplicatedSketch(name.to_string()).into());
                }
                sketch_map.insert(name.as_str(), (idx, sketch));
            }
        }
        if sketch_map.is_empty() {
            return Err(ConfigError::NoSketchFound.into());
        }
        if comp_map.is_empty() {
            return Err(ConfigError::NoCompFound.into());
        }

        //---------return Config Map---------
        Ok(ConfigMap {
            path_map,
            main_config,
            comp_map,
            sketch_map,
        })
    }
    pub fn get_comp(&self, comp_name: &str) -> Result<&Composition> {
        //NOTE: use match to avoid deref of a ref...
        match self.comp_map.get(comp_name) {
            Some(comp) => Ok(comp),
            None => Err(ConfigError::CompNotFound(comp_name.to_string()).into()),
        }
    }
    pub fn get_sketch(&self, sketch_name: &'a str) -> Result<(&Sketch, usize, &'a str)> {
        match self.sketch_map.get(sketch_name) {
            Some((idx, sketch)) => Ok((sketch, *idx, sketch_name)),
            None => Err(ConfigError::SketchNotFound(sketch_name.to_string()).into()),
        }
    }
    pub fn get_sketches(
        &'a self,
        sketch: &'a [String],
    ) -> Result<Vec<(&'a Sketch, usize, &'a str)>> {
        let mut result = Vec::new();
        for sketch_name in sketch {
            let p = self.get_sketch(sketch_name)?;
            result.push(p);
        }
        Ok(result)
    }
    pub fn get_main_behaviour(&self) -> &Behaviour {
        &self.main_config.behaviour
    }
    pub fn get_base_path(&self, idx: usize) -> Result<&ResolvedPath> {
        if idx > self.path_map.len() {
            return Err(
                ConfigError::IndexOutbound(String::from("PathMap"), idx.to_string()).into(),
            );
        }
        Ok(&self.path_map[idx])
    }
    pub fn get_main_base_path(&self) -> Result<&ResolvedPath> {
        self.get_base_path(0)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config file already existed in {0}")]
    ConfigExists(PathBuf),

    #[error("Can't find any composition,check your config,maybe there is a typo?")]
    NoCompFound,

    #[error("Can't find any sketch,check your config,maybe there is a typo?")]
    NoSketchFound,

    #[error("Composition `{0}` is duplicated")]
    DuplicatedComp(String),

    #[error("Sketch `{0}` is duplicated")]
    DuplicatedSketch(String),

    #[error("Composition {0}  not found.")]
    CompNotFound(String),

    #[error("Sketch `{0}` not found.")]
    SketchNotFound(String),

    #[error("Index of `{0}` is out of bound: `{1}`")]
    IndexOutbound(String, String),

    #[error("Check config is set twice,this is a bug,\n please report it with log")]
    BugCheckConfig,
}
