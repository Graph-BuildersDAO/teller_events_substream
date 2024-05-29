use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GraphOutParams {
    pub start_block: u64,
    pub market_registry: String,
    pub tellerv2: String,
    pub collateral_manager: String,
    // pub is_rolloverable: bool
}

#[derive(Debug, Deserialize)]
pub struct MapEventParams {
    pub start_block: Option<u64>,
    pub seed: Option<bool>,
    pub market_registry: Option<String>,
    pub tellerv2: Option<String>,
    pub collateral_manager: Option<String>,
    pub lendercommitment: Option<String>,
    pub lendercommitmentstg: Option<String>,
    pub lendermanager: Option<String>,
    pub marketliquidityrewards: Option<String>,
    pub block_handler_enabled: Option<bool>
}

#[derive(Debug, Deserialize)]
pub struct MapBalanceAllowanceParams {
    pub block_handler_enabled: bool
}

#[derive(Debug, Deserialize)]
pub struct StoreBidsInterStatusParams {
    pub block_handler_enabled: bool
}

#[derive(Debug, Deserialize)]
pub struct ModuleParams {
    pub start_block: Option<u64>,
    pub seed: Option<bool>,
    pub market_registry: Option<String>,
    pub tellerv2: Option<String>,
    pub collateral_manager: Option<String>,
    pub lendercommitment: Option<String>,
    pub lendercommitmentstg: Option<String>,
    pub lendermanager: Option<String>,
    pub marketliquidityrewards: Option<String>,
    pub block_handler_enabled: Option<bool>
}