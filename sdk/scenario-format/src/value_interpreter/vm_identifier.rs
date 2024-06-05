pub const VM_TYPE_LENGTH: usize = 2;

#[derive(Clone, Copy)]
pub struct VMIdentifier {
    pub vm_type: [u8; VM_TYPE_LENGTH],
}

impl Default for VMIdentifier {
    fn default() -> Self {
        Self { vm_type: [0, 0] }
    }
}
