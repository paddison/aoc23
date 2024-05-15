//! Utility functions/traits for the challenges
//!

/// Rotates 90 degrees clockwise, assumes grid is a rectangle
pub trait Rotate {
    fn rotate(&self) -> Self;
}

impl<T: Clone + Copy> Rotate for Vec<Vec<T>> {
    fn rotate(&self) -> Self {
        let width = self.first().map(|l| l.len()).unwrap_or(0);
        assert!(self.iter().all(|l| l.len() == width));
        let height = self.len();
        let mut rotated = Vec::new();

        for col in 0..width {
            let mut new_row = Vec::new();
            for row in (0..height).rev() {
                new_row.push(self[row][col]);
            }
            rotated.push(new_row);
        }

        rotated
    }
}
