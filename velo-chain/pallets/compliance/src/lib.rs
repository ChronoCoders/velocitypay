#![cfg_attr(not(feature = "std"), no_std)]
#![allow(deprecated)]

pub use pallet::*;

/// Trait for compliance verification functionality
pub trait ComplianceCheck<AccountId, Balance> {
    /// Check if a transaction is compliant
    fn check_transaction(account: &AccountId, amount: Balance) -> Result<(), &'static str>;
}

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, BoundedVec};
    use frame_system::pallet_prelude::*;
    use sp_runtime::{
        traits::{CheckedAdd, Saturating, Zero},
        RuntimeDebug,
    };
    use sp_std::vec::Vec;

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    pub enum AlertLevel {
        Low,
        Medium,
        High,
        Critical,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    #[scale_info(skip_type_params(MaxReasonLen))]
    pub struct ComplianceAlert<AccountId, Balance, BlockNumber, MaxReasonLen: Get<u32>> {
        pub account: AccountId,
        pub alert_level: AlertLevel,
        pub amount: Balance,
        pub reason: BoundedVec<u8, MaxReasonLen>,
        pub created_at: BlockNumber,
        pub resolved: bool,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Balance: Parameter
            + Member
            + Default
            + Copy
            + MaxEncodedLen
            + PartialOrd
            + CheckedAdd
            + Saturating;

        #[pallet::constant]
        type DailyTransactionLimit: Get<Self::Balance>;

        #[pallet::constant]
        type SuspiciousAmountThreshold: Get<Self::Balance>;

        #[pallet::constant]
        type MaxReasonLength: Get<u32>;

        /// Number of blocks per day for daily volume calculation
        #[pallet::constant]
        type BlocksPerDay: Get<BlockNumberFor<Self>>;
    }

    #[pallet::storage]
    #[pallet::getter(fn compliance_officer)]
    pub type ComplianceOfficer<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn flagged_accounts)]
    pub type FlaggedAccounts<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn daily_transaction_volume)]
    pub type DailyTransactionVolume<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BlockNumberFor<T>,
        T::Balance,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn alerts)]
    pub type Alerts<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u64,
        ComplianceAlert<T::AccountId, T::Balance, BlockNumberFor<T>, T::MaxReasonLength>,
        OptionQuery,
    >;

    #[pallet::storage]
    #[pallet::getter(fn next_alert_id)]
    pub type NextAlertId<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub compliance_officer: Option<T::AccountId>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                compliance_officer: None,
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
        fn build(&self) {
            if let Some(ref officer) = self.compliance_officer {
                <ComplianceOfficer<T>>::put(officer);
            }
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        ComplianceOfficerSet {
            officer: T::AccountId,
        },
        AccountFlagged {
            account: T::AccountId,
        },
        AccountUnflagged {
            account: T::AccountId,
        },
        AlertCreated {
            alert_id: u64,
            account: T::AccountId,
            alert_level: AlertLevel,
        },
        AlertResolved {
            alert_id: u64,
        },
        SuspiciousActivityDetected {
            account: T::AccountId,
            amount: T::Balance,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        NotComplianceOfficer,
        AccountAlreadyFlagged,
        AccountNotFlagged,
        AlertNotFound,
        AlertAlreadyResolved,
        ReasonTooLong,
        NoComplianceOfficer,
        DailyLimitExceeded,
        InvalidConfiguration,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(Weight::from_parts(10_000_000, 0))]
        pub fn set_compliance_officer(
            origin: OriginFor<T>,
            officer: T::AccountId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            <ComplianceOfficer<T>>::put(&officer);

            Self::deposit_event(Event::ComplianceOfficerSet { officer });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(Weight::from_parts(10_000_000, 0))]
        pub fn flag_account(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            let officer = ensure_signed(origin)?;

            let stored_officer =
                Self::compliance_officer().ok_or(Error::<T>::NoComplianceOfficer)?;
            ensure!(officer == stored_officer, Error::<T>::NotComplianceOfficer);

            ensure!(
                !Self::flagged_accounts(&account),
                Error::<T>::AccountAlreadyFlagged
            );

            <FlaggedAccounts<T>>::insert(&account, true);

            Self::deposit_event(Event::AccountFlagged { account });

            Ok(())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(Weight::from_parts(10_000_000, 0))]
        pub fn unflag_account(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            let officer = ensure_signed(origin)?;

            let stored_officer =
                Self::compliance_officer().ok_or(Error::<T>::NoComplianceOfficer)?;
            ensure!(officer == stored_officer, Error::<T>::NotComplianceOfficer);

            ensure!(
                Self::flagged_accounts(&account),
                Error::<T>::AccountNotFlagged
            );

            <FlaggedAccounts<T>>::remove(&account);

            Self::deposit_event(Event::AccountUnflagged { account });

            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(Weight::from_parts(10_000_000, 0))]
        pub fn create_alert(
            origin: OriginFor<T>,
            account: T::AccountId,
            alert_level: AlertLevel,
            amount: T::Balance,
            reason: Vec<u8>,
        ) -> DispatchResult {
            let officer = ensure_signed(origin)?;

            let stored_officer =
                Self::compliance_officer().ok_or(Error::<T>::NoComplianceOfficer)?;
            ensure!(officer == stored_officer, Error::<T>::NotComplianceOfficer);

            ensure!(
                reason.len() <= T::MaxReasonLength::get() as usize,
                Error::<T>::ReasonTooLong
            );

            let alert_id = Self::next_alert_id();
            let current_block = <frame_system::Pallet<T>>::block_number();

            let reason_bounded = reason.try_into().map_err(|_| Error::<T>::ReasonTooLong)?;

            let alert = ComplianceAlert {
                account: account.clone(),
                alert_level: alert_level.clone(),
                amount,
                reason: reason_bounded,
                created_at: current_block,
                resolved: false,
            };

            <Alerts<T>>::insert(alert_id, alert);
            <NextAlertId<T>>::put(alert_id.saturating_add(1));

            Self::deposit_event(Event::AlertCreated {
                alert_id,
                account,
                alert_level,
            });

            Ok(())
        }

        #[pallet::call_index(4)]
        #[pallet::weight(Weight::from_parts(10_000_000, 0))]
        pub fn resolve_alert(origin: OriginFor<T>, alert_id: u64) -> DispatchResult {
            let officer = ensure_signed(origin)?;

            let stored_officer =
                Self::compliance_officer().ok_or(Error::<T>::NoComplianceOfficer)?;
            ensure!(officer == stored_officer, Error::<T>::NotComplianceOfficer);

            let mut alert = Self::alerts(alert_id).ok_or(Error::<T>::AlertNotFound)?;

            ensure!(!alert.resolved, Error::<T>::AlertAlreadyResolved);

            alert.resolved = true;
            <Alerts<T>>::insert(alert_id, alert);

            Self::deposit_event(Event::AlertResolved { alert_id });

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn is_account_flagged(account: &T::AccountId) -> bool {
            Self::flagged_accounts(account)
        }

        pub fn internal_check_transaction(
            account: &T::AccountId,
            amount: T::Balance,
        ) -> Result<(), &'static str> {
            // Check if account is flagged
            if Self::is_account_flagged(account) {
                return Err("Account is flagged for compliance review");
            }

            // Calculate current day based on block number
            let current_block = <frame_system::Pallet<T>>::block_number();
            let blocks_per_day = T::BlocksPerDay::get();

            // Prevent division by zero
            if blocks_per_day.is_zero() {
                return Err("BlocksPerDay configuration cannot be zero");
            }

            let current_day = current_block / blocks_per_day;

            // Get current daily volume for this account
            let current_volume = Self::daily_transaction_volume(account, current_day);

            // Calculate new total volume
            let new_volume = current_volume.saturating_add(amount);

            // Check if new volume exceeds daily limit
            let daily_limit = T::DailyTransactionLimit::get();
            if new_volume > daily_limit {
                return Err("Daily transaction limit exceeded");
            }

            // Update daily transaction volume
            <DailyTransactionVolume<T>>::insert(account, current_day, new_volume);

            // Check for suspicious amounts
            if amount >= T::SuspiciousAmountThreshold::get() {
                Self::deposit_event(Event::SuspiciousActivityDetected {
                    account: account.clone(),
                    amount,
                });
            }

            Ok(())
        }
    }
}

// Implement the ComplianceCheck trait
impl<T: Config> crate::ComplianceCheck<T::AccountId, T::Balance> for Pallet<T> {
    fn check_transaction(account: &T::AccountId, amount: T::Balance) -> Result<(), &'static str> {
        Pallet::<T>::internal_check_transaction(account, amount)
    }
}