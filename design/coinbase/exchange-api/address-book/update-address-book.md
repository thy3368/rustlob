> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Update address book

> Update an address book entry



## OpenAPI

````yaml put /address-book/{id}
openapi: 3.0.1
info:
  title: REST API
  description: >-
    # Welcome to Coinbase Exchange API

    ## Introduction

    The Exchange Trading APIs allow institutions to place orders and access
    account information. The following API pages detail various REST API
    endpoints we offer for lower-frequency trading and general requests.

    ## Getting Started

    To get started, please visit one of the following pages:

    - [Authentication](/exchange/docs/rest-auth)

    - [Rate Limits](/exchange/docs/rest-rate-limits)

    - [Pagination](/exchange/docs/rest-pagination)

    - [Status Codes](/exchange/docs/rest-requests)

    - [Quickstart](/exchange/docs/getting-started)

    ## FIX API

    - [FIX API reference](/exchange/docs/fix-connectivity)

    ## WebSocket API

    - [WebSocket API reference](/exchange/docs/websocket-overview)
  version: '1.0'
servers:
  - url: https://api.exchange.coinbase.com/
security:
  - ApiKeyAuthKey: []
    ApiKeyAuthPassphrase: []
    ApiKeyAuthSign: []
    ApiKeyAuthTimestamp: []
paths:
  /address-book/{id}:
    put:
      tags:
        - Address book
      summary: Update address book
      description: Update an address book entry
      operationId: ExchangeRESTAPI_PutAddressBookEntry
      parameters:
        - name: id
          in: path
          required: true
          schema:
            type: string
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/apiPublicPutAddressBookEntryRequest'
        required: true
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiAddressBookWhitelistedAddress'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiPublicPutAddressBookEntryRequest:
      required:
        - id
        - label
      type: object
      properties:
        id:
          type: string
        label:
          type: string
        is_verified_self_hosted_wallet:
          type: boolean
          description: >-
            Flag to indicate if the crypto addresses has previously been
            digitally signed and verified when added in the Address Book UI tab
        vasp_id:
          type: string
          description: >-
            The VASP identifier if the address is owned by one of the supported
            Virtual Asset Service Providers
        business_name:
          type: string
          description: Business name
        business_country_code:
          type: string
          description: >-
            The country code (ISO 3166-1 alpha-2) of the originator's account
            location.
        is_certified_self_send:
          type: boolean
          description: Flag to indicate if the user owns the crypto address
        network:
          type: string
          description: >-
            Blockchain network of the address. Provide a single network name
            (e.g., `ethereum`, `bitcoin`, `_ALL_EVM_NETWORKS_`). If omitted or
            `null`, the network field remains unchanged. Use `*` to make the
            address available on all supported networks compatible with the
            asset (e.g., both `ethereum` and `arbitrum` for an ERC-20 token).
            When `currency` is `_ALL_ASSETS_`, `network` is required. Use
            `_ALL_EVM_NETWORKS_` only with `_ALL_ASSETS_` to apply the address
            to all EVM-compatible networks.
        wallet_verification_network:
          type: string
          description: Blockchain network used to verify ownership of the wallet address
    apiAddressBookWhitelistedAddress:
      required:
        - address
        - address_book_added_at
        - address_booked
        - address_info
        - currency
        - display_address
        - id
        - label
      type: object
      properties:
        id:
          type: string
        address:
          type: string
        address_info:
          $ref: '#/components/schemas/apiAddressBookWhitelistedAddressInfo'
        currency:
          type: string
          description: >-
            Asset symbol for the saved address (e.g., `BTC`, `ETH`, `USDC`).
            `_ALL_ASSETS_` indicates that this address is stored globally for
            all assets, rather than a specific one. The `network` field
            determines which blockchain network the address applies to.
        label:
          type: string
        display_address:
          type: string
        address_booked:
          type: boolean
        address_book_added_at:
          type: string
          format: date-time
        address_book_entry_pending_until:
          type: string
          format: date-time
        is_verified_self_hosted_wallet:
          type: boolean
          description: >-
            Flag to indicate if the crypto addresses has previously been
            digitally signed and verified when added in the Address Book UI tab
        vasp_id:
          type: string
          description: >-
            The VASP identifier if the address is owned by one of the supported
            Virtual Asset Service Providers
        business_name:
          type: string
          description: Business name of the originator's account
        business_country_code:
          type: string
          description: >-
            The country code (ISO 3166-1 alpha-2) of the originator's account
            location.
        network:
          type: string
          description: >-
            Blockchain network of the address. Indicates the network scope the
            address is associated with (e.g., `ethereum`, `bitcoin`). If omitted
            or `null`, the address is available on all supported networks
            compatible with the asset (e.g., both `ethereum` and `arbitrum` for
            an ERC-20 token). `_ALL_EVM_NETWORKS_` indicates the address applies
            to all EVM-compatible networks.
      example:
        id: '1'
        address: 1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk
        display_address: 1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk
        address_info:
          address: 1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk
          display_address: 1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk
        currency: BTC
        label: my wallet
        address_booked: true
        address_book_added_at: '2019-02-06T22:27:50.221Z'
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiAddressBookWhitelistedAddressInfo:
      required:
        - address
        - display_address
      type: object
      properties:
        address:
          type: string
        display_address:
          type: string
        destination_tag:
          type: string
  securitySchemes:
    ApiKeyAuthKey:
      type: apiKey
      name: cb-access-key
      in: header
    ApiKeyAuthPassphrase:
      type: apiKey
      name: cb-access-passphrase
      in: header
    ApiKeyAuthSign:
      type: apiKey
      name: cb-access-sign
      in: header
    ApiKeyAuthTimestamp:
      type: apiKey
      name: cb-access-timestamp
      in: header

````