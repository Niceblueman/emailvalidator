use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use schemars::{schema_for, schema_for_value, JsonSchema};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    pub id: String,
    pub object: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub address: Value,
    // pub balance: i64,
    // pub created: i64,
    // pub currency: String,
    // #[serde(rename = "default_source")]
    // pub default_source: Value,
    // pub delinquent: bool,
    pub description: Option<String>,
    // pub discount: Value,
    pub email: Option<String>,
    // #[serde(rename = "invoice_prefix")]
    // pub invoice_prefix: String,
    // #[serde(rename = "invoice_settings")]
    // pub invoice_settings: InvoiceSettings,
    // pub livemode: bool,
    // pub metadata: Metadata,
    pub name: Option<String>,
    // #[serde(rename = "next_invoice_sequence")]
    // pub next_invoice_sequence: i64,
    pub phone: Option<String>,
    // #[serde(rename = "preferred_locales")]
    // pub preferred_locales: Vec<Value>,
    // pub shipping: Value,
    // #[serde(rename = "tax_exempt")]
    // pub tax_exempt: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceSettings {
    #[serde(rename = "custom_fields")]
    pub custom_fields: Value,
    #[serde(rename = "default_payment_method")]
    pub default_payment_method: Value,
    pub footer: Value,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Plans {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<Plan>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Plan {
    pub id: String,
    pub object: String,
    pub active: bool,
    // #[serde(rename = "aggregate_usage")]
    // pub aggregate_usage: Value,
    pub amount: i64,
    #[serde(rename = "amount_decimal")]
    pub amount_decimal: String,
    #[serde(rename = "billing_scheme")]
    pub billing_scheme: String,
    pub created: i64,
    pub currency: String,
    pub interval: String,
    #[serde(rename = "interval_count")]
    pub interval_count: i64,
    pub livemode: bool,
    // pub metadata: Metadata,
    // pub nickname: Value,
    pub product: String,
    // #[serde(rename = "tiers_mode")]
    // pub tiers_mode: Value,
    // #[serde(rename = "transform_usage")]
    // pub transform_usage: Value,
    // #[serde(rename = "trial_period_days")]
    // pub trial_period_days: Value,
    #[serde(rename = "usage_type")]
    pub usage_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Metadata {
}






#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BillingDetails {
    pub address: Address,
    pub email: Option<Value>,
    pub name: Option<Value>,
    pub phone: Option<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Address {
    pub city: Option<Value>,
    pub country: Option<Value>,
    pub line1: Option<Value>,
    pub line2: Option<Value>,
    #[serde(rename = "postal_code")]
    pub postal_code: Option<Value>,
    pub state: Option<Value>,
}





#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    pub brand: String,
    pub checks: Checks,
    pub country: String,
    #[serde(rename = "exp_month")]
    pub exp_month: i64,
    #[serde(rename = "exp_year")]
    pub exp_year: i64,
    pub fingerprint: String,
    pub funding: String,
    // pub installments: Value,
    pub last4: String,
    // pub network: String,
    // #[serde(rename = "three_d_secure")]
    // pub three_d_secure: Value,
    // pub wallet: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Checks {
    // #[serde(rename = "address_line1_check")]
    // pub address_line1_check: Value,
    // #[serde(rename = "address_postal_code_check")]
    // pub address_postal_code_check: Value,
    #[serde(rename = "cvc_check")]
    pub cvc_check: String,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethods {
    pub object: String,
    pub url: String,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub data: Vec<PaymentMethod>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    pub id: String,
    pub object: String,
    #[serde(rename = "billing_details")]
    pub billing_details: BillingDetails,
    pub card: Card,
    pub created: i64,
    pub customer: Option<String>,
    pub livemode: bool,
    pub metadata: Metadata,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CheckoutPaymentMethod {
    pub id: Option<Value>,
    pub object: Option<Value>,
    #[serde(rename = "billing_details")]
    pub billing_details: Option<BillingDetails>,
    pub card: Card,
    pub created: Option<i64>,
    pub customer: Option<Value>,
    pub livemode: Option<bool>,
    pub metadata: Option<Metadata>,
    #[serde(rename = "type")]
    pub type_field: Option<Value>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Networks {
    pub available: Vec<String>,
    // pub preferred: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ThreeDSecureUsage {
    pub supported: bool,
}











#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Subscriptions {
    pub object: String,
    pub data: Vec<Subscription>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Subscription {
    pub id: Option<String>,
    pub object: Option<String>,
    #[serde(rename = "automatic_tax")]
    pub automatic_tax: Option<AutomaticTax>,
    #[serde(rename = "billing_cycle_anchor")]
    pub billing_cycle_anchor: Option<i64>,
    #[serde(rename = "cancel_at")]
    pub cancel_at: Option<i64>,
    #[serde(rename = "cancel_at_period_end")]
    pub cancel_at_period_end: Option<bool>,
    #[serde(rename = "canceled_at")]
    pub canceled_at: Option<i64>,
    #[serde(rename = "collection_method")]
    pub collection_method: Option<String>,
    pub created: Option<i64>,
    #[serde(rename = "current_period_end")]
    pub current_period_end: Option<i64>,
    #[serde(rename = "current_period_start")]
    pub current_period_start: Option<i64>,
    pub customer: Option<String>,
    #[serde(rename = "days_until_due")]
    pub days_until_due: Option<i64>,
    #[serde(rename = "default_payment_method")]
    pub default_payment_method: Option<String>,
    #[serde(rename = "ended_at")]
    pub ended_at: Option<i64>,
    pub items: Option<SubscriptionItems>,
    #[serde(rename = "latest_invoice")]
    pub latest_invoice: Option<String>,
    pub livemode: Option<bool>,
    // #[serde(rename = "payment_settings")]
    // pub payment_settings: PaymentSettings,
    pub quantity: Option<i64>,
    #[serde(rename = "start_date")]
    pub start_date: Option<i64>,
    pub status: Option<String>,
   // #[serde(rename = "application_fee_percent")]
    // pub application_fee_percent: Value,
    // #[serde(rename = "default_source")]
    // pub default_source: Value,
    // #[serde(rename = "default_tax_rates")]
    // pub default_tax_rates: Vec<Value>,
    // pub discount: Value,
    // pub metadata: Metadata4,
    // #[serde(rename = "next_pending_invoice_item_invoice")]
    // pub next_pending_invoice_item_invoice: Value,
    // #[serde(rename = "pause_collection")]
    // pub pause_collection: Value,
    // #[serde(rename = "pending_invoice_item_interval")]
    // pub pending_invoice_item_interval: Value,
    // #[serde(rename = "pending_setup_intent")]
    // pub pending_setup_intent: Value,
    // #[serde(rename = "pending_update")]
    // pub pending_update: Value,
    // pub plan: Plan2,
    // pub schedule: Value,
    // #[serde(rename = "transfer_data")]
    // pub transfer_data: Value,
    // #[serde(rename = "trial_end")]
    // pub trial_end: Value,
    // #[serde(rename = "trial_start")]
    // pub trial_start: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionItems {
    pub object: String,
    pub data: Vec<SubscriptionItem>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionItem {
    pub id: String,
    pub object: String,
    // #[serde(rename = "billing_thresholds")]
    // pub billing_thresholds: Value,
    pub created: i64,
    // pub metadata: Metadata,
    // pub plan: Plan,
    // pub price: Price,
    pub quantity: i64,
    pub subscription: String,
    // #[serde(rename = "tax_rates")]
    // pub tax_rates: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticTax {
    pub enabled: Option<bool>,
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Prices {
    pub object: String,
    pub data: Vec<Price>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PayoutPrices {
    pub object: Value,
    pub data: Vec<PayoutPrice>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: Value, // "/v1/charges?payment_intent=pi_3LgWZ3BnabqfJ3Wf1Kly6TXV"
    pub total_count: i64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Price {
    pub id: String,
    pub object: String,
    pub active: bool,
    #[serde(rename = "billing_scheme")]
    pub billing_scheme: String,
    pub created: i64,
    pub currency: String,
    pub livemode: bool,
    // #[serde(rename = "lookup_key")]
    // pub lookup_key: Value,
    // pub metadata: Metadata,
    // pub nickname: Value,
    pub product: String,
    pub recurring: Recurring,
    #[serde(rename = "tax_behavior")]
    pub tax_behavior: String,
    // #[serde(rename = "tiers_mode")]
    // pub tiers_mode: Value,
    // #[serde(rename = "transform_quantity")]
    // pub transform_quantity: Value,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "unit_amount")]
    pub unit_amount: i64,
    #[serde(rename = "unit_amount_decimal")]
    pub unit_amount_decimal: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PayoutPrice {
    pub amount:i64, // 2000,
    pub amount_captured:i64, // 2000,
    pub amount_refunded:Value, // ,
    pub amount_updates: Vec<Value>, // Array [],
    pub application:Value, // nul,
    pub application_fee:Value, // null,
    pub application_fee_amount:Value, // nul,
    pub balance_transaction: Value, // "txn_3LgWZ3BnabqfJ3Wf1g1tTYDa",
    pub billing_details:Value, // Object { address: {…}, email: "kimo@oldi.dev", name: "abdelkarim ouazmir", … },
    pub calculated_statement_descriptor:Value, // "WWW.DUP.MA",
    pub captured:bool, // true,
    pub created:i64, // 1662827426,
    pub currency: Value, // "usd",
    pub customer:Value, // "cus_MPLGffFH1v6VKB",
    pub description:Value, // nu​​​​​​,
    pub destination:Value, // null,
    pub dispute:Value, // null,
    pub disputed:bool, // false,
    pub failure_balance_transaction:Value, // null,
    pub failure_code:Value, // null,
    pub failure_message:Value, // null,
    pub fraud_details:Value, // Object {  },
    pub id:Value, // "ch_3LgWZ3BnabqfJ3Wf1y7AJiVa",
    pub invoice:Value, // null,
    pub livemode:bool, // false,
    pub metadata:Value, // Object {  },
    pub object:Value, // "charge",
    pub on_behalf_of:Value, // null,
    pub order:Value, // nu,
    pub outcome:Value, // Object { network_status: "approved_by_network", risk_level: "normal", risk_score: 42, … },
    pub paid:bool, // true,
    pub payment_intent:Value, // "pi_3LgWZ3BnabqfJ3Wf1Kly6TXV",
    pub payment_method:Value, // "pm_1LgWZYBnabqfJ3Wfd8pO7v3Z",
    pub payment_method_details: PaymentMethod, // Object { card: {…}, type: "card" },
    pub receipt_email:Value, // "kimo@oldi.dev",
    pub receipt_number:Value, // null,
    pub receipt_url: Value, // "https://pay.stripe.com/receipts/payment/CAcaFwoVYWNjdF8xSmJ1…LBb9h838O0XNzg1ZH70cmZHhW2pbWwlSqd4YKgNilYEVmczvFQeJg2Vhx-A5",
    pub refunded:bool, // false,
    pub refunds:Value, // Object { object: "list", has_more: false, total_count: 0, … },
    pub review:Value, // null,
    pub shipping:Value, // null,
    pub source:Value, // null,
    pub source_transfer:Value, // null,
    pub statement_descriptor:Value, // null,
    pub statement_descriptor_suffix:Value, // null,
    pub status:Value, // "succeeded",
    pub transfer_data:Value, // null,
    pub transfer_group:Value, // null,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Recurring {
    // #[serde(rename = "aggregate_usage")]
    // pub aggregate_usage: Value,
    pub interval: String,
    #[serde(rename = "interval_count")]
    pub interval_count: i64,
    // #[serde(rename = "trial_period_days")]
    // pub trial_period_days: Value,
    #[serde(rename = "usage_type")]
    pub usage_type: String,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Invoices {
    pub object: String,
    pub data: Vec<Invoice>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Invoice {
    pub id: String,
    pub object: String,
    #[serde(rename = "account_country")]
    pub account_country: String,
    // #[serde(rename = "account_name")]
    // pub account_name: Value,
    // #[serde(rename = "account_tax_ids")]
    // pub account_tax_ids: Value,
    #[serde(rename = "amount_due")]
    pub amount_due: i64,
    #[serde(rename = "amount_paid")]
    pub amount_paid: i64,
    #[serde(rename = "amount_remaining")]
    pub amount_remaining: i64,
    #[serde(rename = "application_fee_amount")]
    pub application_fee_amount: Value,
    #[serde(rename = "attempt_count")]
    pub attempt_count: i64,
    pub attempted: bool,
    #[serde(rename = "auto_advance")]
    pub auto_advance: bool,
    #[serde(rename = "automatic_tax")]
    pub automatic_tax: AutomaticTax,
    #[serde(rename = "billing_reason")]
    pub billing_reason: String,
    pub charge: String,
    #[serde(rename = "collection_method")]
    pub collection_method: String,
    pub created: i64,
    pub currency: String,
    // #[serde(rename = "custom_fields")]
    // pub custom_fields: Value,
    pub customer: String,
    // #[serde(rename = "customer_address")]
    // pub customer_address: Value,
    #[serde(rename = "customer_email")]
    pub customer_email: String,
    #[serde(rename = "customer_name")]
    pub customer_name: String,
    #[serde(rename = "customer_phone")]
    pub customer_phone: Option<String>,
    // #[serde(rename = "customer_shipping")]
    // pub customer_shipping: Value,
    #[serde(rename = "customer_tax_exempt")]
    pub customer_tax_exempt: String,
    #[serde(rename = "customer_tax_ids")]
    pub customer_tax_ids: Vec<Value>,
    #[serde(rename = "default_payment_method")]
    pub default_payment_method: Value,
    #[serde(rename = "default_source")]
    pub default_source: Value,
    #[serde(rename = "default_tax_rates")]
    pub default_tax_rates: Vec<Value>,
    pub description: Value,
    pub discount: Value,
    pub discounts: Vec<Value>,
    #[serde(rename = "due_date")]
    pub due_date: Value,
    #[serde(rename = "ending_balance")]
    pub ending_balance: i64,
    pub footer: Value,
    #[serde(rename = "hosted_invoice_url")]
    pub hosted_invoice_url: String,
    #[serde(rename = "invoice_pdf")]
    pub invoice_pdf: String,
    #[serde(rename = "last_finalization_error")]
    pub last_finalization_error: Value,
    pub lines: InvoiceLines,
    pub livemode: bool,
    // pub metadata: Metadata4,
    #[serde(rename = "next_payment_attempt")]
    pub next_payment_attempt: Value,
    pub number: String,
    #[serde(rename = "on_behalf_of")]
    pub on_behalf_of: Value,
    pub paid: bool,
    #[serde(rename = "paid_out_of_band")]
    pub paid_out_of_band: bool,
    #[serde(rename = "payment_intent")]
    pub payment_intent: String,
    // #[serde(rename = "payment_settings")]
    // pub payment_settings: PaymentSettings,
    #[serde(rename = "period_end")]
    pub period_end: i64,
    #[serde(rename = "period_start")]
    pub period_start: i64,
    #[serde(rename = "post_payment_credit_notes_amount")]
    pub post_payment_credit_notes_amount: i64,
    #[serde(rename = "pre_payment_credit_notes_amount")]
    pub pre_payment_credit_notes_amount: i64,
    pub quote: Value,
    #[serde(rename = "receipt_number")]
    pub receipt_number: Value,
    #[serde(rename = "starting_balance")]
    pub starting_balance: i64,
    #[serde(rename = "statement_descriptor")]
    pub statement_descriptor: Value,
    pub status: String,
    #[serde(rename = "status_transitions")]
    pub status_transitions: StatusTransitions,
    pub subscription: String,
    pub subtotal: i64,
    pub tax: Value,
    pub total: i64,
    #[serde(rename = "total_discount_amounts")]
    pub total_discount_amounts: Vec<Value>,
    #[serde(rename = "total_tax_amounts")]
    pub total_tax_amounts: Vec<Value>,
    #[serde(rename = "transfer_data")]
    pub transfer_data: Value,
    #[serde(rename = "webhooks_delivered_at")]
    pub webhooks_delivered_at: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PayoutCheckout {
    api_version: Value,
    created: i64,
    livemode: bool,
    id: Value,
    object: Value,
    data: WebhooksRequestobjects,
    pending_webhooks: i64,
    request: PendingWebhooksRequest,
    #[serde(rename = "type")]
    tpe: Value,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PendingWebhooksRequest {
    pub id: Value,
    pub idempotency_key: Value
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct WebhooksRequestobjects {
    pub object: WebhooksRequestobject,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct WebhooksRequestobject {
    pub amount: i64,
    pub amount_capturable: i64,
    pub amount_details: Value,
    pub amount_received: i64,
    pub application: Value,
    pub application_fee_amount: Value,
    pub automatic_payment_methods: Value,
    pub canceled_at: Value,
    pub cancellation_reason: Value,
    pub capture_method: Value, // "automatic"
    pub charges: PayoutPrices,
    pub client_secret: Value, // "pi_***************"
    pub confirmation_method: Value, // "automatic"
    pub created: i64, // 1662827393
    pub currency: Value, // "usd"
    pub customer: Value, // "cus_MPLGffFH1v6VKB"
    pub description: Value,
    pub id: Value, // "pi_3LgWZ3BnabqfJ3Wf1Kly6TXV"
    pub invoice: Value,
    pub last_payment_error: Value,
    pub livemode: bool,
    pub metadata: Value,
    pub next_action: Value,
    pub object: Value, // "payment_intent"
    pub on_behalf_of: Value,
    pub payment_method: Value, //"pm_1LgWZYBnabqfJ3Wfd8pO7v3Z",
    pub payment_method_options: CheckoutPaymentMethod,
    pub payment_method_types: Vec<String>,
    pub processing: Value,
    pub receipt_email: Value , //"kimo@oldi.dev"
    pub review: Value,
    pub setup_future_usage: Value,
    pub shipping: Value,
    pub source: Value,
    pub statement_descriptor: Value,
    pub statement_descriptor_suffix: Value,
    pub status: Value, //"succeeded"
    pub transfer_data: Value,
    pub transfer_group: Value,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLines {
    pub object: String,
    pub data: Vec<InvoiceLine>,
    #[serde(rename = "has_more")]
    pub has_more: bool,
    #[serde(rename = "total_count")]
    pub total_count: i64,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceLine {
    pub id: String,
    pub object: String,
    pub amount: i64,
    pub currency: String,
    pub description: String,
    // #[serde(rename = "discount_amounts")]
    // pub discount_amounts: Vec<Value>,
    pub discountable: bool,
    pub discounts: Vec<Value>,
    pub livemode: bool,
    // pub metadata: Metadata,
    pub period: Period,
    pub plan: Plan,
    pub price: Price,
    pub proration: bool,
    pub quantity: i64,
    pub subscription: String,
    #[serde(rename = "subscription_item")]
    pub subscription_item: String,
    // #[serde(rename = "tax_amounts")]
    // pub tax_amounts: Vec<Value>,
    // #[serde(rename = "tax_rates")]
    // pub tax_rates: Vec<Value>,
    #[serde(rename = "type")]
    pub type_field: String,
}



#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub end: i64,
    pub start: i64,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct StatusTransitions {
    #[serde(rename = "finalized_at")]
    pub finalized_at: Option<i64>,
    // #[serde(rename = "marked_uncollectible_at")]
    // pub marked_uncollectible_at: Value,
    #[serde(rename = "paid_at")]
    pub paid_at: Option<i64>,
    #[serde(rename = "voided_at")]
    pub voided_at: Option<i64>,
}
