trait Task {
    type Input;
    type Output;

    fn run(&self, input: Self::Input);
    fn on_done(&self, output: Result<Self::Output, &str>);
}

trait Node {
    type Input;

    fn visit(&self, input: Self::Input);
}

struct TaskNode<I, O> {
    task: Box<dyn Task<Input = I, Output = O>>,
    next_nodes: Vec<Box<dyn Node<Input = I>>>,
}

impl<I, O> TaskNode<I, O> {
    fn new(task: Box<dyn Task<Input=I, Output = O>>) -> Self {
        TaskNode {
            task,
            next_nodes: vec![],
        }
    }

    fn add_next_node(&mut self, node: Box<dyn Node<Input = I>>) {
        self.next_nodes.push(node)
    }
}

impl<I, O> Node for TaskNode<I, O> {
    type Input = I;

    fn visit(&self, input: I) {
    }
}

struct CompositeNode<I, O> {
    task: Box<dyn Task<Input = I, Output = O>>,
    next_nodes: Vec<Box<dyn Node<Input = I>>>,
    next_sub_nodes: Vec<Box<dyn Node<Input = I>>>,
}

impl<I, O> CompositeNode<I, O> {
    fn new(task: Box<dyn Task<Input=I, Output=O>>) -> Self {
        CompositeNode {
            task,
            next_nodes: vec![],
            next_sub_nodes: vec![],
        }
    }

    fn add_next_node(&mut self, node: Box<dyn Node<Input = I>>) {
        self.next_nodes.push(node)
    }
    fn add_next_sub_node(&mut self, node: Box<dyn Node<Input = I>>) {
        self.next_sub_nodes.push(node)
    }
}

impl<I, O> Node for CompositeNode<I, O> {
    type Input = I;

    fn visit(&self, input: I) {
    }
}

struct Pipeline {

}
