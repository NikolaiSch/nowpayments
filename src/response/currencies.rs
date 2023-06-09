use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize, Debug)]
pub struct Currencies {
    pub currencies: Vec<String>
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FullCurrencies {
    pub currencies: Vec<SingleCurrency>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleCurrency {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub enable: bool,
    #[serde(rename = "wallet_regex")]
    pub wallet_regex: String,
    pub priority: i64,
    #[serde(rename = "extra_id_exists")]
    pub extra_id_exists: bool,
    #[serde(rename = "extra_id_regex")]
    pub extra_id_regex: Option<String>,
    #[serde(rename = "logo_url")]
    pub logo_url: String,
    pub track: bool,
    #[serde(rename = "cg_id")]
    pub cg_id: String,
    #[serde(rename = "is_maxlimit")]
    pub is_maxlimit: bool,
    pub network: Option<String>,
    #[serde(rename = "smart_contract")]
    pub smart_contract: Option<String>,
    #[serde(rename = "network_precision")]
    pub network_precision: Option<String>,
    #[serde(rename = "explorer_link_hash")]
    pub explorer_link_hash: Option<String>,
    pub precision: i64,
    pub ticker: Option<String>,
    #[serde(rename = "is_defi")]
    pub is_defi: bool,
    #[serde(rename = "is_popular")]
    pub is_popular: bool,
    #[serde(rename = "is_stable")]
    pub is_stable: bool,
    #[serde(rename = "available_for_to_conversion")]
    pub available_for_to_conversion: bool,
}
