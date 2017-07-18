use super::*;

#[derive(Default)]
struct TestHandler {
    data: Vec<i32>,
    expected: Vec<i32>,
}

impl Handler<i32> for TestHandler {
    fn start(&mut self, sender: Sender<i32>) {
        for elem in &self.expected {
            sender.send(elem.clone()).unwrap();
        }
    }
    fn handle(&mut self, i: i32) -> bool {
        self.data.push(i);
        true
    }
    fn end(self) {
        assert_eq!(self.data, self.expected);
    }
}

#[test]
fn run_empty() {
    run(TestHandler {
        expected: vec![],
        ..Default::default()
    });
}

#[test]
fn run_with_data() {
    run(TestHandler {
        expected: vec![1, 2, 3],
        ..Default::default()
    });
}
