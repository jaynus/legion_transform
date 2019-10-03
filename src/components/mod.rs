use crate::math::{Similarity3, Matrix4};

pub type LocalTransform = Similarity3<f32>;
pub struct GlobalTransform(Matrix4<f32>);
