// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Events {
    #[prost(message, repeated, tag="1")]
    pub tellerv2_accepted_bids: ::prost::alloc::vec::Vec<Tellerv2AcceptedBid>,
    #[prost(message, repeated, tag="2")]
    pub tellerv2_admin_changeds: ::prost::alloc::vec::Vec<Tellerv2AdminChanged>,
    #[prost(message, repeated, tag="3")]
    pub tellerv2_beacon_upgradeds: ::prost::alloc::vec::Vec<Tellerv2BeaconUpgraded>,
    #[prost(message, repeated, tag="4")]
    pub tellerv2_cancelled_bids: ::prost::alloc::vec::Vec<Tellerv2CancelledBid>,
    #[prost(message, repeated, tag="5")]
    pub tellerv2_fee_paids: ::prost::alloc::vec::Vec<Tellerv2FeePaid>,
    #[prost(message, repeated, tag="6")]
    pub tellerv2_initializeds: ::prost::alloc::vec::Vec<Tellerv2Initialized>,
    #[prost(message, repeated, tag="7")]
    pub tellerv2_loan_liquidateds: ::prost::alloc::vec::Vec<Tellerv2LoanLiquidated>,
    #[prost(message, repeated, tag="8")]
    pub tellerv2_loan_repaids: ::prost::alloc::vec::Vec<Tellerv2LoanRepaid>,
    #[prost(message, repeated, tag="9")]
    pub tellerv2_loan_repayments: ::prost::alloc::vec::Vec<Tellerv2LoanRepayment>,
    #[prost(message, repeated, tag="10")]
    pub tellerv2_market_forwarder_approveds: ::prost::alloc::vec::Vec<Tellerv2MarketForwarderApproved>,
    #[prost(message, repeated, tag="11")]
    pub tellerv2_market_forwarder_renounceds: ::prost::alloc::vec::Vec<Tellerv2MarketForwarderRenounced>,
    #[prost(message, repeated, tag="12")]
    pub tellerv2_market_owner_cancelled_bids: ::prost::alloc::vec::Vec<Tellerv2MarketOwnerCancelledBid>,
    #[prost(message, repeated, tag="13")]
    pub tellerv2_ownership_transferreds: ::prost::alloc::vec::Vec<Tellerv2OwnershipTransferred>,
    #[prost(message, repeated, tag="14")]
    pub tellerv2_pauseds: ::prost::alloc::vec::Vec<Tellerv2Paused>,
    #[prost(message, repeated, tag="15")]
    pub tellerv2_protocol_fee_sets: ::prost::alloc::vec::Vec<Tellerv2ProtocolFeeSet>,
    #[prost(message, repeated, tag="16")]
    pub tellerv2_submitted_bids: ::prost::alloc::vec::Vec<Tellerv2SubmittedBid>,
    #[prost(message, repeated, tag="17")]
    pub tellerv2_trusted_market_forwarder_sets: ::prost::alloc::vec::Vec<Tellerv2TrustedMarketForwarderSet>,
    #[prost(message, repeated, tag="18")]
    pub tellerv2_unpauseds: ::prost::alloc::vec::Vec<Tellerv2Unpaused>,
    #[prost(message, repeated, tag="19")]
    pub tellerv2_upgradeds: ::prost::alloc::vec::Vec<Tellerv2Upgraded>,
    #[prost(message, repeated, tag="20")]
    pub marketregistry_admin_changeds: ::prost::alloc::vec::Vec<MarketregistryAdminChanged>,
    #[prost(message, repeated, tag="21")]
    pub marketregistry_beacon_upgradeds: ::prost::alloc::vec::Vec<MarketregistryBeaconUpgraded>,
    #[prost(message, repeated, tag="22")]
    pub marketregistry_borrower_attestations: ::prost::alloc::vec::Vec<MarketregistryBorrowerAttestation>,
    #[prost(message, repeated, tag="23")]
    pub marketregistry_borrower_exit_markets: ::prost::alloc::vec::Vec<MarketregistryBorrowerExitMarket>,
    #[prost(message, repeated, tag="24")]
    pub marketregistry_borrower_revocations: ::prost::alloc::vec::Vec<MarketregistryBorrowerRevocation>,
    #[prost(message, repeated, tag="25")]
    pub marketregistry_initializeds: ::prost::alloc::vec::Vec<MarketregistryInitialized>,
    #[prost(message, repeated, tag="26")]
    pub marketregistry_lender_attestations: ::prost::alloc::vec::Vec<MarketregistryLenderAttestation>,
    #[prost(message, repeated, tag="27")]
    pub marketregistry_lender_exit_markets: ::prost::alloc::vec::Vec<MarketregistryLenderExitMarket>,
    #[prost(message, repeated, tag="28")]
    pub marketregistry_lender_revocations: ::prost::alloc::vec::Vec<MarketregistryLenderRevocation>,
    #[prost(message, repeated, tag="29")]
    pub marketregistry_market_closeds: ::prost::alloc::vec::Vec<MarketregistryMarketClosed>,
    #[prost(message, repeated, tag="30")]
    pub marketregistry_market_createds: ::prost::alloc::vec::Vec<MarketregistryMarketCreated>,
    #[prost(message, repeated, tag="31")]
    pub marketregistry_set_bid_expiration_times: ::prost::alloc::vec::Vec<MarketregistrySetBidExpirationTime>,
    #[prost(message, repeated, tag="32")]
    pub marketregistry_set_market_borrower_attestations: ::prost::alloc::vec::Vec<MarketregistrySetMarketBorrowerAttestation>,
    #[prost(message, repeated, tag="33")]
    pub marketregistry_set_market_fees: ::prost::alloc::vec::Vec<MarketregistrySetMarketFee>,
    #[prost(message, repeated, tag="34")]
    pub marketregistry_set_market_fee_recipients: ::prost::alloc::vec::Vec<MarketregistrySetMarketFeeRecipient>,
    #[prost(message, repeated, tag="35")]
    pub marketregistry_set_market_lender_attestations: ::prost::alloc::vec::Vec<MarketregistrySetMarketLenderAttestation>,
    #[prost(message, repeated, tag="36")]
    pub marketregistry_set_market_owners: ::prost::alloc::vec::Vec<MarketregistrySetMarketOwner>,
    #[prost(message, repeated, tag="37")]
    pub marketregistry_set_market_payment_types: ::prost::alloc::vec::Vec<MarketregistrySetMarketPaymentType>,
    #[prost(message, repeated, tag="38")]
    pub marketregistry_set_market_uris: ::prost::alloc::vec::Vec<MarketregistrySetMarketUri>,
    #[prost(message, repeated, tag="39")]
    pub marketregistry_set_payment_cycles: ::prost::alloc::vec::Vec<MarketregistrySetPaymentCycle>,
    #[prost(message, repeated, tag="40")]
    pub marketregistry_set_payment_cycle_durations: ::prost::alloc::vec::Vec<MarketregistrySetPaymentCycleDuration>,
    #[prost(message, repeated, tag="41")]
    pub marketregistry_set_payment_default_durations: ::prost::alloc::vec::Vec<MarketregistrySetPaymentDefaultDuration>,
    #[prost(message, repeated, tag="42")]
    pub marketregistry_upgradeds: ::prost::alloc::vec::Vec<MarketregistryUpgraded>,
    #[prost(message, repeated, tag="43")]
    pub lendercommitment_admin_changeds: ::prost::alloc::vec::Vec<LendercommitmentAdminChanged>,
    #[prost(message, repeated, tag="44")]
    pub lendercommitment_beacon_upgradeds: ::prost::alloc::vec::Vec<LendercommitmentBeaconUpgraded>,
    #[prost(message, repeated, tag="45")]
    pub lendercommitment_created_commitments: ::prost::alloc::vec::Vec<LendercommitmentCreatedCommitment>,
    #[prost(message, repeated, tag="46")]
    pub lendercommitment_deleted_commitments: ::prost::alloc::vec::Vec<LendercommitmentDeletedCommitment>,
    #[prost(message, repeated, tag="47")]
    pub lendercommitment_exercised_commitments: ::prost::alloc::vec::Vec<LendercommitmentExercisedCommitment>,
    #[prost(message, repeated, tag="48")]
    pub lendercommitment_initializeds: ::prost::alloc::vec::Vec<LendercommitmentInitialized>,
    #[prost(message, repeated, tag="49")]
    pub lendercommitment_updated_commitments: ::prost::alloc::vec::Vec<LendercommitmentUpdatedCommitment>,
    #[prost(message, repeated, tag="50")]
    pub lendercommitment_updated_commitment_borrowers: ::prost::alloc::vec::Vec<LendercommitmentUpdatedCommitmentBorrowers>,
    #[prost(message, repeated, tag="51")]
    pub lendercommitment_upgradeds: ::prost::alloc::vec::Vec<LendercommitmentUpgraded>,
    #[prost(message, repeated, tag="52")]
    pub lendercommitmentstaging_admin_changeds: ::prost::alloc::vec::Vec<LendercommitmentstagingAdminChanged>,
    #[prost(message, repeated, tag="53")]
    pub lendercommitmentstaging_beacon_upgradeds: ::prost::alloc::vec::Vec<LendercommitmentstagingBeaconUpgraded>,
    #[prost(message, repeated, tag="54")]
    pub lendercommitmentstaging_created_commitments: ::prost::alloc::vec::Vec<LendercommitmentstagingCreatedCommitment>,
    #[prost(message, repeated, tag="55")]
    pub lendercommitmentstaging_deleted_commitments: ::prost::alloc::vec::Vec<LendercommitmentstagingDeletedCommitment>,
    #[prost(message, repeated, tag="56")]
    pub lendercommitmentstaging_exercised_commitments: ::prost::alloc::vec::Vec<LendercommitmentstagingExercisedCommitment>,
    #[prost(message, repeated, tag="57")]
    pub lendercommitmentstaging_extension_addeds: ::prost::alloc::vec::Vec<LendercommitmentstagingExtensionAdded>,
    #[prost(message, repeated, tag="58")]
    pub lendercommitmentstaging_extension_revokeds: ::prost::alloc::vec::Vec<LendercommitmentstagingExtensionRevoked>,
    #[prost(message, repeated, tag="59")]
    pub lendercommitmentstaging_initializeds: ::prost::alloc::vec::Vec<LendercommitmentstagingInitialized>,
    #[prost(message, repeated, tag="60")]
    pub lendercommitmentstaging_updated_commitments: ::prost::alloc::vec::Vec<LendercommitmentstagingUpdatedCommitment>,
    #[prost(message, repeated, tag="61")]
    pub lendercommitmentstaging_updated_commitment_borrowers: ::prost::alloc::vec::Vec<LendercommitmentstagingUpdatedCommitmentBorrowers>,
    #[prost(message, repeated, tag="62")]
    pub lendercommitmentstaging_upgradeds: ::prost::alloc::vec::Vec<LendercommitmentstagingUpgraded>,
    #[prost(message, repeated, tag="63")]
    pub collateralmanager_admin_changeds: ::prost::alloc::vec::Vec<CollateralmanagerAdminChanged>,
    #[prost(message, repeated, tag="64")]
    pub collateralmanager_beacon_upgradeds: ::prost::alloc::vec::Vec<CollateralmanagerBeaconUpgraded>,
    #[prost(message, repeated, tag="65")]
    pub collateralmanager_collateral_claimeds: ::prost::alloc::vec::Vec<CollateralmanagerCollateralClaimed>,
    #[prost(message, repeated, tag="66")]
    pub collateralmanager_collateral_committeds: ::prost::alloc::vec::Vec<CollateralmanagerCollateralCommitted>,
    #[prost(message, repeated, tag="67")]
    pub collateralmanager_collateral_depositeds: ::prost::alloc::vec::Vec<CollateralmanagerCollateralDeposited>,
    #[prost(message, repeated, tag="68")]
    pub collateralmanager_collateral_escrow_deployeds: ::prost::alloc::vec::Vec<CollateralmanagerCollateralEscrowDeployed>,
    #[prost(message, repeated, tag="69")]
    pub collateralmanager_collateral_withdrawns: ::prost::alloc::vec::Vec<CollateralmanagerCollateralWithdrawn>,
    #[prost(message, repeated, tag="70")]
    pub collateralmanager_initializeds: ::prost::alloc::vec::Vec<CollateralmanagerInitialized>,
    #[prost(message, repeated, tag="71")]
    pub collateralmanager_ownership_transferreds: ::prost::alloc::vec::Vec<CollateralmanagerOwnershipTransferred>,
    #[prost(message, repeated, tag="72")]
    pub collateralmanager_upgradeds: ::prost::alloc::vec::Vec<CollateralmanagerUpgraded>,
    #[prost(message, repeated, tag="73")]
    pub lendermanager_admin_changeds: ::prost::alloc::vec::Vec<LendermanagerAdminChanged>,
    #[prost(message, repeated, tag="74")]
    pub lendermanager_approvals: ::prost::alloc::vec::Vec<LendermanagerApproval>,
    #[prost(message, repeated, tag="75")]
    pub lendermanager_approval_for_alls: ::prost::alloc::vec::Vec<LendermanagerApprovalForAll>,
    #[prost(message, repeated, tag="76")]
    pub lendermanager_beacon_upgradeds: ::prost::alloc::vec::Vec<LendermanagerBeaconUpgraded>,
    #[prost(message, repeated, tag="77")]
    pub lendermanager_initializeds: ::prost::alloc::vec::Vec<LendermanagerInitialized>,
    #[prost(message, repeated, tag="78")]
    pub lendermanager_ownership_transferreds: ::prost::alloc::vec::Vec<LendermanagerOwnershipTransferred>,
    #[prost(message, repeated, tag="79")]
    pub lendermanager_transfers: ::prost::alloc::vec::Vec<LendermanagerTransfer>,
    #[prost(message, repeated, tag="80")]
    pub lendermanager_upgradeds: ::prost::alloc::vec::Vec<LendermanagerUpgraded>,
    #[prost(message, repeated, tag="81")]
    pub marketliquidityrewards_admin_changeds: ::prost::alloc::vec::Vec<MarketliquidityrewardsAdminChanged>,
    #[prost(message, repeated, tag="82")]
    pub marketliquidityrewards_beacon_upgradeds: ::prost::alloc::vec::Vec<MarketliquidityrewardsBeaconUpgraded>,
    #[prost(message, repeated, tag="83")]
    pub marketliquidityrewards_claimed_rewards: ::prost::alloc::vec::Vec<MarketliquidityrewardsClaimedRewards>,
    #[prost(message, repeated, tag="84")]
    pub marketliquidityrewards_created_allocations: ::prost::alloc::vec::Vec<MarketliquidityrewardsCreatedAllocation>,
    #[prost(message, repeated, tag="85")]
    pub marketliquidityrewards_decreased_allocations: ::prost::alloc::vec::Vec<MarketliquidityrewardsDecreasedAllocation>,
    #[prost(message, repeated, tag="86")]
    pub marketliquidityrewards_deleted_allocations: ::prost::alloc::vec::Vec<MarketliquidityrewardsDeletedAllocation>,
    #[prost(message, repeated, tag="87")]
    pub marketliquidityrewards_increased_allocations: ::prost::alloc::vec::Vec<MarketliquidityrewardsIncreasedAllocation>,
    #[prost(message, repeated, tag="88")]
    pub marketliquidityrewards_initializeds: ::prost::alloc::vec::Vec<MarketliquidityrewardsInitialized>,
    #[prost(message, repeated, tag="89")]
    pub marketliquidityrewards_updated_allocations: ::prost::alloc::vec::Vec<MarketliquidityrewardsUpdatedAllocation>,
    #[prost(message, repeated, tag="90")]
    pub marketliquidityrewards_upgradeds: ::prost::alloc::vec::Vec<MarketliquidityrewardsUpgraded>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2AcceptedBid {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2AdminChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2BeaconUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2CancelledBid {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bid_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2FeePaid {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub fee_type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2Initialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2LoanLiquidated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub liquidator: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2LoanRepaid {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2LoanRepayment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
    #[prost(bytes="vec", tag="7")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2MarketForwarderApproved {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub forwarder: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2MarketForwarderRenounced {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub forwarder: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2MarketOwnerCancelledBid {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bid_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2OwnershipTransferred {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2Paused {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub account: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2ProtocolFeeSet {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub new_fee: u64,
    #[prost(uint64, tag="6")]
    pub old_fee: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2SubmittedBid {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub borrower: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub receiver: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="8")]
    pub metadata_uri: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="9")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2TrustedMarketForwarderSet {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub forwarder: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="7")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2Unpaused {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub account: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tellerv2Upgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryAdminChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryBeaconUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryBorrowerAttestation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub borrower: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryBorrowerExitMarket {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub borrower: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryBorrowerRevocation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub borrower: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryInitialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryLenderAttestation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryLenderExitMarket {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryLenderRevocation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryMarketClosed {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryMarketCreated {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub market_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetBidExpirationTime {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub duration: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetMarketBorrowerAttestation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub required: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetMarketFee {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub fee_pct: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetMarketFeeRecipient {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub new_recipient: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetMarketLenderAttestation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub required: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetMarketOwner {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetMarketPaymentType {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub payment_type: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetMarketUri {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetPaymentCycle {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub payment_cycle_type: u64,
    #[prost(uint64, tag="7")]
    pub value: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetPaymentCycleDuration {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub duration: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistrySetPaymentDefaultDuration {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub duration: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketregistryUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentAdminChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentBeaconUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentCreatedCommitment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub log_ordinal: u64,
    #[prost(string, tag="6")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub lending_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="10")]
    pub token_amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentDeletedCommitment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentExercisedCommitment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub borrower: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub token_amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="10")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentInitialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentUpdatedCommitment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub lending_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub token_amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="10")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentUpdatedCommitmentBorrowers {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingAdminChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingBeaconUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingCreatedCommitment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub log_ordinal: u64,
    #[prost(string, tag="6")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub lending_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="10")]
    pub token_amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingDeletedCommitment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingExercisedCommitment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub borrower: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub token_amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="10")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingExtensionAdded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingExtensionRevoked {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub extension: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub sender: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingInitialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingUpdatedCommitment {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub lending_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub token_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingUpdatedCommitmentBorrowers {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendercommitmentstagingUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerAdminChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerBeaconUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerCollateralClaimed {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub u_bid_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerCollateralCommitted {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub u_bid_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub u_type: u64,
    #[prost(bytes="vec", tag="7")]
    pub u_collateral_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub u_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub u_token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerCollateralDeposited {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub u_bid_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub u_type: u64,
    #[prost(bytes="vec", tag="7")]
    pub u_collateral_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub u_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub u_token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerCollateralEscrowDeployed {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub u_bid_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub u_collateral_escrow: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerCollateralWithdrawn {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub u_bid_id: ::prost::alloc::string::String,
    #[prost(uint64, tag="6")]
    pub u_type: u64,
    #[prost(bytes="vec", tag="7")]
    pub u_collateral_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub u_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub u_token_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="10")]
    pub u_recipient: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerInitialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerOwnershipTransferred {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollateralmanagerUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendermanagerAdminChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendermanagerApproval {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub approved: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendermanagerApprovalForAll {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub operator: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="7")]
    pub approved: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendermanagerBeaconUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendermanagerInitialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendermanagerOwnershipTransferred {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_owner: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendermanagerTransfer {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub from: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub to: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub token_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendermanagerUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsAdminChanged {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub previous_admin: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub new_admin: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsBeaconUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub beacon: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsClaimedRewards {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub allocation_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub bid_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="8")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="10")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsCreatedAllocation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub allocation_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub allocator: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="9")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsDecreasedAllocation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub allocation_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="8")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsDeletedAllocation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub allocation_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsIncreasedAllocation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub allocation_id: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub amount: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="8")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsInitialized {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(uint64, tag="5")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsUpdatedAllocation {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub allocation_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub evt_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag="7")]
    pub log_ordinal: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketliquidityrewardsUpgraded {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(bytes="vec", tag="5")]
    pub implementation: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketPlace {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub marketplace_id: u64,
    #[prost(bytes="vec", tag="3")]
    pub owner: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub fee_recipient: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub metadata_uri: ::prost::alloc::string::String,
    #[prost(bool, tag="6")]
    pub is_market_open: bool,
    #[prost(uint64, tag="7")]
    pub payment_default_duration: u64,
    #[prost(uint64, tag="8")]
    pub payment_cycle_duration: u64,
    #[prost(string, tag="9")]
    pub payment_cycle_type: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub payment_type: ::prost::alloc::string::String,
    #[prost(uint64, tag="11")]
    pub bid_expiration_time: u64,
    #[prost(bool, tag="12")]
    pub borrower_attestation_required: bool,
    #[prost(bool, tag="13")]
    pub lender_attestation_required: bool,
    #[prost(uint64, tag="14")]
    pub marketplace_fee_percent: u64,
    #[prost(uint64, tag="15")]
    pub duration_total: u64,
    #[prost(uint64, tag="16")]
    pub duration_average: u64,
    #[prost(uint64, tag="17")]
    pub total_number_of_lenders: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Token {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub token_type: ::prost::alloc::string::String,
    #[prost(uint64, optional, tag="4")]
    pub nft_id: ::core::option::Option<u64>,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint64, tag="7")]
    pub decimals: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitmentEventData {
    #[prost(string, tag="1")]
    pub evt_tx_hash: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub evt_index: u32,
    #[prost(message, optional, tag="3")]
    pub evt_block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint64, tag="4")]
    pub evt_block_number: u64,
    #[prost(string, tag="5")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="6")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="8")]
    pub lending_token: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="9")]
    pub token_amount: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitmentsCall {
    #[prost(string, tag="1")]
    pub max_principal: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub expiration: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub max_duration: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub min_interest_rate: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub collateral_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="6")]
    pub collateral_token_id: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub max_principal_per_collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub collateral_token_type: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="9")]
    pub lender: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="10")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="11")]
    pub principal_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag="12")]
    pub is_rolloverable: bool,
    #[prost(bytes="vec", tag="13")]
    pub forwarder: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="14")]
    pub commitment_id: ::prost::alloc::string::String,
    #[prost(enumeration="CommitmentEventType", tag="15")]
    pub event_type: i32,
    #[prost(message, optional, tag="16")]
    pub event_data: ::core::option::Option<CommitmentEventData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationEventData {
    #[prost(message, optional, tag="1")]
    pub created: ::core::option::Option<MarketliquidityrewardsCreatedAllocation>,
    #[prost(message, optional, tag="2")]
    pub claimed: ::core::option::Option<MarketliquidityrewardsClaimedRewards>,
    #[prost(message, optional, tag="3")]
    pub decreased: ::core::option::Option<MarketliquidityrewardsDecreasedAllocation>,
    #[prost(message, optional, tag="4")]
    pub increased: ::core::option::Option<MarketliquidityrewardsIncreasedAllocation>,
    #[prost(message, optional, tag="5")]
    pub updated: ::core::option::Option<MarketliquidityrewardsUpdatedAllocation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationsCall {
    #[prost(bytes="vec", tag="1")]
    pub allocator: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub reward_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="3")]
    pub reward_token_amount: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub market_id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="5")]
    pub required_principal_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub required_collateral_token_address: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="7")]
    pub minimum_collateral_per_principal_amount: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub reward_per_loan_principal_amount: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub bid_start_time_min: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub bid_start_time_max: ::prost::alloc::string::String,
    #[prost(string, tag="11")]
    pub allocation_strategy: ::prost::alloc::string::String,
    #[prost(enumeration="AllocationsEventType", tag="12")]
    pub event_type: i32,
    #[prost(message, optional, tag="13")]
    pub event_data: ::core::option::Option<AllocationEventData>,
    #[prost(string, tag="14")]
    pub allocation_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payments {
    #[prost(message, repeated, tag="1")]
    pub payments: ::prost::alloc::vec::Vec<Payment>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payment {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub bid: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub principal: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub interest: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub payment_date: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub outstanding_capital: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub status: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceAllowance {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(enumeration="BalanceAllowanceType", tag="2")]
    pub event_type: i32,
    #[prost(string, tag="3")]
    pub event_value: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub evt_tx_hash: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BalanceAllowances {
    #[prost(message, repeated, tag="1")]
    pub triggers: ::prost::alloc::vec::Vec<BalanceAllowance>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommitmentEventType {
    Created = 0,
    Updated = 1,
    Deleted = 2,
    Extended = 3,
    Exercised = 4,
}
impl CommitmentEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommitmentEventType::Created => "Created",
            CommitmentEventType::Updated => "Updated",
            CommitmentEventType::Deleted => "Deleted",
            CommitmentEventType::Extended => "Extended",
            CommitmentEventType::Exercised => "Exercised",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Created" => Some(Self::Created),
            "Updated" => Some(Self::Updated),
            "Deleted" => Some(Self::Deleted),
            "Extended" => Some(Self::Extended),
            "Exercised" => Some(Self::Exercised),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AllocationsEventType {
    AllocationCreated = 0,
    AllocationClaimed = 1,
    AllocationDecreased = 2,
    AllocationIncreased = 3,
    AllocationUpdated = 4,
}
impl AllocationsEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AllocationsEventType::AllocationCreated => "AllocationCreated",
            AllocationsEventType::AllocationClaimed => "AllocationClaimed",
            AllocationsEventType::AllocationDecreased => "AllocationDecreased",
            AllocationsEventType::AllocationIncreased => "AllocationIncreased",
            AllocationsEventType::AllocationUpdated => "AllocationUpdated",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AllocationCreated" => Some(Self::AllocationCreated),
            "AllocationClaimed" => Some(Self::AllocationClaimed),
            "AllocationDecreased" => Some(Self::AllocationDecreased),
            "AllocationIncreased" => Some(Self::AllocationIncreased),
            "AllocationUpdated" => Some(Self::AllocationUpdated),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BalanceAllowanceType {
    Transfer = 0,
    Approval = 1,
}
impl BalanceAllowanceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BalanceAllowanceType::Transfer => "Transfer",
            BalanceAllowanceType::Approval => "Approval",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Transfer" => Some(Self::Transfer),
            "Approval" => Some(Self::Approval),
            _ => None,
        }
    }
}
// @@protoc_insertion_point(module)
