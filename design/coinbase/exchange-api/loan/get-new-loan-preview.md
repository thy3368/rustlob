> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Get new loan preview

> This API is similar to lending-overview but is used to preview the results of opening a new loan. The values returned in the preview response take the existing loans, collateral and the potential change being previewed into account. Note the preview request accepts native currency amounts as input.

<Info>
  **Coinbase Exchange Loans Program**

  See [Coinbase Exchange Loans Program](https://coinbase.bynder.com/m/47c334b9a63ed3e4/original/exchange-Loans-Program.pdf) for program details including qualification criteria and sample terms.
</Info>

<Warning>
  **Caution**

  The lending rate limit is [10 RPS per profile](/exchange/rest-api/rate-limits).
</Warning>


## OpenAPI

````yaml GET /loans/loan-preview
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
  /loans/loan-preview:
    get:
      tags:
        - Loans
      summary: Get new loan preview
      description: >-
        This API is similar to lending-overview but is used to preview the
        results of opening a new loan. The values returned in the preview
        response take the existing loans, collateral and the potential change
        being previewed into account. Note the preview request accepts native
        currency amounts as input.
      operationId: ExchangeRESTAPI_GetLoanPreview
      parameters:
        - name: currency
          in: query
          schema:
            type: string
        - name: native_amount
          in: query
          schema:
            type: string
      responses:
        '200':
          description: A successful response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiGetPreviewResponse'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiGetPreviewResponse:
      type: object
      properties:
        before:
          $ref: '#/components/schemas/apiLendingOverview'
        after:
          $ref: '#/components/schemas/apiLendingOverview'
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiLendingOverview:
      type: object
      properties:
        open_loan_value:
          type: string
        collateral_value:
          type: string
        collateralization_percentage:
          type: string
        available_to_borrow:
          type: string
        available_per_asset:
          type: object
          additionalProperties:
            $ref: '#/components/schemas/apiAmount'
        withdrawal_restricted:
          type: boolean
        credit_limit_value:
          type: string
        available_credit_value:
          type: string
        collateralization_percentage_open_only:
          type: string
        pending_loan_value:
          type: string
        initial_margin_percentage:
          type: string
        minimum_margin_percentage:
          type: string
        unlock_margin_percentage:
          type: string
    apiAmount:
      type: object
      properties:
        native:
          type: string
        notional:
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