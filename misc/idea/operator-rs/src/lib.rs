/// A minimal operator trait.
pub trait Operator {
    fn forward(&self, x: f32) -> f32;
}

/// A simple linear operator: y = weight * x + bias.
#[derive(Debug, Clone, Copy)]
pub struct LinearOperator {
    pub weight: f32,
    pub bias: f32,
}

impl LinearOperator {
    pub fn new(weight: f32, bias: f32) -> Self {
        Self { weight, bias }
    }
}

impl Operator for LinearOperator {
    fn forward(&self, x: f32) -> f32 {
        self.weight * x + self.bias
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear_operator_computes_expected_value() {
        let op = LinearOperator::new(2.0, 1.0);
        let y = op.forward(3.0);
        assert_eq!(y, 7.0);
    }

    #[test]
    fn linear_operator_supports_negative_input() {
        let op = LinearOperator::new(0.5, -1.0);
        let y = op.forward(-4.0);
        assert_eq!(y, -3.0);
    }

    #[test]
    fn operator_trait_can_be_used_polymorphically() {
        let op: Box<dyn Operator> = Box::new(LinearOperator::new(3.0, 0.0));
        assert_eq!(op.forward(2.0), 6.0);
    }
}
