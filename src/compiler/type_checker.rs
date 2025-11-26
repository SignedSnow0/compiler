use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet};

type VariableName = String;
type TypeVariable = String;

struct TypeVariableGenerator {
    counter: usize,
    mappings: HashMap<VariableName, MonomorphicType>,
}

#[derive(Clone, Debug)]
enum MonomorphicType {
    Variable(VariableName),
    FunctionApplication(VariableName, Vec<MonomorphicType>),
}

#[derive(Clone, Debug)]
enum PolymorphicType {
    MonomorphicType(MonomorphicType),
    TypeQuantifier(VariableName, Box<PolymorphicType>),
}

#[derive(Debug)]
struct Context(HashMap<VariableName, PolymorphicType>);

#[derive(Debug)]
struct Substitution {
    mappings: HashMap<VariableName, MonomorphicType>,
}

impl MonomorphicType {
    pub fn unify(&self, other: &MonomorphicType) -> Result<Substitution> {
        match (self, other) {
            (MonomorphicType::Variable(self_name), MonomorphicType::Variable(other_name))
                if self_name == other_name =>
            {
                Ok(Substitution::new(HashMap::new()))
            }
            (MonomorphicType::Variable(self_name), _) => {
                if other.contains(self)? {
                    Err(anyhow!("Infinite type detected during unification"))
                } else {
                    let mut mappings = HashMap::new();
                    mappings.insert(self_name.clone(), other.clone());
                    Ok(Substitution::new(mappings))
                }
            }
            (_, MonomorphicType::Variable(_)) => other.unify(self),
            (
                MonomorphicType::FunctionApplication(self_type, self_args),
                MonomorphicType::FunctionApplication(other_type, other_args),
            ) => {
                if self_type != other_type {
                    Err(anyhow!(
                        "Cannot unify application with different types: {} vs {}",
                        self_type,
                        other_type
                    ))
                } else if self_args.len() != other_args.len() {
                    Err(anyhow!(
                        "Cannot unify application with different argument lengths"
                    ))
                } else {
                    let mut substitution = Substitution::new(HashMap::new());
                    for (a0, a1) in self_args.iter().zip(other_args) {
                        let sub = substitution
                            .apply_mono(a0)
                            .unify(&substitution.apply_mono(a1))?;
                        substitution = substitution.combine(&sub);
                    }

                    Ok(substitution)
                }
            }
        }
    }

    fn contains(&self, other: &MonomorphicType) -> Result<bool> {
        match self {
            MonomorphicType::Variable(var_name) => match other {
                MonomorphicType::Variable(other_name) => Ok(var_name == other_name),
                MonomorphicType::FunctionApplication(_, _) => Err(anyhow::anyhow!(
                    "Passed FunctionApplicaton but expected Variable"
                )),
            },
            MonomorphicType::FunctionApplication(_, args) => {
                Ok(args.iter().any(|arg| arg.contains(other).unwrap_or(false)))
            }
        }
    }
}

impl TypeVariableGenerator {
    pub fn new() -> Self {
        TypeVariableGenerator {
            counter: 0,
            mappings: HashMap::new(),
        }
    }

    pub fn generate(&mut self) -> TypeVariable {
        let var_name = format!("t{}", self.counter);
        self.counter += 1;
        var_name
    }

    pub fn instantiate(&mut self, poly: &PolymorphicType) -> MonomorphicType {
        match poly {
            PolymorphicType::MonomorphicType(mono) => match mono {
                MonomorphicType::Variable(var_name) => {
                    if let Some(replacement) = self.mappings.get(var_name) {
                        replacement.clone()
                    } else {
                        mono.clone()
                    }
                }
                MonomorphicType::FunctionApplication(type_name, args) => {
                    let new_args = args
                        .iter()
                        .map(|arg| self.instantiate(&PolymorphicType::MonomorphicType(arg.clone())))
                        .collect();
                    MonomorphicType::FunctionApplication(type_name.clone(), new_args)
                }
            },
            PolymorphicType::TypeQuantifier(var_name, inner_poly) => {
                let new_type = self.generate();
                self.mappings.insert(
                    var_name.clone(),
                    MonomorphicType::Variable(new_type.clone()),
                );
                self.instantiate(inner_poly)
            }
        }
    }
}

impl Substitution {
    pub fn new(mappings: HashMap<VariableName, MonomorphicType>) -> Self {
        Substitution { mappings }
    }

