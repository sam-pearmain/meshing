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
}