use crate::val::Val;
use crate::{env::Env, utils};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct BindingUsage {
    pub(crate) name: String,
}

impl BindingUsage {
    pub(super) fn new(s: &str) -> Result<(&str, Self), String> {
        let (s, name) = utils::extract_ident(s)?;

        Ok((
            s,
            Self {
                name: name.to_string(),
            },
        ))
    }

    pub(super) fn eval(&self, env: &Env) -> Result<Val, String> {
        env.get_binding(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::env::Env;

    use super::*;

    #[test]
    fn parse_binding_usage() {
        assert_eq!(
            BindingUsage::new("abc"),
            Ok((
                "",
                BindingUsage {
                    name: "abc".to_string(),
                },
            )),
        );
    }

    #[test]
    fn eval_existing_binding_usage() {
        let mut env = Env::default();
        env.store_binding("foo".to_string(), Val::Number(10));

        assert_eq!(
            BindingUsage {
                name: "foo".to_string(),
            }
            .eval(&env),
            Ok(Val::Number(10)),
        );
    }

    #[test]
    fn eval_non_existent_binding_usage() {
        let empty_env = Env::default();

        assert_eq!(
            BindingUsage {
                name: "i_dont_exist".to_string(),
            }
            .eval(&empty_env),
            Err("binding with name 'i_dont_exist' does not exist".to_string())
        );
    }
}
