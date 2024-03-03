//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    super::resource::{
        file_type, stack, Chunk, FileInfoChunk, FromStream, ResourceStack, ResourceTag,
    },
    crate::support::{self, Error},
    byteorder::ReadBytesExt,
    fehler::{throw, throws},
    id_tree::*,
    std::{
        fs::File,
        io::{BufRead, BufReader},
    },
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

#[derive(Default)]
enum ActorData {
    #[default]
    None,
    Light(),
    Camera(),
    Bounds(),
    Plane(),
}

#[derive(Default)]
pub struct Actor {
    transform: (),
    materials: (),
    data: ActorData,
}

impl Actor {
    pub fn load_many<P: AsRef<std::path::Path>>(filename: P) -> Result<Actor, Error> {
        let mut file = BufReader::new(File::open(filename)?);
        <Self as FromStream>::from_stream(&mut file)
    }
}

impl FromStream for Actor {
    type Output = Actor;

    #[throws]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut stack = ResourceStack::default();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::FileInfo(FileInfoChunk { file_type, .. }) => {
                    if file_type != file_type::ACTOR {
                        throw!(support::Error::InvalidResourceType {
                            expected: file_type::ACTOR,
                            received: file_type,
                        });
                    }
                }

                Chunk::Actor(actor) => {
                    stack.push(stack::ACTOR, ResourceTag::Actor(Box::new(actor)));
                }
                Chunk::ActorModel(model) => {
                    let mut actor = stack.top(stack::ACTOR);
                    actor.model = Models::find(model.identifier);
                }
                Chunk::ActorTransform(_) => {
                    let transform = stack.pop(stack::TRANSFORM);
                    let mut actor = stack.top(stack::ACTOR);
                    actor.transform = transform;
                }
                Chunk::ActorMaterial(material) => {
                    let mut actor = stack.top(stack::ACTOR);
                    actor.material = Materials::find(material.identifier);
                }
                Chunk::ActorLight(_) => {
                    let light: Result<ResourceTag, Error> = stack.pop(stack::LIGHT);
                    actor.data = ActorData::Light(light);
                }
                Chunk::ActorCamera(_) => {
                    let camera: Result<ResourceTag, Error> = stack.pop(stack::CAMERA);
                    actor.data = ActorData::Camera(camera);
                }
                Chunk::ActorBounds(_) => {
                    let bounds = stack.pop(stack::BOUNDS);
                    let mut actor = stack.top(stack::ACTOR);
                    actor.data = ActorData::Bounds(bounds);
                }
                Chunk::ActorClipPlane(_) => {
                    let plane = stack.pop(stack::PLANE);
                    let mut actor = stack.top(stack::ACTOR);
                    actor.data = ActorData::ClipPlane(plane);
                }
                Chunk::ActorAddChild(_) => {
                    let child = stack.pop(stack::ACTOR);
                    let mut actor = stack.top(stack::ACTOR);
                    actor.chidren.push(child);
                }

                Chunk::TransformMatrix34(_) => {
                    stack.push(stack::TRANSFORM, transform);
                }
                Chunk::TransformMatrix34LP(_) => {
                    stack.push(stack::TRANSFORM, transform);
                }
                Chunk::TransformQuat(_) => {
                    stack.push(stack::TRANSFORM, transform);
                }
                Chunk::TransformEuler(_) => {
                    stack.push(stack::TRANSFORM, transform);
                }
                Chunk::TransformLookUp(_) => {
                    stack.push(stack::TRANSFORM, transform);
                }
                Chunk::TransformTranslation(_) => {
                    stack.push(stack::TRANSFORM, transform);
                }
                Chunk::TransformIdentity() => {
                    stack.push(stack::TRANSFORM, transform);
                }

                Chunk::Bounds(_) => {
                    stack.push(stack::BOUNDS, bounds);
                }
                Chunk::Light(_) => {
                    stack.push(stack::LIGHT, light);
                }
                Chunk::Camera(_) => {
                    stack.push(stack::CAMERA, camera);
                }
                Chunk::Plane(_) => {
                    stack.push(stack::PLANE, plane);
                }

                _ => unimplemented!(), // unexpected type here
            }
        }

        actor
    }
}
