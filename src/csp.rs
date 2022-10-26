use std::collections::*;
use std::{fmt, hash::Hash};

pub trait Constraint<V, D> {
    fn variables(&self) -> Vec<V>;
    // to be implemented by the problem at-hand, e.g., Australian map colouring problem
    fn is_satisfied(&self, assignment: &HashMap<V, D>) -> bool;
}

pub struct CSP<V, D, C>
where
    C: Constraint<V, D>,
{
    pub variables: Vec<V>,
    pub domains: HashMap<V, Vec<D>>,
    pub constraints: HashMap<V, Vec<C>>,
}

impl<V, D, C> CSP<V, D, C>
where
    V: Clone + fmt::Debug + Hash + Ord,
    D: Clone + fmt::Debug,
    C: Clone + Constraint<V, D>,
{
    pub fn new(variables: Vec<V>, domains: HashMap<V, Vec<D>>) -> Result<Self, &'static str> {
        let mut constraints = HashMap::<V, Vec<C>>::new();

        for variable in &variables {
            constraints.insert(variable.clone(), Vec::new());

            if !domains.contains_key(variable) {
                return Err("Every variable should have a doamin assigned to it.");
            }
        }

        Ok(Self {
            variables,
            domains,
            constraints,
        })
    }

    pub fn add_constraint(&mut self, constraint: C) -> Result<(), &'static str> {
        for variable in constraint.variables() {
            if !self.variables.contains(&variable) {
                return Err("Variable in constraint is not in CSP");
            }

            if let Some(constraints) = self.constraints.get_mut(&variable) {
                constraints.push(constraint.clone());
            }
        }

        Ok(())
    }

    fn is_consistent(&self, variable: V, assignment: &HashMap<V, D>) -> bool {
        for constraints in self.constraints.get(&variable) {
            for constraint in constraints {
                if !constraint.is_satisfied(&assignment) {
                    return false;
                }
            }
        }

        true
    }

    pub fn backtracking_search_with_assignment(
        &self,
        assignment: HashMap<V, D>,
    ) -> Option<HashMap<V, D>> {
        //println!("{:?}", assignment);

        // Assignment is complete if every variable is assigned (base case)
        if assignment.len() == self.variables.len() {
            return Some(assignment);
        }

        let unassigned = self.variables.iter().find(|v| !assignment.contains_key(v));

        //println!("Unassigned: {:?}", unassigned);

        if let Some(unassigned) = unassigned {
            for domains in self.domains.get(unassigned) {
                for value in domains {
                    let mut local_assignment = assignment.clone();

                    local_assignment.insert(unassigned.clone(), value.clone());

                    if self.is_consistent(unassigned.clone(), &local_assignment) {
                        let result =
                            self.backtracking_search_with_assignment(local_assignment.clone());

                        return result;
                    }
                }
            }
        }

        None
    }

    pub fn backtracking_search(&self) -> Option<HashMap<V, D>> {
        self.backtracking_search_with_assignment(HashMap::new())
    }
}
