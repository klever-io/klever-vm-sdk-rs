use super::KDALocalRole;
use bitflags::bitflags;

bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct KDALocalRoleFlags: i32 {
        const NONE                  = 0b00000000_00000000;
        const MINT                  = 0b00000000_00000001;
        const SET_ITO_PRICES        = 0b00000000_00000010;
        const DEPOSIT               = 0b00000000_00000100;
        const TRANSFER              = 0b00000000_00001000;
    }
}

impl KDALocalRoleFlags {
    pub fn has_role(&self, role: &KDALocalRole) -> bool {
        *self & role.to_flag() != KDALocalRoleFlags::NONE
    }

    pub fn iter_roles(&self) -> impl Iterator<Item = &KDALocalRole> {
        KDALocalRole::iter_all().filter(move |role| self.has_role(role))
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_flags_has_role() {
        let flags = KDALocalRoleFlags::MINT;
        assert!(flags.has_role(&KDALocalRole::Mint));
        let flags = KDALocalRoleFlags::MINT | KDALocalRoleFlags::TRANSFER;
        assert!(flags.has_role(&KDALocalRole::Mint));
        let flags = KDALocalRoleFlags::NONE;
        assert!(!flags.has_role(&KDALocalRole::Mint));
        let flags = KDALocalRoleFlags::TRANSFER;
        assert!(!flags.has_role(&KDALocalRole::Mint));
    }

    #[test]
    fn test_flags_iter_role() {
        let flags = KDALocalRoleFlags::MINT;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&KDALocalRole>>(),
            alloc::vec![&KDALocalRole::Mint],
        );

        let flags = KDALocalRoleFlags::MINT | KDALocalRoleFlags::TRANSFER;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&KDALocalRole>>(),
            alloc::vec![&KDALocalRole::Mint, &KDALocalRole::Transfer],
        );

        let flags = KDALocalRoleFlags::NONE;
        assert_eq!(
            flags.iter_roles().collect::<Vec<&KDALocalRole>>(),
            Vec::<&KDALocalRole>::new(),
        );
    }
}
