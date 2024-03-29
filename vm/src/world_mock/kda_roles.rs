use std::fmt::{self, Write};

#[derive(Clone, Default, Debug)]
pub struct KdaRoles(Vec<Vec<u8>>);

impl KdaRoles {
    pub fn new(roles: Vec<Vec<u8>>) -> Self {
        KdaRoles(roles)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get(&self) -> Vec<Vec<u8>> {
        self.0.clone()
    }
}

impl fmt::Display for KdaRoles {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut kda_buf = String::new();
        let kda_keys: Vec<Vec<u8>> = self.clone().0.to_vec();

        for value in &kda_keys {
            write!(kda_buf, "{}", hex::encode(value.as_slice()))?;
        }
        Ok(())
    }
}
