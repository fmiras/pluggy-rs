use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]

pub enum ExecutionErrorCodes {
    InvalidCredentials,
    AlreadyLoggedIn,
    UnexpectedError,
    InvalidCredentialsMfa,
    SiteNotAvailable,
    AccountLocked,
    AccountCredentialsReset,
    ConnectionError,
    AccountNeedsAction,
    UserAuthorizationPending,
    UserAuthorizationNotGranted,
    UserInputTimeout,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionErrorResult {
    pub code: ExecutionErrorCodes,
    pub message: String,
    pub provider_message: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionStatus {
    LoginInProgress,
    WaitingUserInput,
    WaitingUserAction,
    LoginMfaInProgress,
    AccountsInProgress,
    TransactionsInProgress,
    PaymentDataInProgress,
    CreditcardsInProgress,
    InvestmentsInProgress,
    InvestmentsTransactionsInProgress,
    OpportunitiesInProgress,
    IdentityInProgress,
    MergeError,
    Error,
    Success,
    PartialSuccess,
    Creating,
    CreateError,
    Created,
}
