<!--
SPDX-FileCopyrightText: 2026 Kevin Monaghan
SPDX-License-Identifier: MIT-0
-->

# Wallet REST coverage and protocol evidence

Checked **2026-09-27** against the official catalog: 50 REST operations.
Bindings and synthetic transport tests establish the described JSON contracts;
no credentialed or live mutation probe was run.

The product owns provider-native asset, capital, account, and Travel Rule DTOs.
Balances preserve free/locked and asset identities. Networks use validated native
identities without asset/address inference. Network metadata and separate
Spot/Funding/Margin/Futures account categories remain visible. No balances are
added, no reporting currency is chosen, and no withdrawal destination or
jurisdictional questionnaire is selected by the library. Financial strings and
integer/number wire fees parse directly into Decimal; malformed values fail.
Dynamic asset-detail objects are maps keyed by validated `Asset` identities.

Wallet balance queries require a caller-selected `quoteAsset` and return
`QuotedWalletBalance`, which retains that request provenance beside native
wallet rows. Dust conversion results similarly retain the chosen target asset.
The provenance wrapper is not a fabricated venue response field. Fixed BTC/BNB
valuation fields retain their documented names; the client never computes them.

Withdrawals require a caller-owned `WithdrawalId`. Errors retain that identity,
HTTP status, venue code, and quota evidence. Transfer endpoints without a caller
ID retain their native venue transaction identity; uncertain results require
venue history reads. Every mutation is attempted once, including after truncated
bodies, timeout, 5xx, and decode failure. Read-only POST queries report `ReadFailed`.
Unknown codes and every 5xx remain ambiguous for mutations. Sensitive addresses,
tags, questionnaires, personal data, and unknown fields have redacted Debug.
Callers explicitly access provider data; the client never logs raw bodies.

SAPI limits are independent per endpoint: IP **12,000/minute** or UID
**180,000/minute**, according to the operation's documented scope. Withdrawal
history also retains its documented **10 requests/second** limit. `Budgets::sapi`
creates shared owners; clones share them, and `for_account` keeps the common IP
owner while creating a distinct UID owner. Wallet and Convert can receive the same
owners. SAPI admissions do not charge Spot/Futures aggregate counters. Response
`X-SAPI-USED-IP-WEIGHT-1M`, `X-SAPI-USED-UID-WEIGHT-1M`, and `Retry-After` evidence
updates the originating endpoint scope before attempting to read its body.
A documented `418` IP ban additionally blocks every shared endpoint/account owner.
Admission refuses immediately; it never queues or retries.

Configuration uses the documented production SAPI host, milliseconds, caller
credentials and runtime. It starts no background tasks. HTTPS is required except
exact loopback fixture hosts; ambient proxies and dependency retries are disabled.
A canceled future does not establish venue cancellation. Caller deadlines are
rechecked immediately before sending.

## Sources and verification

