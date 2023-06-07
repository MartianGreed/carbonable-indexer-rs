use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{
    domain::{crypto::U256, Erc20, HumanComprehensibleU256, Mass, SlotValue},
    infrastructure::starknet::model::{StarknetValue, StarknetValueResolver},
};

use super::customer::{CustomerToken, CustomerTokenWithSlotValue};

#[derive(Debug, Serialize, Deserialize)]
pub struct UriViewModel {
    pub uri: String,
    pub address: String,
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FarmingProjectsViewModel {
    pub id: Uuid,
    pub address: String,
    pub name: String,
    pub slug: String,
    pub uri: UriViewModel,
}

impl From<tokio_postgres::Row> for FarmingProjectsViewModel {
    fn from(value: tokio_postgres::Row) -> Self {
        Self {
            id: value.get(0),
            address: value.get(1),
            name: value.get(2),
            slug: value.get(3),
            uri: UriViewModel {
                uri: value.get(4),
                address: value.get(5),
                data: value.get(6),
            },
        }
    }
}

#[derive(Debug)]
pub struct CustomerGlobalDataForComputation {
    pub id: uuid::Uuid,
    pub unit_price: U256,
    pub payment_decimals: U256,
    pub payment_symbol: String,
    pub project_slot: U256,
    pub project_address: String,
    pub value_decimals: U256,
    pub ton_equivalent: U256,
    pub yielder_address: String,
    pub offseter_address: String,
    pub slot: U256,
    pub project_value: U256,
}

impl From<tokio_postgres::Row> for CustomerGlobalDataForComputation {
    fn from(value: tokio_postgres::Row) -> Self {
        Self {
            id: value.get(0),
            unit_price: value.get(1),
            payment_decimals: value.get(2),
            payment_symbol: value.get(3),
            project_slot: value.get(4),
            project_address: value.get(5),
            value_decimals: value.get(6),
            ton_equivalent: value.get(7),
            yielder_address: value.get(8),
            offseter_address: value.get(9),
            slot: value.get(10),
            project_value: value.get(11),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CustomerGlobalData {
    // slot value
    pub total_deposited_value: SlotValue,
    // erc 20
    pub total_investment: Erc20,
    // erc 20
    pub total_yielder_claimable: Erc20,
    // mass in gram
    pub total_offseter_claimable: Mass<U256>,
}
#[derive(Debug, Default, Serialize)]
pub struct DisplayableCustomerGlobalData {
    pub total_deposited_value: HumanComprehensibleU256<U256>,
    pub total_investment: HumanComprehensibleU256<U256>,
    pub total_yielder_claimable: HumanComprehensibleU256<U256>,
    pub total_offseter_claimable: HumanComprehensibleU256<U256>,
}

impl From<CustomerGlobalData> for DisplayableCustomerGlobalData {
    fn from(value: CustomerGlobalData) -> Self {
        Self {
            total_deposited_value: value.total_deposited_value.into(),
            total_investment: value.total_investment.into(),
            total_yielder_claimable: value.total_yielder_claimable.into(),
            total_offseter_claimable: value.total_offseter_claimable.into(),
        }
    }
}

impl CustomerGlobalData {
    pub fn merge(mut self, other: Self) -> Self {
        self.total_deposited_value += other.total_deposited_value;
        self.total_investment += other.total_investment;
        self.total_yielder_claimable += other.total_yielder_claimable;
        self.total_offseter_claimable += other.total_offseter_claimable;
        self
    }
}

#[derive(Debug)]
pub struct CompleteFarmingData {
    pub id: Uuid,
    pub address: String,
    pub times: Vec<PrimitiveDateTime>,
    pub absorptions: Vec<U256>,
    pub ton_equivalent: U256,
    pub value_decimals: U256,
    pub payment_decimals: U256,
    pub payment_symbol: String,
    pub payment_address: Option<String>,
    pub offseter_address: Option<String>,
    pub yielder_id: Option<Uuid>,
    pub yielder_address: Option<String>,
    pub minter_id: Option<Uuid>,
    pub total_value: Option<U256>,
    pub project_abi: Option<serde_json::Value>,
    pub minter_abi: Option<serde_json::Value>,
    pub offseter_abi: Option<serde_json::Value>,
    pub yielder_abi: Option<serde_json::Value>,
    pub payment_abi: Option<serde_json::Value>,
}
impl CompleteFarmingData {
    pub fn final_absorption(&self) -> U256 {
        *self.absorptions.last().unwrap_or(&U256::zero())
    }
}

impl From<tokio_postgres::Row> for CompleteFarmingData {
    fn from(value: tokio_postgres::Row) -> Self {
        Self {
            id: value.get(0),
            address: value.get(1),
            times: value.get(2),
            absorptions: value.get(3),
            ton_equivalent: value.get(4),
            value_decimals: value.get(5),
            payment_decimals: value.get(6),
            payment_symbol: value.get(7),
            payment_address: value.get(8),
            offseter_address: value.get(9),
            yielder_id: value.get(10),
            yielder_address: value.get(11),
            minter_id: value.get(12),
            total_value: value.get(13),
            project_abi: value.get(14),
            minter_abi: value.get(15),
            offseter_abi: value.get(16),
            yielder_abi: value.get(17),
            payment_abi: value.get(18),
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct UnconnectedFarmingData {
    pub apr: ProjectApr,
    #[serde(flatten)]
    pub status: ProjectStatus,
    pub tvl: HumanComprehensibleU256<U256>,
    pub total_removal: HumanComprehensibleU256<U256>,
}

#[derive(Debug, Clone, Default, Serialize)]
#[serde(untagged)]
pub enum ProjectApr {
    #[default]
    #[serde(rename = "n/a")]
    None,
    Value(BigDecimal),
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ProjectStatus {
    #[default]
    Upcoming,
    Ended,
    Live,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct ContractsList {
    pub yielder: String,
    pub yielder_abi: serde_json::Value,
    pub offseter: String,
    pub offseter_abi: serde_json::Value,
    pub project: String,
    pub project_abi: serde_json::Value,
    pub payment: String,
    pub payment_abi: serde_json::Value,
}

#[derive(Default, Debug, Serialize)]
pub struct CustomerListingProjectData {
    pub customer_stake: HumanComprehensibleU256<U256>,
    pub customer_investment: HumanComprehensibleU256<U256>,
    pub payment_decimals: u32,
    pub ton_equivalent: U256,
    pub vesting_to_claim: HumanComprehensibleU256<U256>,
    pub absorption_to_claim: HumanComprehensibleU256<U256>,
    pub undeposited: HumanComprehensibleU256<U256>,
    /// min_to_claim in kg
    pub min_to_claim: HumanComprehensibleU256<U256>,
    pub contracts: ContractsList,
}

impl
    From<(
        Vec<Vec<FieldElement>>,
        CustomerGlobalDataForComputation,
        CompleteFarmingData,
        U256,
    )> for CustomerListingProjectData
{
    fn from(
        value: (
            Vec<Vec<FieldElement>>,
            CustomerGlobalDataForComputation,
            CompleteFarmingData,
            U256,
        ),
    ) -> Self {
        let blockchain_response = value.0;
        let project_data = value.1;
        let farming_data = value.2;
        let value_of = value.3;

        let yielder_claimable: U256 = StarknetValue::new(blockchain_response[0].clone())
            .resolve("u256")
            .into();
        let offseter_claimable: U256 = StarknetValue::new(blockchain_response[1].clone())
            .resolve("u256")
            .into();
        let yielder_deposited: U256 = StarknetValue::new(blockchain_response[2].clone())
            .resolve("u256")
            .into();
        let offseter_deposited: U256 = StarknetValue::new(blockchain_response[3].clone())
            .resolve("u256")
            .into();
        let min_claimable: U256 = StarknetValue::new(blockchain_response[4].clone())
            .resolve("u256")
            .into();

        let total_value = yielder_deposited + offseter_deposited;
        Self {
            customer_stake: SlotValue::from_blockchain(total_value, farming_data.value_decimals)
                .into(),
            customer_investment: Erc20::from_blockchain(
                (total_value) * project_data.unit_price,
                farming_data.payment_decimals,
                farming_data.payment_symbol.clone(),
            )
            .into(),
            payment_decimals: project_data.payment_decimals.into(),
            ton_equivalent: farming_data.ton_equivalent,
            vesting_to_claim: Erc20::from_blockchain(
                yielder_claimable,
                farming_data.payment_decimals,
                farming_data.payment_symbol,
            )
            .into(),
            absorption_to_claim: Mass::<U256>::from_blockchain(
                offseter_claimable,
                farming_data.ton_equivalent,
            )
            .into(),
            undeposited: SlotValue::from_blockchain(value_of, farming_data.value_decimals).into(),
            min_to_claim: Mass::<U256>::from_blockchain(min_claimable, farming_data.ton_equivalent)
                .into(),
            contracts: ContractsList {
                yielder: farming_data.yielder_address.unwrap_or_default(),
                yielder_abi: farming_data.yielder_abi.unwrap_or_default(),
                offseter: farming_data.offseter_address.unwrap_or_default(),
                offseter_abi: farming_data.offseter_abi.unwrap_or_default(),
                project: farming_data.address.to_string(),
                project_abi: farming_data.project_abi.unwrap_or_default(),
                payment: farming_data.payment_address.unwrap_or_default(),
                payment_abi: farming_data.payment_abi.unwrap_or_default(),
            },
        }
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct Overview {
    total_removal: HumanComprehensibleU256<U256>,
    tvl: HumanComprehensibleU256<U256>,
    apr: ProjectApr,
    total_yielded: HumanComprehensibleU256<U256>,
    total_offseted: HumanComprehensibleU256<U256>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct PoolLiquidity<T> {
    total: HumanComprehensibleU256<T>,
    available: HumanComprehensibleU256<T>,
}

#[derive(Default, Clone, Debug, Serialize)]
pub struct CarbonCredits {
    generated_credits: HumanComprehensibleU256<BigDecimal>,
    to_be_generated: HumanComprehensibleU256<BigDecimal>,
    r#yield: PoolLiquidity<Erc20>,
    offset: PoolLiquidity<Mass<U256>>,
    min_to_claim: HumanComprehensibleU256<BigDecimal>,
}
#[derive(Default, Clone, Debug, Serialize)]
pub struct Allocation {
    total: HumanComprehensibleU256<U256>,
    r#yield: HumanComprehensibleU256<U256>,
    offseted: HumanComprehensibleU256<U256>,
    undeposited: HumanComprehensibleU256<U256>,
    tokens: Vec<CustomerTokenWithSlotValue>,
}
#[derive(Default, Clone, Debug, Serialize)]
pub struct CustomerDetailsProjectData {
    overview: Overview,
    carbon_credits: CarbonCredits,
    allocation: Allocation,
    contracts: ContractsList,
    payment_decimals: u32,
    ton_equivalent: BigDecimal,
    unit_price: HumanComprehensibleU256<U256>,
}

impl CustomerDetailsProjectData {
    pub fn with_contracts(
        &mut self,
        project_data: &CustomerGlobalDataForComputation,
        farming_data: &CompleteFarmingData,
    ) -> &mut Self {
        self.contracts = ContractsList {
            yielder: String::from(&project_data.yielder_address),
            offseter: String::from(&project_data.offseter_address),
            project: String::from(&farming_data.address),
            payment: String::from(&farming_data.payment_address.clone().unwrap_or_default()),
            yielder_abi: farming_data.yielder_abi.clone().into(),
            offseter_abi: farming_data.offseter_abi.clone().into(),
            project_abi: farming_data.project_abi.clone().into(),
            payment_abi: farming_data.payment_abi.clone().into(),
        };
        self
    }

    pub fn with_apr(&mut self, apr: ProjectApr) -> &mut Self {
        self.overview.apr = apr;
        self
    }

    pub fn compute_blockchain_data(
        &mut self,
        data: Vec<Vec<FieldElement>>,
        project: &CompleteFarmingData,
        farming_data: &CustomerGlobalDataForComputation,
        value_of: &U256,
        customer_tokens: &mut [CustomerToken],
    ) -> &mut Self {
        let current_absorption: U256 = StarknetValue::new(data[0].clone()).resolve("u256").into();
        let offseter_deposited_of: U256 =
            StarknetValue::new(data[1].clone()).resolve("u256").into();
        let yielder_deposited_of: U256 = StarknetValue::new(data[2].clone()).resolve("u256").into();
        let claimable_of: U256 = StarknetValue::new(data[3].clone()).resolve("u256").into();
        let releasable_of: U256 = StarknetValue::new(data[4].clone()).resolve("u256").into();
        let claimed_of: U256 = StarknetValue::new(data[5].clone()).resolve("u256").into();
        let released_of: U256 = StarknetValue::new(data[6].clone()).resolve("u256").into();
        let offseter_total_deposited: U256 =
            StarknetValue::new(data[7].clone()).resolve("u256").into();
        let yielder_total_deposited: U256 =
            StarknetValue::new(data[8].clone()).resolve("u256").into();
        let project_value: U256 = farming_data.project_value;
        let min_to_claim: U256 = StarknetValue::new(data[9].clone()).resolve("u256").into();

        self.overview.total_removal =
            Mass::<U256>::from_blockchain(current_absorption, project.ton_equivalent).into();
        self.overview.total_yielded =
            SlotValue::from_blockchain(yielder_total_deposited, project.value_decimals).into();
        self.overview.total_offseted =
            SlotValue::from_blockchain(offseter_total_deposited, project.value_decimals).into();

        self.overview.tvl = Erc20::from_blockchain(
            farming_data.unit_price * (offseter_total_deposited + yielder_total_deposited),
            farming_data.payment_decimals,
            farming_data.payment_symbol.clone(),
        )
        .into();

        self.carbon_credits.generated_credits = Mass::<BigDecimal>::from_blockchain(
            current_absorption.to_big_decimal(0)
                * (offseter_deposited_of.to_big_decimal(0)
                    + yielder_deposited_of.to_big_decimal(0)
                    + value_of.to_big_decimal(0))
                / project_value.to_big_decimal(0),
        )
        .into();
        self.carbon_credits.to_be_generated = Mass::<BigDecimal>::from_blockchain(
            (project.final_absorption().to_big_decimal(0) - current_absorption.to_big_decimal(0))
                * ((offseter_deposited_of.to_big_decimal(0)
                    + yielder_deposited_of.to_big_decimal(0)
                    + value_of.to_big_decimal(0))
                    / project_value.to_big_decimal(0)),
        )
        .into();

        self.carbon_credits.r#yield = PoolLiquidity {
            available: Erc20::from_blockchain(
                releasable_of,
                project.payment_decimals,
                project.payment_symbol.clone(),
            )
            .into(),
            total: Erc20::from_blockchain(
                released_of,
                project.payment_decimals,
                project.payment_symbol.clone(),
            )
            .into(),
        };
        self.carbon_credits.offset = PoolLiquidity {
            available: Mass::<U256>::from_blockchain(claimable_of, project.ton_equivalent).into(),
            total: Mass::<U256>::from_blockchain(claimed_of, project.ton_equivalent).into(),
        };
        self.carbon_credits.min_to_claim =
            Mass::<BigDecimal>::from_blockchain(min_to_claim.to_big_decimal(0)).into();

        self.allocation.total = SlotValue::from_blockchain(
            *value_of + (yielder_deposited_of + offseter_deposited_of),
            project.value_decimals,
        )
        .into();
        self.allocation.r#yield =
            SlotValue::from_blockchain(yielder_deposited_of, project.value_decimals).into();
        self.allocation.offseted =
            SlotValue::from_blockchain(offseter_deposited_of, project.value_decimals).into();
        self.allocation.undeposited =
            SlotValue::from_blockchain(*value_of, project.value_decimals).into();
        self.allocation.tokens = customer_tokens
            .iter()
            .map(|ct| CustomerTokenWithSlotValue {
                wallet: ct.wallet.clone(),
                project_address: ct.project_address.clone(),
                slot: ct.slot,
                token_id: ct.token_id,
                value: SlotValue::from_blockchain(ct.value, project.value_decimals),
            })
            .collect();

        self.ton_equivalent = project.ton_equivalent.to_big_decimal(0);
        self.payment_decimals = project.payment_decimals.into();
        self.unit_price = Erc20::from_blockchain(
            farming_data.unit_price,
            farming_data.payment_decimals,
            farming_data.payment_symbol.to_string(),
        )
        .into();

        self
    }

    pub fn build(&self) -> Self {
        self.clone()
    }
}
