use prost::Message;
use substreams::pb::sf::substreams::index::v1::Keys;
use substreams::errors::Error;
use crate::pb::contract::v1::Events;

#[substreams::handlers::map]
fn index_events(events: Events) -> Result<Keys, Error> {

    let mut keys = Keys::default();

    if events.encoded_len() > 0 {
        keys.keys.push("events".to_string());
        
        if !events.tellerv2_accepted_bids.is_empty() {
            keys.keys.push("tellerv2_accepted_bids".to_string());
        }
        if !events.tellerv2_admin_changeds.is_empty() {
            keys.keys.push("tellerv2_admin_changeds".to_string());
        }
        if !events.tellerv2_beacon_upgradeds.is_empty() {
            keys.keys.push("tellerv2_beacon_upgradeds".to_string());
        }
        if !events.tellerv2_cancelled_bids.is_empty() {
            keys.keys.push("tellerv2_cancelled_bids".to_string());
        }
        if !events.tellerv2_fee_paids.is_empty() {
            keys.keys.push("tellerv2_fee_paids".to_string());
        }
        if !events.tellerv2_initializeds.is_empty() {
            keys.keys.push("tellerv2_initializeds".to_string());
        }
        if !events.tellerv2_loan_liquidateds.is_empty() {
            keys.keys.push("tellerv2_loan_liquidateds".to_string());
        }
        if !events.tellerv2_loan_repaids.is_empty() {
            keys.keys.push("tellerv2_loan_repaids".to_string());
        }
        if !events.tellerv2_loan_repayments.is_empty() {
            keys.keys.push("tellerv2_loan_repayments".to_string());
        }
        if !events.tellerv2_market_forwarder_approveds.is_empty() {
            keys.keys.push("tellerv2_market_forwarder_approveds".to_string());
        }
        if !events.tellerv2_market_forwarder_renounceds.is_empty() {
            keys.keys.push("tellerv2_market_forwarder_renounceds".to_string());
        }
        if !events.tellerv2_market_owner_cancelled_bids.is_empty() {
            keys.keys.push("tellerv2_market_owner_cancelled_bids".to_string());
        }
        if !events.tellerv2_ownership_transferreds.is_empty() {
            keys.keys.push("tellerv2_ownership_transferreds".to_string());
        }
        if !events.tellerv2_pauseds.is_empty() {
            keys.keys.push("tellerv2_pauseds".to_string());
        }
        if !events.tellerv2_protocol_fee_sets.is_empty() {
            keys.keys.push("tellerv2_protocol_fee_sets".to_string());
        }
        if !events.tellerv2_submitted_bids.is_empty() {
            keys.keys.push("tellerv2_submitted_bids".to_string());
        }
        if !events.tellerv2_trusted_market_forwarder_sets.is_empty() {
            keys.keys.push("tellerv2_trusted_market_forwarder_sets".to_string());
        }
        if !events.tellerv2_unpauseds.is_empty() {
            keys.keys.push("tellerv2_unpauseds".to_string());
        }
        if !events.tellerv2_upgradeds.is_empty() {
            keys.keys.push("tellerv2_upgradeds".to_string());
        }
        if !events.marketregistry_admin_changeds.is_empty() {
            keys.keys.push("marketregistry_admin_changeds".to_string());
        }
        if !events.marketregistry_beacon_upgradeds.is_empty() {
            keys.keys.push("marketregistry_beacon_upgradeds".to_string());
        }
        if !events.marketregistry_borrower_attestations.is_empty() {
            keys.keys.push("marketregistry_borrower_attestations".to_string());
        }
        if !events.marketregistry_borrower_exit_markets.is_empty() {
            keys.keys.push("marketregistry_borrower_exit_markets".to_string());
        }
        if !events.marketregistry_borrower_revocations.is_empty() {
            keys.keys.push("marketregistry_borrower_revocations".to_string());
        }
        if !events.marketregistry_initializeds.is_empty() {
            keys.keys.push("marketregistry_initializeds".to_string());
        }
        if !events.marketregistry_lender_attestations.is_empty() {
            keys.keys.push("marketregistry_lender_attestations".to_string());
        }
        if !events.marketregistry_lender_exit_markets.is_empty() {
            keys.keys.push("marketregistry_lender_exit_markets".to_string());
        }
        if !events.marketregistry_lender_revocations.is_empty() {
            keys.keys.push("marketregistry_lender_revocations".to_string());
        }
        if !events.marketregistry_market_closeds.is_empty() {
            keys.keys.push("marketregistry_market_closeds".to_string());
        }
        if !events.marketregistry_market_createds.is_empty() {
            keys.keys.push("marketregistry_market_createds".to_string());
        }
        if !events.marketregistry_set_bid_expiration_times.is_empty() {
            keys.keys.push("marketregistry_set_bid_expiration_times".to_string());
        }
        if !events.marketregistry_set_market_borrower_attestations.is_empty() {
            keys.keys.push("marketregistry_set_market_borrower_attestations".to_string());
        }
        if !events.marketregistry_set_market_fees.is_empty() {
            keys.keys.push("marketregistry_set_market_fees".to_string());
        }
        if !events.marketregistry_set_market_fee_recipients.is_empty() {
            keys.keys.push("marketregistry_set_market_fee_recipients".to_string());
        }
        if !events.marketregistry_set_market_lender_attestations.is_empty() {
            keys.keys.push("marketregistry_set_market_lender_attestations".to_string());
        }
        if !events.marketregistry_set_market_owners.is_empty() {
            keys.keys.push("marketregistry_set_market_owners".to_string());
        }
        if !events.marketregistry_set_market_payment_types.is_empty() {
            keys.keys.push("marketregistry_set_market_payment_types".to_string());
        }
        if !events.marketregistry_set_market_uris.is_empty() {
            keys.keys.push("marketregistry_set_market_uris".to_string());
        }
        if !events.marketregistry_set_payment_cycles.is_empty() {
            keys.keys.push("marketregistry_set_payment_cycles".to_string());
        }
        if !events.marketregistry_set_payment_cycle_durations.is_empty() {
            keys.keys.push("marketregistry_set_payment_cycle_durations".to_string());
        }
        if !events.marketregistry_set_payment_default_durations.is_empty() {
            keys.keys.push("marketregistry_set_payment_default_durations".to_string());
        }
        if !events.marketregistry_upgradeds.is_empty() {
            keys.keys.push("marketregistry_upgradeds".to_string());
        }
        if !events.lendercommitment_admin_changeds.is_empty() {
            keys.keys.push("lendercommitment_admin_changeds".to_string());
        }
        if !events.lendercommitment_beacon_upgradeds.is_empty() {
            keys.keys.push("lendercommitment_beacon_upgradeds".to_string());
        }
        if !events.lendercommitment_created_commitments.is_empty() {
            keys.keys.push("lendercommitment_created_commitments".to_string());
        }
        if !events.lendercommitment_deleted_commitments.is_empty() {
            keys.keys.push("lendercommitment_deleted_commitments".to_string());
        }
        if !events.lendercommitment_exercised_commitments.is_empty() {
            keys.keys.push("lendercommitment_exercised_commitments".to_string());
        }
        if !events.lendercommitment_initializeds.is_empty() {
            keys.keys.push("lendercommitment_initializeds".to_string());
        }
        if !events.lendercommitment_updated_commitments.is_empty() {
            keys.keys.push("lendercommitment_updated_commitments".to_string());
        }
        if !events.lendercommitment_updated_commitment_borrowers.is_empty() {
            keys.keys.push("lendercommitment_updated_commitment_borrowers".to_string());
        }
        if !events.lendercommitment_upgradeds.is_empty() {
            keys.keys.push("lendercommitment_upgradeds".to_string());
        }
        if !events.lendercommitmentstaging_admin_changeds.is_empty() {
            keys.keys.push("lendercommitmentstaging_admin_changeds".to_string());
        }
        if !events.lendercommitmentstaging_beacon_upgradeds.is_empty() {
            keys.keys.push("lendercommitmentstaging_beacon_upgradeds".to_string());
        }
        if !events.lendercommitmentstaging_created_commitments.is_empty() {
            keys.keys.push("lendercommitmentstaging_created_commitments".to_string());
        }
        if !events.lendercommitmentstaging_deleted_commitments.is_empty() {
            keys.keys.push("lendercommitmentstaging_deleted_commitments".to_string());
        }
        if !events.lendercommitmentstaging_exercised_commitments.is_empty() {
            keys.keys.push("lendercommitmentstaging_exercised_commitments".to_string());
        }
        if !events.lendercommitmentstaging_extension_addeds.is_empty() {
            keys.keys.push("lendercommitmentstaging_extension_addeds".to_string());
        }
        if !events.lendercommitmentstaging_extension_revokeds.is_empty() {
            keys.keys.push("lendercommitmentstaging_extension_revokeds".to_string());
        }
        if !events.lendercommitmentstaging_initializeds.is_empty() {
            keys.keys.push("lendercommitmentstaging_initializeds".to_string());
        }
        if !events.lendercommitmentstaging_updated_commitments.is_empty() {
            keys.keys.push("lendercommitmentstaging_updated_commitments".to_string());
        }
        if !events.lendercommitmentstaging_updated_commitment_borrowers.is_empty() {
            keys.keys.push("lendercommitmentstaging_updated_commitment_borrowers".to_string());
        }
        if !events.lendercommitmentstaging_upgradeds.is_empty() {
            keys.keys.push("lendercommitmentstaging_upgradeds".to_string());
        }
        if !events.collateralmanager_admin_changeds.is_empty() {
            keys.keys.push("collateralmanager_admin_changeds".to_string());
        }
        if !events.collateralmanager_beacon_upgradeds.is_empty() {
            keys.keys.push("collateralmanager_beacon_upgradeds".to_string());
        }
        if !events.collateralmanager_collateral_claimeds.is_empty() {
            keys.keys.push("collateralmanager_collateral_claimeds".to_string());
        }
        if !events.collateralmanager_collateral_committeds.is_empty() {
            keys.keys.push("collateralmanager_collateral_committeds".to_string());
        }
        if !events.collateralmanager_collateral_depositeds.is_empty() {
            keys.keys.push("collateralmanager_collateral_depositeds".to_string());
        }
        if !events.collateralmanager_collateral_escrow_deployeds.is_empty() {
            keys.keys.push("collateralmanager_collateral_escrow_deployeds".to_string());
        }
        if !events.collateralmanager_collateral_withdrawns.is_empty() {
            keys.keys.push("collateralmanager_collateral_withdrawns".to_string());
        }
        if !events.collateralmanager_initializeds.is_empty() {
            keys.keys.push("collateralmanager_initializeds".to_string());
        }
        if !events.collateralmanager_ownership_transferreds.is_empty() {
            keys.keys.push("collateralmanager_ownership_transferreds".to_string());
        }
        if !events.collateralmanager_upgradeds.is_empty() {
            keys.keys.push("collateralmanager_upgradeds".to_string());
        }
        if !events.lendermanager_admin_changeds.is_empty() {
            keys.keys.push("lendermanager_admin_changeds".to_string());
        }
        if !events.lendermanager_approvals.is_empty() {
            keys.keys.push("lendermanager_approvals".to_string());
        }
        if !events.lendermanager_approval_for_alls.is_empty() {
            keys.keys.push("lendermanager_approval_for_alls".to_string());
        }
        if !events.lendermanager_beacon_upgradeds.is_empty() {
            keys.keys.push("lendermanager_beacon_upgradeds".to_string());
        }
        if !events.lendermanager_initializeds.is_empty() {
            keys.keys.push("lendermanager_initializeds".to_string());
        }
        if !events.lendermanager_ownership_transferreds.is_empty() {
            keys.keys.push("lendermanager_ownership_transferreds".to_string());
        }
        if !events.lendermanager_transfers.is_empty() {
            keys.keys.push("lendermanager_transfers".to_string());
        }
        if !events.lendermanager_upgradeds.is_empty() {
            keys.keys.push("lendermanager_upgradeds".to_string());
        }
        if !events.marketliquidityrewards_admin_changeds.is_empty() {
            keys.keys.push("marketliquidityrewards_admin_changeds".to_string());
        }
        if !events.marketliquidityrewards_beacon_upgradeds.is_empty() {
            keys.keys.push("marketliquidityrewards_beacon_upgradeds".to_string());
        }
        if !events.marketliquidityrewards_claimed_rewards.is_empty() {
            keys.keys.push("marketliquidityrewards_claimed_rewards".to_string());
        }
        if !events.marketliquidityrewards_created_allocations.is_empty() {
            keys.keys.push("marketliquidityrewards_created_allocations".to_string());
        }
        if !events.marketliquidityrewards_decreased_allocations.is_empty() {
            keys.keys.push("marketliquidityrewards_decreased_allocations".to_string());
        }
        if !events.marketliquidityrewards_deleted_allocations.is_empty() {
            keys.keys.push("marketliquidityrewards_deleted_allocations".to_string());
        }
        if !events.marketliquidityrewards_increased_allocations.is_empty() {
            keys.keys.push("marketliquidityrewards_increased_allocations".to_string());
        }
        if !events.marketliquidityrewards_initializeds.is_empty() {
            keys.keys.push("marketliquidityrewards_initializeds".to_string());
        }
        if !events.marketliquidityrewards_updated_allocations.is_empty() {
            keys.keys.push("marketliquidityrewards_updated_allocations".to_string());
        }
        if !events.marketliquidityrewards_upgradeds.is_empty() {
            keys.keys.push("marketliquidityrewards_upgradeds".to_string());
        }
    }
    



    Ok(keys)
}