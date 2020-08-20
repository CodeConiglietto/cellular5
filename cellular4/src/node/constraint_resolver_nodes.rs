pub enum SNFloatNormaliserNodes{
    Random,
    Constant {value: SNFloatNormaliser},
    IfElse{child_predicate: BooleanNodes, child_a: SNFloatNormaliser, child_b: SNFloatNormaliser},
}

impl Node for SNFloatNormaliserNodes {
    type Output = SNFloatNormaliser;

    fn compute(&self, mut compute_arg: ComArg) -> Self::Output {
        use SNFloatNormaliserNodes::*;

        match self {
            Random => SNFloatNormaliser::generate(),
            Constant {value} => value,
            IFElse{value} => {
                if child_predicate.compute(compute_arg).into_inner()
                {
                    child_a
                }else{
                    child_b
                }
            },
        }
    }
}