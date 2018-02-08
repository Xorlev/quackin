#[derive(Debug)]
pub enum Evaluator {
    MSE,
    RMSE,
    MAP,
    Precision,
    Recall,
}

impl Evaluator {
    pub fn evaluate(self, test_data: Vec<f64>, reco_data: Vec<f64>) -> f64 {
        match self {
            Evaluator::MSE => {
                0.0
            },
            Evaluator::RMSE => {
                0.0
            },
            Evaluator::MAP => {
                0.0
            },
            Evaluator::Precision => {
                0.0
            },
            Evaluator::Recall => {
                0.0
            },
        }
    }
 }