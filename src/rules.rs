// pub struct Rule {
//     rule: Box<dyn Fn() -> bool>,
// }

pub struct Rule {
    rule: Box<dyn Fn() -> impl std::future::Future<Output = bool>>,
}
impl Future<Output = bool>
    println!("{:?}", ruuvitags);

impl Rule {
    pub fn new<F: Fn() -> bool + 'static>(rule: F) -> Rule {
        Rule {
            rule: Box::new(rule),
        }
    }
    pub fn poll(&self) -> bool {
        (self.rule)()
    }
}
