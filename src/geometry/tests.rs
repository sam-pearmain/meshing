#[cfg(test)]
mod tests {
    use crate::geometry::vertex::Vertex;

    #[test]
    fn test_vertex_creation() {
        let vertex = Vertex {
            id: 1,
            x: 0.0,
            y: 0.0,
        };

        assert_eq!(vertex.id, 1);
        assert_eq!(vertex.x, 0.0);
        assert_eq!(vertex.y, 0.0);
    }

    #[test]
    fn add_f32_f64() {
        let float1: f32 = 1.0;
        let float2: f64 = 2.0;

        // floats have to be of the same type to implement add
        let result = float1 as f64 + float2;
        assert_eq!(result, 3.0);
    }
}