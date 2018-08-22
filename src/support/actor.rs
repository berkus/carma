//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use byteorder::ReadBytesExt;
use crate::support::{self, resource::Chunk, Error};
use id_tree::*;
use log::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

// Typical actor tree:
// Root
// +--Actor(NAME.ACT)
//    +--Transform()
//    +--MeshfileRef
//    +--Actor(SUBACT.ACT)
//       +--Transform()
//       +--MeshfileRef
//
#[derive(Debug)]
pub enum ActorNode {
    Root,
    Actor { name: String, visible: bool },
    // First 3x3 is scale? or maybe SQT?
    // Last 3 is translate, -x is to the left, -z is to the front
    Transform([f32; 12]),
    MeshfileRef(String),
    MaterialRef(String),
}

pub struct Actor {
    tree: Tree<ActorNode>,
    root_id: NodeId,
}

impl Actor {
    pub fn new(tree: Tree<ActorNode>) -> Self {
        let mut tree = tree;
        let root_id = tree
            .insert(Node::new(ActorNode::Root), InsertBehavior::AsRoot)
            .unwrap();
        Self {
            tree: tree,
            root_id: root_id,
        }
    }

    pub fn traverse(&self) -> PreOrderTraversal<ActorNode> {
        self.tree.traverse_pre_order(&self.root_id).unwrap()
    }

    pub fn get_node_id_depth(&self, node: &NodeId) -> usize {
        let mut depth = 1;
        for _ in self.tree.ancestors(node).unwrap() {
            depth += 1;
        }
        depth
    }

    pub fn get_node_depth(&self, node: &Node<ActorNode>) -> usize {
        if let Some(parent_id) = node.parent() {
            self.get_node_id_depth(parent_id) + 1
        } else {
            1
        }
    }

    pub fn dump(&self) {
        for node in self.tree.traverse_pre_order(&self.root_id).unwrap() {
            if let Some(parent) = node.parent() {
                print!("  ");
                for _ in self.tree.ancestors(parent).unwrap() {
                    print!("  ");
                }
            }
            println!("{:?}", node.data());
        }
    }

    pub fn dump_actor_points(&self) {
        for node in self.tree.traverse_pre_order(&self.root_id).unwrap() {
            if let &ActorNode::Root = node.data() {
                println!("{:?}", node.data());
            }
            if let &ActorNode::Actor {
                name: _,
                visible: _,
            } = node.data()
            {
                if let Some(parent) = node.parent() {
                    print!("  ");
                    for _ in self.tree.ancestors(parent).unwrap() {
                        print!("  ");
                    }
                }
                println!("{:?}", node.data());
            }
        }
    }

    pub fn load<R: ReadBytesExt + BufRead>(rdr: &mut R) -> Result<Actor, Error> {
        use id_tree::InsertBehavior::*;

        let mut actor = Actor::new(TreeBuilder::new().with_node_capacity(5).build());

        {
            let mut current_actor = actor.root_id.clone();
            let mut last_actor = current_actor.clone();

            // Read chunks until last chunk is encountered.
            // Certain chunks initialize certain properties.
            loop {
                let c = Chunk::load(rdr)?;
                match c {
                    Chunk::ActorName { name, visible } => {
                        trace!("Actor {} visible {}", name, visible);
                        let child_id: NodeId = actor
                            .tree
                            .insert(
                                Node::new(ActorNode::Actor { name, visible }),
                                UnderNode(&current_actor),
                            ).unwrap();
                        last_actor = child_id.clone();
                    }
                    Chunk::ActorTransform(transform) => {
                        actor
                            .tree
                            .insert(
                                Node::new(ActorNode::Transform(transform)),
                                // Transform is unconditionally attached to the last loaded actor
                                UnderNode(&last_actor),
                            ).unwrap();
                    }
                    Chunk::MaterialRef(name) => {
                        actor
                            .tree
                            .insert(
                                Node::new(ActorNode::MaterialRef(name)),
                                UnderNode(&current_actor),
                            ).unwrap();
                    }
                    Chunk::MeshFileRef(name) => {
                        actor
                            .tree
                            .insert(
                                Node::new(ActorNode::MeshfileRef(name)),
                                UnderNode(&current_actor),
                            ).unwrap();
                    }
                    Chunk::ActorNodeDown() => {
                        current_actor = last_actor.clone();
                    }
                    Chunk::ActorNodeUp() => {
                        let node = actor.tree.get(&current_actor).unwrap();
                        if let Some(parent) = node.parent() {
                            current_actor = parent.clone();
                        }
                    }
                    Chunk::Null() => break,
                    Chunk::FileHeader { file_type } => {
                        if file_type != support::ACTOR_FILE_TYPE {
                            panic!("Invalid model file type {}", file_type);
                        }
                    }
                    _ => unimplemented!(), // unexpected type here
                }
            }
        }

        Ok(actor)
    }

    pub fn load_from(fname: String) -> Result<Actor, Error> {
        let file = File::open(fname)?;
        let mut file = BufReader::new(file);
        let m = Actor::load(&mut file)?;
        Ok(m)
    }
}
