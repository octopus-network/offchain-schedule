# Offchain Schedule

- Schedule some offchain jobs for octopus network.

# Current schedule list

1. Lpos-Market Reward Distribution
2. Cross Chain token send
3. Fetch validator_set from Restaking-Base in Anchor
4. Send Vsc Packet to Appchain


# How to use

1. Config env variables

    Check .env.example file.

```shell
NEAR_ENV=testnet
NEAR_CLI_TESTNET_RPC_SERVER_URL=https://rpc.testnet.near.org
NEAR_CLI_MAINNET_RPC_SERVER_URL=https://rpc.mainnet.near.org
SCHEDULE_SIGNER=xxx.testnet
LPOS_MARKET_CONTRACT=contract-4.lpos-market.testnet
OTTO_TOKEN_CONTRACT=oct.beta_oct_relay.testnet
APPCHAIN_REGISTRY_CONTRACT=registry.test_oct.testnet
CROSS_CHAIN_TRANSFER_RECEIVER=channel-0.ef.transfer.v5.nearibc.testnet
DST_CHAIN_TRANSFER_RECEIVER=cosmos1sqkln20dw9a5779nyjw0eka4xsqrk9hdgrueql
```

2. Run binary file.
