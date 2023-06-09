use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Currencies {
    pub currencies: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullCurrencies {
    pub currencies: Vec<SingleCurrency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SingleCurrency {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub enable: bool,
    pub wallet_regex: String,
    pub priority: i64,
    pub extra_id_exists: bool,
    pub extra_id_regex: Option<String>,
    pub logo_url: String,
    pub track: bool,
    pub cg_id: String,
    pub is_maxlimit: bool,
    pub network: Option<String>,
    pub smart_contract: Option<String>,
    pub network_precision: Option<String>,
    pub explorer_link_hash: Option<String>,
    pub precision: i64,
    pub ticker: Option<String>,
    pub is_defi: bool,
    pub is_popular: bool,
    pub is_stable: bool,
    pub available_for_to_conversion: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SelectedCurrencies {
    #[serde(rename = "selectedCurrencies")]
    pub selected_currencies: Vec<String>,
}
