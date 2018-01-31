

pub trait Evaluator {
    fn evaluate(test_data: Vec<f64>, reco_data: Vec<f64>) -> f64;
}

pub enum Measures {
    MSE(MSE),
    RMSE(RMSE),
    MAP(MAP),
    Precision(Precision),
    Recall(Recall),
    NDCG(NDCG),
    AUC(AUC),
}

impl Evaluator for Measures {
    
}

pub struct MSE {

}

pub struct RMSE {

}

pub struct MAP {

}

pub struct Precision {

}

pub struct Recall {

}

pub struct NDCG {

}

pub struct AUC {

}
