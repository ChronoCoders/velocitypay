use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{BoundedVec, pallet_prelude::Get};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum MintRequestStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum BurnRequestStatus {
    Pending,
    Reserved,
    Approved,
    Rejected,
    Completed,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxBankRefLen))]
pub struct MintRequest<AccountId, Balance, BlockNumber, MaxBankRefLen: Get<u32>> {
    pub requester: AccountId,
    pub amount: Balance,
    pub bank_reference: BoundedVec<u8, MaxBankRefLen>,
    pub status: MintRequestStatus,
    pub requested_at: BlockNumber,
    pub processed_at: Option<BlockNumber>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxBankAcctLen))]
pub struct BurnRequest<AccountId, Balance, BlockNumber, MaxBankAcctLen: Get<u32>> {
    pub requester: AccountId,
    pub amount: Balance,
    pub bank_account: BoundedVec<u8, MaxBankAcctLen>,
    pub status: BurnRequestStatus,
    pub requested_at: BlockNumber,
    pub processed_at: Option<BlockNumber>,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum TransactionType {
    Mint,
    Burn,
    Transfer,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(MaxTxHashLen))]
pub struct AuditLog<AccountId, Balance, BlockNumber, MaxTxHashLen: Get<u32>> {
    pub transaction_type: TransactionType,
    pub from: Option<AccountId>,
    pub to: Option<AccountId>,
    pub amount: Balance,
    pub timestamp: BlockNumber,
    pub transaction_hash: BoundedVec<u8, MaxTxHashLen>,
}
