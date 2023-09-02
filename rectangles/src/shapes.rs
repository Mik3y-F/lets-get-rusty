#[derive(Debug, PartialEq)]
pub struct Rectangles {
    width: u32,
    height: u32,
}

impl Rectangles {
    pub fn new(width: u32, height: u32) -> Result<Self, DimensionError> {
        if width == 0 {
            return Err(DimensionError::WidthLessThanZero);
        }
        if height == 0 {
            return Err(DimensionError::HeightLessThanZero);
        }
        Ok(Self { width, height })
    }

    pub fn square(size: u32) -> Self {
        Self::new(size, size).expect("failed to create square")
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn can_hold(&self, other_rectangle: &Rectangles) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DimensionError {
    WidthLessThanZero,
    HeightLessThanZero,
}

impl std::fmt::Display for DimensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DimensionError::WidthLessThanZero => write!(f, "Width should be greater than zero."),
            DimensionError::HeightLessThanZero => write!(f, "Height should be greater than zero."),
        }
    }
}

impl std::error::Error for DimensionError {}

#[cfg(test)]
mod rectangle_new_tests {
    use super::*;

    #[test]
    fn test_new() {
        let rect1 = Rectangles::new(30, 50).expect("Invalid dimensions");
        assert_eq!(rect1.width, 30);
        assert_eq!(rect1.height, 50);
    }

    #[test]
    fn test_new_with_zero_width() {
        let rect1 = Rectangles::new(0, 50);
        assert_eq!(rect1, Err(DimensionError::WidthLessThanZero));
    }

    #[test]
    fn test_new_with_zero_height() {
        let rect1 = Rectangles::new(30, 0);
        assert_eq!(rect1, Err(DimensionError::HeightLessThanZero));
    }
}

#[cfg(test)]
mod rectangle_area_tests {
    use super::*;

    #[test]
    fn test_area() {
        let rect1 = Rectangles {
            width: 30,
            height: 50,
        };
        assert_eq!(rect1.area(), 1500);
    }
}

#[cfg(test)]
mod rectangle_can_hold_test {
    use super::*;

    #[test]
    fn test_can_hold_smaller_rectangle() {
        let rect1 = Rectangles {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangles {
            width: 10,
            height: 20,
        };

        assert_eq!(rect1.can_hold(&rect2), true);
    }

    #[test]
    fn test_can_hold_bigger_rectangle() {
        let rect1 = Rectangles {
            width: 30,
            height: 50,
        };

        let rect2 = Rectangles {
            width: 10,
            height: 20,
        };

        assert_eq!(rect2.can_hold(&rect1), false);
    }
}
