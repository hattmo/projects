pub mod system {
    pub const KVM_CREATE_VM: u64 = 0x00_00_AE_01;
    pub const KVM_SET_USER_MEMORY_REGION: u64 = 0x40_20_AE_46;
    pub const KVM_SET_USER_MEMORY_REGION2: u64 = 0x40_a0_ae_49;
}
