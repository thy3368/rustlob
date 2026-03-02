> ## Documentation Index
> Fetch the complete documentation index at: https://docs.cdp.coinbase.com/llms.txt
> Use this file to discover all available pages before exploring further.

# Repay loan interest

> Submit an interest repayment for a loan.



## OpenAPI

````yaml POST  /loans/repay-interest
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
  /loans/repay-interest:
    post:
      tags:
        - Loans
      summary: Repay loan interest
      description: Submit an interest repayment for a loan.
      operationId: ExchangeRESTAPI_RepayInterest
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/apiRepayInterestRequest'
        required: true
      responses:
        '200':
          description: A successful response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiRepayInterestResponse'
        '500':
          description: An unexpected error response.
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/apiErrorResponse'
components:
  schemas:
    apiRepayInterestRequest:
      type: object
      properties:
        idem:
          type: string
        from_profile_id:
          type: string
        currency:
          type: string
        native_amount:
          type: string
    apiRepayInterestResponse:
      type: object
      properties:
        repayment:
          $ref: '#/components/schemas/apiInterestRepayment'
    apiErrorResponse:
      type: object
      properties:
        message:
          title: message
          pattern: ^[a-zA-Z0-9]{1, 32}$
          type: string
    apiInterestRepayment:
      type: object
      properties:
        id:
          type: string
        native_amount:
          type: string
        status:
          $ref: '#/components/schemas/apiRepaymentStatus'
        type:
          $ref: '#/components/schemas/apiRepaymentType'
    apiRepaymentStatus:
      type: string
      default: REPAYMENT_UNSET
      enum:
        - REPAYMENT_UNSET
        - REPAYMENT_PENDING
        - REPAYMENT_COMPLETED
        - REPAYMENT_CANCELLED
        - REPAYMENT_EXPIRED
        - REPAYMENT_COMMITTED
        - REPAYMENT_REJECTED
    apiRepaymentType:
      type: string
      default: REPAYMENT_TYPE_UNSET
      enum:
        - REPAYMENT_TYPE_UNSET
        - PRINCIPAL_RETURN
        - PRINCIPAL_RECALL
        - INTEREST_PAYMENT
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