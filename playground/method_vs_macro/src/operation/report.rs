pub struct EventPublisher {
    pub name: String,
    pub topic: String,
}
impl EventPublisher {
    pub fn new(name: String, topic: String) -> Self {
        Self { name, topic }
    }
    pub fn send(&self) {
        println!("Event sent by {}", self.name);
    }
}

pub struct ProcessOptions<'a> {
    pub topic: &'a str,
}
pub trait Operate {
    fn operation_name(&self) -> &str;
    fn process(&self, options: &ProcessOptions);
}

pub struct ReportPublisher<'a> {
    pub provider: Option<&'a EventPublisher>,
}
impl<'a> ReportPublisher<'a> {
    pub fn new(provider: &'a EventPublisher) -> Self {
        Self { provider: Some(provider) }
    }
}
pub struct Onboarding {
    pub name: String,
    pub config: ProcessOptions<'static>,
}
impl Operate for Onboarding {
    fn operation_name(&self) -> &str {
        "onboarding"
    }
    fn process(&self, options: &ProcessOptions) {
        println!("Processing {} with topic {}", self.operation_name(), options.topic);
    }
}