- [Wallet general transport, signing and independent SAPI limits](https://developers.binance.com/en/docs/products/wallet/general-info)
- [Wallet error codes](https://developers.binance.com/en/docs/products/wallet/error-code)
- [Wallet account catalog](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account)
- [Wallet asset catalog](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset)
- [Capital, networks and withdrawal quota](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital)
- [Travel Rule catalog](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule)

`schema/wallet-rest.json` contains normalized facts, source URLs, original schema
SHA-256 and check date. Provider prose/examples were excluded. The generator
corrects sample asset keys, exact financial types, native identities and required
receipt evidence. `schema/wallet-coverage.json` and bindings regenerate offline.
`tests/wallet_contract.rs` covers precision, malformed/missing asset evidence,
withdrawal caller IDs and redaction, truncated mutations without retry, quota
headers, independent endpoints, read-only POST failure, isolated transfer symbols,
comma-separated dust assets, requested quote/target asset provenance, and native
capital/dividend history range and withdrawal ID-list bounds.
Human execution review is required before merge. [Issue #16](https://github.com/SharurTrading/binance-rs/issues/16)
tracks this implementation; Margin and Options trading remain separate products.

## REST operations

| Operation | Method | Path | Weight scope | Weight |
| --- | --- | --- | --- | ---: |
| [accountApiTradingStatus](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-api-trading-status) | GET | `/sapi/v1/account/apiTradingStatus` | IP | 1 |
| [accountInfo](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-info) | GET | `/sapi/v1/account/info` | IP | 1 |
| [accountStatus](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#account-status) | GET | `/sapi/v1/account/status` | IP | 1 |
| [dailyAccountSnapshot](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#daily-account-snapshot) | GET | `/sapi/v1/accountSnapshot` | IP | 2400 |
| [disableFastWithdrawSwitch](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#disable-fast-withdraw-switch) | POST | `/sapi/v1/account/disableFastWithdrawSwitch` | IP | 1 |
| [enableFastWithdrawSwitch](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#enable-fast-withdraw-switch) | POST | `/sapi/v1/account/enableFastWithdrawSwitch` | IP | 1 |
| [getApiKeyPermission](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/account#get-api-key-permission) | GET | `/sapi/v1/account/apiRestrictions` | IP | 1 |
| [assetDetail](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#asset-detail) | GET | `/sapi/v1/asset/assetDetail` | IP | 1 |
| [assetDividendRecord](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#asset-dividend-record) | GET | `/sapi/v1/asset/assetDividend` | IP | 10 |
| [dustConvert](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-convert) | POST | `/sapi/v1/asset/dust-convert/convert` | UID | 10 |
| [dustConvertibleAssets](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-convertible-assets) | POST | `/sapi/v1/asset/dust-convert/query-convertible-assets` | IP | 1 |
| [dustlog](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dustlog) | GET | `/sapi/v1/asset/dribblet` | IP | 1 |
| [dustTransfer](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#dust-transfer) | POST | `/sapi/v1/asset/dust` | UID | 10 |
| [fundingWallet](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#funding-wallet) | POST | `/sapi/v1/asset/get-funding-asset` | IP | 1 |
| [getSpotAssetTags](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-spot-asset-tags) | GET | `/sapi/v1/spot/asset/tags` | IP | 100 |
| [getAssetsThatCanBeConvertedIntoBnb](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-assets-that-can-be-converted-into-bnb) | POST | `/sapi/v1/asset/dust-btc` | IP | 1 |
| [getCloudMiningPaymentAndRefundHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-cloud-mining-payment-and-refund-history) | GET | `/sapi/v1/asset/ledger-transfer/cloud-mining/queryByPage` | UID | 600 |
| [getOpenSymbolList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#get-open-symbol-list) | GET | `/sapi/v1/spot/open-symbol-list` | IP | 100 |
| [queryUserDelegationHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-delegation-history) | GET | `/sapi/v1/asset/custody/transfer-history` | IP | 60 |
| [queryUserUniversalTransferHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-universal-transfer-history) | GET | `/sapi/v1/asset/transfer` | IP | 1 |
| [userUniversalTransfer](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-universal-transfer) | POST | `/sapi/v1/asset/transfer` | UID | 300 |
| [queryUserWalletBalance](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#query-user-wallet-balance) | GET | `/sapi/v1/asset/wallet/balance` | IP | 60 |
| [toggleBnbBurnOnSpotTradeAndMarginInterest](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#toggle-bnb-burn-on-spot-trade-and-margin-interest) | POST | `/sapi/v1/bnbBurn` | IP | 1 |
| [tradeFee](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#trade-fee) | GET | `/sapi/v1/asset/tradeFee` | IP | 1 |
| [userAsset](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/asset#user-asset) | POST | `/sapi/v3/asset/getUserAsset` | IP | 5 |
| [allCoinsInformation](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#all-coins-information) | GET | `/sapi/v1/capital/config/getall` | IP | 10 |
| [depositAddress](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#deposit-address) | GET | `/sapi/v1/capital/deposit/address` | IP | 10 |
| [depositHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#deposit-history) | GET | `/sapi/v1/capital/deposit/hisrec` | IP | 1 |
| [fetchDepositAddressListWithNetwork](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-deposit-address-list-with-network) | GET | `/sapi/v1/capital/deposit/address/list` | IP | 10 |
| [fetchWithdrawAddressList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-withdraw-address-list) | GET | `/sapi/v1/capital/withdraw/address/list` | IP | 10 |
| [fetchWithdrawQuota](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#fetch-withdraw-quota) | GET | `/sapi/v1/capital/withdraw/quota` | IP | 10 |
| [oneClickArrivalDepositApply](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#one-click-arrival-deposit-apply) | POST | `/sapi/v1/capital/deposit/credit-apply` | IP | 1 |
| [withdraw](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#withdraw) | POST | `/sapi/v1/capital/withdraw/apply` | UID | 900 |
| [withdrawHistory](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/capital#withdraw-history) | GET | `/sapi/v1/capital/withdraw/history` | UID | 18000 |
| [getSymbolsDelistScheduleForSpot](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/others#get-symbols-delist-schedule-for-spot) | GET | `/sapi/v1/spot/delist-schedule` | IP | 100 |
| [systemStatus](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/others#system-status) | GET | `/sapi/v1/system/status` | IP | 1 |
| [brokerWithdraw](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#broker-withdraw) | POST | `/sapi/v1/localentity/broker/withdraw/apply` | UID | 600 |
| [checkQuestionnaireRequirements](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#check-questionnaire-requirements) | GET | `/sapi/v1/localentity/questionnaire-requirements` | IP | 1 |
| [getCountryList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#get-country-list) | GET | `/sapi/v1/localentity/country/list` | IP | 1 |
| [depositHistoryTravelRule](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-travel-rule) | GET | `/sapi/v1/localentity/deposit/history` | IP | 1 |
| [depositHistoryV2](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#deposit-history-v2) | GET | `/sapi/v2/localentity/deposit/history` | IP | 1 |
| [fetchAddressVerificationList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#fetch-address-verification-list) | GET | `/sapi/v1/addressVerify/list` | IP | 1 |
| [getRegionList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#get-region-list) | GET | `/sapi/v1/localentity/region/list` | IP | 1 |
| [submitDepositQuestionnaire](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire) | PUT | `/sapi/v1/localentity/broker/deposit/provide-info` | UID | 600 |
| [submitDepositQuestionnaireTravelRule](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire-travel-rule) | PUT | `/sapi/v1/localentity/deposit/provide-info` | UID | 600 |
| [submitDepositQuestionnaireV2](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#submit-deposit-questionnaire-v2) | PUT | `/sapi/v2/localentity/deposit/provide-info` | UID | 600 |
| [vaspList](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#vasp-list) | GET | `/sapi/v1/localentity/vasp` | IP | 1 |
| [withdrawHistoryV1](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v1) | GET | `/sapi/v1/localentity/withdraw/history` | IP | 1 |
| [withdrawHistoryV2](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-history-v2) | GET | `/sapi/v2/localentity/withdraw/history` | IP | 1 |
| [withdrawTravelRule](https://developers.binance.com/en/docs/catalog/core-trading-wallet/api/rest-api/travel-rule#withdraw-travel-rule) | POST | `/sapi/v1/localentity/withdraw/apply` | UID | 600 |
