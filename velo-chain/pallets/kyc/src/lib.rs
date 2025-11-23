#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

/// Trait for KYC verification functionality
pub trait KycVerification<AccountId> {
    /// Check if an account is KYC verified
    fn is_verified(account: &AccountId) -> bool;
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub enum KycStatus {
        NotSubmitted,
        Pending,
        Verified,
        Rejected,
    }

    impl Default for KycStatus {
        fn default() -> Self {
            KycStatus::NotSubmitted
        }
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    pub struct KycData<T: Config> {
        pub account: T::AccountId,
        pub document_hash: T::Hash,
        pub status: KycStatus,
        pub submitted_at: BlockNumberFor<T>,
        pub verified_at: Option<BlockNumberFor<T>>,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    }

    #[pallet::storage]
    #[pallet::getter(fn kyc_data)]
    pub type KycDatabase<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, KycData<T>>;

    #[pallet::storage]
    #[pallet::getter(fn kyc_verifier)]
    pub type KycVerifier<T: Config> = StorageValue<_, T::AccountId>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        KycSubmitted {
            account: T::AccountId,
            document_hash: T::Hash,
        },
        KycVerified {
            account: T::AccountId,
        },
        KycRejected {
            account: T::AccountId,
        },
        VerifierSet {
            verifier: T::AccountId,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        KycAlreadySubmitted,
        KycNotSubmitted,
        NotVerifier,
        InvalidStatus,
        CannotVerifySelf,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)]
        pub fn set_verifier(origin: OriginFor<T>, verifier: T::AccountId) -> DispatchResult {
            ensure_root(origin)?;
            <KycVerifier<T>>::put(&verifier);
            Self::deposit_event(Event::VerifierSet { verifier });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000)]
        pub fn submit_kyc(
            origin: OriginFor<T>,
            document_hash: T::Hash,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if KYC already exists
            if let Some(existing) = <KycDatabase<T>>::get(&who) {
                // Only allow resubmission if previously rejected
                ensure!(
                    existing.status == KycStatus::Rejected,
                    Error::<T>::KycAlreadySubmitted
                );
            }

            let current_block = <frame_system::Pallet<T>>::block_number();

            let kyc_data = KycData {
                account: who.clone(),
                document_hash: document_hash.clone(),
                status: KycStatus::Pending,
                submitted_at: current_block,
                verified_at: None,
            };

            <KycDatabase<T>>::insert(&who, kyc_data);

            Self::deposit_event(Event::KycSubmitted {
                account: who,
                document_hash,
            });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(10_000)]
        pub fn verify_kyc(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            let verifier = ensure_signed(origin)?;
            let authorized_verifier = Self::kyc_verifier().ok_or(Error::<T>::NotVerifier)?;
            ensure!(verifier == authorized_verifier, Error::<T>::NotVerifier);

            // Prevent verifier from verifying themselves
            ensure!(account != verifier, Error::<T>::CannotVerifySelf);

            let mut kyc_data = Self::kyc_data(&account).ok_or(Error::<T>::KycNotSubmitted)?;

            ensure!(
                kyc_data.status == KycStatus::Pending,
                Error::<T>::InvalidStatus
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            kyc_data.status = KycStatus::Verified;
            kyc_data.verified_at = Some(current_block);

            <KycDatabase<T>>::insert(&account, kyc_data);

            Self::deposit_event(Event::KycVerified { account });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(10_000)]
        pub fn reject_kyc(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            let verifier = ensure_signed(origin)?;
            let authorized_verifier = Self::kyc_verifier().ok_or(Error::<T>::NotVerifier)?;
            ensure!(verifier == authorized_verifier, Error::<T>::NotVerifier);

            let mut kyc_data = Self::kyc_data(&account).ok_or(Error::<T>::KycNotSubmitted)?;

            ensure!(
                kyc_data.status == KycStatus::Pending,
                Error::<T>::InvalidStatus
            );

            kyc_data.status = KycStatus::Rejected;

            <KycDatabase<T>>::insert(&account, kyc_data);

            Self::deposit_event(Event::KycRejected { account });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn is_verified(account: &T::AccountId) -> bool {
            if let Some(kyc) = Self::kyc_data(account) {
                kyc.status == KycStatus::Verified
            } else {
                false
            }
        }
    }
}

// Implement the KycVerification trait by delegating to the inherent method
impl<T: Config> crate::KycVerification<T::AccountId> for Pallet<T> {
    fn is_verified(account: &T::AccountId) -> bool {
        Pallet::<T>::is_verified(account)
    }
}