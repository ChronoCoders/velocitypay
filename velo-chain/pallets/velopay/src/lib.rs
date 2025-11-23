#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        pallet_prelude::*,
        traits::{Currency, ReservableCurrency, ExistenceRequirement},
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{CheckedAdd, CheckedSub, CheckedDiv, Saturating, Zero};
    use pallet_kyc::KycVerification;
    use pallet_compliance::ComplianceCheck;

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum MintRequestStatus {
        Pending,
        Completed,
        Rejected,
    }

    impl Default for MintRequestStatus {
        fn default() -> Self {
            MintRequestStatus::Pending
        }
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum BurnRequestStatus {
        Pending,
        Reserved,
        Completed,
        Rejected,
    }

    impl Default for BurnRequestStatus {
        fn default() -> Self {
            BurnRequestStatus::Pending
        }
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct MintRequest<T: Config> {
        pub requester: T::AccountId,
        pub amount: BalanceOf<T>,
        pub status: MintRequestStatus,
        pub created_at: BlockNumberFor<T>,
        pub processed_at: Option<BlockNumberFor<T>>,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct BurnRequest<T: Config> {
        pub requester: T::AccountId,
        pub amount: BalanceOf<T>,
        pub status: BurnRequestStatus,
        pub created_at: BlockNumberFor<T>,
        pub processed_at: Option<BlockNumberFor<T>>,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// KYC verification provider
        type KycVerification: pallet_kyc::KycVerification<Self::AccountId>;

        /// Compliance check provider
        type ComplianceCheck: pallet_compliance::ComplianceCheck<Self::AccountId, BalanceOf<Self>>;

        #[pallet::constant]
        type MaxTransactionFee: Get<u32>;
    }

    #[pallet::storage]
    #[pallet::getter(fn total_supply)]
    pub type TotalSupply<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn mint_authority)]
    pub type MintAuthority<T: Config> = StorageValue<_, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn burn_authority)]
    pub type BurnAuthority<T: Config> = StorageValue<_, T::AccountId>;

    #[pallet::storage]
    #[pallet::getter(fn transaction_fee)]
    pub type TransactionFee<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn is_paused)]
    pub type Paused<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn mint_request)]
    pub type MintRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        MintRequest<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn next_mint_request_id)]
    pub type NextMintRequestId<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn burn_request)]
    pub type BurnRequests<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        BurnRequest<T>,
    >;

    #[pallet::storage]
    #[pallet::getter(fn next_burn_request_id)]
    pub type NextBurnRequestId<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn reserved_burns)]
    pub type ReservedBurns<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        BalanceOf<T>,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        MintAuthoritySet { authority: T::AccountId },
        BurnAuthoritySet { authority: T::AccountId },
        TransactionFeeSet { fee: u32 },
        SystemPaused,
        SystemUnpaused,
        MintRequested {
            request_id: u64,
            requester: T::AccountId,
            amount: BalanceOf<T>,
        },
        MintApproved {
            request_id: u64,
            requester: T::AccountId,
            amount: BalanceOf<T>,
        },
        MintRejected {
            request_id: u64,
            requester: T::AccountId,
        },
        BurnRequested {
            request_id: u64,
            requester: T::AccountId,
            amount: BalanceOf<T>,
        },
        BurnReserved {
            request_id: u64,
            requester: T::AccountId,
            amount: BalanceOf<T>,
        },
        BurnApproved {
            request_id: u64,
            requester: T::AccountId,
            amount: BalanceOf<T>,
        },
        BurnRejected {
            request_id: u64,
            requester: T::AccountId,
        },
        Transfer {
            from: T::AccountId,
            to: T::AccountId,
            amount: BalanceOf<T>,
            fee: BalanceOf<T>,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NoMintAuthority,
        NoBurnAuthority,
        NotMintAuthority,
        NotBurnAuthority,
        SystemPaused,
        InsufficientBalance,
        Overflow,
        MintRequestNotFound,
        InvalidMintRequestStatus,
        BurnRequestNotFound,
        InvalidBurnRequestStatus,
        InsufficientReservedBalance,
        KYCNotVerified,
        FeeCalculationFailed,
        InvalidAmount,
        ComplianceCheckFailed,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn set_mint_authority(
            origin: OriginFor<T>,
            authority: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            <MintAuthority<T>>::put(&authority);
            Self::deposit_event(Event::MintAuthoritySet { authority });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn set_burn_authority(
            origin: OriginFor<T>,
            authority: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;
            <BurnAuthority<T>>::put(&authority);
            Self::deposit_event(Event::BurnAuthoritySet { authority });
            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn set_transaction_fee(origin: OriginFor<T>, fee_basis_points: u32) -> DispatchResult {
            ensure_root(origin)?;
            ensure!(
                fee_basis_points <= T::MaxTransactionFee::get(),
                Error::<T>::Overflow
            );
            <TransactionFee<T>>::put(fee_basis_points);
            Self::deposit_event(Event::TransactionFeeSet {
                fee: fee_basis_points,
            });
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn pause(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            <Paused<T>>::put(true);
            Self::deposit_event(Event::SystemPaused);
            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(10_000)]
        pub fn unpause(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            <Paused<T>>::put(false);
            Self::deposit_event(Event::SystemUnpaused);
            Ok(())
        }

        #[pallet::call_index(5)]
        #[pallet::weight(10_000)]
        pub fn request_mint(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let requester = ensure_signed(origin)?;
            ensure!(!Self::is_paused(), Error::<T>::SystemPaused);

            // Validate amount is not zero
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);

            // Check KYC status
            ensure!(
                T::KycVerification::is_verified(&requester),
                Error::<T>::KYCNotVerified
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            let request_id = Self::next_mint_request_id();

            let request = MintRequest {
                requester: requester.clone(),
                amount,
                status: MintRequestStatus::Pending,
                created_at: current_block,
                processed_at: None,
            };

            <MintRequests<T>>::insert(request_id, request);
            <NextMintRequestId<T>>::put(request_id.saturating_add(1));

            Self::deposit_event(Event::MintRequested {
                request_id,
                requester,
                amount,
            });

            Ok(())
        }

        #[pallet::call_index(6)]
        #[pallet::weight(10_000)]
        pub fn approve_mint(
            origin: OriginFor<T>,
            request_id: u64,
        ) -> DispatchResult {
            let approver = ensure_signed(origin)?;
            let authority = Self::mint_authority().ok_or(Error::<T>::NoMintAuthority)?;
            ensure!(approver == authority, Error::<T>::NotMintAuthority);
            ensure!(!Self::is_paused(), Error::<T>::SystemPaused);

            let mut request = Self::mint_request(request_id)
                .ok_or(Error::<T>::MintRequestNotFound)?;

            ensure!(
                request.status == MintRequestStatus::Pending,
                Error::<T>::InvalidMintRequestStatus
            );

            let _imbalance = T::Currency::deposit_creating(&request.requester, request.amount);

            let new_supply = Self::total_supply()
                .checked_add(&request.amount)
                .ok_or(Error::<T>::Overflow)?;
            <TotalSupply<T>>::put(new_supply);

            let current_block = <frame_system::Pallet<T>>::block_number();
            request.status = MintRequestStatus::Completed;
            request.processed_at = Some(current_block);

            // Extract values before inserting
            let requester = request.requester.clone();
            let amount = request.amount;

            <MintRequests<T>>::insert(request_id, request);

            Self::deposit_event(Event::MintApproved {
                request_id,
                requester,
                amount,
            });

            Ok(())
        }

        #[pallet::call_index(7)]
        #[pallet::weight(10_000)]
        pub fn reject_mint(
            origin: OriginFor<T>,
            request_id: u64,
        ) -> DispatchResult {
            let approver = ensure_signed(origin)?;
            let authority = Self::mint_authority().ok_or(Error::<T>::NoMintAuthority)?;
            ensure!(approver == authority, Error::<T>::NotMintAuthority);

            let mut request = Self::mint_request(request_id)
                .ok_or(Error::<T>::MintRequestNotFound)?;

            ensure!(
                request.status == MintRequestStatus::Pending,
                Error::<T>::InvalidMintRequestStatus
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            request.status = MintRequestStatus::Rejected;
            request.processed_at = Some(current_block);

            let requester = request.requester.clone();
            <MintRequests<T>>::insert(request_id, request);

            Self::deposit_event(Event::MintRejected {
                request_id,
                requester,
            });

            Ok(())
        }

        #[pallet::call_index(8)]
        #[pallet::weight(10_000)]
        pub fn request_burn(
            origin: OriginFor<T>,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let requester = ensure_signed(origin)?;
            ensure!(!Self::is_paused(), Error::<T>::SystemPaused);

            // Validate amount is not zero
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);

            // Check KYC status
            ensure!(
                T::KycVerification::is_verified(&requester),
                Error::<T>::KYCNotVerified
            );

            let free_balance = T::Currency::free_balance(&requester);
            ensure!(free_balance >= amount, Error::<T>::InsufficientBalance);

            T::Currency::reserve(&requester, amount)?;

            <ReservedBurns<T>>::mutate(&requester, |reserved| {
                *reserved = reserved.saturating_add(amount);
            });

            let current_block = <frame_system::Pallet<T>>::block_number();
            let request_id = Self::next_burn_request_id();

            let request = BurnRequest {
                requester: requester.clone(),
                amount,
                status: BurnRequestStatus::Reserved,
                created_at: current_block,
                processed_at: None,
            };

            <BurnRequests<T>>::insert(request_id, request);
            <NextBurnRequestId<T>>::put(request_id.saturating_add(1));

            Self::deposit_event(Event::BurnReserved {
                request_id,
                requester,
                amount,
            });

            Ok(())
        }

        #[pallet::call_index(9)]
        #[pallet::weight(10_000)]
        pub fn approve_burn(
            origin: OriginFor<T>,
            request_id: u64,
        ) -> DispatchResult {
            let approver = ensure_signed(origin)?;
            let authority = Self::burn_authority().ok_or(Error::<T>::NoBurnAuthority)?;
            ensure!(approver == authority, Error::<T>::NotBurnAuthority);
            ensure!(!Self::is_paused(), Error::<T>::SystemPaused);

            let mut request = Self::burn_request(request_id)
                .ok_or(Error::<T>::BurnRequestNotFound)?;

            ensure!(
                request.status == BurnRequestStatus::Reserved,
                Error::<T>::InvalidBurnRequestStatus
            );

            // Slash reserved and ensure full amount is slashed
            let (_, remaining) = T::Currency::slash_reserved(&request.requester, request.amount);
            ensure!(remaining.is_zero(), Error::<T>::InsufficientReservedBalance);

            let new_supply = Self::total_supply()
                .checked_sub(&request.amount)
                .ok_or(Error::<T>::Overflow)?;
            <TotalSupply<T>>::put(new_supply);

            <ReservedBurns<T>>::mutate(&request.requester, |reserved| {
                *reserved = reserved.saturating_sub(request.amount);
            });

            let current_block = <frame_system::Pallet<T>>::block_number();
            request.status = BurnRequestStatus::Completed;
            request.processed_at = Some(current_block);

            let requester = request.requester.clone();
            let amount = request.amount;

            <BurnRequests<T>>::insert(request_id, request);

            Self::deposit_event(Event::BurnApproved {
                request_id,
                requester,
                amount,
            });

            Ok(())
        }

        #[pallet::call_index(10)]
        #[pallet::weight(10_000)]
        pub fn reject_burn(
            origin: OriginFor<T>,
            request_id: u64,
        ) -> DispatchResult {
            let approver = ensure_signed(origin)?;
            let authority = Self::burn_authority().ok_or(Error::<T>::NoBurnAuthority)?;
            ensure!(approver == authority, Error::<T>::NotBurnAuthority);

            let mut request = Self::burn_request(request_id)
                .ok_or(Error::<T>::BurnRequestNotFound)?;

            ensure!(
                request.status == BurnRequestStatus::Reserved,
                Error::<T>::InvalidBurnRequestStatus
            );

            let _balance = T::Currency::unreserve(&request.requester, request.amount);

            <ReservedBurns<T>>::mutate(&request.requester, |reserved| {
                *reserved = reserved.saturating_sub(request.amount);
            });

            let current_block = <frame_system::Pallet<T>>::block_number();
            request.status = BurnRequestStatus::Rejected;
            request.processed_at = Some(current_block);

            let requester = request.requester.clone();
            <BurnRequests<T>>::insert(request_id, request);

            Self::deposit_event(Event::BurnRejected {
                request_id,
                requester,
            });

            Ok(())
        }

        #[pallet::call_index(11)]
        #[pallet::weight(10_000)]
        pub fn transfer(
            origin: OriginFor<T>,
            dest: T::AccountId,
            amount: BalanceOf<T>,
        ) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(!Self::is_paused(), Error::<T>::SystemPaused);

            // Validate amount is not zero
            ensure!(!amount.is_zero(), Error::<T>::InvalidAmount);

            // Prevent self-transfer
            ensure!(sender != dest, Error::<T>::InvalidAmount);

            // Check KYC status for both sender and receiver
            ensure!(
                T::KycVerification::is_verified(&sender),
                Error::<T>::KYCNotVerified
            );
            ensure!(
                T::KycVerification::is_verified(&dest),
                Error::<T>::KYCNotVerified
            );

            // Perform compliance checks for both sender and receiver
            T::ComplianceCheck::check_transaction(&sender, amount)
                .map_err(|_| Error::<T>::ComplianceCheckFailed)?;
            T::ComplianceCheck::check_transaction(&dest, amount)
                .map_err(|_| Error::<T>::ComplianceCheckFailed)?;

            let fee_basis_points = Self::transaction_fee();
            let fee = amount
                .saturating_mul(fee_basis_points.into())
                .checked_div(&10000u32.into())
                .ok_or(Error::<T>::FeeCalculationFailed)?;

            // Calculate total amount needed (amount + fee)
            let total_amount = amount
                .checked_add(&fee)
                .ok_or(Error::<T>::Overflow)?;

            // Verify sender has sufficient balance for amount + fee
            let sender_balance = T::Currency::free_balance(&sender);
            ensure!(sender_balance >= total_amount, Error::<T>::InsufficientBalance);

            T::Currency::transfer(
                &sender,
                &dest,
                amount,
                ExistenceRequirement::KeepAlive,
            )?;

            if !fee.is_zero() {
                let authority = Self::mint_authority().ok_or(Error::<T>::NoMintAuthority)?;
                T::Currency::transfer(
                    &sender,
                    &authority,
                    fee,
                    ExistenceRequirement::KeepAlive,
                )?;
            }

            Self::deposit_event(Event::Transfer {
                from: sender,
                to: dest,
                amount,
                fee,
            });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn calculate_fee(amount: BalanceOf<T>) -> BalanceOf<T> {
            let fee_basis_points = Self::transaction_fee();
            amount
                .saturating_mul(fee_basis_points.into())
                .checked_div(&10000u32.into())
                .unwrap_or_else(Zero::zero)
        }
    }
}