use crate::graph::{Ids, PValue, Writable};

#[derive(Debug, PartialEq, Clone)]
pub struct Bytecode<V, E, P>
where
    V: Writable,
    E: Writable,
    P: Writable,
{
    sources: Vec<Instruction<V, E, P>>,
    steps: Vec<Instruction<V, E, P>>,
}

impl<V, E, P> Default for Bytecode<V, E, P>
where
    V: Writable,
    E: Writable,
    P: Writable,
{
    fn default() -> Self {
        Self {
            sources: vec![],
            steps: vec![],
        }
    }
}

impl<V, E, P> Bytecode<V, E, P>
where
    V: Writable,
    E: Writable,
    P: Writable,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_source(&mut self, i: Instruction<V, E, P>) {
        self.sources.push(i);
    }

    pub fn add_step(&mut self, i: Instruction<V, E, P>) {
        self.steps.push(i);
    }

    pub fn steps(&self) -> &Vec<Instruction<V, E, P>> {
        &self.steps
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vert(pub(crate) Ids);

#[derive(Debug, PartialEq, Clone)]
pub struct Edge(pub(crate) Ids);

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction<V, E, P>
where
    V: Writable,
    E: Writable,
    P: Writable,
{
    Vert(Vert),
    Edge(Edge),
    AddV(V),
    AddE(E),
    Property(P, PValue<V, E, P>),
}