use onnxruntime::{
    environment::Environment, ndarray::Array, tensor::OrtOwnedTensor, GraphOptimizationLevel,
    LoggingLevel,
};
// use tracing::Level;
// use tracing_subscriber::FmtSubscriber;

type Error = Box<dyn std::error::Error>;

pub fn run_model() -> Result<(), Error> {
    // Setup the example's log level.
    // NOTE: ONNX Runtime's log level is controlled separately when building the environment.
    // let subscriber = FmtSubscriber::builder()
    //     .with_max_level(Level::TRACE)
    //     .finish();

    // tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let environment = Environment::builder()
        .with_name("test")
        // The ONNX Runtime's log level can be different than the one of the wrapper crate or the application.
        .with_log_level(LoggingLevel::Info)
        .build()?;

    let mut session = environment
        .new_session_builder()?
        .with_optimization_level(GraphOptimizationLevel::Basic)?
        .with_number_threads(1)?
        // NOTE: The example uses SqueezeNet 1.0 (ONNX version: 1.3, Opset version: 8),
        //       _not_ SqueezeNet 1.1 as downloaded by '.with_model_downloaded(ImageClassification::SqueezeNet)'
        //       Obtain it with:
        //          curl -LO "https://github.com/onnx/models/raw/master/vision/classification/squeezenet/model/squeezenet1.0-8.onnx"
        .with_model_from_file("/Users/ben/Projects/camtrap-detector/md_v5a.0.0.onnx")?;

    for i in 0..100 {
        let input0_shape: Vec<usize> = session.inputs[0].dimensions().map(|d| d.unwrap()).collect();
        let output0_shape: Vec<usize> = session.outputs[0]
            .dimensions()
            .map(|d| d.unwrap())
            .collect();

        assert_eq!(input0_shape, [1, 3, 640, 640]);
        assert_eq!(output0_shape, [1, 25500, 8]);

        // initialize input data with values in [0.0, 1.0]
        let n: u32 = session.inputs[0]
            .dimensions
            .iter()
            .map(|d| d.unwrap())
            .product();
        let array = Array::linspace(0.0_f32, 1.0, n as usize)
            .into_shape(input0_shape)
            .unwrap();
        let input_tensor_values = vec![array];

        let outputs: Vec<OrtOwnedTensor<f32, _>> = session.run(input_tensor_values)?;

        assert_eq!(outputs[0].shape(), output0_shape.as_slice());
        for i in 0..5 {
            println!("Score for class [{}] =  {}", i, outputs[0][[0, i, 0]]);
        }
    }

    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run_model();
    }
}
