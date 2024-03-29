use super::{CheckKdaData, CheckKdaInstance, CheckKdaInstances};
use crate::{
    scenario::model::{BigUintValue, CheckValue, U64Value},
    scenario_format::{
        interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
        serde_raw::{CheckKdaRaw, ValueSubTree},
    },
};
use num_bigint::BigUint;

#[derive(Debug, Clone)]
pub enum CheckKda {
    Short(BigUintValue),
    Full(CheckKdaData),
}

impl CheckKda {
    pub fn convert_to_short_if_possible(&mut self) {
        if let CheckKda::Full(kda_check) = self {
            let has_single_fungible_instance =
                if let CheckKdaInstances::Equal(check_instance) = &kda_check.instances {
                    check_instance.len() == 1 && check_instance[0].is_simple_fungible()
                } else {
                    false
                };

            if has_single_fungible_instance
                && kda_check.frozen.is_star()
                && kda_check.last_nonce.is_star()
            {
                let balance =
                    if let CheckKdaInstances::Equal(check_instances) = &kda_check.instances {
                        match &check_instances[0].balance {
                            CheckValue::Star => BigUintValue {
                                original: ValueSubTree::Str("*".to_string()),
                                value: BigUint::from(0u32),
                            },
                            CheckValue::Equal(val) => val.clone(),
                        }
                    } else {
                        unreachable!();
                    };

                *self = CheckKda::Short(balance);
            }
        }
    }

    pub fn convert_to_full(&mut self) {
        if let CheckKda::Short(prev_balance_check) = self {
            let new_instances_check = vec![CheckKdaInstance {
                balance: CheckValue::Equal(prev_balance_check.clone()),
                ..Default::default()
            }];

            let new_kda_check = CheckKdaData {
                instances: CheckKdaInstances::Equal(new_instances_check),
                ..Default::default()
            };
            *self = CheckKda::Full(new_kda_check);
        }
    }

    pub fn add_balance_check<N, V>(&mut self, nonce_expr: N, balance_expr: V)
    where
        U64Value: InterpretableFrom<N>,
        BigUintValue: InterpretableFrom<V>,
    {
        let ctx = InterpreterContext::default();
        let nonce = U64Value::interpret_from(nonce_expr, &ctx);
        let balance = BigUintValue::interpret_from(balance_expr, &ctx);

        self.convert_to_full();

        if let CheckKda::Full(prev_kda_check) = self {
            match &mut prev_kda_check.instances {
                CheckKdaInstances::Star => {
                    let new_instances_check = vec![CheckKdaInstance {
                        nonce,
                        balance: CheckValue::Equal(balance),
                        ..Default::default()
                    }];

                    prev_kda_check.instances = CheckKdaInstances::Equal(new_instances_check);
                },
                CheckKdaInstances::Equal(kda_instance_check) => {
                    if let Some(i) = kda_instance_check
                        .iter()
                        .position(|item| item.nonce.value == nonce.value)
                    {
                        kda_instance_check[i].balance = CheckValue::Equal(balance);
                    } else {
                        kda_instance_check.push(CheckKdaInstance {
                            nonce,
                            balance: CheckValue::Equal(balance),
                            ..Default::default()
                        });
                    }
                },
            }
        }
    }
}

impl InterpretableFrom<CheckKdaRaw> for CheckKda {
    fn interpret_from(from: CheckKdaRaw, context: &InterpreterContext) -> Self {
        match from {
            CheckKdaRaw::Full(m) => CheckKda::Full(CheckKdaData::interpret_from(m, context)),
            CheckKdaRaw::Short(v) => CheckKda::Short(BigUintValue::interpret_from(v, context)),
        }
    }
}

impl IntoRaw<CheckKdaRaw> for CheckKda {
    fn into_raw(mut self) -> CheckKdaRaw {
        self.convert_to_short_if_possible();

        match self {
            CheckKda::Full(m) => CheckKdaRaw::Full(m.into_raw()),
            CheckKda::Short(v) => CheckKdaRaw::Short(v.into_raw()),
        }
    }
}
