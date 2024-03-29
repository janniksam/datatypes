use multiversx_sc::{
    api::ManagedTypeApi,
    types::{
        BigFloat, BigInt, BigUint, EgldOrEsdtTokenIdentifier, EgldOrEsdtTokenPayment,
        EsdtTokenPayment, ManagedAddress, ManagedBuffer, ManagedVec, TokenIdentifier,
    },
};

multiversx_sc::derive_imports!();

#[repr(u8)]
#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq)]
pub enum MaintenanceStatus {
    NotSet = 0,
    InMaintenance = 1,
    NotInMaintenance = 2,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, ManagedVecItem)]
pub struct SubType<M: ManagedTypeApi> {
    pub big_unsigned_integer: BigUint<M>,
    pub address: ManagedAddress<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct AllDataTypes<M: ManagedTypeApi> {
    // primitives
    pub boolean: bool,
    pub unsigned8: u8,
    pub unsigned16: u16,
    pub unsigned32: u32,
    pub unsigned64: u64,
    pub unsigned_size: usize,
    pub signed8: i8,
    pub signed16: i16,
    pub signed32: i32,
    pub signed64: i64,
    pub enumeration: MaintenanceStatus,
    // complex
    pub managed_buffer: ManagedBuffer<M>,
    pub sub_type: SubType<M>,
    pub big_unsigned_integer: BigUint<M>,
    pub big_integer: BigInt<M>,
    pub big_float: BigFloat<M>,
    // other
    pub address: ManagedAddress<M>,
    pub token_identifer: TokenIdentifier<M>,
    pub egld_or_esdt_token_identifer: EgldOrEsdtTokenIdentifier<M>,
    pub esdt_token_payment: EsdtTokenPayment<M>,
    pub egld_or_esdt_token_payment: EgldOrEsdtTokenPayment<M>,
    // options
    pub option_of_biguint_set: Option<BigUint<M>>,
    pub option_of_biguint_not_set: Option<BigUint<M>>,
    pub option_of_subtype_set: Option<SubType<M>>,
    pub option_of_subtype_not_set: Option<SubType<M>>,
    // lists, managed lists, vecs
    pub managed_vec_of_u16: ManagedVec<M, u16>,
    pub managed_vec_of_subtype: ManagedVec<M, SubType<M>>,
}
