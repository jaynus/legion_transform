use crate::math::{Matrix4, Similarity3};

// A local transform is anything that can be converted into a Similarity3.
pub type LocalTransform = Similarity3<f32>;

// A global transform is just a Matrix4 and is updated by the TransformSystem.
pub type GlobalTransform = Matrix4<f32>;
