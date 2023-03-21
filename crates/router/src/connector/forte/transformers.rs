use masking::Secret;
use serde::{Deserialize, Serialize};

use crate::{
    connector::utils::PaymentsAuthorizeRequestData,
    core::errors,
    types::{self, api, storage::enums},
};
#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ForteAddressData {
    first_name: Option<Secret<String>>,
    last_name: Option<Secret<String>>,
}
//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Eq, PartialEq)]
pub struct FortePaymentsRequest {
    authorization_amount: i64,
    subtotal_amount:i64,
   billing_address:ForteAddressData,
    card: ForteCard2,
}

#[derive(Default, Debug, Serialize,Deserialize, Eq, PartialEq,Clone)]
pub struct ForteCard {
    name: Secret<String>,
    number: Secret<String, common_utils::pii::CardNumber>,
    expiry_month: Secret<String>,
    expiry_year: Secret<String>,
    cvc: Secret<String>,
    complete: bool,
}
#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ForteCard2 {
    card_type: Option<String>,
    name_on_card: Secret<String>,
    account_number: Secret<String, common_utils::pii::CardNumber>,  
    expire_month: Secret<String>,
    expire_year: Secret<String>,
    card_verification_value: Secret<String>,
}

impl TryFrom<&types::PaymentsAuthorizeRouterData> for FortePaymentsRequest {
    type Error = error_stack::Report<errors::ConnectorError>;
    // fn try_from(item: &types::PaymentsAuthorizeRouterData) -> Result<Self, Self::Error> {
    //     match item.request.payment_method_data.clone() {
    //         api::PaymentMethodData::Card(req_card) => {
    //             let card = ForteCard2 {
    //                 name_on_card: req_card.card_holder_name,
    //                 account_number: req_card.card_number,
    //                 card_type: req_card.card_issuer,
    //                 expire_month: req_card.card_exp_month,
    //                 expire_year: req_card.card_exp_year,
    //                 card_verification_value: req_card.card_cvc,
    //             };
    //             Ok(Self {
    //                 authorization_amount: item.request.amount,
    //                 subtotal_amount:item.request.amount,
    //                 card,
    //                 billing_address,
    //             })
    //         }
    //         _ => Err(errors::ConnectorError::NotImplemented("Payment methods".to_string()).into()),
    //     }
    // }
    fn try_from(item: &types::PaymentsAuthorizeRouterData) -> Result<Self, Self::Error> {
        let shipping_address = match item.address.shipping.clone() {
            Some (mut shipping) => ForteAddressData {
                    first_name: shipping.address.as_mut().and_then(|a| a.first_name.take()),
                    last_name: shipping.address.as_mut().and_then(|a| a.last_name.take()),
            },
            None => ForteAddressData::default(),
        };

        let card = match item.request.payment_method_data.clone() {
            api::PaymentMethodData::Card(req_card) => Ok(ForteCard2{
                    name_on_card: req_card.card_holder_name,
                    account_number: req_card.card_number,
                    card_type: req_card.card_issuer,
                    expire_month: req_card.card_exp_month,
                    expire_year: req_card.card_exp_year,
                    card_verification_value: req_card.card_cvc,
            }),
            _ => Err(errors::ConnectorError::NotImplemented(
                "payment method".to_string(),
            )),
        }?;
        Ok(Self {
                    authorization_amount: item.request.amount,
                    subtotal_amount:item.request.amount,
                    card,
                    billing_address: shipping_address,
        })
    }
}

//TODO: Fill the struct with respective fields
// Auth Struct
pub struct ForteAuthType {
    pub(super) api_key: String,
    pub(super)  key1:String,

   
}

