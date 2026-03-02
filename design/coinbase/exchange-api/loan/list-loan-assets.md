> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# List loan assets

> Get a list of lendable assets, a map of assets we accept as collateral for loans, and the haircut weight.

<Info>
  **Coinbase Exchange Loans Program**

  See [Coinbase Exchange Loans Program](https://coinbase.bynder.com/m/47c334b9a63ed3e4/original/exchange-Loans-Program.pdf) for program details including qualification criteria and sample terms.
</Info>


## OpenAPI

````yaml GET /loans/assets
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
  /loans/assets:
    get:
      tags:
        - Loans
      summary: List loan assets
      description: >-
        Get a list of lendable assets, a map of assets we accept as collateral
        for loans, and the haircut weight.
      operationId: ExchangeRESTAPI_GetLoanAssets
      responses:
        '200':
          description: A successful response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiGetLoanAssetsResponse'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiGetLoanAssetsResponse:
      type: object
      properties:
        collateral_assets:
          type: object
          additionalProperties:
            type: string
        diversification_ratio:
          type: string
        borrowable_assets:
          type: array
          items:
            type: string
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