# Salesforce Schema Simplify

Extremely simple CLI tool that removes extra thick crud from the output of `SObjectDescribe`, simplifies / humanizes & outputs to a pretty-print JSON file.

## What?

Salesforce provides a _monumental_ amount of data on SObjects when you call this endpoint =>
`services/data/{apiVersion}/sobjects/{sObjectName}/describe`,

Unfortunately the response from the above endpoint contains a lot of superfluous data that makes reading the output (as a human) quite brain numbing;
this tool removes a lot of extraneous properties and flattens the data to (hopefully) only contain relevant data to assist with developing CRUD tools for Salesforce SObjects.

**TL;DR**
- Removes the following properties from individual `fields`:
  - `autoNumber`
  - `calculated`
  - `autoNumber`
  - `createable`
  - `custom`
  - `defaultValue`
  - `externalId`
  - `htmlFormatted`
  - `inlineHelpText`
  - `length`
  - `nillable`
  - `unique`
  - `updateable`
- Flattens the objects inside a field `picklistValues` to be an array of strings.
- Removes the following root properties:
  - `networkScopeFieldName`
  - `searchLayoutable`
  - `activateable`
  - `createable`
  - `custom`
  - `customSetting`
  - `deletable`
  - `deprecatedAndHidden`
  - `feedEnabled`
  - `hasSubtypes`
  - `isSubtype`
  - `keyPrefix`
  - `label`
  - `labelPlural`
  - `layoutable`
  - `mergeable`
  - `mruEnabled`
  - `queryable`
  - `replicateable`
  - `retrieveable`
  - `searchable`
  - `triggerable`
  - `undeletable`
  - `updateable`
  - `urls`

## Make it do the thing!

```shell
sf-schema-simplify --input SFObject.json --output SFObjectSimplified.json
```
