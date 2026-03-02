> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# List loans

> Accepts zero or more loan IDs as input. If no loan IDs are specified, it returns all loans for the user. Otherwise it returns only the loan IDs specified.

<Info>
  **Coinbase Exchange Loans Program**

  See [Coinbase Exchange Loans Program](https://coinbase.bynder.com/m/47c334b9a63ed3e4/original/exchange-Loans-Program.pdf) for program details including qualification criteria and sample terms.
</Info>

<Warning>
  **Caution**

  The lending rate limit is [10 RPS per profile](/exchange/rest-api/rate-limits).
</Warning>


## OpenAPI

````yaml GET /loans
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
  /loans:
    get:
      tags:
        - Loans
      summary: List loans
      description: >-
        Accepts zero or more loan IDs as input. If no loan IDs are specified, it
        returns all loans for the user. Otherwise it returns only the loan IDs
        specified.
      operationId: ExchangeRESTAPI_GetLoans
      parameters:
        - name: ids
          in: query
          schema:
            type: string
      responses:
        '200':
          description: ''
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: '#/components/schemas/apiLoan'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiLoan:
      type: object
      properties:
        id:
          type: string
        currency:
          type: string
        principal_amount:
          type: string
        outstanding_principal_amount:
          type: string
        interest_rate:
          type: string
        interest_currency:
          type: string
        status:
          $ref: '#/components/schemas/apiLoanStatus'
        effective_at:
          type: string
          format: date-time
        closed_at:
          type: string
          format: date-time
        term_start_date:
          type: string
          format: date-time
        term_end_date:
          type: string
          format: date-time
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiLoanStatus:
      type: string
      default: loan_pending
      enum:
        - loan_pending
        - loan_active
        - loan_canceled
        - loan_closed
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