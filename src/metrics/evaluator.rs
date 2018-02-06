

pub trait Evaluator {
    fn evaluate(&self, test_data: Vec<f64>, reco_data: Vec<f64>) -> f64;
}

pub enum Measures {
    MSE,
    RMSE,
    MAP,
    Precision,
    Recall,
}

impl Evaluator for Measures {
    fn evaluate(&self, test_data: Vec<f64>, reco_data: Vec<f64>) -> f64 {
        match *self {
            Measures::MSE => 0.0,
            Measures::RMSE => 0.0,
            Measures::MAP => 0.0,
            Measures::Precision => 0.0,
            Measures::Recall => 0.0,            
        }
    }
}
