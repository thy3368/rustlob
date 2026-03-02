> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get address book

> Get all addresses stored in the address book.



## OpenAPI

````yaml GET /address-book
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
  /address-book:
    get:
      tags:
        - Address book
      summary: Get address book
      description: Get all addresses stored in the address book.
      operationId: ExchangeRESTAPI_GetAddressBook
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/GetAddressBookResponseAddressInfo'
        '401':
          description: Unauthorized.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiUnauthorizedResponse'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    GetAddressBookResponseAddressInfo:
      required:
        - address
        - address_book_added_at
        - currency
        - id
        - label
      type: object
      properties:
        id:
          type: string
        address:
          type: string
        destination_tag:
          type: string
        currency:
          type: string
          description: >-
            Asset symbol for the saved address (e.g., `BTC`, `ETH`, `USDC`).
            `_ALL_ASSETS_` indicates that this address is stored globally for
            all assets, rather than a specific one. The `network` field
            determines which blockchain network the address applies to.
        label:
          type: string
        address_book_added_at:
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
          description: >-
            Business name of the originator's account - only populated for
            travel rules regions
        business_country_code:
          type: string
          description: >-
            The country code (ISO 3166-1 alpha-2) of the originator's account
            location - only populated for travel rules regions
        network:
          type: string
          description: >-
            Blockchain network of the address. If omitted or `null`, the address
            is available on all supported networks compatible with the asset
            (e.g., both `ethereum` and `arbitrum` for an ERC-20 token). When
            `currency` is `_ALL_ASSETS_` and network is `_ALL_EVM_NETWORKS_`,
            the address is available on all assets with EVM-compatible networks.
      example:
        id: e9c483b8-c502-11ec-9d64-0242ac120002
        address: 1JmYrFBLMSCLBwoL87gdQ5Qc9MLvb2egKk
        currency: ETH
        label: my wallet
        address_book_added_at: '2019-02-06T22:27:50.221Z'
        is_verified_self_hosted_wallet: false
        business_name: Example Business
        business_country_code: US
    apiUnauthorizedResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
      description: >-
        UnauthorizedResponse is the response message for endpoints in
        rest-gateway that requires authentication.

        This message is used to generate the Exchange REST API documentation
        using OpenAPI format.
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
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