#![no_std]

use types::all_data_types::SubType;

use crate::types::all_data_types::{AllDataTypes, MaintenanceStatus};

multiversx_sc::imports!();

mod types;

#[multiversx_sc::contract]
pub trait Contract {
    #[init]
    fn init(&self) {}

    #[view(getAllDataTypes)]
    fn get_all_datatypes(&self) -> AllDataTypes<Self::Api> {
        let all_types = AllDataTypes {
            // primitives
            boolean: true,
            unsigned8: 10,
            unsigned16: 1000,
            unsigned32: 100000,
            unsigned64: 1000000000000000000,
            unsigned_size: 200000,
            signed8: 5,
            signed16: 500,
            signed32: 50000,
            signed64: 5000000000000000000,
            enumeration: MaintenanceStatus::InMaintenance,
            // complex
            managed_buffer: ManagedBuffer::from("abcd"),
            sub_type: SubType {
                address: self.blockchain().get_sc_address(),
                big_unsigned_integer: BigUint::from(100000000u64),
            },
            big_unsigned_integer: BigUint::from(10000000000000000000u64).pow(2),
            big_integer: BigInt::from(10000000000000000i64).pow(2),
            big_float: BigFloat::from(10000000000000000i64),
            // other
            address: self.blockchain().get_sc_address(),
            token_identifer: TokenIdentifier::from("ABCDEF-123456"),
            egld_or_esdt_token_identifer: EgldOrEsdtTokenIdentifier::esdt("ABCDEF-123456"),
            esdt_token_payment: EsdtTokenPayment::new(
                TokenIdentifier::from("ABCDEF-123456"),
                0,
                BigUint::from(1000000u64),
            ),
            egld_or_esdt_token_payment: EgldOrEsdtTokenPayment::new(
                EgldOrEsdtTokenIdentifier::esdt("ABCDEF-123456"),
                0,
                BigUint::from(2000000u64),
            ),
            // options, lists, etc.
            option_of_biguint_set: Option::Some(BigUint::from(420u64)),
            option_of_biguint_not_set: Option::None,
            option_of_subtype_set: Option::Some(SubType {
                big_unsigned_integer: BigUint::from(12u8),
                address: self.blockchain().get_sc_address(),
            }),
            option_of_subtype_not_set: Option::None,
            // managed lists
            managed_vec_of_u16: self.get_sample_mvec(),
            managed_vec_of_subtype: self.get_sample_mvec_complex(),
        };
        return all_types;
    }

    fn get_sample_mvec(&self) -> ManagedVec<u16> {
        let mut vec = ManagedVec::new();
        vec.push(12u16);
        vec.push(26u16);
        vec.push(3u16);
        return vec;
    }

    fn get_sample_mvec_complex(&self) -> ManagedVec<SubType<Self::Api>> {
        let mut vec = ManagedVec::new();
        vec.push(SubType {
            address: self.blockchain().get_sc_address(),
            big_unsigned_integer: BigUint::from(14u8),
        });
        vec.push(SubType {
            address: self.blockchain().get_owner_address(),
            big_unsigned_integer: BigUint::from(133u8),
        });
        return vec;
    }
}
