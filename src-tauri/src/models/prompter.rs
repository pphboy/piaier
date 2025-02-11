use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};

use super::mpstep;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "prompter")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: String,
    pub title: String,
    pub content: String,
    pub itype: String,

    pub model_name: String,
    pub ptype: String,

    #[sea_orm(ignore)]
    pub steps: Vec<mpstep::Model>,
    pub nodes: String,
    pub edges: String,
}

impl Model {
    pub fn get_nodes(&self) -> Vec<Node> {
        serde_json::from_str(&self.nodes).unwrap()
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        serde_json::from_str(&self.edges).unwrap()
    }

    pub fn set_nodes(&mut self, nodes: Vec<Node>) {
        self.nodes = serde_json::to_string(&nodes).unwrap();
    }

    pub fn set_edges(&mut self, edges: Vec<Edge>) {
        self.edges = serde_json::to_string(&edges).unwrap();
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Edge {
    pub id: String,
    pub r#type: String,
    pub source: String,
    pub target: String,
    pub source_x: Option<f64>,
    pub source_y: Option<f64>,
    pub target_x: Option<f64>,
    pub target_y: Option<f64>,
    pub updatable: Option<bool>,
    pub data: Option<EdgesData>,
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgesData {
    pub stepid: i32,
    pub prompter_uuid: String,
    pub node_name: String,
    pub cond_var: String,
    pub stype: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub r#type: String,
    pub initialized: bool,
    pub position: NodePosition,
    pub data: NodeData,
    pub class: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodePosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeData {
    pub label: String,
    pub data: Option<EdgesData>,
}