    pub fn combine(&self, other: &Substitution) -> Substitution {
        let mut new_mappings = self.mappings.clone();
        new_mappings.extend(
            other
                .mappings
                .iter()
                .map(|(k, v)| (k.clone(), self.apply_mono(v))),
        );

        Substitution {
            mappings: new_mappings,
        }
    }

    pub fn apply_mono(&self, item: &MonomorphicType) -> MonomorphicType {
        match item {
            MonomorphicType::Variable(var_name) => {
                if let Some(replacement) = self.mappings.get(var_name) {
                    replacement.clone()
                } else {
                    item.clone()
                }
            }
            MonomorphicType::FunctionApplication(origin_type, args) => {
                let new_args = args.iter().map(|arg| self.apply_mono(arg)).collect();
                MonomorphicType::FunctionApplication(origin_type.clone(), new_args)
            }
        }
    }

    pub fn apply_poly(&self, item: &PolymorphicType) -> PolymorphicType {
        match item {
            PolymorphicType::MonomorphicType(mono) => {
                PolymorphicType::MonomorphicType(self.apply_mono(mono))
            }
            PolymorphicType::TypeQuantifier(var_name, inner_poly) => {
                let new_inner = self.apply_poly(inner_poly);
                PolymorphicType::TypeQuantifier(var_name.clone(), Box::new(new_inner))
            }
        }
    }

    pub fn apply_context(&self, item: &Context) -> Context {
        let map = item
            .0
            .iter()
            .map(|(k, v)| (k.clone(), self.apply_poly(v)))
            .collect();
        Context(map)
    }
}

impl MonomorphicType {
    pub fn free_vars(&self) -> Vec<VariableName> {
        match self {
            MonomorphicType::Variable(var_name) => vec![var_name.clone()],
            MonomorphicType::FunctionApplication(_, args) => {
                let mut vars = Vec::new();
                for arg in args {
                    vars.extend(arg.free_vars());
                }
                vars
            }
        }
    }
}

impl PolymorphicType {
    pub fn free_vars(&self) -> Vec<VariableName> {
        match self {
            PolymorphicType::MonomorphicType(mono) => mono.free_vars(),
            PolymorphicType::TypeQuantifier(var_name, inner_poly) => {
                let mut vars = inner_poly.free_vars();
                vars.retain(|v| v != var_name);
                vars
            }
        }
    }
}

impl Context {
    pub fn new() -> Self {
        Context(HashMap::new())
    }

    pub fn generalise(&self, mono: &MonomorphicType) -> PolymorphicType {
        let quantifiers = Self::diff(mono.free_vars(), self.free_vars());
        let p_type = PolymorphicType::MonomorphicType(mono.clone());
        quantifiers.into_iter().fold(p_type, |acc, var_name| {
            PolymorphicType::TypeQuantifier(var_name, Box::new(acc))
        })
    }

    fn free_vars(&self) -> Vec<VariableName> {
        self.0
            .iter()
            .map(|(_, v)| v.free_vars())
            .flatten()
            .collect()
    }

