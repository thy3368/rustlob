> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Convert currency

> Converts funds from `from` currency to `to` currency. Funds are converted on the `from` account in the `profile_id` profile.

<Warning>
  **Caution**

  Users whose USD and USDC accounts are unified do not have access to the conversion endpoint, and conversions from USDC to USD are automatic upon deposit.
</Warning>

## API Key Permissions

This endpoint requires the "trade" permission.

## Response

A successful conversion is assigned a conversion ID. The corresponding ledger entries for a conversion reference this conversion ID.


## OpenAPI

````yaml POST /conversions
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
  /conversions:
    post:
      tags:
        - Conversions
      summary: Convert currency
      description: >-
        Converts funds from `from` currency to `to` currency. Funds are
        converted on the `from` account in the `profile_id` profile.
      operationId: ExchangeRESTAPI_PostConversion
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/apiPostConversionRequest'
        required: true
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiConversion'
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
        '503':
          description: Service Unavailable.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiPostConversionRequest:
      required:
        - amount
        - from
        - to
      type: object
      properties:
        profile_id:
          type: string
        from:
          type: string
        to:
          type: string
        amount:
          type: string
        nonce:
          type: string
        idem:
          type: string
    apiConversion:
      required:
        - amount
        - fee_amount
        - from
        - from_account_id
        - id
        - to
        - to_account_id
      type: object
      properties:
        id:
          type: string
        amount:
          type: string
        from_account_id:
          type: string
        to_account_id:
          type: string
        from:
          type: string
        to:
          type: string
        fee_amount:
          type: string
        created_at:
          type: string
      example:
        id: c5aaf125-d99e-41fe-82ea-ad068038b278
        amount: '11.00000000'
        from_account_id: 5dcc143c-fb96-4f72-aebf-a165e3d29b53
        to_account_id: 6100247f-90fc-4335-ac17-d99839f0c909
        from: USDC
        to: USD
        fee_amount: '0.0000000000000000'
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