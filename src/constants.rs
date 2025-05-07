/// Constants for glTF component types
pub mod component_type {
    pub const BYTE: usize = 5120;
    pub const UNSIGNED_BYTE: usize = 5121;
    pub const SHORT: usize = 5122;
    pub const UNSIGNED_SHORT: usize = 5123;
    pub const UNSIGNED_INT: usize = 5125;
    pub const FLOAT: usize = 5126;
}

/// Primitive types (for accessor.type field)
pub mod accessor_type {
    pub const SCALAR: &str = "SCALAR";
    pub const VEC2: &str = "VEC2";
    pub const VEC3: &str = "VEC3";
    pub const VEC4: &str = "VEC4";
    pub const MAT2: &str = "MAT2";
    pub const MAT3: &str = "MAT3";
    pub const MAT4: &str = "MAT4";
}

/// Target values for bufferView.target
pub mod buffer_view_target {
    pub const ARRAY_BUFFER: usize = 34962;
    pub const ELEMENT_ARRAY_BUFFER: usize = 34963;
}

/// Sampler filter values
pub mod sampler_filter {
    pub const NEAREST: usize = 9728;
    pub const LINEAR: usize = 9729;
    pub const NEAREST_MIPMAP_NEAREST: usize = 9984;
    pub const LINEAR_MIPMAP_NEAREST: usize = 9985;
    pub const NEAREST_MIPMAP_LINEAR: usize = 9986;
    pub const LINEAR_MIPMAP_LINEAR: usize = 9987;
}

/// Sampler wrap values
pub mod sampler_wrap {
    pub const REPEAT: usize = 10497;
    pub const CLAMP_TO_EDGE: usize = 33071;
    pub const MIRRORED_REPEAT: usize = 33648;
}

/// Alpha mode values
pub mod alpha_mode {
    pub const OPAQUE: &str = "OPAQUE";
    pub const MASK: &str = "MASK";
    pub const BLEND: &str = "BLEND";
}

/// Primitive mode values
pub mod primitive_mode {
    pub const POINTS: usize = 0;
    pub const LINES: usize = 1;
    pub const LINE_LOOP: usize = 2;
    pub const LINE_STRIP: usize = 3;
    pub const TRIANGLES: usize = 4;
    pub const TRIANGLE_STRIP: usize = 5;
    pub const TRIANGLE_FAN: usize = 6;
}
