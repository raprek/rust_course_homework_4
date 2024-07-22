pub const VEC3_LEN: usize = 3;
#[derive(Default)]
pub struct Vec3 {
    value: [i32; VEC3_LEN],
}

impl Vec3 {
    pub fn vec3_vector_sum(a: Vec3, b: Vec3) -> Vec3 {
        let mut c = Vec3::default();
        for i in 0..3 {
            c.value[i] = a.value[i] + b.value[i];
        }
        c
    }

    pub fn vec3_scalar_sum(a: Vec3, b: Vec3) -> i32 {
        let mut c = 0;
        for i in 0..VEC3_LEN {
            c += a.value[i] + b.value[i];
        }
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_vector_sum() {
        let ref_test = Vec3::vec3_vector_sum(Vec3 { value: [1, 2, 3] }, Vec3 { value: [3, 2, 1] });
        assert_eq!(ref_test.value[0], 4);
        assert_eq!(ref_test.value[1], 4);
        assert_eq!(ref_test.value[2], 4);
    }

    #[test]
    fn test_vec3_scalar_sum() {
        assert_eq!(
            Vec3::vec3_scalar_sum(Vec3 { value: [1, 2, 3] }, Vec3 { value: [3, 2, 1] }),
            12
        )
    }

    #[test]
    fn test_default_vec3() {
        assert_eq!(Vec3::default().value, [0; 3]);
    }
}
