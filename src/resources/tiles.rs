use crate::core::Status;

use super::{ResourceLoader, ResourceManager};

pub type TileMapManager = ResourceManager<String, TileMap, TileMapLoader>;

impl TileMapManager {
    pub fn create(resource_path: &str) -> Self {
        TileMapManager::new(resource_path, TileMapLoader {})
    }
}

pub struct TileMapLoader;

impl ResourceLoader<TileMap> for TileMapLoader {
    type Args = str;

    fn load(&self, path: &str, resource: &str) -> Result<TileMap, Status> {
        let filename = format!("{path}/{resource}.json");
        let json = std::fs::read(&filename).expect(&format!("Failed to read '{filename}'"));
        match serde_json::from_slice::<TileMap>(&json) {
            Ok(tilemap) => Ok(tilemap),
            Err(e) => Err(Status::new("Failed to parse tile map: {}", e)),
        }
    }
}

use serde::{Deserialize, Serialize};

/// Tile map representation of Tiled Map Editor (https://www.mapeditor.org/ ).
#[derive(Debug, Serialize, Deserialize)]
pub struct TileMap {
    pub height: usize,
    pub width: usize,
    pub tileheight: usize,
    pub tilewidth: usize,
    pub orientation: String,
    pub infinite: bool,
    pub tilesets: Vec<TileSet>,
    pub layers: Vec<Layer>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileSet {
    pub firstgid: u32,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    #[serde(rename = "type")]
    pub layer_type: String,

    #[serde(default)]
    pub data: Vec<u32>,
    #[serde(default)]
    pub height: usize,
    #[serde(default)]
    pub width: usize,

    #[serde(default)]
    pub objects: Vec<Object>,

    pub id: u32,
    pub name: String,
    pub opacity: f64,
    pub visible: bool,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {}
