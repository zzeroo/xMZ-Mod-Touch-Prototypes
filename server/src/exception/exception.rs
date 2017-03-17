use action::Action;

pub enum ExceptionType {
    WartungsIntervall,
    Kabelbruch,
    SensorMaxDirectValue,
    SensorMaxAverage1,
    SensorMaxAverage2,
}

pub struct Exception<'a> {
    exception_type: ExceptionType,
    pub actions: Vec<Action<'a>>,
}

impl<'a> Exception<'a> {
    pub fn new(exception_type: ExceptionType) -> Self {
        Exception {
            exception_type: exception_type,
            actions: vec![],
        }
    }

}
