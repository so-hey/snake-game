use burn::{
    config::Config,
    module::Module,
    prelude::Backend,
    record::{CompactRecorder, Recorder},
    tensor::{cast::ToElement, Tensor},
};

use crate::training::TrainingConfig;

pub fn infer<B: Backend>(artifact_dir: &str, device: B::Device, item: Tensor<B, 3>) -> u8 {
    let config = TrainingConfig::load(format!("{artifact_dir}/{artifact_dir}.config.json"))
        .expect("Config should exist for the model");
    let record = CompactRecorder::new()
        .load(format!("{artifact_dir}/model").into(), &device)
        .expect("Trained model should exist");

    let model = config.model.init::<B>(&device).load_record(record);

    let output = model.forward(item);
    let predicted = output.argmax(1).flatten::<1>(0, 1).into_scalar();

    predicted.to_u8()
}
