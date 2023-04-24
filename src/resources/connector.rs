use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectorType {
    PersonalBank,
    BusinessBank,
    Invoice,
    Investment,
    Telecommunication,
    DigitalEconomy,
    PaymentAccount,
    Other,
}

#[derive(Debug, Deserialize)]
pub enum Country {
    AR,
    BR,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialType {
    Number,
    Password,
    Text,
    Image,
    Select,
    #[serde(rename = "ethaddress")]
    EthAddress,
}

#[derive(Debug, Deserialize)]
pub struct CredentialSelectOption {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ConnectorCredential {
    pub label: String,
    pub name: String,
    #[serde(rename = "type")]
    pub credential_type: Option<CredentialType>,
    pub mfa: Option<bool>,
    pub data: Option<String>,
    pub assistive_text: Option<String>,
    pub options: Option<Vec<CredentialSelectOption>>,
    pub validation: Option<String>,
    pub validation_message: Option<String>,
    pub placeholder: Option<String>,
    pub optional: Option<bool>,
    pub instructions: Option<String>,
    pub expires_at: Option<String>, // date
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectorStatus {
    Online,
    Offline,
    Unstable,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConnectorStage {
    Beta,
}

#[derive(Debug, Deserialize)]
pub struct ConnectorHealth {
    pub status: ConnectorStatus,
    pub stage: Option<ConnectorStage>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProductType {
    Accounts,
    CreditCards,
    Transactions,
    PaymentData,
    Investments,
    InvestmentsTransactions,
    Identity,
    BrokerageNote,
    Opportunities,
    Portfolio,
    IncomeReports,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connector {
    pub id: i32,
    pub name: String,
    pub institution_url: String,
    pub image_url: String,
    pub primary_color: String,
    #[serde(rename = "type")]
    pub connector_type: ConnectorType,
    pub country: Country,
    pub credentials: Vec<ConnectorCredential>,
    #[serde(rename = "hasMFA")]
    pub has_mfa: bool,
    pub oauth: Option<bool>,
    pub oauth_url: Option<String>,
    pub health: Option<ConnectorHealth>,
    pub reset_password_url: Option<String>,
    pub products: Vec<ProductType>,
    pub created_at: String, // date
}
