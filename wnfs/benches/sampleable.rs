use proptest::{
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};

pub trait Sampleable {
    type Value;

    fn sample(&self, runner: &mut TestRunner) -> Self::Value;
}

impl<V, S> Sampleable for S
where
    S: Strategy<Value = V>,
{
    type Value = V;

    fn sample(&self, runner: &mut TestRunner) -> Self::Value {
        self.new_tree(runner)
            .expect("Couldn't generate test value")
            .current()
    }
}
