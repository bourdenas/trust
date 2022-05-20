use crate::components::{Position, ScriptState, Sprite};
use crate::trust::{action, Action, AnimationScriptAction, SceneNodeAction, Vector};
use sdl2::rect::Point;
use specs::prelude::*;

pub struct ActionExecutor;

impl ActionExecutor {
    pub fn new() -> Self {
        ActionExecutor {}
    }

    pub fn execute(&self, action: Action, world: &mut World) -> Option<u32> {
        match action.action {
            Some(action::Action::Quit(..)) => None,
            Some(action::Action::CreateSceneNode(action)) => self.create_scene_node(action, world),
            Some(action::Action::PlayAnimation(action)) => self.play_animation(action, world),
            _ => None,
        }
    }

    fn create_scene_node(
        &self,
        scene_node_action: SceneNodeAction,
        world: &mut World,
    ) -> Option<u32> {
        if let Some(node) = scene_node_action.scene_node {
            let entity = world
                .create_entity()
                .with(Position(make_point(
                    &node.position.expect("Node missing position"),
                )))
                .with(Sprite {
                    resource: node.sprite_id,
                    frame_index: node.frame_index as usize,
                })
                .build();
            return Some(entity.id());
        }
        None
    }

    fn play_animation(
        &self,
        script_action: AnimationScriptAction,
        world: &mut World,
    ) -> Option<u32> {
        if let Some(script) = script_action.script {
            let entity = world.entities().entity(script_action.scene_node_id);

            let mut scripts = world.write_storage::<ScriptState>();
            if let Err(e) = scripts.insert(entity, ScriptState::new(script)) {
                println!("play_animation: {}", e);
            }
        }
        None
    }
}

fn make_point(vec: &Vector) -> Point {
    Point::new(vec.x as i32, vec.y as i32)
}
