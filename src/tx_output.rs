// Copyright (c) 2024-2025 RBB S.r.l
// opensource@mintlayer.org
// SPDX-License-Identifier: MIT
// Licensed under the MIT License;
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// https://github.com/mintlayer/mintlayer-core-primitives/blob/master/LICENSE
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use parity_scale_codec::{Decode, Encode};

use crate::{
    Amount, DelegationId, Destination, NftIssuance, OutputTimeLock, OutputValue, PerThousand,
    PoolId, PscVec, TokenId, TokenIssuance, VrfPublicKey,
};

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, strum::EnumDiscriminants)]
#[strum_discriminants(name(TxOutputTag), derive(strum::EnumIter))]
pub enum TxOutput {
    /// Transfer an output value, giving the provided Destination the authority to
    /// spend it (no conditions).
    #[codec(index = 0)]
    Transfer(OutputValue, Destination),

    /// Same as Transfer, but with the condition that the output can only be
    /// spent after some point in time.
    #[codec(index = 1)]
    LockThenTransfer(OutputValue, Destination, OutputTimeLock),

    /// Burn an amount (whether coin or token).
    #[codec(index = 2)]
    Burn(OutputValue),

    /// Output type that is used to create a stake pool.
    #[codec(index = 3)]
    CreateStakePool(PoolId, StakePoolData),

    /// Output type that represents spending of a stake pool output in a block
    /// reward in order to produce a block.
    #[codec(index = 4)]
    ProduceBlockFromStake(Destination, PoolId),

    /// Create a delegation account to a specific pool, defined by its id.
    /// Takes the owner destination, which is the address authorized to withdraw from the delegation.
    #[codec(index = 5)]
    CreateDelegationId(Destination, PoolId),

    /// Transfer an amount to a delegation.
    #[codec(index = 6)]
    DelegateStaking(Amount, DelegationId),

    /// Issues a new fungible token.
    #[codec(index = 7)]
    IssueFungibleToken(TokenIssuance),

    /// Issue an NFT.
    #[codec(index = 8)]
    IssueNft(TokenId, NftIssuance, Destination),

    /// Deposit data into the blockchain.
    #[codec(index = 9)]
    DataDeposit(PscVec<u8>),

    /// Transfer an output value under Hashed TimeLock Contract.
    #[codec(index = 10)]
    Htlc(OutputValue, HashedTimelockContract),

    /// Create an order.
    #[codec(index = 11)]
    CreateOrder(OrderData),
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct StakePoolData {
    pub pledge: Amount,
    pub staker: Destination,
    pub vrf_public_key: VrfPublicKey,
    pub decommission_key: Destination,
    pub margin_ratio_per_thousand: PerThousand,
    pub cost_per_block: Amount,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct OrderData {
    /// The key that can authorize conclusion of an order
    pub conclude_key: Destination,
    /// `Ask` and `give` fields represent amounts of currencies
    /// that an order maker wants to exchange.
    /// E.g. Creator of an order asks for 5 coins and gives 10 tokens in
    /// exchange.
    pub ask: OutputValue,
    pub give: OutputValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
pub struct HashedTimelockContract {
    /// Can be spent either via `spend_key` by someone who knows the secret.
    pub secret_hash: HtlcSecretHash,
    pub spend_key: Destination,

    /// Or via `refund_key` after the timelock expires.
    pub refund_timelock: OutputTimeLock,
    pub refund_key: Destination,
}

pub const HTLC_SECRET_HASH_SIZE: usize = 20;

// Note: Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord are already implemented by the macro,
// so no need to derive them.
fixed_hash::construct_fixed_hash! {
    #[derive(Encode, Decode)]
    pub struct HtlcSecretHash(HTLC_SECRET_HASH_SIZE);
}