impl TryFrom<&types::ConnectorAuthType> for ForteAuthType {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(auth_type: &types::ConnectorAuthType) -> Result<Self, Self::Error> {
        match auth_type {
            types::ConnectorAuthType::BodyKey { api_key,key1} => Ok(Self {
                api_key: api_key.to_string(),
                key1:key1.to_string()

            }),
            
            _ => Err(errors::ConnectorError::FailedToObtainAuthType.into()),
        }
    }
}
// PaymentsResponse
//TODO: Append the remaining status flags
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FortePaymentStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<FortePaymentStatus> for enums::AttemptStatus {
    fn from(item: FortePaymentStatus) -> Self {
        match item {
            FortePaymentStatus::Succeeded => Self::Charged,
            FortePaymentStatus::Failed => Self::Failure,
            FortePaymentStatus::Processing => Self::Authorizing,
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FortePaymentsResponse {
    transaction_id: String,
    location_id: Option<String>,
    organization_id:Option<String>,
    status:Option<String>,
    action: Option<String>,
    authorization_amount: i64,
    authorization_code: Option<String>,
    entered_by: Option<String>,
    received_date:Option<String>,
    billing_address: ForteAddressData,
    card: ForteCard2,
    response: ForteResponse,
    links: ForteLinks,
}

impl<F, T>
    TryFrom<types::ResponseRouterData<F, FortePaymentsResponse, T, types::PaymentsResponseData>>
    for types::RouterData<F, T, types::PaymentsResponseData>
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: types::ResponseRouterData<F, FortePaymentsResponse, T, types::PaymentsResponseData>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            status: enums::AttemptStatus::Started,
            response: Ok(types::PaymentsResponseData::TransactionResponse {
                resource_id: types::ResponseId::ConnectorTransactionId(item.response.transaction_id),
                redirection_data: None,
                mandate_reference: None,
                connector_metadata: None,
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
// REFUND :
// Type definition for RefundRequest
#[derive(Default, Debug, Serialize)]
pub struct ForteRefundRequest {
    pub amount: i64,
}

impl<F> TryFrom<&types::RefundsRouterData<F>> for ForteRefundRequest {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(item: &types::RefundsRouterData<F>) -> Result<Self, Self::Error> {
        Ok(Self {
            amount: item.request.amount,
        })
    }
}

// Type definition for Refund Response

#[allow(dead_code)]
#[derive(Debug, Serialize, Default, Deserialize, Clone)]
pub enum RefundStatus {
    Succeeded,
    Failed,
    #[default]
    Processing,
}

impl From<RefundStatus> for enums::RefundStatus {
    fn from(item: RefundStatus) -> Self {
        match item {
            RefundStatus::Succeeded => Self::Success,
            RefundStatus::Failed => Self::Failure,
            RefundStatus::Processing => Self::Pending,
            //TODO: Review mapping
        }
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    id: String,
    status: RefundStatus,
}

impl TryFrom<types::RefundsResponseRouterData<api::Execute, RefundResponse>>
    for types::RefundsRouterData<api::Execute>
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: types::RefundsResponseRouterData<api::Execute, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            response: Ok(types::RefundsResponseData {
                connector_refund_id: item.response.id.to_string(),
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

impl TryFrom<types::RefundsResponseRouterData<api::RSync, RefundResponse>>
    for types::RefundsRouterData<api::RSync>
{
    type Error = error_stack::Report<errors::ConnectorError>;
    fn try_from(
        item: types::RefundsResponseRouterData<api::RSync, RefundResponse>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            response: Ok(types::RefundsResponseData {
                connector_refund_id: item.response.id.to_string(),
                refund_status: enums::RefundStatus::from(item.response.status),
            }),
            ..item.data
        })
    }
}

//TODO: Fill the struct with respective fields
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct ForteErrorResponse {
    pub status_code: u16,
    pub code: String,
    pub message: String,
    pub reason: Option<String>,
}

//AUTH START

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForteLinks {
    disputes: Option<String>,
    settlements: Option<String>
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ForteResponse {
    environment: Option<String>,
    response_type: Option<String>,
    response_code: Option<String>,
    response_desc: Option<String>,
    authorization_code: Option<String>,
    avs_result: Option<String>,
    cvv_result: Option<String>,
}
#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct FortePaymentSyncCard{
    name_on_card:Option<String>,
    last_4_account_number:Option<String>,
    masked_account_number:Option<String>,
    expire_month:i64,
    expire_year:i64,
    card_type:Option<String>
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct PaymentsResponseData{
    transaction_id: String,
    location_id: Option<String>,
    organization_id:Option<String>,
    status:Option<String>,
    action: Option<String>,
    authorization_amount: i64,
    authorization_code: Option<String>,
    entered_by: Option<String>,
    received_date:Option<String>,
    billing_address: ForteAddressData,
    card: ForteCard2,
    response: ForteResponse,
    links: ForteLinks,
}
#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct ForteCard1 {
    card_type: Option<String>,
    name_on_card: Option<String>,
    account_number: Secret<String, common_utils::pii::CardNumber>,
    expire_month: Option<String>,
    expire_year: Option<String>,
    card_verification_value: Option<String>,
    complete: bool,
}
#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct PaymentsAuthorizeData{
    subtotal_amount: i64,
    authorization_amount: i64,
    billing_address:ForteAddressData,
    card:ForteCard
}
//AUTH ENDS

//SYNC STARTS
#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct PaymentsSyncData{
    action:String,
    transaction_id:String,
    authorization_code:String
}
//capture starts
#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct PaymentsCaptureData{
    action:String,
    transaction_id:String,
    authorization_code:String
}

//void
#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct PaymentsCancelData{
    action:String,
    authorization_code:String,
    entered_by:String
}
