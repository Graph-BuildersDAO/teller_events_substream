use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use serde_qs;
use substreams_ethereum::Event;

use crate::pb::contract::v1 as contract;
use crate::abi;
use crate::common::conversion::convert_address_to_string;
use crate::types::params::MapEventParams;

fn map_tellerv2_events(blk: &eth::Block, events: &mut contract::Events, contract: String) {

    let tracked_contract = Hex::decode(contract).unwrap();
    
    events.tellerv2_accepted_bids.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::AcceptedBid::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2AcceptedBid {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                                lender: event.lender,
                                evt_address: log.address.clone(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_admin_changeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::AdminChanged::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2AdminChanged {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_admin: event.new_admin,
                                previous_admin: event.previous_admin,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_beacon_upgradeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::BeaconUpgraded::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2BeaconUpgraded {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                beacon: event.beacon,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_cancelled_bids.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::CancelledBid::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2CancelledBid {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_fee_paids.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::FeePaid::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2FeePaid {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                amount: event.amount.to_string(),
                                bid_id: event.bid_id.to_string(),
                                fee_type: Hex(event.fee_type.hash).to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_initializeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::Initialized::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2Initialized {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                version: event.version.to_u64(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_loan_liquidateds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::LoanLiquidated::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2LoanLiquidated {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                                liquidator: event.liquidator,
                                log_ordinal: log.ordinal
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_loan_repaids.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::LoanRepaid::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2LoanRepaid {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                                evt_address: log.address.clone(),
                                log_ordinal: log.ordinal
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_loan_repayments.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::LoanRepayment::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2LoanRepayment {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                                log_ordinal: log.ordinal,
                                evt_address: log.address.clone()
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_market_forwarder_approveds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::tellerv2_contract::events::MarketForwarderApproved::match_and_decode(log) {
                        return Some(contract::Tellerv2MarketForwarderApproved {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            forwarder: event.forwarder,
                            market_id: event.market_id.to_string(),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.tellerv2_market_forwarder_renounceds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::tellerv2_contract::events::MarketForwarderRenounced::match_and_decode(log) {
                        return Some(contract::Tellerv2MarketForwarderRenounced {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            forwarder: event.forwarder,
                            market_id: event.market_id.to_string(),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.tellerv2_market_owner_cancelled_bids.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::tellerv2_contract::events::MarketOwnerCancelledBid::match_and_decode(log) {
                        return Some(contract::Tellerv2MarketOwnerCancelledBid {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            bid_id: event.bid_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.tellerv2_ownership_transferreds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::OwnershipTransferred::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::Tellerv2OwnershipTransferred {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_owner: event.new_owner,
                                previous_owner: event.previous_owner,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_pauseds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::Paused::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2Paused {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                account: event.account,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_protocol_fee_sets.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::ProtocolFeeSet::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2ProtocolFeeSet {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_fee: event.new_fee.to_u64(),
                                old_fee: event.old_fee.to_u64(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_submitted_bids.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::SubmittedBid::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2SubmittedBid {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                bid_id: event.bid_id.to_string(),
                                borrower: event.borrower,
                                metadata_uri: Vec::from(event.metadata_uri),
                                receiver: event.receiver,
                                evt_address: log.address.clone(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_trusted_market_forwarder_sets.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::tellerv2_contract::events::TrustedMarketForwarderSet::match_and_decode(log) {
                        return Some(contract::Tellerv2TrustedMarketForwarderSet {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            forwarder: event.forwarder,
                            market_id: event.market_id.to_string(),
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.tellerv2_unpauseds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::Unpaused::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2Unpaused {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                account: event.account,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.tellerv2_upgradeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::tellerv2_contract::events::Upgraded::match_and_decode(log)
                        {
                            return Some(contract::Tellerv2Upgraded {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                implementation: event.implementation,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}

fn map_marketregistry_events(blk: &eth::Block, events: &mut contract::Events, contract: String) {

    let tracked_contract = Hex::decode(contract).unwrap();

    events.marketregistry_admin_changeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::AdminChanged::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::MarketregistryAdminChanged {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_admin: event.new_admin,
                                previous_admin: event.previous_admin,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_beacon_upgradeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::BeaconUpgraded::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::MarketregistryBeaconUpgraded {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                beacon: event.beacon,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_borrower_attestations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::BorrowerAttestation::match_and_decode(log) {
                        return Some(contract::MarketregistryBorrowerAttestation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            borrower: event.borrower,
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_borrower_exit_markets.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::BorrowerExitMarket::match_and_decode(log) {
                        return Some(contract::MarketregistryBorrowerExitMarket {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            borrower: event.borrower,
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_borrower_revocations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::BorrowerRevocation::match_and_decode(log) {
                        return Some(contract::MarketregistryBorrowerRevocation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            borrower: event.borrower,
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_initializeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::Initialized::match_and_decode(log)
                        {
                            return Some(contract::MarketregistryInitialized {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                version: event.version.to_u64(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_lender_attestations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::LenderAttestation::match_and_decode(log) {
                        return Some(contract::MarketregistryLenderAttestation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            lender: event.lender,
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_lender_exit_markets.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::LenderExitMarket::match_and_decode(log) {
                        return Some(contract::MarketregistryLenderExitMarket {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            lender: event.lender,
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_lender_revocations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::LenderRevocation::match_and_decode(log) {
                        return Some(contract::MarketregistryLenderRevocation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            lender: event.lender,
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_market_closeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::MarketClosed::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::MarketregistryMarketClosed {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                market_id: event.market_id.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_market_createds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::MarketCreated::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::MarketregistryMarketCreated {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                market_id: event.market_id.to_string(),
                                owner: event.owner,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_set_bid_expiration_times.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::SetBidExpirationTime::match_and_decode(log) {
                        return Some(contract::MarketregistrySetBidExpirationTime {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            duration: event.duration.to_u64(),
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_set_market_borrower_attestations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::SetMarketBorrowerAttestation::match_and_decode(log) {
                        return Some(contract::MarketregistrySetMarketBorrowerAttestation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            market_id: event.market_id.to_string(),
                            required: event.required,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_set_market_fees.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::SetMarketFee::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::MarketregistrySetMarketFee {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                fee_pct: event.fee_pct.to_u64(),
                                market_id: event.market_id.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_set_market_fee_recipients.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::SetMarketFeeRecipient::match_and_decode(log) {
                        return Some(contract::MarketregistrySetMarketFeeRecipient {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            market_id: event.market_id.to_string(),
                            new_recipient: event.new_recipient,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_set_market_lender_attestations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::SetMarketLenderAttestation::match_and_decode(log) {
                        return Some(contract::MarketregistrySetMarketLenderAttestation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            market_id: event.market_id.to_string(),
                            required: event.required,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_set_market_owners.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::SetMarketOwner::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::MarketregistrySetMarketOwner {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                market_id: event.market_id.to_string(),
                                new_owner: event.new_owner,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_set_market_payment_types.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::SetMarketPaymentType::match_and_decode(log) {
                        return Some(contract::MarketregistrySetMarketPaymentType {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            market_id: event.market_id.to_string(),
                            payment_type: event.payment_type.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_set_market_uris.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::SetMarketUri::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::MarketregistrySetMarketUri {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                market_id: event.market_id.to_string(),
                                uri: event.uri,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_set_payment_cycles.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::SetPaymentCycle::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::MarketregistrySetPaymentCycle {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                market_id: event.market_id.to_string(),
                                payment_cycle_type: event.payment_cycle_type.to_u64(),
                                value: event.value.to_u64(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.marketregistry_set_payment_cycle_durations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::SetPaymentCycleDuration::match_and_decode(log) {
                        return Some(contract::MarketregistrySetPaymentCycleDuration {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            duration: event.duration.to_u64(),
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_set_payment_default_durations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketregistry_contract::events::SetPaymentDefaultDuration::match_and_decode(log) {
                        return Some(contract::MarketregistrySetPaymentDefaultDuration {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            duration: event.duration.to_u64(),
                            market_id: event.market_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketregistry_upgradeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::marketregistry_contract::events::Upgraded::match_and_decode(log)
                        {
                            return Some(contract::MarketregistryUpgraded {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                implementation: event.implementation,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}

fn map_lendercommitment_events(blk: &eth::Block, events: &mut contract::Events, contract: String) {

    let tracked_contract = Hex::decode(contract).unwrap();

    events.lendercommitment_admin_changeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendercommitment_contract::events::AdminChanged::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::LendercommitmentAdminChanged {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_admin: event.new_admin,
                                previous_admin: event.previous_admin,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.lendercommitment_beacon_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitment_contract::events::BeaconUpgraded::match_and_decode(log) {
                        return Some(contract::LendercommitmentBeaconUpgraded {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitment_created_commitments.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitment_contract::events::CreatedCommitment::match_and_decode(log) {
                        return Some(contract::LendercommitmentCreatedCommitment {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            log_ordinal: log.ordinal,
                            commitment_id: event.commitment_id.to_string(),
                            lender: event.lender,
                            lending_token: event.lending_token,
                            market_id: event.market_id.to_string(),
                            token_amount: event.token_amount.to_string(),
                            evt_address: log.address.clone(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitment_deleted_commitments.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitment_contract::events::DeletedCommitment::match_and_decode(log) {
                        return Some(contract::LendercommitmentDeletedCommitment {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            commitment_id: event.commitment_id.to_string(),
                            log_ordinal: log.ordinal,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitment_exercised_commitments.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitment_contract::events::ExercisedCommitment::match_and_decode(log) {
                        return Some(contract::LendercommitmentExercisedCommitment {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            bid_id: event.bid_id.to_string(),
                            borrower: event.borrower,
                            commitment_id: event.commitment_id.to_string(),
                            token_amount: event.token_amount.to_string(),
                            evt_address: log.address.clone(),
                            log_ordinal: log.ordinal
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitment_initializeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendercommitment_contract::events::Initialized::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::LendercommitmentInitialized {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                version: event.version.to_u64(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.lendercommitment_updated_commitments.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitment_contract::events::UpdatedCommitment::match_and_decode(log) {
                        return Some(contract::LendercommitmentUpdatedCommitment {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            commitment_id: event.commitment_id.to_string(),
                            lender: event.lender,
                            lending_token: event.lending_token,
                            market_id: event.market_id.to_string(),
                            token_amount: event.token_amount.to_string(),
                            evt_address: log.address.clone(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitment_updated_commitment_borrowers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitment_contract::events::UpdatedCommitmentBorrowers::match_and_decode(log) {
                        return Some(contract::LendercommitmentUpdatedCommitmentBorrowers {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            commitment_id: event.commitment_id.to_string(),
                            evt_address: log.address.clone(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitment_upgradeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendercommitment_contract::events::Upgraded::match_and_decode(log)
                        {
                            return Some(contract::LendercommitmentUpgraded {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                implementation: event.implementation,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}

fn map_lendercommitmentstaging_events(blk: &eth::Block, events: &mut contract::Events, contract: String) {

    let tracked_contract = Hex::decode(contract).unwrap();

    events.lendercommitmentstaging_admin_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::AdminChanged::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingAdminChanged {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_beacon_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::BeaconUpgraded::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingBeaconUpgraded {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_created_commitments.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::CreatedCommitment::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingCreatedCommitment {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            log_ordinal: log.ordinal,
                            commitment_id: event.commitment_id.to_string(),
                            lender: event.lender,
                            lending_token: event.lending_token,
                            market_id: event.market_id.to_string(),
                            token_amount: event.token_amount.to_string(),
                            evt_address: log.address.clone()
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_deleted_commitments.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::DeletedCommitment::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingDeletedCommitment {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            commitment_id: event.commitment_id.to_string(),
                            log_ordinal: log.ordinal,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_exercised_commitments.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::ExercisedCommitment::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingExercisedCommitment {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            bid_id: event.bid_id.to_string(),
                            borrower: event.borrower,
                            commitment_id: event.commitment_id.to_string(),
                            token_amount: event.token_amount.to_string(),
                            evt_address: log.address.clone(),
                            log_ordinal: log.ordinal
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_extension_addeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::ExtensionAdded::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingExtensionAdded {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            extension: event.extension,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_extension_revokeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::ExtensionRevoked::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingExtensionRevoked {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            extension: event.extension,
                            sender: event.sender,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingInitialized {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_updated_commitments.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::UpdatedCommitment::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingUpdatedCommitment {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            commitment_id: event.commitment_id.to_string(),
                            lender: event.lender,
                            lending_token: event.lending_token,
                            market_id: event.market_id.to_string(),
                            token_amount: event.token_amount.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_updated_commitment_borrowers.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::UpdatedCommitmentBorrowers::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingUpdatedCommitmentBorrowers {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            commitment_id: event.commitment_id.to_string(),
                            evt_address: log.address.clone(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendercommitmentstaging_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendercommitmentstaging_contract::events::Upgraded::match_and_decode(log) {
                        return Some(contract::LendercommitmentstagingUpgraded {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
}

fn map_collateralmanager_events(blk: &eth::Block, events: &mut contract::Events, contract: String) {

    let tracked_contract = Hex::decode(contract).unwrap();

    events.collateralmanager_admin_changeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::collateralmanager_contract::events::AdminChanged::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::CollateralmanagerAdminChanged {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_admin: event.new_admin,
                                previous_admin: event.previous_admin,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.collateralmanager_beacon_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::collateralmanager_contract::events::BeaconUpgraded::match_and_decode(log) {
                        return Some(contract::CollateralmanagerBeaconUpgraded {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.collateralmanager_collateral_claimeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::collateralmanager_contract::events::CollateralClaimed::match_and_decode(log) {
                        return Some(contract::CollateralmanagerCollateralClaimed {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_bid_id: event.u_bid_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.collateralmanager_collateral_committeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::collateralmanager_contract::events::CollateralCommitted::match_and_decode(log) {
                        return Some(contract::CollateralmanagerCollateralCommitted {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_amount: event.u_amount.to_string(),
                            u_bid_id: event.u_bid_id.to_string(),
                            u_collateral_address: event.u_collateral_address,
                            u_token_id: event.u_token_id.to_string(),
                            u_type: event.u_type.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.collateralmanager_collateral_depositeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::collateralmanager_contract::events::CollateralDeposited::match_and_decode(log) {
                        return Some(contract::CollateralmanagerCollateralDeposited {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_amount: event.u_amount.to_string(),
                            u_bid_id: event.u_bid_id.to_string(),
                            u_collateral_address: event.u_collateral_address,
                            u_token_id: event.u_token_id.to_string(),
                            u_type: event.u_type.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.collateralmanager_collateral_escrow_deployeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::collateralmanager_contract::events::CollateralEscrowDeployed::match_and_decode(log) {
                        return Some(contract::CollateralmanagerCollateralEscrowDeployed {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_bid_id: event.u_bid_id.to_string(),
                            u_collateral_escrow: event.u_collateral_escrow,
                        });
                    }

                    None
                })
        })
        .collect());
    events.collateralmanager_collateral_withdrawns.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::collateralmanager_contract::events::CollateralWithdrawn::match_and_decode(log) {
                        return Some(contract::CollateralmanagerCollateralWithdrawn {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            u_amount: event.u_amount.to_string(),
                            u_bid_id: event.u_bid_id.to_string(),
                            u_collateral_address: event.u_collateral_address,
                            u_recipient: event.u_recipient,
                            u_token_id: event.u_token_id.to_string(),
                            u_type: event.u_type.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.collateralmanager_initializeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::collateralmanager_contract::events::Initialized::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::CollateralmanagerInitialized {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                version: event.version.to_u64(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.collateralmanager_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::collateralmanager_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::CollateralmanagerOwnershipTransferred {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.collateralmanager_upgradeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::collateralmanager_contract::events::Upgraded::match_and_decode(log)
                        {
                            return Some(contract::CollateralmanagerUpgraded {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                implementation: event.implementation,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}

fn map_lendermanager_events(blk: &eth::Block, events: &mut contract::Events, contract: String) {

    let tracked_contract = Hex::decode(contract).unwrap();

    events.lendermanager_admin_changeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendermanager_contract::events::AdminChanged::match_and_decode(log)
                        {
                            return Some(contract::LendermanagerAdminChanged {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                new_admin: event.new_admin,
                                previous_admin: event.previous_admin,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.lendermanager_approvals.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendermanager_contract::events::Approval::match_and_decode(log)
                        {
                            return Some(contract::LendermanagerApproval {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                approved: event.approved,
                                owner: event.owner,
                                token_id: event.token_id.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.lendermanager_approval_for_alls.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendermanager_contract::events::ApprovalForAll::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::LendermanagerApprovalForAll {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                approved: event.approved,
                                operator: event.operator,
                                owner: event.owner,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.lendermanager_beacon_upgradeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendermanager_contract::events::BeaconUpgraded::match_and_decode(
                                log,
                            )
                        {
                            return Some(contract::LendermanagerBeaconUpgraded {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                beacon: event.beacon,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.lendermanager_initializeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendermanager_contract::events::Initialized::match_and_decode(log)
                        {
                            return Some(contract::LendermanagerInitialized {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                version: event.version.to_u64(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.lendermanager_ownership_transferreds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::lendermanager_contract::events::OwnershipTransferred::match_and_decode(log) {
                        return Some(contract::LendermanagerOwnershipTransferred {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_owner: event.new_owner,
                            previous_owner: event.previous_owner,
                        });
                    }

                    None
                })
        })
        .collect());
    events.lendermanager_transfers.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendermanager_contract::events::Transfer::match_and_decode(log)
                        {
                            return Some(contract::LendermanagerTransfer {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                from: event.from,
                                to: event.to,
                                token_id: event.token_id.to_string(),
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
    events.lendermanager_upgradeds.append(
        &mut blk
            .receipts()
            .flat_map(|view| {
                view.receipt
                    .logs
                    .iter()
                    .filter(|log| log.address == tracked_contract)
                    .filter_map(|log| {
                        if let Some(event) =
                            abi::lendermanager_contract::events::Upgraded::match_and_decode(log)
                        {
                            return Some(contract::LendermanagerUpgraded {
                                evt_tx_hash: convert_address_to_string(
                                    &view.transaction.hash,
                                ),
                                evt_index: log.block_index,
                                evt_block_time: Some(blk.timestamp().to_owned()),
                                evt_block_number: blk.number,
                                implementation: event.implementation,
                            });
                        }

                        None
                    })
            })
            .collect(),
    );
}

fn map_marketliquidityrewards_events(blk: &eth::Block, events: &mut contract::Events, contract: String) {

    let tracked_contract = Hex::decode(contract).unwrap();

    events.marketliquidityrewards_admin_changeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::AdminChanged::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsAdminChanged {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            new_admin: event.new_admin,
                            previous_admin: event.previous_admin,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_beacon_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::BeaconUpgraded::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsBeaconUpgraded {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            beacon: event.beacon,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_claimed_rewards.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::ClaimedRewards::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsClaimedRewards {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            allocation_id: event.allocation_id.to_string(),
                            amount: event.amount.to_string(),
                            bid_id: event.bid_id.to_string(),
                            recipient: event.recipient,
                            evt_address: log.address.clone(),
                            log_ordinal: log.ordinal,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_created_allocations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::CreatedAllocation::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsCreatedAllocation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            allocation_id: event.allocation_id.to_string(),
                            allocator: event.allocator,
                            market_id: event.market_id.to_string(),
                            evt_address: log.address.clone(),
                            log_ordinal: log.ordinal,

                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_decreased_allocations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::DecreasedAllocation::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsDecreasedAllocation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            allocation_id: event.allocation_id.to_string(),
                            amount: event.amount.to_string(),
                            evt_address: log.address.clone(),
                            log_ordinal: log.ordinal,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_deleted_allocations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::DeletedAllocation::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsDeletedAllocation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            allocation_id: event.allocation_id.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_increased_allocations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::IncreasedAllocation::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsIncreasedAllocation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            allocation_id: event.allocation_id.to_string(),
                            amount: event.amount.to_string(),
                            evt_address: log.address.clone(),
                            log_ordinal: log.ordinal,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_initializeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::Initialized::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsInitialized {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            version: event.version.to_u64(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_updated_allocations.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::UpdatedAllocation::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsUpdatedAllocation {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            allocation_id: event.allocation_id.to_string(),
                            evt_address: log.address.clone(),
                            log_ordinal: log.ordinal,
                        });
                    }

                    None
                })
        })
        .collect());
    events.marketliquidityrewards_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == tracked_contract)
                .filter_map(|log| {
                    if let Some(event) = abi::marketliquidityrewards_contract::events::Upgraded::match_and_decode(log) {
                        return Some(contract::MarketliquidityrewardsUpgraded {
                            evt_tx_hash: convert_address_to_string(&view.transaction.hash),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn map_events(params: String, blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    
    let module_params: MapEventParams = serde_qs::from_str(params.as_str()).unwrap();

    map_tellerv2_events(&blk, &mut events, module_params.tellerv2.unwrap());
    map_marketregistry_events(&blk, &mut events, module_params.market_registry.unwrap());
    map_lendercommitment_events(&blk, &mut events, module_params.lendercommitment.unwrap());
    map_lendercommitmentstaging_events(&blk, &mut events, module_params.lendercommitmentstg.unwrap());
    map_collateralmanager_events(&blk, &mut events, module_params.collateral_manager.unwrap());
    map_lendermanager_events(&blk, &mut events, module_params.lendermanager.unwrap());
    map_marketliquidityrewards_events(&blk, &mut events, module_params.marketliquidityrewards.unwrap());

    Ok(events)
}