    fn diff<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
    where
        T: std::hash::Hash + Eq + Clone,
    {
        let b: HashSet<T> = b.into_iter().collect();
        a.into_iter().filter(|item| !b.contains(item)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_mono_variable() {
        let mut mappings = HashMap::new();
        mappings.insert("x".to_string(), MonomorphicType::Variable("y".to_string()));
        let substitution = Substitution::new(mappings);

        let mono = MonomorphicType::Variable("x".to_string());
        let result = substitution.apply_mono(&mono);
        match result {
            MonomorphicType::Variable(var_name) => {
                assert_eq!(var_name, "y");
            }
            _ => panic!("Expected MonomorphicType::Variable"),
        }
    }

    #[test]
    fn test_apply_mono_function_application() {
        let mut mappings = HashMap::new();
        mappings.insert("x".to_string(), MonomorphicType::Variable("y".to_string()));
        let substitution = Substitution::new(mappings);

        let mono = MonomorphicType::FunctionApplication(
            "->".to_string(),
            vec![
                MonomorphicType::Variable("x".to_string()),
                MonomorphicType::Variable("z".to_string()),
            ],
        );
        let result = substitution.apply_mono(&mono);
        match result {
            MonomorphicType::FunctionApplication(origin_type, args) => {
                assert_eq!(origin_type, "->");
                assert_eq!(args.len(), 2);
                match &args[0] {
                    MonomorphicType::Variable(vn) => {
                        assert_eq!(vn, "y");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
                match &args[1] {
                    MonomorphicType::Variable(vn) => {
                        assert_eq!(vn, "z");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
            }
            _ => panic!("Expected MonomorphicType::FunctionApplicaton"),
        }
    }

    #[test]
    fn test_combine() {
        let mut mappings1 = HashMap::new();
        mappings1.insert("x".to_string(), MonomorphicType::Variable("y".to_string()));
        let substitution1 = Substitution::new(mappings1);

        let mut mappings2 = HashMap::new();
        mappings2.insert(
            "z".to_string(),
            MonomorphicType::FunctionApplication(
                "->".to_string(),
                vec![MonomorphicType::Variable("x".to_string())],
            ),
        );
        let substitution2 = Substitution::new(mappings2);

        let combined = substitution1.combine(&substitution2);

        let mono = MonomorphicType::Variable("z".to_string());
        let result = combined.apply_mono(&mono);
        match result {
            MonomorphicType::FunctionApplication(origin_type, args) => {
                assert_eq!(origin_type, "->");
                assert_eq!(args.len(), 1);
                match &args[0] {
                    MonomorphicType::Variable(vn) => {
                        assert_eq!(vn, "y");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
            }
            _ => panic!("Expected MonomorphicType::Variable"),
        }
    }

    #[test]
    fn test_instantiate() {
        let mut generator = TypeVariableGenerator::new();
        let poly = PolymorphicType::TypeQuantifier(
            "z".to_string(),
            Box::new(PolymorphicType::MonomorphicType(MonomorphicType::Variable(
                "z".to_string(),
            ))),
        );
        let instantiated = generator.instantiate(&poly);
        match instantiated {
            MonomorphicType::Variable(var_name) => {
                assert_eq!(var_name, "t0");
            }
            _ => panic!("Expected MonomorphicType::Variable"),
        }

        let poly = PolymorphicType::TypeQuantifier(
            "z".to_string(),
            Box::new(PolymorphicType::MonomorphicType(
                MonomorphicType::FunctionApplication(
                    "->".to_string(),
                    vec![
                        MonomorphicType::Variable("z".to_string()),
                        MonomorphicType::Variable("y".to_string()),
                    ],
                ),
            )),
        );
        let instantiated = generator.instantiate(&poly);
        match instantiated {
            MonomorphicType::FunctionApplication(origin_type, args) => {
                assert_eq!(origin_type, "->");
                assert_eq!(args.len(), 2);
                match &args[0] {
                    MonomorphicType::Variable(vn) => {
                        assert_eq!(vn, "t1");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
                match &args[1] {
                    MonomorphicType::Variable(vn) => {
                        assert_eq!(vn, "y");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
            }
            _ => panic!("Expected MonomorphicType::FunctionApplicaton"),
        }

        let poly = PolymorphicType::TypeQuantifier(
            "y".to_string(),
            Box::new(PolymorphicType::TypeQuantifier(
                "z".to_string(),
                Box::new(PolymorphicType::MonomorphicType(
                    MonomorphicType::FunctionApplication(
                        "->".to_string(),
                        vec![
                            MonomorphicType::Variable("z".to_string()),
                            MonomorphicType::Variable("y".to_string()),
                        ],
                    ),
                )),
            )),
        );
        let instantiated = generator.instantiate(&poly);
        match instantiated {
            MonomorphicType::FunctionApplication(origin_type, args) => {
                assert_eq!(origin_type, "->");
                assert_eq!(args.len(), 2);
                match &args[0] {
                    MonomorphicType::Variable(vn) => {
                        assert_eq!(vn, "t3");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
                match &args[1] {
                    MonomorphicType::Variable(vn) => {
                        assert_eq!(vn, "t2");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
            }
            _ => panic!("Expected MonomorphicType::FunctionApplicaton"),
        }
    }

    #[test]
    fn test_generalise() {
        let mut context = Context::new();
        context.0.insert(
            "x".to_string(),
            PolymorphicType::MonomorphicType(MonomorphicType::Variable("t0".to_string())),
        );

        let quantifier = context.generalise(&MonomorphicType::Variable("t1".to_string()));
        match quantifier {
            PolymorphicType::TypeQuantifier(var_name, inner_poly) => {
                assert_eq!(var_name, "t1");
                match *inner_poly {
                    PolymorphicType::MonomorphicType(MonomorphicType::Variable(vn)) => {
                        assert_eq!(vn, "t1");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
            }
            _ => panic!("Expected TypeQuantifier"),
        }

        let mut context = Context::new();
        context.0.insert(
            "x".to_string(),
            PolymorphicType::TypeQuantifier(
                "t0".to_string(),
                Box::new(PolymorphicType::MonomorphicType(MonomorphicType::Variable(
                    "t0".to_string(),
                ))),
            ),
        );

        let quantifier = context.generalise(&MonomorphicType::Variable("t0".to_string()));
        match quantifier {
            PolymorphicType::TypeQuantifier(var_name, inner_poly) => {
                assert_eq!(var_name, "t0");
                match *inner_poly {
                    PolymorphicType::MonomorphicType(MonomorphicType::Variable(vn)) => {
                        assert_eq!(vn, "t0");
                    }
                    _ => panic!("Expected MonomorphicType::Variable"),
                }
            }
            _ => panic!("Expected PolymorphicType::Quantifier"),
        }

        let mut context = Context::new();
        context.0.insert(
            "x".to_string(),
            PolymorphicType::TypeQuantifier(
                "t1".to_string(),
                Box::new(PolymorphicType::MonomorphicType(MonomorphicType::Variable(
                    "t0".to_string(),
                ))),
            ),
        );

        let quantifier = context.generalise(&MonomorphicType::Variable("t0".to_string()));
        match quantifier {
            PolymorphicType::MonomorphicType(MonomorphicType::Variable(vn)) => {
                assert_eq!(vn, "t0");
            }
            _ => panic!("Expected MonomorphicType::Variable"),
        }
    }

    #[test]
    fn test_unify() {
        let a = MonomorphicType::FunctionApplication(
            "->".to_string(),
            vec![
                MonomorphicType::Variable("a".to_string()),
                MonomorphicType::Variable("b".to_string()),
            ],
        );
        let b = MonomorphicType::FunctionApplication(
            "->".to_string(),
            vec![
                MonomorphicType::FunctionApplication("i32".to_string(), vec![]),
                MonomorphicType::FunctionApplication("bool".to_string(), vec![]),
            ],
        );

        let sub = a.unify(&b).unwrap();

        println!("Substitution: {:?}", sub);
        match sub.mappings.get("a") {
            Some(MonomorphicType::FunctionApplication(origin_type, args)) => {
                assert_eq!(origin_type, "i32");
                assert!(args.is_empty());
            }
            _ => panic!("Expected mapping for 'a' to be a FunctionApplicaton"),
        }
        match sub.mappings.get("b") {
            Some(MonomorphicType::FunctionApplication(origin_type, args)) => {
                assert_eq!(origin_type, "bool");
                assert!(args.is_empty());
            }
            _ => panic!("Expected mapping for 'b' to be a FunctionApplicaton"),
        }

        let a = MonomorphicType::FunctionApplication(
            "->".to_string(),
            vec![
                MonomorphicType::Variable("a".to_string()),
                MonomorphicType::FunctionApplication("bool".to_string(), vec![]),
            ],
        );
        let b = MonomorphicType::FunctionApplication(
            "->".to_string(),
            vec![
                MonomorphicType::FunctionApplication("i32".to_string(), vec![]),
                MonomorphicType::Variable("b".to_string()),
            ],
        );

        let sub = a.unify(&b).unwrap();
        match sub.mappings.get("a") {
            Some(MonomorphicType::FunctionApplication(origin_type, args)) => {
                assert_eq!(origin_type, "i32");
                assert!(args.is_empty());
            }
            _ => panic!("Expected mapping for 'a' to be a FunctionApplicaton"),
        }
        match sub.mappings.get("b") {
            Some(MonomorphicType::FunctionApplication(origin_type, args)) => {
                assert_eq!(origin_type, "bool");
                assert!(args.is_empty());
            }
            _ => panic!("Expected mapping for 'b' to be a FunctionApplicaton"),
        }

        let a = MonomorphicType::FunctionApplication(
            "->".to_string(),
            vec![
                MonomorphicType::Variable("a".to_string()),
                MonomorphicType::Variable("a".to_string()),
            ],
        );
        let b = MonomorphicType::FunctionApplication(
            "->".to_string(),
            vec![
                MonomorphicType::FunctionApplication("i32".to_string(), vec![]),
                MonomorphicType::FunctionApplication("bool".to_string(), vec![]),
            ],
        );

        let result = a.unify(&b);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap().to_string(),
            "Cannot unify application with different types: i32 vs bool"
        );
    }
}
