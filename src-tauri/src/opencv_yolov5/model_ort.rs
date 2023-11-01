use std::sync::Arc;

use ndarray::CowArray;
use ort::{
    tensor::ndarray_tensor, Environment, ExecutionProvider, GraphOptimizationLevel, Session,
    SessionBuilder, Value,
};

use super::YoloImageDetections;

pub struct Model {
    environment: Arc<Environment>,
    session: Session,
}

impl Model {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let environment = Environment::builder()
            .with_name("CamTrap Detector")
            .with_execution_providers([
                ExecutionProvider::CUDA(Default::default()),
                ExecutionProvider::CoreML(Default::default()),
            ])
            .build()?
            .into_arc();

        let session = SessionBuilder::new(&environment)?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .with_model_from_file("../md_v5a.0.0-dynamic.onnx")?;

        if let Ok(metadata) = session.metadata() {
            println!();
            println!("MODEL METADATA");
            if let Ok(name) = metadata.name() {
                println!("name = {:?}", name);
            }
            if let Ok(description) = metadata.description() {
                println!("description = {:?}", description);
            }
            if let Ok(producer) = metadata.producer() {
                println!("producer = {:?}", producer);
            }
            if let Ok(version) = metadata.version() {
                println!("version = {:?}", version);
            }
        }

        println!();
        println!("INPUTS = {:?}", session.inputs);

        println!();
        println!("OUTPUTS = {:?}", session.outputs);

        let cuda_enabled = ExecutionProvider::CUDA(Default::default()).is_available();
        println!("cuda_enabled = {:?}", cuda_enabled);

        // let array = ndarray::Array::from_shape_vec(
        //     (1, 3, 1280, 1280),
        //     vec![0.0; 1 * 3 * 1280 * 1280],
        // ).unwrap().into_dyn();

        // let array: ndarray::ArrayBase<ndarray::CowRepr<'_, f32>, ndarray::Dim<ndarray::IxDynImpl>> =
        //     CowArray::from(ndarray::Array::from_shape_vec(
        //         (1, 3, 640, 640),
        //         vec![0.0; 1 * 3 * 640 * 640],
        //     )?)
        //     .into_dyn();

        // // warm up model
        // let inputs = Value::from_array(
        //     session.allocator(),
        //     &array, // Pass the CowRepr array reference here
        // )
        // .unwrap();
        // let outputs = session.run(vec![inputs]);

        // let start_time = std::time::Instant::now();
        // let mut i = 0;
        // for x in 1..1000 {
        //     println!("x = {}", x);
        //     let inputs = Value::from_array(
        //         session.allocator(),
        //         &array, // Pass the CowRepr array reference here
        //     )
        //     .unwrap();
        //     let outputs = session.run(vec![inputs]);
        //     // println!("OUTPUTS = {:?}", outputs.unwrap());
        //     i += 1;
        // }
        // let end_time = std::time::Instant::now();

        // println!("total time = {:?}", end_time - start_time);
        // println!("average time = {:?}", (end_time - start_time) / i);
        // println!(
        //     "images per second = {:?}",
        //     i as f64 / (end_time - start_time).as_secs_f64()
        // );

        // panic!();

        Ok(Self {
            environment,
            session,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_model() {
        let model = Model::new().unwrap();
    }
}

pub struct YoloModel {
    model: Model,
}

impl YoloModel {
    pub fn new_from_file(
        path: &str,
        input_size: (usize, usize),
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!("Warning: YoloModel::new_from_file is not implemented");
        let model = Model::new()?;

        Ok(Self { model })
    }

    pub fn detect(
        &mut self,
        image_path: &str,
        confidence_threshold: f32,
        iou_threshold: f32,
    ) -> Result<YoloImageDetections, Box<dyn std::error::Error>> {
        println!("Warning: YoloModel::detect is not implemented");
        let image = image::open(image_path)?;

        let array: ndarray::ArrayBase<ndarray::CowRepr<'_, f32>, ndarray::Dim<ndarray::IxDynImpl>> =
            CowArray::from(ndarray::Array::from_shape_vec(
                (1, 3, 640, 640),
                vec![0.0; 1 * 3 * 640 * 640],
            )?)
            .into_dyn();


        let inputs = Value::from_array(
            self.model.session.allocator(),
            &array, // Pass the CowRepr array reference here
        )
        .unwrap();
        let outputs = self.model.session.run(vec![inputs]);

        let detections = YoloImageDetections {
            file: "mock".to_string(),
            image_width: 111,
            image_height: 111,
            detections: vec![],
        };

        Ok(detections)
    }
}
