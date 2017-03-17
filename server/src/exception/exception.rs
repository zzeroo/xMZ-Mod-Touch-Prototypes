pub enum ExceptionType {
    WartungsIntervall,
    Kabelbruch,
    SensorMaxDirectValue,
    SensorMaxAverage1,
    SensorMaxAverage2,
}

pub struct Exception {
    exception_type: ExceptionType,
}

impl Exception {
    pub fn new(exception_type: ExceptionType) -> Self {
        Exception {
            exception_type: exception_type,
        }
    }

    pub fn actions(&self) {

    }
}
