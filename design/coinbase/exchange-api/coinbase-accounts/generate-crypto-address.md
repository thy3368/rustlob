> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Generate crypto address

> Generates a one-time crypto address for depositing crypto.

<Info>
  **Info**

  You can generate an address for crypto deposits. See the [Coinbase Accounts](/api-reference/exchange-api/rest-api/coinbase-accounts/get-all-coinbase-wallets) section for information on how to retrieve your coinbase account ID.
</Info>

## API Key Permissions

This endpoint requires the "transfer" permission. API key must belong to default profile.


## OpenAPI

````yaml POST /coinbase-accounts/{account_id}/addresses
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
  /coinbase-accounts/{account_id}/addresses:
    post:
      tags:
        - Coinbase accounts
      summary: Generate crypto address
      description: Generates a one-time crypto address for depositing crypto.
      operationId: ExchangeRESTAPI_PostCoinbaseAccountAddresses
      parameters:
        - name: account_id
          in: path
          required: true
          schema:
            type: string
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/apiPostCoinbaseAccountAddressesRequest'
        required: true
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiCoinbaseAccountGeneratedDepositAddress'
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
    apiPostCoinbaseAccountAddressesRequest:
      type: object
      properties:
        account_id:
          type: string
        profile_id:
          type: string
        network:
          type: string
    apiCoinbaseAccountGeneratedDepositAddress:
      required:
        - address
        - address_info
        - created_at
        - id
        - name
        - network
        - resource
        - resource_path
        - updated_at
        - uri_scheme
        - warnings
      type: object
      properties:
        id:
          type: string
        address:
          type: string
        address_info:
          $ref: '#/components/schemas/apiCoinbaseAccountAddressInfo'
        name:
          type: string
        created_at:
          type: string
          format: date-time
        updated_at:
          type: string
          format: date-time
        network:
          type: string
        uri_scheme:
          type: string
        resource:
          type: string
        resource_path:
          type: string
        warnings:
          type: array
          items:
            $ref: '#/components/schemas/apiCoinbaseAccountAddressWarning'
        legacy_address:
          type: string
        destination_tag:
          type: string
        deposit_uri:
          type: string
        callback_url:
          type: string
        qr_code_image_url:
          type: string
        address_label:
          type: string
        share_address_copy:
          $ref: '#/components/schemas/apiShareAddressInfo'
        exchange_deposit_address:
          type: boolean
        inline_warning:
          $ref: '#/components/schemas/apiInlineWarning'
      example:
        id: fc9fed1e-d25b-54d8-b52b-7fa250c9ae2d
        address: 1.2165647733617772e+48
        address_info:
          address: 1.2165647733617772e+48
        name: New exchange deposit address
        created_at: '2020-03-31T02:38:44.000Z'
        updated_at: '2020-03-31T02:38:44.000Z'
        network: ethereum
        uri_scheme: ethereum
        resource: address
        resource_path: >-
          /v2/accounts/faf4b657-a48c-56b2-bec2-77567e3f4f97/addresses/fc9fed1e-d25b-54d8-b52b-7fa250c9ae2d
        warnings:
          - title: Only send Orchid (OXT) to this address
            details: >-
              Sending any other digital asset, including Ethereum (ETH), will
              result in permanent loss.
            image_url: >-
              https://dynamic-assets.coinbase.com/22b24ad9886150535671f158ccb0dd9d12089803728551c998e17e0f503484e9c38f3e8735354b5e622753684f040488b08d55b8ef5fef51592680f0c572bdfe/asset_icons/023010d790b9b1f47bc285802eafeab3d83c4de2029fe808d59935fbc54cdd7c.png
        deposit_uri: >-
          ethereum:0x4575f41308ec1483f3d399aa9a2826d74da13deb/transfer?address=0xd518A6B23D8bCA15B9cC46a00Be8a818E34Ca79E
        exchange_deposit_address: true
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
    apiCoinbaseAccountAddressInfo:
      required:
        - address
      type: object
      properties:
        address:
          type: string
        destination_tag:
          type: string
    apiCoinbaseAccountAddressWarning:
      required:
        - details
        - image_url
        - title
      type: object
      properties:
        title:
          type: string
        details:
          type: string
        image_url:
          type: string
    apiShareAddressInfo:
      type: object
      properties:
        line1:
          type: string
        line2:
          type: string
    apiInlineWarning:
      type: object
      properties:
        text:
          type: string
        tooltip:
          $ref: '#/components/schemas/apiInlineWarningTooltip'
    apiInlineWarningTooltip:
      type: object
      properties:
        title:
          type: string
        subtitle:
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