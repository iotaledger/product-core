## Classes

<dl>
<dt><a href="#Client">Client</a></dt>
<dd></dd>
<dt><a href="#Config">Config</a></dt>
<dd><p>Options to configure a new <a href="#Client">Client</a>.</p>
</dd>
<dt><a href="#Credential">Credential</a></dt>
<dd></dd>
<dt><a href="#CredentialValidationOptions">CredentialValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating credentials.</p>
</dd>
<dt><a href="#CredentialValidator">CredentialValidator</a></dt>
<dd></dd>
<dt><a href="#DID">DID</a></dt>
<dd></dd>
<dt><a href="#DIDUrl">DIDUrl</a></dt>
<dd></dd>
<dt><a href="#DiffChainHistory">DiffChainHistory</a></dt>
<dd></dd>
<dt><a href="#DiffMessage">DiffMessage</a></dt>
<dd><p>Defines the difference between two DID <code>Document</code>s&#39; JSON representations.</p>
</dd>
<dt><a href="#Document">Document</a></dt>
<dd></dd>
<dt><a href="#DocumentHistory">DocumentHistory</a></dt>
<dd><p>A DID Document&#39;s history and current state.</p>
</dd>
<dt><a href="#DocumentMetadata">DocumentMetadata</a></dt>
<dd><p>Additional attributes related to an IOTA DID Document.</p>
</dd>
<dt><a href="#Duration">Duration</a></dt>
<dd><p>A span of time.</p>
</dd>
<dt><a href="#ExplorerUrl">ExplorerUrl</a></dt>
<dd></dd>
<dt><a href="#IntegrationChainHistory">IntegrationChainHistory</a></dt>
<dd></dd>
<dt><a href="#KeyCollection">KeyCollection</a></dt>
<dd></dd>
<dt><a href="#KeyPair">KeyPair</a></dt>
<dd></dd>
<dt><a href="#MethodScope">MethodScope</a></dt>
<dd><p>Supported verification method types.</p>
</dd>
<dt><a href="#MethodType">MethodType</a></dt>
<dd><p>Supported verification method types.</p>
</dd>
<dt><a href="#Network">Network</a></dt>
<dd></dd>
<dt><a href="#Presentation">Presentation</a></dt>
<dd></dd>
<dt><a href="#PresentationValidationOptions">PresentationValidationOptions</a></dt>
<dd><p>Options to declare validation criteria when validating presentation.</p>
</dd>
<dt><a href="#PresentationValidator">PresentationValidator</a></dt>
<dd></dd>
<dt><a href="#ProofPurpose">ProofPurpose</a></dt>
<dd><p>Associates a purpose with a <code>Signature</code>.</p>
<p>See <a href="https://w3c-ccg.github.io/security-vocab/#proofPurpose">https://w3c-ccg.github.io/security-vocab/#proofPurpose</a></p>
</dd>
<dt><a href="#Receipt">Receipt</a></dt>
<dd></dd>
<dt><a href="#ResolvedDocument">ResolvedDocument</a></dt>
<dd><p>An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
merged with one or more <code>DiffMessages</code>.</p>
</dd>
<dt><a href="#Resolver">Resolver</a></dt>
<dd></dd>
<dt><a href="#ResolverBuilder">ResolverBuilder</a></dt>
<dd><p>Builder for configuring [<code>Clients</code>][Client] when constructing a [<code>Resolver</code>].</p>
</dd>
<dt><a href="#Service">Service</a></dt>
<dd><p>A DID Document Service used to enable trusted interactions associated
with a DID subject.</p>
<p>See: <a href="https://www.w3.org/TR/did-core/#services">https://www.w3.org/TR/did-core/#services</a></p>
</dd>
<dt><a href="#SignatureOptions">SignatureOptions</a></dt>
<dd><p>Holds additional options for creating signatures.
See <code>ISignatureOptions</code>.</p>
</dd>
<dt><a href="#Timestamp">Timestamp</a></dt>
<dd></dd>
<dt><a href="#VerificationMethod">VerificationMethod</a></dt>
<dd></dd>
<dt><a href="#VerifierOptions">VerifierOptions</a></dt>
<dd><p>Holds additional signature verification options.
See <code>IVerifierOptions</code>.</p>
</dd>
</dl>

## Members

<dl>
<dt><a href="#MethodRelationship">MethodRelationship</a></dt>
<dd></dd>
<dt><a href="#KeyType">KeyType</a></dt>
<dd></dd>
<dt><a href="#DIDMessageEncoding">DIDMessageEncoding</a></dt>
<dd></dd>
<dt><a href="#Digest">Digest</a></dt>
<dd></dd>
<dt><a href="#SubjectHolderRelationship">SubjectHolderRelationship</a></dt>
<dd><p>Declares how credential subjects must relate to the presentation holder during validation.
See <code>PresentationValidationOptions::subject_holder_relationship</code>.</p>
<p>See also the <a href="https://www.w3.org/TR/vc-data-model/#subject-holder-relationships">Subject-Holder Relationship</a> section of the specification.</p>
</dd>
<dt><a href="#AlwaysSubject">AlwaysSubject</a></dt>
<dd><p>The holder must always match the subject on all credentials, regardless of their <a href="https://www.w3.org/TR/vc-data-model/#nontransferable-property"><code>nonTransferable</code></a> property.
This variant is the default used if no other variant is specified when constructing a new
<code>PresentationValidationOptions</code>.</p>
</dd>
<dt><a href="#SubjectOnNonTransferable">SubjectOnNonTransferable</a></dt>
<dd><p>The holder must match the subject only for credentials where the <a href="https://www.w3.org/TR/vc-data-model/#nontransferable-property"><code>nonTransferable</code></a> property is <code>true</code>.</p>
</dd>
<dt><a href="#Any">Any</a></dt>
<dd><p>The holder is not required to have any kind of relationship to any credential subject.</p>
</dd>
<dt><a href="#FailFast">FailFast</a></dt>
<dd><p>Declares when validation should return if an error occurs.</p>
</dd>
<dt><a href="#AllErrors">AllErrors</a></dt>
<dd><p>Return all errors that occur during validation.</p>
</dd>
<dt><a href="#FirstError">FirstError</a></dt>
<dd><p>Return after the first error occurs.</p>
</dd>
</dl>

## Functions

<dl>
<dt><a href="#start">start()</a></dt>
<dd><p>Initializes the console error panic hook for better error messages</p>
</dd>
</dl>

<a name="Client"></a>

## Client
**Kind**: global class  

* [Client](#Client)
    * [new Client()](#new_Client_new)
    * _instance_
        * [.network()](#Client+network) ⇒ [<code>Network</code>](#Network)
        * [.publishDocument(document)](#Client+publishDocument) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
        * [.publishDiff(message_id, diff)](#Client+publishDiff) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
        * [.publishJSON(index, data)](#Client+publishJSON) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
        * [.publishJsonWithRetry(index, data, interval, max_attempts)](#Client+publishJsonWithRetry) ⇒ <code>Promise.&lt;any&gt;</code>
        * [.resolve(did)](#Client+resolve) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.resolveHistory(did)](#Client+resolveHistory) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
        * [.resolveDiffHistory(document)](#Client+resolveDiffHistory) ⇒ [<code>Promise.&lt;DiffChainHistory&gt;</code>](#DiffChainHistory)
    * _static_
        * [.fromConfig(config)](#Client.fromConfig) ⇒ [<code>Client</code>](#Client)
        * [.fromNetwork(network)](#Client.fromNetwork) ⇒ [<code>Client</code>](#Client)

<a name="new_Client_new"></a>

### new Client()
Creates a new `Client` with default settings.

<a name="Client+network"></a>

### client.network() ⇒ [<code>Network</code>](#Network)
Returns the `Client` Tangle network.

**Kind**: instance method of [<code>Client</code>](#Client)  
<a name="Client+publishDocument"></a>

### client.publishDocument(document) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
Publishes an `IotaDocument` to the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="Client+publishDiff"></a>

### client.publishDiff(message_id, diff) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
Publishes a `DiffMessage` to the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 
| diff | [<code>DiffMessage</code>](#DiffMessage) | 

<a name="Client+publishJSON"></a>

### client.publishJSON(index, data) ⇒ [<code>Promise.&lt;Receipt&gt;</code>](#Receipt)
Publishes arbitrary JSON data to the specified index on the Tangle.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| index | <code>string</code> | 
| data | <code>any</code> | 

<a name="Client+publishJsonWithRetry"></a>

### client.publishJsonWithRetry(index, data, interval, max_attempts) ⇒ <code>Promise.&lt;any&gt;</code>
Publishes arbitrary JSON data to the specified index on the Tangle.
Retries (promotes or reattaches) the message until it’s included (referenced by a milestone).
Default interval is 5 seconds and max attempts is 40.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| index | <code>string</code> | 
| data | <code>any</code> | 
| interval | <code>number</code> \| <code>undefined</code> | 
| max_attempts | <code>number</code> \| <code>undefined</code> | 

<a name="Client+resolve"></a>

### client.resolve(did) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetch the DID document specified by the given `DID`.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

<a name="Client+resolveHistory"></a>

### client.resolveHistory(did) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
Returns the message history of the given DID.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

<a name="Client+resolveDiffHistory"></a>

### client.resolveDiffHistory(document) ⇒ [<code>Promise.&lt;DiffChainHistory&gt;</code>](#DiffChainHistory)
Returns the `DiffChainHistory` of a diff chain starting from a document on the
integration chain.

NOTE: the document must have been published to the tangle and have a valid message id and
capability invocation method.

**Kind**: instance method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| document | [<code>ResolvedDocument</code>](#ResolvedDocument) | 

<a name="Client.fromConfig"></a>

### Client.fromConfig(config) ⇒ [<code>Client</code>](#Client)
Creates a new `Client` with settings from the given `Config`.

**Kind**: static method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| config | [<code>Config</code>](#Config) | 

<a name="Client.fromNetwork"></a>

### Client.fromNetwork(network) ⇒ [<code>Client</code>](#Client)
Creates a new `Client` with default settings for the given `Network`.

**Kind**: static method of [<code>Client</code>](#Client)  

| Param | Type |
| --- | --- |
| network | [<code>Network</code>](#Network) | 

<a name="Config"></a>

## Config
Options to configure a new [Client](#Client).

**Kind**: global class  

* [Config](#Config)
    * [new Config()](#new_Config_new)
    * _instance_
        * [.setNetwork(network)](#Config+setNetwork)
        * [.setEncoding(encoding)](#Config+setEncoding)
        * [.setNode(url)](#Config+setNode)
        * [.setPrimaryNode(url, jwt, username, password)](#Config+setPrimaryNode)
        * [.setPrimaryPoWNode(url, jwt, username, password)](#Config+setPrimaryPoWNode)
        * [.setPermanode(url, jwt, username, password)](#Config+setPermanode)
        * [.setNodeAuth(url, jwt, username, password)](#Config+setNodeAuth)
        * [.setNodeSyncInterval(value)](#Config+setNodeSyncInterval)
        * [.setNodeSyncDisabled()](#Config+setNodeSyncDisabled)
        * [.setQuorum(value)](#Config+setQuorum)
        * [.setQuorumSize(value)](#Config+setQuorumSize)
        * [.setQuorumThreshold(value)](#Config+setQuorumThreshold)
        * [.setLocalPoW(value)](#Config+setLocalPoW)
        * [.setFallbackToLocalPoW(value)](#Config+setFallbackToLocalPoW)
        * [.setTipsInterval(value)](#Config+setTipsInterval)
        * [.setRequestTimeout(value)](#Config+setRequestTimeout)
    * _static_
        * [.fromNetwork(network)](#Config.fromNetwork) ⇒ [<code>Config</code>](#Config)

<a name="new_Config_new"></a>

### new Config()
Creates a new `Config`.

<a name="Config+setNetwork"></a>

### config.setNetwork(network)
Sets the IOTA Tangle network.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| network | [<code>Network</code>](#Network) | 

<a name="Config+setEncoding"></a>

### config.setEncoding(encoding)
Sets the DID message encoding used when publishing to the Tangle.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| encoding | <code>number</code> | 

<a name="Config+setNode"></a>

### config.setNode(url)
Adds an IOTA node by its URL.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 

<a name="Config+setPrimaryNode"></a>

### config.setPrimaryNode(url, jwt, username, password)
Adds an IOTA node by its URL to be used as primary node.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="Config+setPrimaryPoWNode"></a>

### config.setPrimaryPoWNode(url, jwt, username, password)
Adds an IOTA node by its URL to be used as primary PoW node (for remote PoW).

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="Config+setPermanode"></a>

### config.setPermanode(url, jwt, username, password)
Adds a permanode by its URL.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="Config+setNodeAuth"></a>

### config.setNodeAuth(url, jwt, username, password)
Adds an IOTA node by its URL.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 
| jwt | <code>string</code> \| <code>undefined</code> | 
| username | <code>string</code> \| <code>undefined</code> | 
| password | <code>string</code> \| <code>undefined</code> | 

<a name="Config+setNodeSyncInterval"></a>

### config.setNodeSyncInterval(value)
Sets the node sync interval.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config+setNodeSyncDisabled"></a>

### config.setNodeSyncDisabled()
Disables the node sync process.

**Kind**: instance method of [<code>Config</code>](#Config)  
<a name="Config+setQuorum"></a>

### config.setQuorum(value)
Enables/disables quorum.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="Config+setQuorumSize"></a>

### config.setQuorumSize(value)
Sets the number of nodes used for quorum.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config+setQuorumThreshold"></a>

### config.setQuorumThreshold(value)
Sets the quorum threshold.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config+setLocalPoW"></a>

### config.setLocalPoW(value)
Sets whether proof-of-work (PoW) is performed locally or remotely.

Default: false.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="Config+setFallbackToLocalPoW"></a>

### config.setFallbackToLocalPoW(value)
Sets whether the PoW should be done locally in case a node doesn't support remote PoW.

Default: true.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>boolean</code> | 

<a name="Config+setTipsInterval"></a>

### config.setTipsInterval(value)
Sets the number of seconds that new tips will be requested during PoW.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config+setRequestTimeout"></a>

### config.setRequestTimeout(value)
Sets the default request timeout.

**Kind**: instance method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| value | <code>number</code> | 

<a name="Config.fromNetwork"></a>

### Config.fromNetwork(network) ⇒ [<code>Config</code>](#Config)
Creates a new `Config` for the given IOTA Tangle network.

**Kind**: static method of [<code>Config</code>](#Config)  

| Param | Type |
| --- | --- |
| network | [<code>Network</code>](#Network) | 

<a name="Credential"></a>

## Credential
**Kind**: global class  

* [Credential](#Credential)
    * _instance_
        * [.toJSON()](#Credential+toJSON) ⇒ <code>any</code>
    * _static_
        * [.extend(value)](#Credential.extend) ⇒ [<code>Credential</code>](#Credential)
        * [.issue(issuer_doc, subject_data, credential_type, credential_id)](#Credential.issue) ⇒ [<code>Credential</code>](#Credential)
        * [.fromJSON(json)](#Credential.fromJSON) ⇒ [<code>Credential</code>](#Credential)

<a name="Credential+toJSON"></a>

### credential.toJSON() ⇒ <code>any</code>
Serializes a `Credential` object as a JSON object.

**Kind**: instance method of [<code>Credential</code>](#Credential)  
<a name="Credential.extend"></a>

### Credential.extend(value) ⇒ [<code>Credential</code>](#Credential)
**Kind**: static method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="Credential.issue"></a>

### Credential.issue(issuer_doc, subject_data, credential_type, credential_id) ⇒ [<code>Credential</code>](#Credential)
**Kind**: static method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| issuer_doc | [<code>Document</code>](#Document) | 
| subject_data | <code>any</code> | 
| credential_type | <code>string</code> \| <code>undefined</code> | 
| credential_id | <code>string</code> \| <code>undefined</code> | 

<a name="Credential.fromJSON"></a>

### Credential.fromJSON(json) ⇒ [<code>Credential</code>](#Credential)
Deserializes a `Credential` object from a JSON object.

**Kind**: static method of [<code>Credential</code>](#Credential)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="CredentialValidationOptions"></a>

## CredentialValidationOptions
Options to declare validation criteria when validating credentials.

**Kind**: global class  

* [CredentialValidationOptions](#CredentialValidationOptions)
    * [new CredentialValidationOptions(options)](#new_CredentialValidationOptions_new)
    * _instance_
        * [.toJSON()](#CredentialValidationOptions+toJSON) ⇒ <code>any</code>
    * _static_
        * [.default()](#CredentialValidationOptions.default) ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)
        * [.fromJSON(json)](#CredentialValidationOptions.fromJSON) ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)

<a name="new_CredentialValidationOptions_new"></a>

### new CredentialValidationOptions(options)
Creates a new `CredentialValidationOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>ICredentialValidationOptions</code> | 

<a name="CredentialValidationOptions+toJSON"></a>

### credentialValidationOptions.toJSON() ⇒ <code>any</code>
Serializes a `CredentialValidationOptions` as a JSON object.

**Kind**: instance method of [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)  
<a name="CredentialValidationOptions.default"></a>

### CredentialValidationOptions.default() ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)
Creates a new `CredentialValidationOptions` with defaults.

**Kind**: static method of [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)  
<a name="CredentialValidationOptions.fromJSON"></a>

### CredentialValidationOptions.fromJSON(json) ⇒ [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)
Deserializes a `CredentialValidationOptions` from a JSON object.

**Kind**: static method of [<code>CredentialValidationOptions</code>](#CredentialValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="CredentialValidator"></a>

## CredentialValidator
**Kind**: global class  

* [CredentialValidator](#CredentialValidator)
    * [.validate(credential, issuer, options, fail_fast)](#CredentialValidator.validate)
    * [.checkStructure(credential)](#CredentialValidator.checkStructure)
    * [.checkExpiresOnOrAfter(credential, timestamp)](#CredentialValidator.checkExpiresOnOrAfter)
    * [.checkIssuedOnOrBefore(credential, timestamp)](#CredentialValidator.checkIssuedOnOrBefore)
    * [.verifySignature(credential, trusted_issuers, options)](#CredentialValidator.verifySignature)
    * [.check_subject_holder_relationship(credential, holder_url, relationship)](#CredentialValidator.check_subject_holder_relationship)

<a name="CredentialValidator.validate"></a>

### CredentialValidator.validate(credential, issuer, options, fail_fast)
Validates a `Credential`.

The following properties are validated according to `options`:
- the issuer's signature,
- the expiration date,
- the issuance date,
- the semantic structure.

### Warning
The lack of an error returned from this method is in of itself not enough to conclude that the credential can be
trusted. This section contains more information on additional checks that should be carried out before and after
calling this method.

#### The state of the issuer's DID Document
The caller must ensure that `issuer` represents an up-to-date DID Document. The convenience method
`Resolver::resolveCredentialIssuer` can help extract the latest available state of the issuer's DID Document.

#### Properties that are not validated
 There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
`credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
These should be manually checked after validation, according to your requirements.

### Errors
An error is returned whenever a validated condition is not satisfied.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| issuer | [<code>Document</code>](#Document) \| [<code>ResolvedDocument</code>](#ResolvedDocument) | 
| options | [<code>CredentialValidationOptions</code>](#CredentialValidationOptions) | 
| fail_fast | <code>number</code> | 

<a name="CredentialValidator.checkStructure"></a>

### CredentialValidator.checkStructure(credential)
Validates the semantic structure of the `Credential`.

### Warning
This does not validate against the credential's schema nor the structure of the subject claims.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="CredentialValidator.checkExpiresOnOrAfter"></a>

### CredentialValidator.checkExpiresOnOrAfter(credential, timestamp)
Validate that the credential expires on or after the specified timestamp.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="CredentialValidator.checkIssuedOnOrBefore"></a>

### CredentialValidator.checkIssuedOnOrBefore(credential, timestamp)
Validate that the credential is issued on or before the specified timestamp.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="CredentialValidator.verifySignature"></a>

### CredentialValidator.verifySignature(credential, trusted_issuers, options)
Verify the signature using the DID Document of a trusted issuer.

# Warning
The caller must ensure that the DID Documents of the trusted issuers are up-to-date.
### Errors
This method immediately returns an error if
the credential issuer' url cannot be parsed to a DID belonging to one of the trusted issuers. Otherwise an attempt
to verify the credential's signature will be made and an error is returned upon failure.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| trusted_issuers | [<code>Array.&lt;Document&gt;</code>](#Document) \| [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument) | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="CredentialValidator.check_subject_holder_relationship"></a>

### CredentialValidator.check\_subject\_holder\_relationship(credential, holder_url, relationship)
Validate that the relationship between the `holder` and the credential subjects is in accordance with
`relationship`. The `holder_url` parameter is expected to be the URL of the holder.

**Kind**: static method of [<code>CredentialValidator</code>](#CredentialValidator)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 
| holder_url | <code>string</code> | 
| relationship | <code>number</code> | 

<a name="DID"></a>

## DID
**Kind**: global class  

* [DID](#DID)
    * [new DID(key, network)](#new_DID_new)
    * _instance_
        * [.networkName](#DID+networkName) ⇒ <code>string</code>
        * [.tag](#DID+tag) ⇒ <code>string</code>
        * [.network()](#DID+network) ⇒ [<code>Network</code>](#Network)
        * [.join(segment)](#DID+join) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toUrl()](#DID+toUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.intoUrl()](#DID+intoUrl) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toString()](#DID+toString) ⇒ <code>string</code>
        * [.toJSON()](#DID+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromBase58(key, network)](#DID.fromBase58) ⇒ [<code>DID</code>](#DID)
        * [.parse(input)](#DID.parse) ⇒ [<code>DID</code>](#DID)

<a name="new_DID_new"></a>

### new DID(key, network)
Creates a new `DID` from a `KeyPair` object.


| Param | Type |
| --- | --- |
| key | [<code>KeyPair</code>](#KeyPair) | 
| network | <code>string</code> \| <code>undefined</code> | 

<a name="DID+networkName"></a>

### did.networkName ⇒ <code>string</code>
Returns the IOTA tangle network of the `DID`.

**Kind**: instance property of [<code>DID</code>](#DID)  
<a name="DID+tag"></a>

### did.tag ⇒ <code>string</code>
Returns the unique tag of the `DID`.

**Kind**: instance property of [<code>DID</code>](#DID)  
<a name="DID+network"></a>

### did.network() ⇒ [<code>Network</code>](#Network)
Returns the IOTA tangle network of the `DID`.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+join"></a>

### did.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Construct a new `DIDUrl` by joining with a relative DID Url string.

**Kind**: instance method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="DID+toUrl"></a>

### did.toUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Clones the `DID` into a `DIDUrl`.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+intoUrl"></a>

### did.intoUrl() ⇒ [<code>DIDUrl</code>](#DIDUrl)
Converts the `DID` into a `DIDUrl`.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+toString"></a>

### did.toString() ⇒ <code>string</code>
Returns the `DID` as a string.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID+toJSON"></a>

### did.toJSON() ⇒ <code>any</code>
Serializes a `DID` as a JSON object.

**Kind**: instance method of [<code>DID</code>](#DID)  
<a name="DID.fromBase58"></a>

### DID.fromBase58(key, network) ⇒ [<code>DID</code>](#DID)
Creates a new `DID` from a base58-encoded public key.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| network | <code>string</code> \| <code>undefined</code> | 

<a name="DID.parse"></a>

### DID.parse(input) ⇒ [<code>DID</code>](#DID)
Parses a `DID` from the input string.

**Kind**: static method of [<code>DID</code>](#DID)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="DIDUrl"></a>

## DIDUrl
**Kind**: global class  

* [DIDUrl](#DIDUrl)
    * _instance_
        * [.did](#DIDUrl+did) ⇒ [<code>DID</code>](#DID)
        * [.url_str](#DIDUrl+url_str) ⇒ <code>string</code>
        * [.fragment](#DIDUrl+fragment) ⇒ <code>string</code> \| <code>undefined</code>
        * [.fragment](#DIDUrl+fragment)
        * [.path](#DIDUrl+path) ⇒ <code>string</code> \| <code>undefined</code>
        * [.path](#DIDUrl+path)
        * [.query](#DIDUrl+query) ⇒ <code>string</code> \| <code>undefined</code>
        * [.query](#DIDUrl+query)
        * [.join(segment)](#DIDUrl+join) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.toString()](#DIDUrl+toString) ⇒ <code>string</code>
        * [.toJSON()](#DIDUrl+toJSON) ⇒ <code>any</code>
    * _static_
        * [.parse(input)](#DIDUrl.parse) ⇒ [<code>DIDUrl</code>](#DIDUrl)

<a name="DIDUrl+did"></a>

### didUrl.did ⇒ [<code>DID</code>](#DID)
Return the `DID` section of the `DIDUrl`.

Note: clones the data

**Kind**: instance property of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+url_str"></a>

### didUrl.url\_str ⇒ <code>string</code>
Return the relative DID Url as a string, including only the path, query, and fragment.

**Kind**: instance property of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+fragment"></a>

### didUrl.fragment ⇒ <code>string</code> \| <code>undefined</code>
Returns the `DIDUrl` method fragment, if any. Excludes the leading '#'.

**Kind**: instance property of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+fragment"></a>

### didUrl.fragment
Sets the `fragment` component of the `DIDUrl`.

**Kind**: instance property of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+path"></a>

### didUrl.path ⇒ <code>string</code> \| <code>undefined</code>
Returns the `DIDUrl` path.

**Kind**: instance property of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+path"></a>

### didUrl.path
Sets the `path` component of the `DIDUrl`.

**Kind**: instance property of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+query"></a>

### didUrl.query ⇒ <code>string</code> \| <code>undefined</code>
Returns the `DIDUrl` method query, if any. Excludes the leading '?'.

**Kind**: instance property of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+query"></a>

### didUrl.query
Sets the `query` component of the `DIDUrl`.

**Kind**: instance property of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| value | <code>string</code> \| <code>undefined</code> | 

<a name="DIDUrl+join"></a>

### didUrl.join(segment) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Append a string representing a path, query, and/or fragment to this `DIDUrl`.

Must begin with a valid delimiter character: '/', '?', '#'. Overwrites the existing URL
segment and any following segments in order of path, query, then fragment.

I.e.
- joining a path will clear the query and fragment.
- joining a query will clear the fragment.
- joining a fragment will only overwrite the fragment.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| segment | <code>string</code> | 

<a name="DIDUrl+toString"></a>

### didUrl.toString() ⇒ <code>string</code>
Returns the `DIDUrl` as a string.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl+toJSON"></a>

### didUrl.toJSON() ⇒ <code>any</code>
Serializes a `DIDUrl` as a JSON object.

**Kind**: instance method of [<code>DIDUrl</code>](#DIDUrl)  
<a name="DIDUrl.parse"></a>

### DIDUrl.parse(input) ⇒ [<code>DIDUrl</code>](#DIDUrl)
Parses a `DIDUrl` from the input string.

**Kind**: static method of [<code>DIDUrl</code>](#DIDUrl)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="DiffChainHistory"></a>

## DiffChainHistory
**Kind**: global class  

* [DiffChainHistory](#DiffChainHistory)
    * _instance_
        * [.chainData()](#DiffChainHistory+chainData) ⇒ [<code>Array.&lt;DiffMessage&gt;</code>](#DiffMessage)
        * [.spam()](#DiffChainHistory+spam) ⇒ <code>Array.&lt;string&gt;</code>
        * [.toJSON()](#DiffChainHistory+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#DiffChainHistory.fromJSON) ⇒ [<code>DiffChainHistory</code>](#DiffChainHistory)

<a name="DiffChainHistory+chainData"></a>

### diffChainHistory.chainData() ⇒ [<code>Array.&lt;DiffMessage&gt;</code>](#DiffMessage)
Returns an `Array` of the diff chain `DiffMessages`.

NOTE: this clones the field.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory+spam"></a>

### diffChainHistory.spam() ⇒ <code>Array.&lt;string&gt;</code>
Returns an `Array` of `MessageIds` as strings.

NOTE: this clones the field.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory+toJSON"></a>

### diffChainHistory.toJSON() ⇒ <code>any</code>
Serializes as a JSON object.

**Kind**: instance method of [<code>DiffChainHistory</code>](#DiffChainHistory)  
<a name="DiffChainHistory.fromJSON"></a>

### DiffChainHistory.fromJSON(json) ⇒ [<code>DiffChainHistory</code>](#DiffChainHistory)
Deserializes from a JSON object.

**Kind**: static method of [<code>DiffChainHistory</code>](#DiffChainHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DiffMessage"></a>

## DiffMessage
Defines the difference between two DID `Document`s' JSON representations.

**Kind**: global class  

* [DiffMessage](#DiffMessage)
    * _instance_
        * [.did](#DiffMessage+did) ⇒ [<code>DID</code>](#DID)
        * [.diff](#DiffMessage+diff) ⇒ <code>string</code>
        * [.messageId](#DiffMessage+messageId) ⇒ <code>string</code>
        * [.messageId](#DiffMessage+messageId)
        * [.previousMessageId](#DiffMessage+previousMessageId) ⇒ <code>string</code>
        * [.previousMessageId](#DiffMessage+previousMessageId)
        * [.proof](#DiffMessage+proof) ⇒ <code>any</code>
        * [.id()](#DiffMessage+id) ⇒ [<code>DID</code>](#DID)
        * [.merge(document)](#DiffMessage+merge) ⇒ [<code>Document</code>](#Document)
        * [.toJSON()](#DiffMessage+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#DiffMessage.fromJSON) ⇒ [<code>DiffMessage</code>](#DiffMessage)

<a name="DiffMessage+did"></a>

### diffMessage.did ⇒ [<code>DID</code>](#DID)
Returns the DID of the associated DID Document.

**Kind**: instance property of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+diff"></a>

### diffMessage.diff ⇒ <code>string</code>
Returns the raw contents of the DID Document diff as a JSON string.

NOTE: clones the data.

**Kind**: instance property of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+messageId"></a>

### diffMessage.messageId ⇒ <code>string</code>
Returns the message_id of the DID Document diff.

**Kind**: instance property of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+messageId"></a>

### diffMessage.messageId
Sets the message_id of the DID Document diff.

**Kind**: instance property of [<code>DiffMessage</code>](#DiffMessage)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="DiffMessage+previousMessageId"></a>

### diffMessage.previousMessageId ⇒ <code>string</code>
Returns the Tangle message id of the previous DID Document diff.

**Kind**: instance property of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+previousMessageId"></a>

### diffMessage.previousMessageId
Sets the Tangle message id of the previous DID Document diff.

**Kind**: instance property of [<code>DiffMessage</code>](#DiffMessage)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="DiffMessage+proof"></a>

### diffMessage.proof ⇒ <code>any</code>
Returns the `proof` object.

**Kind**: instance property of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+id"></a>

### diffMessage.id() ⇒ [<code>DID</code>](#DID)
Returns the DID of the associated DID Document.

NOTE: clones the data.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage+merge"></a>

### diffMessage.merge(document) ⇒ [<code>Document</code>](#Document)
Returns a new DID Document which is the result of merging `self`
with the given Document.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="DiffMessage+toJSON"></a>

### diffMessage.toJSON() ⇒ <code>any</code>
Serializes a `DiffMessage` as a JSON object.

**Kind**: instance method of [<code>DiffMessage</code>](#DiffMessage)  
<a name="DiffMessage.fromJSON"></a>

### DiffMessage.fromJSON(json) ⇒ [<code>DiffMessage</code>](#DiffMessage)
Deserializes a `DiffMessage` from a JSON object.

**Kind**: static method of [<code>DiffMessage</code>](#DiffMessage)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Document"></a>

## Document
**Kind**: global class  

* [Document](#Document)
    * [new Document(keypair, network, fragment)](#new_Document_new)
    * _instance_
        * [.id](#Document+id) ⇒ [<code>DID</code>](#DID)
        * [.metadata](#Document+metadata) ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)
        * [.metadataCreated](#Document+metadataCreated) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.metadataCreated](#Document+metadataCreated)
        * [.metadataUpdated](#Document+metadataUpdated) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.metadataUpdated](#Document+metadataUpdated)
        * [.metadataPreviousMessageId](#Document+metadataPreviousMessageId) ⇒ <code>string</code>
        * [.metadataPreviousMessageId](#Document+metadataPreviousMessageId)
        * [.metadataProof](#Document+metadataProof) ⇒ <code>any</code>
        * [.setController(controllers)](#Document+setController)
        * [.controller()](#Document+controller) ⇒ [<code>Array.&lt;DID&gt;</code>](#DID)
        * [.setAlsoKnownAs(urls)](#Document+setAlsoKnownAs)
        * [.alsoKnownAs()](#Document+alsoKnownAs) ⇒ <code>Array.&lt;string&gt;</code>
        * [.setPropertyUnchecked(key, value)](#Document+setPropertyUnchecked)
        * [.properties()](#Document+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.service()](#Document+service) ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
        * [.insertService(service)](#Document+insertService) ⇒ <code>boolean</code>
        * [.removeService(did)](#Document+removeService)
        * [.methods()](#Document+methods) ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
        * [.insertMethod(method, scope)](#Document+insertMethod)
        * [.removeMethod(did)](#Document+removeMethod)
        * [.defaultSigningMethod()](#Document+defaultSigningMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.resolveMethod(query, scope)](#Document+resolveMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.resolveSigningMethod(query)](#Document+resolveSigningMethod) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.revokeMerkleKey(query, index)](#Document+revokeMerkleKey) ⇒ <code>boolean</code>
        * [.attachMethodRelationship(did_url, relationship)](#Document+attachMethodRelationship) ⇒ <code>boolean</code>
        * [.detachMethodRelationship(did_url, relationship)](#Document+detachMethodRelationship) ⇒ <code>boolean</code>
        * [.signSelf(key_pair, method_query)](#Document+signSelf)
        * [.signDocument(document, key_pair, method_query)](#Document+signDocument)
        * [.signCredential(data, args, options)](#Document+signCredential) ⇒ [<code>Credential</code>](#Credential)
        * [.signPresentation(data, args, options)](#Document+signPresentation) ⇒ [<code>Presentation</code>](#Presentation)
        * [.signData(data, args, options)](#Document+signData) ⇒ <code>any</code>
        * [.verifyData(data, options)](#Document+verifyData) ⇒ <code>boolean</code>
        * [.verifyDocument(signed)](#Document+verifyDocument)
        * [.diff(other, message_id, key, method_query)](#Document+diff) ⇒ [<code>DiffMessage</code>](#DiffMessage)
        * [.verifyDiff(diff)](#Document+verifyDiff)
        * [.mergeDiff(diff)](#Document+mergeDiff)
        * [.integrationIndex()](#Document+integrationIndex) ⇒ <code>string</code>
        * [.toJSON()](#Document+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromVerificationMethod(method)](#Document.fromVerificationMethod) ⇒ [<code>Document</code>](#Document)
        * [.isSigningMethodType(method_type)](#Document.isSigningMethodType) ⇒ <code>boolean</code>
        * [.verifyRootDocument(document)](#Document.verifyRootDocument)
        * [.diffIndex(message_id)](#Document.diffIndex) ⇒ <code>string</code>
        * [.fromJSON(json)](#Document.fromJSON) ⇒ [<code>Document</code>](#Document)

<a name="new_Document_new"></a>

### new Document(keypair, network, fragment)
Creates a new DID Document from the given `KeyPair`, network, and verification method
fragment name.

The DID Document will be pre-populated with a single verification method
derived from the provided `KeyPair` embedded as a capability invocation
verification relationship. This method will have the DID URL fragment
`#sign-0` by default and can be easily retrieved with `Document::defaultSigningMethod`.

NOTE: the generated document is unsigned, see `Document::signSelf`.

Arguments:

* keypair: the initial verification method is derived from the public key with this keypair.
* network: Tangle network to use for the DID, default `Network::mainnet`.
* fragment: name of the initial verification method, default "sign-0".


| Param | Type |
| --- | --- |
| keypair | [<code>KeyPair</code>](#KeyPair) | 
| network | <code>string</code> \| <code>undefined</code> | 
| fragment | <code>string</code> \| <code>undefined</code> | 

<a name="Document+id"></a>

### document.id ⇒ [<code>DID</code>](#DID)
Returns the DID Document `id`.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+metadata"></a>

### document.metadata ⇒ [<code>DocumentMetadata</code>](#DocumentMetadata)
Returns the metadata associated with this document.

NOTE: clones the data. Use the `metadataCreated`, `metadataUpdated`,
`metadataPreviousMessageId`, `metadataProof` properties instead.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+metadataCreated"></a>

### document.metadataCreated ⇒ [<code>Timestamp</code>](#Timestamp)
Returns the timestamp of when the DID document was created.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+metadataCreated"></a>

### document.metadataCreated
Sets the timestamp of when the DID document was created.

**Kind**: instance property of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="Document+metadataUpdated"></a>

### document.metadataUpdated ⇒ [<code>Timestamp</code>](#Timestamp)
Returns the timestamp of the last DID document update.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+metadataUpdated"></a>

### document.metadataUpdated
Sets the timestamp of the last DID document update.

**Kind**: instance property of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| timestamp | [<code>Timestamp</code>](#Timestamp) | 

<a name="Document+metadataPreviousMessageId"></a>

### document.metadataPreviousMessageId ⇒ <code>string</code>
Returns the previous integration chain message id.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+metadataPreviousMessageId"></a>

### document.metadataPreviousMessageId
Sets the previous integration chain message id.

**Kind**: instance property of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="Document+metadataProof"></a>

### document.metadataProof ⇒ <code>any</code>
Returns the `proof` object.

**Kind**: instance property of [<code>Document</code>](#Document)  
<a name="Document+setController"></a>

### document.setController(controllers)
Sets the controllers of the DID Document.

Note: Duplicates will be ignored.
Use `null` to remove all controllers.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| controllers | [<code>DID</code>](#DID) \| [<code>Array.&lt;DID&gt;</code>](#DID) \| <code>null</code> | 

<a name="Document+controller"></a>

### document.controller() ⇒ [<code>Array.&lt;DID&gt;</code>](#DID)
Returns a list of document controllers.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+setAlsoKnownAs"></a>

### document.setAlsoKnownAs(urls)
Sets the `alsoKnownAs` property in the DID document.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| urls | <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>null</code> | 

<a name="Document+alsoKnownAs"></a>

### document.alsoKnownAs() ⇒ <code>Array.&lt;string&gt;</code>
Returns a set of the document's `alsoKnownAs`.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+setPropertyUnchecked"></a>

### document.setPropertyUnchecked(key, value)
Adds a custom property to the DID Document.
If the value is set to `null`, the custom property will be removed.

### WARNING
This method can overwrite existing properties like `id` and result in an invalid document.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| key | <code>string</code> | 
| value | <code>any</code> | 

<a name="Document+properties"></a>

### document.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom DID Document properties.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+service"></a>

### document.service() ⇒ [<code>Array.&lt;Service&gt;</code>](#Service)
Return a set of all [Services](#Service) in the document.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+insertService"></a>

### document.insertService(service) ⇒ <code>boolean</code>
Add a new [Service](#Service) to the document.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| service | [<code>Service</code>](#Service) | 

<a name="Document+removeService"></a>

### document.removeService(did)
Remove a [Service](#Service) identified by the given [DIDUrl](#DIDUrl) from the document.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="Document+methods"></a>

### document.methods() ⇒ [<code>Array.&lt;VerificationMethod&gt;</code>](#VerificationMethod)
Returns a list of all [VerificationMethod](#VerificationMethod) in the DID Document.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+insertMethod"></a>

### document.insertMethod(method, scope)
Adds a new Verification Method to the DID Document.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 
| scope | [<code>MethodScope</code>](#MethodScope) | 

<a name="Document+removeMethod"></a>

### document.removeMethod(did)
Removes all references to the specified Verification Method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did | [<code>DIDUrl</code>](#DIDUrl) | 

<a name="Document+defaultSigningMethod"></a>

### document.defaultSigningMethod() ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Returns a copy of the first `VerificationMethod` with a capability invocation relationship
capable of signing this DID document.

Throws an error if no signing method is present.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+resolveMethod"></a>

### document.resolveMethod(query, scope) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Returns a copy of the first `VerificationMethod` with an `id` property
matching the provided `query`.

Throws an error if the method is not found.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| scope | [<code>MethodScope</code>](#MethodScope) \| <code>undefined</code> | 

<a name="Document+resolveSigningMethod"></a>

### document.resolveSigningMethod(query) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Attempts to resolve the given method query into a method capable of signing a document update.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+revokeMerkleKey"></a>

### document.revokeMerkleKey(query, index) ⇒ <code>boolean</code>
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 
| index | <code>number</code> | 

<a name="Document+attachMethodRelationship"></a>

### document.attachMethodRelationship(did_url, relationship) ⇒ <code>boolean</code>
Attaches the relationship to the given method, if the method exists.

Note: The method needs to be in the set of verification methods,
so it cannot be an embedded one.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did_url | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="Document+detachMethodRelationship"></a>

### document.detachMethodRelationship(did_url, relationship) ⇒ <code>boolean</code>
Detaches the given relationship from the given method, if the method exists.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| did_url | [<code>DIDUrl</code>](#DIDUrl) | 
| relationship | <code>number</code> | 

<a name="Document+signSelf"></a>

### document.signSelf(key_pair, method_query)
Signs the DID document with the verification method specified by `method_query`.
The `method_query` may be the full `DIDUrl` of the method or just its fragment,
e.g. "#sign-0".

NOTE: does not validate whether the private key of the given `key_pair` corresponds to the
verification method. See `Document::verifySelfSigned`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| key_pair | [<code>KeyPair</code>](#KeyPair) | 
| method_query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+signDocument"></a>

### document.signDocument(document, key_pair, method_query)
Signs another DID document using the verification method specified by `method_query`.
The `method_query` may be the full `DIDUrl` of the method or just its fragment,
e.g. "#sign-0".

`Document.signSelf` should be used in general, this throws an error if trying to operate
on the same document. This is intended for signing updates to a document where a sole
capability invocation method is rotated or replaced entirely.

NOTE: does not validate whether the private key of the given `key_pair` corresponds to the
verification method. See [Document.verifyDocument](Document.verifyDocument).

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 
| key_pair | [<code>KeyPair</code>](#KeyPair) | 
| method_query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+signCredential"></a>

### document.signCredential(data, args, options) ⇒ [<code>Credential</code>](#Credential)
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| args | <code>any</code> | 
| options | [<code>SignatureOptions</code>](#SignatureOptions) | 

<a name="Document+signPresentation"></a>

### document.signPresentation(data, args, options) ⇒ [<code>Presentation</code>](#Presentation)
**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| args | <code>any</code> | 
| options | [<code>SignatureOptions</code>](#SignatureOptions) | 

<a name="Document+signData"></a>

### document.signData(data, args, options) ⇒ <code>any</code>
Creates a signature for the given `data` with the specified DID Document
Verification Method.

An additional `proof` property is required if using a Merkle Key
Collection verification Method.

NOTE: use `signSelf` or `signDocument` for DID Documents.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| args | <code>any</code> | 
| options | [<code>SignatureOptions</code>](#SignatureOptions) | 

<a name="Document+verifyData"></a>

### document.verifyData(data, options) ⇒ <code>boolean</code>
Verifies the authenticity of `data` using the target verification method.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| data | <code>any</code> | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="Document+verifyDocument"></a>

### document.verifyDocument(signed)
Verifies that the signature on the DID document `signed` was generated by a valid method from
this DID document.

# Errors

Fails if:
- The signature proof section is missing in the `signed` document.
- The method is not found in this document.
- An unsupported verification method is used.
- The signature verification operation fails.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| signed | [<code>Document</code>](#Document) | 

<a name="Document+diff"></a>

### document.diff(other, message_id, key, method_query) ⇒ [<code>DiffMessage</code>](#DiffMessage)
Generate a `DiffMessage` between two DID Documents and sign it using the specified
`key` and `method`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| other | [<code>Document</code>](#Document) | 
| message_id | <code>string</code> | 
| key | [<code>KeyPair</code>](#KeyPair) | 
| method_query | [<code>DIDUrl</code>](#DIDUrl) \| <code>string</code> | 

<a name="Document+verifyDiff"></a>

### document.verifyDiff(diff)
Verifies the signature of the `diff` was created using a capability invocation method
in this DID Document.

# Errors

Fails if an unsupported verification method is used or the verification operation fails.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| diff | [<code>DiffMessage</code>](#DiffMessage) | 

<a name="Document+mergeDiff"></a>

### document.mergeDiff(diff)
Verifies a `DiffMessage` signature and attempts to merge the changes into `self`.

**Kind**: instance method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| diff | [<code>DiffMessage</code>](#DiffMessage) | 

<a name="Document+integrationIndex"></a>

### document.integrationIndex() ⇒ <code>string</code>
Returns the Tangle index of the integration chain for this DID.

This is simply the tag segment of the `DID`.
E.g.
For a document with DID: did:iota:1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI,
`doc.integration_index()` == "1234567890abcdefghijklmnopqrstuvxyzABCDEFGHI"

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document+toJSON"></a>

### document.toJSON() ⇒ <code>any</code>
Serializes a `Document` as a JSON object.

**Kind**: instance method of [<code>Document</code>](#Document)  
<a name="Document.fromVerificationMethod"></a>

### Document.fromVerificationMethod(method) ⇒ [<code>Document</code>](#Document)
Creates a new DID Document from the given `VerificationMethod`.

NOTE: the generated document is unsigned, see `Document::signSelf`.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method | [<code>VerificationMethod</code>](#VerificationMethod) | 

<a name="Document.isSigningMethodType"></a>

### Document.isSigningMethodType(method_type) ⇒ <code>boolean</code>
Returns whether the given [MethodType](#MethodType) can be used to sign document updates.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| method_type | [<code>MethodType</code>](#MethodType) | 

<a name="Document.verifyRootDocument"></a>

### Document.verifyRootDocument(document)
Verifies whether `document` is a valid root DID document according to the IOTA DID method
specification.

It must be signed using a verification method with a public key whose BLAKE2b-256 hash matches
the DID tag.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| document | [<code>Document</code>](#Document) | 

<a name="Document.diffIndex"></a>

### Document.diffIndex(message_id) ⇒ <code>string</code>
Returns the Tangle index of the DID diff chain. This should only be called on documents
published on the integration chain.

This is the Base58-btc encoded SHA-256 digest of the hex-encoded message id.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="Document.fromJSON"></a>

### Document.fromJSON(json) ⇒ [<code>Document</code>](#Document)
Deserializes a `Document` from a JSON object.

**Kind**: static method of [<code>Document</code>](#Document)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DocumentHistory"></a>

## DocumentHistory
A DID Document's history and current state.

**Kind**: global class  

* [DocumentHistory](#DocumentHistory)
    * _instance_
        * [.integrationChainData()](#DocumentHistory+integrationChainData) ⇒ [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.integrationChainSpam()](#DocumentHistory+integrationChainSpam) ⇒ <code>Array.&lt;string&gt;</code>
        * [.diffChainData()](#DocumentHistory+diffChainData) ⇒ [<code>Array.&lt;DiffMessage&gt;</code>](#DiffMessage)
        * [.diffChainSpam()](#DocumentHistory+diffChainSpam) ⇒ <code>Array.&lt;string&gt;</code>
        * [.toJSON()](#DocumentHistory+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#DocumentHistory.fromJSON) ⇒ [<code>DocumentHistory</code>](#DocumentHistory)

<a name="DocumentHistory+integrationChainData"></a>

### documentHistory.integrationChainData() ⇒ [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Returns an `Array` of integration chain `Documents`.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+integrationChainSpam"></a>

### documentHistory.integrationChainSpam() ⇒ <code>Array.&lt;string&gt;</code>
Returns an `Array` of message id strings for "spam" messages on the same index
as the integration chain.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+diffChainData"></a>

### documentHistory.diffChainData() ⇒ [<code>Array.&lt;DiffMessage&gt;</code>](#DiffMessage)
Returns an `Array` of diff chain `DiffMessages`.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+diffChainSpam"></a>

### documentHistory.diffChainSpam() ⇒ <code>Array.&lt;string&gt;</code>
Returns an `Array` of message id strings for "spam" messages on the same index
as the diff chain.

NOTE: clones the data.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory+toJSON"></a>

### documentHistory.toJSON() ⇒ <code>any</code>
Serializes `DocumentHistory` as a JSON object.

**Kind**: instance method of [<code>DocumentHistory</code>](#DocumentHistory)  
<a name="DocumentHistory.fromJSON"></a>

### DocumentHistory.fromJSON(json) ⇒ [<code>DocumentHistory</code>](#DocumentHistory)
Deserializes `DocumentHistory` from a JSON object.

**Kind**: static method of [<code>DocumentHistory</code>](#DocumentHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="DocumentMetadata"></a>

## DocumentMetadata
Additional attributes related to an IOTA DID Document.

**Kind**: global class  

* [DocumentMetadata](#DocumentMetadata)
    * [.created](#DocumentMetadata+created) ⇒ [<code>Timestamp</code>](#Timestamp)
    * [.updated](#DocumentMetadata+updated) ⇒ [<code>Timestamp</code>](#Timestamp)
    * [.previousMessageId](#DocumentMetadata+previousMessageId) ⇒ <code>string</code>
    * [.proof](#DocumentMetadata+proof) ⇒ <code>any</code>

<a name="DocumentMetadata+created"></a>

### documentMetadata.created ⇒ [<code>Timestamp</code>](#Timestamp)
Returns the timestamp of when the DID document was created.

**Kind**: instance property of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+updated"></a>

### documentMetadata.updated ⇒ [<code>Timestamp</code>](#Timestamp)
Returns the timestamp of the last DID document update.

**Kind**: instance property of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+previousMessageId"></a>

### documentMetadata.previousMessageId ⇒ <code>string</code>
**Kind**: instance property of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="DocumentMetadata+proof"></a>

### documentMetadata.proof ⇒ <code>any</code>
Returns a reference to the `proof`.

**Kind**: instance property of [<code>DocumentMetadata</code>](#DocumentMetadata)  
<a name="Duration"></a>

## Duration
A span of time.

**Kind**: global class  

* [Duration](#Duration)
    * [.seconds(seconds)](#Duration.seconds) ⇒ [<code>Duration</code>](#Duration)
    * [.minutes(minutes)](#Duration.minutes) ⇒ [<code>Duration</code>](#Duration)
    * [.hours(hours)](#Duration.hours) ⇒ [<code>Duration</code>](#Duration)
    * [.days(days)](#Duration.days) ⇒ [<code>Duration</code>](#Duration)
    * [.weeks(weeks)](#Duration.weeks) ⇒ [<code>Duration</code>](#Duration)

<a name="Duration.seconds"></a>

### Duration.seconds(seconds) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of seconds.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| seconds | <code>number</code> | 

<a name="Duration.minutes"></a>

### Duration.minutes(minutes) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of minutes.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| minutes | <code>number</code> | 

<a name="Duration.hours"></a>

### Duration.hours(hours) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of hours.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| hours | <code>number</code> | 

<a name="Duration.days"></a>

### Duration.days(days) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of days.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| days | <code>number</code> | 

<a name="Duration.weeks"></a>

### Duration.weeks(weeks) ⇒ [<code>Duration</code>](#Duration)
Create a new `Duration` with the given number of weeks.

**Kind**: static method of [<code>Duration</code>](#Duration)  

| Param | Type |
| --- | --- |
| weeks | <code>number</code> | 

<a name="ExplorerUrl"></a>

## ExplorerUrl
**Kind**: global class  

* [ExplorerUrl](#ExplorerUrl)
    * _instance_
        * [.messageUrl(message_id)](#ExplorerUrl+messageUrl) ⇒ <code>string</code>
        * [.resolverUrl(did)](#ExplorerUrl+resolverUrl) ⇒ <code>string</code>
        * [.toString()](#ExplorerUrl+toString) ⇒ <code>string</code>
    * _static_
        * [.parse(url)](#ExplorerUrl.parse) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
        * [.mainnet()](#ExplorerUrl.mainnet) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
        * [.devnet()](#ExplorerUrl.devnet) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)

<a name="ExplorerUrl+messageUrl"></a>

### explorerUrl.messageUrl(message_id) ⇒ <code>string</code>
Returns the web explorer URL of the given `message_id`.

E.g. https://explorer.iota.org/mainnet/message/{message_id}

**Kind**: instance method of [<code>ExplorerUrl</code>](#ExplorerUrl)  

| Param | Type |
| --- | --- |
| message_id | <code>string</code> | 

<a name="ExplorerUrl+resolverUrl"></a>

### explorerUrl.resolverUrl(did) ⇒ <code>string</code>
Returns the web identity resolver URL for the given DID.

E.g. https://explorer.iota.org/mainnet/identity-resolver/{did}

**Kind**: instance method of [<code>ExplorerUrl</code>](#ExplorerUrl)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

<a name="ExplorerUrl+toString"></a>

### explorerUrl.toString() ⇒ <code>string</code>
**Kind**: instance method of [<code>ExplorerUrl</code>](#ExplorerUrl)  
<a name="ExplorerUrl.parse"></a>

### ExplorerUrl.parse(url) ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
Constructs a new Tangle explorer URL from a string.

Use `ExplorerUrl::mainnet` or `ExplorerUrl::devnet` unless using a private Tangle
or local explorer.

**Kind**: static method of [<code>ExplorerUrl</code>](#ExplorerUrl)  

| Param | Type |
| --- | --- |
| url | <code>string</code> | 

<a name="ExplorerUrl.mainnet"></a>

### ExplorerUrl.mainnet() ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
Returns the Tangle explorer URL for the mainnet.

**Kind**: static method of [<code>ExplorerUrl</code>](#ExplorerUrl)  
<a name="ExplorerUrl.devnet"></a>

### ExplorerUrl.devnet() ⇒ [<code>ExplorerUrl</code>](#ExplorerUrl)
Returns the Tangle explorer URL for the devnet.

**Kind**: static method of [<code>ExplorerUrl</code>](#ExplorerUrl)  
<a name="IntegrationChainHistory"></a>

## IntegrationChainHistory
**Kind**: global class  

* [IntegrationChainHistory](#IntegrationChainHistory)
    * _instance_
        * [.chainData()](#IntegrationChainHistory+chainData) ⇒ [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
        * [.spam()](#IntegrationChainHistory+spam) ⇒ <code>Array.&lt;string&gt;</code>
        * [.toJSON()](#IntegrationChainHistory+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#IntegrationChainHistory.fromJSON) ⇒ [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)

<a name="IntegrationChainHistory+chainData"></a>

### integrationChainHistory.chainData() ⇒ [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Returns an `Array` of the integration chain `Documents`.

NOTE: this clones the field.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory+spam"></a>

### integrationChainHistory.spam() ⇒ <code>Array.&lt;string&gt;</code>
Returns an `Array` of `MessageIds` as strings.

NOTE: this clones the field.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory+toJSON"></a>

### integrationChainHistory.toJSON() ⇒ <code>any</code>
Serializes as a JSON object.

**Kind**: instance method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  
<a name="IntegrationChainHistory.fromJSON"></a>

### IntegrationChainHistory.fromJSON(json) ⇒ [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)
Deserializes from a JSON object.

**Kind**: static method of [<code>IntegrationChainHistory</code>](#IntegrationChainHistory)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="KeyCollection"></a>

## KeyCollection
**Kind**: global class  

* [KeyCollection](#KeyCollection)
    * [new KeyCollection(type_, count)](#new_KeyCollection_new)
    * _instance_
        * [.length](#KeyCollection+length) ⇒ <code>number</code>
        * [.isEmpty()](#KeyCollection+isEmpty) ⇒ <code>boolean</code>
        * [.keypair(index)](#KeyCollection+keypair) ⇒ [<code>KeyPair</code>](#KeyPair) \| <code>undefined</code>
        * [.public(index)](#KeyCollection+public) ⇒ <code>string</code> \| <code>undefined</code>
        * [.private(index)](#KeyCollection+private) ⇒ <code>string</code> \| <code>undefined</code>
        * [.merkleRoot(digest)](#KeyCollection+merkleRoot) ⇒ <code>string</code>
        * [.merkleProof(digest, index)](#KeyCollection+merkleProof) ⇒ <code>string</code> \| <code>undefined</code>
        * [.toJSON()](#KeyCollection+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#KeyCollection.fromJSON) ⇒ [<code>KeyCollection</code>](#KeyCollection)

<a name="new_KeyCollection_new"></a>

### new KeyCollection(type_, count)
Creates a new `KeyCollection` with the specified key type.


| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 
| count | <code>number</code> | 

<a name="KeyCollection+length"></a>

### keyCollection.length ⇒ <code>number</code>
Returns the number of keys in the collection.

**Kind**: instance property of [<code>KeyCollection</code>](#KeyCollection)  
<a name="KeyCollection+isEmpty"></a>

### keyCollection.isEmpty() ⇒ <code>boolean</code>
Returns `true` if the collection contains no keys.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  
<a name="KeyCollection+keypair"></a>

### keyCollection.keypair(index) ⇒ [<code>KeyPair</code>](#KeyPair) \| <code>undefined</code>
Returns the keypair at the specified `index`.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="KeyCollection+public"></a>

### keyCollection.public(index) ⇒ <code>string</code> \| <code>undefined</code>
Returns the public key at the specified `index` as a base58-encoded string.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="KeyCollection+private"></a>

### keyCollection.private(index) ⇒ <code>string</code> \| <code>undefined</code>
Returns the private key at the specified `index` as a base58-encoded string.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| index | <code>number</code> | 

<a name="KeyCollection+merkleRoot"></a>

### keyCollection.merkleRoot(digest) ⇒ <code>string</code>
**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| digest | <code>number</code> | 

<a name="KeyCollection+merkleProof"></a>

### keyCollection.merkleProof(digest, index) ⇒ <code>string</code> \| <code>undefined</code>
**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| digest | <code>number</code> | 
| index | <code>number</code> | 

<a name="KeyCollection+toJSON"></a>

### keyCollection.toJSON() ⇒ <code>any</code>
Serializes a `KeyCollection` object as a JSON object.

**Kind**: instance method of [<code>KeyCollection</code>](#KeyCollection)  
<a name="KeyCollection.fromJSON"></a>

### KeyCollection.fromJSON(json) ⇒ [<code>KeyCollection</code>](#KeyCollection)
Deserializes a `KeyCollection` object from a JSON object.

**Kind**: static method of [<code>KeyCollection</code>](#KeyCollection)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="KeyPair"></a>

## KeyPair
**Kind**: global class  

* [KeyPair](#KeyPair)
    * [new KeyPair(type_)](#new_KeyPair_new)
    * _instance_
        * [.type](#KeyPair+type) ⇒ <code>number</code>
        * [.public](#KeyPair+public) ⇒ <code>string</code>
        * [.private](#KeyPair+private) ⇒ <code>string</code>
        * [.toJSON()](#KeyPair+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromBase58(type_, public_key, private_key)](#KeyPair.fromBase58) ⇒ [<code>KeyPair</code>](#KeyPair)
        * [.fromJSON(json)](#KeyPair.fromJSON) ⇒ [<code>KeyPair</code>](#KeyPair)

<a name="new_KeyPair_new"></a>

### new KeyPair(type_)
Generates a new `KeyPair` object.


| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 

<a name="KeyPair+type"></a>

### keyPair.type ⇒ <code>number</code>
Returns the private key as a base58-encoded string.

**Kind**: instance property of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+public"></a>

### keyPair.public ⇒ <code>string</code>
Returns the public key as a base58-encoded string.

**Kind**: instance property of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+private"></a>

### keyPair.private ⇒ <code>string</code>
Returns the private key as a base58-encoded string.

**Kind**: instance property of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair+toJSON"></a>

### keyPair.toJSON() ⇒ <code>any</code>
Serializes a `KeyPair` object as a JSON object.

**Kind**: instance method of [<code>KeyPair</code>](#KeyPair)  
<a name="KeyPair.fromBase58"></a>

### KeyPair.fromBase58(type_, public_key, private_key) ⇒ [<code>KeyPair</code>](#KeyPair)
Parses a `KeyPair` object from base58-encoded public/private keys.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| type_ | <code>number</code> | 
| public_key | <code>string</code> | 
| private_key | <code>string</code> | 

<a name="KeyPair.fromJSON"></a>

### KeyPair.fromJSON(json) ⇒ [<code>KeyPair</code>](#KeyPair)
Deserializes a `KeyPair` object from a JSON object.

**Kind**: static method of [<code>KeyPair</code>](#KeyPair)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="MethodScope"></a>

## MethodScope
Supported verification method types.

**Kind**: global class  

* [MethodScope](#MethodScope)
    * _instance_
        * [.toString()](#MethodScope+toString) ⇒ <code>string</code>
        * [.toJSON()](#MethodScope+toJSON) ⇒ <code>any</code>
    * _static_
        * [.VerificationMethod()](#MethodScope.VerificationMethod) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.Authentication()](#MethodScope.Authentication) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.AssertionMethod()](#MethodScope.AssertionMethod) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.KeyAgreement()](#MethodScope.KeyAgreement) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.CapabilityDelegation()](#MethodScope.CapabilityDelegation) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.CapabilityInvocation()](#MethodScope.CapabilityInvocation) ⇒ [<code>MethodScope</code>](#MethodScope)
        * [.fromJSON(json)](#MethodScope.fromJSON) ⇒ [<code>MethodScope</code>](#MethodScope)

<a name="MethodScope+toString"></a>

### methodScope.toString() ⇒ <code>string</code>
Returns the `MethodScope` as a string.

**Kind**: instance method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope+toJSON"></a>

### methodScope.toJSON() ⇒ <code>any</code>
Serializes a `MethodScope` object as a JSON object.

**Kind**: instance method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.VerificationMethod"></a>

### MethodScope.VerificationMethod() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.Authentication"></a>

### MethodScope.Authentication() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.AssertionMethod"></a>

### MethodScope.AssertionMethod() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.KeyAgreement"></a>

### MethodScope.KeyAgreement() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.CapabilityDelegation"></a>

### MethodScope.CapabilityDelegation() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.CapabilityInvocation"></a>

### MethodScope.CapabilityInvocation() ⇒ [<code>MethodScope</code>](#MethodScope)
**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  
<a name="MethodScope.fromJSON"></a>

### MethodScope.fromJSON(json) ⇒ [<code>MethodScope</code>](#MethodScope)
Deserializes a `MethodScope` object from a JSON object.

**Kind**: static method of [<code>MethodScope</code>](#MethodScope)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="MethodType"></a>

## MethodType
Supported verification method types.

**Kind**: global class  

* [MethodType](#MethodType)
    * _instance_
        * [.toJSON()](#MethodType+toJSON) ⇒ <code>any</code>
    * _static_
        * [.Ed25519VerificationKey2018()](#MethodType.Ed25519VerificationKey2018) ⇒ [<code>MethodType</code>](#MethodType)
        * [.MerkleKeyCollection2021()](#MethodType.MerkleKeyCollection2021) ⇒ [<code>MethodType</code>](#MethodType)
        * [.fromJSON(json)](#MethodType.fromJSON) ⇒ [<code>MethodType</code>](#MethodType)

<a name="MethodType+toJSON"></a>

### methodType.toJSON() ⇒ <code>any</code>
Serializes a `MethodType` object as a JSON object.

**Kind**: instance method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType.Ed25519VerificationKey2018"></a>

### MethodType.Ed25519VerificationKey2018() ⇒ [<code>MethodType</code>](#MethodType)
**Kind**: static method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType.MerkleKeyCollection2021"></a>

### MethodType.MerkleKeyCollection2021() ⇒ [<code>MethodType</code>](#MethodType)
**Kind**: static method of [<code>MethodType</code>](#MethodType)  
<a name="MethodType.fromJSON"></a>

### MethodType.fromJSON(json) ⇒ [<code>MethodType</code>](#MethodType)
Deserializes a `MethodType` object from a JSON object.

**Kind**: static method of [<code>MethodType</code>](#MethodType)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Network"></a>

## Network
**Kind**: global class  

* [Network](#Network)
    * _instance_
        * [.name](#Network+name) ⇒ <code>string</code>
        * [.defaultNodeURL](#Network+defaultNodeURL) ⇒ <code>string</code> \| <code>undefined</code>
        * [.toString()](#Network+toString) ⇒ <code>string</code>
    * _static_
        * [.try_from_name(name)](#Network.try_from_name) ⇒ [<code>Network</code>](#Network)
        * [.mainnet()](#Network.mainnet) ⇒ [<code>Network</code>](#Network)
        * [.devnet()](#Network.devnet) ⇒ [<code>Network</code>](#Network)

<a name="Network+name"></a>

### network.name ⇒ <code>string</code>
**Kind**: instance property of [<code>Network</code>](#Network)  
<a name="Network+defaultNodeURL"></a>

### network.defaultNodeURL ⇒ <code>string</code> \| <code>undefined</code>
Returns the node URL of the Tangle network.

**Kind**: instance property of [<code>Network</code>](#Network)  
<a name="Network+toString"></a>

### network.toString() ⇒ <code>string</code>
**Kind**: instance method of [<code>Network</code>](#Network)  
<a name="Network.try_from_name"></a>

### Network.try\_from\_name(name) ⇒ [<code>Network</code>](#Network)
Parses the provided string to a `Network`.

**Kind**: static method of [<code>Network</code>](#Network)  

| Param | Type |
| --- | --- |
| name | <code>string</code> | 

<a name="Network.mainnet"></a>

### Network.mainnet() ⇒ [<code>Network</code>](#Network)
**Kind**: static method of [<code>Network</code>](#Network)  
<a name="Network.devnet"></a>

### Network.devnet() ⇒ [<code>Network</code>](#Network)
**Kind**: static method of [<code>Network</code>](#Network)  
<a name="Presentation"></a>

## Presentation
**Kind**: global class  

* [Presentation](#Presentation)
    * [new Presentation(holder_doc, credential_data, presentation_type, presentation_id)](#new_Presentation_new)
    * _instance_
        * [.toJSON()](#Presentation+toJSON) ⇒ <code>any</code>
        * [.verifiableCredential()](#Presentation+verifiableCredential) ⇒ [<code>Array.&lt;Credential&gt;</code>](#Credential)
    * _static_
        * [.fromJSON(json)](#Presentation.fromJSON) ⇒ [<code>Presentation</code>](#Presentation)

<a name="new_Presentation_new"></a>

### new Presentation(holder_doc, credential_data, presentation_type, presentation_id)

| Param | Type |
| --- | --- |
| holder_doc | [<code>Document</code>](#Document) | 
| credential_data | <code>any</code> | 
| presentation_type | <code>string</code> \| <code>undefined</code> | 
| presentation_id | <code>string</code> \| <code>undefined</code> | 

<a name="Presentation+toJSON"></a>

### presentation.toJSON() ⇒ <code>any</code>
Serializes a `Presentation` object as a JSON object.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation+verifiableCredential"></a>

### presentation.verifiableCredential() ⇒ [<code>Array.&lt;Credential&gt;</code>](#Credential)
Returns a copy of the credentials contained in the presentation.

**Kind**: instance method of [<code>Presentation</code>](#Presentation)  
<a name="Presentation.fromJSON"></a>

### Presentation.fromJSON(json) ⇒ [<code>Presentation</code>](#Presentation)
Deserializes a `Presentation` object from a JSON object.

**Kind**: static method of [<code>Presentation</code>](#Presentation)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="PresentationValidationOptions"></a>

## PresentationValidationOptions
Options to declare validation criteria when validating presentation.

**Kind**: global class  

* [PresentationValidationOptions](#PresentationValidationOptions)
    * [new PresentationValidationOptions(options)](#new_PresentationValidationOptions_new)
    * _instance_
        * [.toJSON()](#PresentationValidationOptions+toJSON) ⇒ <code>any</code>
    * _static_
        * [.default()](#PresentationValidationOptions.default) ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)
        * [.fromJSON(json)](#PresentationValidationOptions.fromJSON) ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)

<a name="new_PresentationValidationOptions_new"></a>

### new PresentationValidationOptions(options)
Creates a new `PresentationValidationOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>IPresentationValidationOptions</code> | 

<a name="PresentationValidationOptions+toJSON"></a>

### presentationValidationOptions.toJSON() ⇒ <code>any</code>
Serializes a `PresentationValidationOptions` as a JSON object.

**Kind**: instance method of [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)  
<a name="PresentationValidationOptions.default"></a>

### PresentationValidationOptions.default() ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)
Creates a new `PresentationValidationOptions` with defaults.

**Kind**: static method of [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)  
<a name="PresentationValidationOptions.fromJSON"></a>

### PresentationValidationOptions.fromJSON(json) ⇒ [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)
Deserializes a `PresentationValidationOptions` from a JSON object.

**Kind**: static method of [<code>PresentationValidationOptions</code>](#PresentationValidationOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="PresentationValidator"></a>

## PresentationValidator
**Kind**: global class  

* [PresentationValidator](#PresentationValidator)
    * [.validate(presentation, holder, issuers, options, fail_fast)](#PresentationValidator.validate)
    * [.verifyPresentationSignature(presentation, holder, options)](#PresentationValidator.verifyPresentationSignature)
    * [.checkStructure(presentation)](#PresentationValidator.checkStructure)

<a name="PresentationValidator.validate"></a>

### PresentationValidator.validate(presentation, holder, issuers, options, fail_fast)
Validate a `Presentation`.

The following properties are validated according to `options`:
- the semantic structure of the presentation,
- the holder's signature,
- the relationship between the holder and the credential subjects,
- the signatures and some properties of the constituent credentials (see
`CredentialValidator::validate`).

### Warning
The lack of an error returned from this method is in of itself not enough to conclude that the presentation can be
trusted. This section contains more information on additional checks that should be carried out before and after
calling this method.

#### The state of the supplied DID Documents.
The caller must ensure that the DID Documents in `holder` and `issuers` are up-to-date. The convenience methods
`Resolver::resolve_presentation_holder` and `Resolver::resolve_presentation_issuers`
can help extract the latest available states of these DID Documents.

#### Properties that are not validated
 There are many properties defined in [The Verifiable Credentials Data Model](https://www.w3.org/TR/vc-data-model/) that are **not** validated, such as:
`credentialStatus`, `type`, `credentialSchema`, `refreshService`, **and more**.
These should be manually checked after validation, according to your requirements.

### Errors
An error is returned whenever a validated condition is not satisfied.

**Kind**: static method of [<code>PresentationValidator</code>](#PresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| holder | [<code>Document</code>](#Document) \| [<code>ResolvedDocument</code>](#ResolvedDocument) | 
| issuers | [<code>Array.&lt;Document&gt;</code>](#Document) \| [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument) | 
| options | [<code>PresentationValidationOptions</code>](#PresentationValidationOptions) | 
| fail_fast | <code>number</code> | 

<a name="PresentationValidator.verifyPresentationSignature"></a>

### PresentationValidator.verifyPresentationSignature(presentation, holder, options)
Verify the presentation's signature using the resolved document of the holder.

### Warning
The caller must ensure that the DID Document of the holder is up-to-date.

### Errors
Fails if the `holder` does not match the `presentation`'s holder property.
Fails if signature verification against the holder document fails.

**Kind**: static method of [<code>PresentationValidator</code>](#PresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| holder | [<code>Document</code>](#Document) \| [<code>ResolvedDocument</code>](#ResolvedDocument) | 
| options | [<code>VerifierOptions</code>](#VerifierOptions) | 

<a name="PresentationValidator.checkStructure"></a>

### PresentationValidator.checkStructure(presentation)
Validates the semantic structure of the `Presentation`.

**Kind**: static method of [<code>PresentationValidator</code>](#PresentationValidator)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="ProofPurpose"></a>

## ProofPurpose
Associates a purpose with a `Signature`.

See https://w3c-ccg.github.io/security-vocab/#proofPurpose

**Kind**: global class  

* [ProofPurpose](#ProofPurpose)
    * _instance_
        * [.toJSON()](#ProofPurpose+toJSON) ⇒ <code>any</code>
    * _static_
        * [.assertionMethod()](#ProofPurpose.assertionMethod) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
        * [.authentication()](#ProofPurpose.authentication) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
        * [.fromJSON(json)](#ProofPurpose.fromJSON) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)

<a name="ProofPurpose+toJSON"></a>

### proofPurpose.toJSON() ⇒ <code>any</code>
Serializes a `ProofPurpose` object as a JSON object.

**Kind**: instance method of [<code>ProofPurpose</code>](#ProofPurpose)  
<a name="ProofPurpose.assertionMethod"></a>

### ProofPurpose.assertionMethod() ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
Purpose is to assert a claim.
See https://www.w3.org/TR/did-core/#assertion

**Kind**: static method of [<code>ProofPurpose</code>](#ProofPurpose)  
<a name="ProofPurpose.authentication"></a>

### ProofPurpose.authentication() ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
Purpose is to authenticate the signer.
See https://www.w3.org/TR/did-core/#authentication

**Kind**: static method of [<code>ProofPurpose</code>](#ProofPurpose)  
<a name="ProofPurpose.fromJSON"></a>

### ProofPurpose.fromJSON(json) ⇒ [<code>ProofPurpose</code>](#ProofPurpose)
Deserializes a `ProofPurpose` object from a JSON object.

**Kind**: static method of [<code>ProofPurpose</code>](#ProofPurpose)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Receipt"></a>

## Receipt
**Kind**: global class  

* [Receipt](#Receipt)
    * _instance_
        * [.network](#Receipt+network) ⇒ [<code>Network</code>](#Network)
        * [.messageId](#Receipt+messageId) ⇒ <code>string</code>
        * [.networkId](#Receipt+networkId) ⇒ <code>string</code>
        * [.nonce](#Receipt+nonce) ⇒ <code>string</code>
        * [.toJSON()](#Receipt+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#Receipt.fromJSON) ⇒ [<code>Receipt</code>](#Receipt)

<a name="Receipt+network"></a>

### receipt.network ⇒ [<code>Network</code>](#Network)
Returns the associated IOTA Tangle `Network`.

**Kind**: instance property of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+messageId"></a>

### receipt.messageId ⇒ <code>string</code>
Returns the message `id`.

**Kind**: instance property of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+networkId"></a>

### receipt.networkId ⇒ <code>string</code>
Returns the message `network_id`.

**Kind**: instance property of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+nonce"></a>

### receipt.nonce ⇒ <code>string</code>
Returns the message `nonce`.

**Kind**: instance property of [<code>Receipt</code>](#Receipt)  
<a name="Receipt+toJSON"></a>

### receipt.toJSON() ⇒ <code>any</code>
Serializes a `Receipt` as a JSON object.

**Kind**: instance method of [<code>Receipt</code>](#Receipt)  
<a name="Receipt.fromJSON"></a>

### Receipt.fromJSON(json) ⇒ [<code>Receipt</code>](#Receipt)
Deserializes a `Receipt` from a JSON object.

**Kind**: static method of [<code>Receipt</code>](#Receipt)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="ResolvedDocument"></a>

## ResolvedDocument
An IOTA DID document resolved from the Tangle. Represents an integration chain message possibly
merged with one or more `DiffMessages`.

**Kind**: global class  

* [ResolvedDocument](#ResolvedDocument)
    * _instance_
        * [.document](#ResolvedDocument+document) ⇒ [<code>Document</code>](#Document)
        * [.diffMessageId](#ResolvedDocument+diffMessageId) ⇒ <code>string</code>
        * [.diffMessageId](#ResolvedDocument+diffMessageId)
        * [.integrationMessageId](#ResolvedDocument+integrationMessageId) ⇒ <code>string</code>
        * [.integrationMessageId](#ResolvedDocument+integrationMessageId)
        * [.mergeDiffMessage(diff_message)](#ResolvedDocument+mergeDiffMessage)
        * [.intoDocument()](#ResolvedDocument+intoDocument) ⇒ [<code>Document</code>](#Document)
        * [.toJSON()](#ResolvedDocument+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(json)](#ResolvedDocument.fromJSON) ⇒ [<code>ResolvedDocument</code>](#ResolvedDocument)

<a name="ResolvedDocument+document"></a>

### resolvedDocument.document ⇒ [<code>Document</code>](#Document)
Returns the inner DID document.

NOTE: clones the data. Use `intoDocument()` for efficiency.

**Kind**: instance property of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+diffMessageId"></a>

### resolvedDocument.diffMessageId ⇒ <code>string</code>
Returns the diff chain message id.

**Kind**: instance property of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+diffMessageId"></a>

### resolvedDocument.diffMessageId
Sets the diff chain message id.

**Kind**: instance property of [<code>ResolvedDocument</code>](#ResolvedDocument)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="ResolvedDocument+integrationMessageId"></a>

### resolvedDocument.integrationMessageId ⇒ <code>string</code>
Returns the integration chain message id.

**Kind**: instance property of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+integrationMessageId"></a>

### resolvedDocument.integrationMessageId
Sets the integration chain message id.

**Kind**: instance property of [<code>ResolvedDocument</code>](#ResolvedDocument)  

| Param | Type |
| --- | --- |
| value | <code>string</code> | 

<a name="ResolvedDocument+mergeDiffMessage"></a>

### resolvedDocument.mergeDiffMessage(diff_message)
Attempts to merge changes from a `DiffMessage` into this document and
updates the `ResolvedDocument::diffMessageId`.

If merging fails the document remains unmodified, otherwise this represents
the merged document state.

See `Document::mergeDiff`.

# Errors

Fails if the merge operation or signature verification on the diff fails.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  

| Param | Type |
| --- | --- |
| diff_message | [<code>DiffMessage</code>](#DiffMessage) | 

<a name="ResolvedDocument+intoDocument"></a>

### resolvedDocument.intoDocument() ⇒ [<code>Document</code>](#Document)
Consumes this object and returns the inner DID document.

NOTE: trying to use the `ResolvedDocument` after calling this will throw an error.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument+toJSON"></a>

### resolvedDocument.toJSON() ⇒ <code>any</code>
Serializes a `Document` object as a JSON object.

**Kind**: instance method of [<code>ResolvedDocument</code>](#ResolvedDocument)  
<a name="ResolvedDocument.fromJSON"></a>

### ResolvedDocument.fromJSON(json) ⇒ [<code>ResolvedDocument</code>](#ResolvedDocument)
Deserializes a `Document` object from a JSON object.

**Kind**: static method of [<code>ResolvedDocument</code>](#ResolvedDocument)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="Resolver"></a>

## Resolver
**Kind**: global class  

* [Resolver](#Resolver)
    * [new Resolver()](#new_Resolver_new)
    * [.getClient(network_name)](#Resolver+getClient) ⇒ [<code>Client</code>](#Client) \| <code>undefined</code>
    * [.resolve(did)](#Resolver+resolve) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
    * [.resolveHistory(did)](#Resolver+resolveHistory) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
    * [.resolveDiffHistory(document)](#Resolver+resolveDiffHistory) ⇒ [<code>Promise.&lt;DiffChainHistory&gt;</code>](#DiffChainHistory)
    * [.resolveCredentialIssuer(credential)](#Resolver+resolveCredentialIssuer) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
    * [.resolvePresentationIssuers(presentation)](#Resolver+resolvePresentationIssuers) ⇒ <code>Promise.&lt;Array.&lt;ResolvedDocument&gt;&gt;</code>
    * [.resolvePresentationHolder(presentation)](#Resolver+resolvePresentationHolder) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
    * [.verifyPresentation(presentation, options, fail_fast, holder, issuers)](#Resolver+verifyPresentation) ⇒ <code>Promise.&lt;void&gt;</code>

<a name="new_Resolver_new"></a>

### new Resolver()
Constructs a new `Resolver` with a default `Client` for
the `Mainnet`.

<a name="Resolver+getClient"></a>

### resolver.getClient(network_name) ⇒ [<code>Client</code>](#Client) \| <code>undefined</code>
Returns the `Client` corresponding to the given network name if one exists.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| network_name | <code>string</code> | 

<a name="Resolver+resolve"></a>

### resolver.resolve(did) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetches the `Document` of the given `DID`.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

<a name="Resolver+resolveHistory"></a>

### resolver.resolveHistory(did) ⇒ [<code>Promise.&lt;DocumentHistory&gt;</code>](#DocumentHistory)
Fetches the `DocumentHistory` of the given `DID`.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) \| <code>string</code> | 

<a name="Resolver+resolveDiffHistory"></a>

### resolver.resolveDiffHistory(document) ⇒ [<code>Promise.&lt;DiffChainHistory&gt;</code>](#DiffChainHistory)
Returns the `DiffChainHistory` of a diff chain starting from a `Document` on the
integration chain.

NOTE: the document must have been published to the Tangle and have a valid message id.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| document | [<code>ResolvedDocument</code>](#ResolvedDocument) | 

<a name="Resolver+resolveCredentialIssuer"></a>

### resolver.resolveCredentialIssuer(credential) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetches the DID Document of the issuer on a `Credential`.

### Errors

Errors if the issuer URL is not a valid `DID` or document resolution fails.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| credential | [<code>Credential</code>](#Credential) | 

<a name="Resolver+resolvePresentationIssuers"></a>

### resolver.resolvePresentationIssuers(presentation) ⇒ <code>Promise.&lt;Array.&lt;ResolvedDocument&gt;&gt;</code>
Fetches all DID Documents of `Credential` issuers contained in a `Presentation`.
Issuer documents are returned in arbitrary order.

### Errors

Errors if any issuer URL is not a valid `DID` or document resolution fails.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="Resolver+resolvePresentationHolder"></a>

### resolver.resolvePresentationHolder(presentation) ⇒ [<code>Promise.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument)
Fetches the DID Document of the holder of a `Presentation`.

### Errors

Errors if the holder URL is missing, is not a valid `DID`, or document resolution fails.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 

<a name="Resolver+verifyPresentation"></a>

### resolver.verifyPresentation(presentation, options, fail_fast, holder, issuers) ⇒ <code>Promise.&lt;void&gt;</code>
Verifies a `Presentation`.

### Important
See `PresentationValidator::validate` for information about which properties get
validated and what is expected of the optional arguments `holder` and `issuer`.

### Resolution
The DID Documents for the `holder` and `issuers` are optionally resolved if not given.
If you already have up-to-date versions of these DID Documents, you may want
to use `PresentationValidator::validate`.
See also `Resolver::resolvePresentationIssuers` and `Resolver::resolvePresentationHolder`.

### Errors
Errors from resolving the holder and issuer DID Documents, if not provided, will be returned immediately.
Otherwise, errors from validating the presentation and its credentials will be returned
according to the `fail_fast` parameter.

**Kind**: instance method of [<code>Resolver</code>](#Resolver)  

| Param | Type |
| --- | --- |
| presentation | [<code>Presentation</code>](#Presentation) | 
| options | [<code>PresentationValidationOptions</code>](#PresentationValidationOptions) | 
| fail_fast | <code>number</code> | 
| holder | [<code>ResolvedDocument</code>](#ResolvedDocument) \| <code>undefined</code> | 
| issuers | [<code>Array.&lt;ResolvedDocument&gt;</code>](#ResolvedDocument) \| <code>undefined</code> | 

<a name="ResolverBuilder"></a>

## ResolverBuilder
Builder for configuring [`Clients`][Client] when constructing a [`Resolver`].

**Kind**: global class  

* [ResolverBuilder](#ResolverBuilder)
    * [new ResolverBuilder()](#new_ResolverBuilder_new)
    * [.client(client)](#ResolverBuilder+client) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
    * [.clientConfig(config)](#ResolverBuilder+clientConfig) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
    * [.build()](#ResolverBuilder+build) ⇒ [<code>Promise.&lt;Resolver&gt;</code>](#Resolver)

<a name="new_ResolverBuilder_new"></a>

### new ResolverBuilder()
Constructs a new `ResolverBuilder` with no `Clients` configured.

<a name="ResolverBuilder+client"></a>

### resolverBuilder.client(client) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
Inserts a `Client`.

NOTE: replaces any previous `Client` or `Config` with the same network name.

**Kind**: instance method of [<code>ResolverBuilder</code>](#ResolverBuilder)  

| Param | Type |
| --- | --- |
| client | [<code>Client</code>](#Client) | 

<a name="ResolverBuilder+clientConfig"></a>

### resolverBuilder.clientConfig(config) ⇒ [<code>ResolverBuilder</code>](#ResolverBuilder)
Inserts a `Config` used to create a `Client`.

NOTE: replaces any previous `Client` or `Config` with the same network name.

**Kind**: instance method of [<code>ResolverBuilder</code>](#ResolverBuilder)  

| Param | Type |
| --- | --- |
| config | [<code>Config</code>](#Config) | 

<a name="ResolverBuilder+build"></a>

### resolverBuilder.build() ⇒ [<code>Promise.&lt;Resolver&gt;</code>](#Resolver)
Constructs a new [`Resolver`] based on the builder configuration.

**Kind**: instance method of [<code>ResolverBuilder</code>](#ResolverBuilder)  
<a name="Service"></a>

## Service
A DID Document Service used to enable trusted interactions associated
with a DID subject.

See: https://www.w3.org/TR/did-core/#services

**Kind**: global class  

* [Service](#Service)
    * [new Service(service)](#new_Service_new)
    * _instance_
        * [.id](#Service+id) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.type](#Service+type) ⇒ <code>string</code>
        * [.serviceEndpoint](#Service+serviceEndpoint) ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
        * [.properties()](#Service+properties) ⇒ <code>Map.&lt;string, any&gt;</code>
        * [.toJSON()](#Service+toJSON) ⇒ <code>any</code>
    * _static_
        * [.fromJSON(value)](#Service.fromJSON) ⇒ [<code>Service</code>](#Service)

<a name="new_Service_new"></a>

### new Service(service)

| Param | Type |
| --- | --- |
| service | <code>IService</code> | 

<a name="Service+id"></a>

### service.id ⇒ [<code>DIDUrl</code>](#DIDUrl)
Returns a copy of the `Service` id.

**Kind**: instance property of [<code>Service</code>](#Service)  
<a name="Service+type"></a>

### service.type ⇒ <code>string</code>
Returns a copy of the `Service` type.

**Kind**: instance property of [<code>Service</code>](#Service)  
<a name="Service+serviceEndpoint"></a>

### service.serviceEndpoint ⇒ <code>string</code> \| <code>Array.&lt;string&gt;</code> \| <code>Map.&lt;string, Array.&lt;string&gt;&gt;</code>
Returns a copy of the `Service` endpoint.

**Kind**: instance property of [<code>Service</code>](#Service)  
<a name="Service+properties"></a>

### service.properties() ⇒ <code>Map.&lt;string, any&gt;</code>
Returns a copy of the custom properties on the `Service`.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service+toJSON"></a>

### service.toJSON() ⇒ <code>any</code>
Serializes a `Service` object as a JSON object.

**Kind**: instance method of [<code>Service</code>](#Service)  
<a name="Service.fromJSON"></a>

### Service.fromJSON(value) ⇒ [<code>Service</code>](#Service)
Deserializes a `Service` object from a JSON object.

**Kind**: static method of [<code>Service</code>](#Service)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="SignatureOptions"></a>

## SignatureOptions
Holds additional options for creating signatures.
See `ISignatureOptions`.

**Kind**: global class  

* [SignatureOptions](#SignatureOptions)
    * [new SignatureOptions(options)](#new_SignatureOptions_new)
    * [.default()](#SignatureOptions.default) ⇒ [<code>SignatureOptions</code>](#SignatureOptions)

<a name="new_SignatureOptions_new"></a>

### new SignatureOptions(options)
Creates a new `SignatureOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>ISignatureOptions</code> | 

<a name="SignatureOptions.default"></a>

### SignatureOptions.default() ⇒ [<code>SignatureOptions</code>](#SignatureOptions)
Creates a new `SignatureOptions` with default options.

**Kind**: static method of [<code>SignatureOptions</code>](#SignatureOptions)  
<a name="Timestamp"></a>

## Timestamp
**Kind**: global class  

* [Timestamp](#Timestamp)
    * _instance_
        * [.toRFC3339()](#Timestamp+toRFC3339) ⇒ <code>string</code>
        * [.checkedAdd(duration)](#Timestamp+checkedAdd) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.checkedSub(duration)](#Timestamp+checkedSub) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
        * [.toJSON()](#Timestamp+toJSON) ⇒ <code>any</code>
    * _static_
        * [.parse(input)](#Timestamp.parse) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.nowUTC()](#Timestamp.nowUTC) ⇒ [<code>Timestamp</code>](#Timestamp)
        * [.fromJSON(json)](#Timestamp.fromJSON) ⇒ [<code>Timestamp</code>](#Timestamp)

<a name="Timestamp+toRFC3339"></a>

### timestamp.toRFC3339() ⇒ <code>string</code>
Returns the `Timestamp` as an RFC 3339 `String`.

**Kind**: instance method of [<code>Timestamp</code>](#Timestamp)  
<a name="Timestamp+checkedAdd"></a>

### timestamp.checkedAdd(duration) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Computes `self + duration`

Returns `null` if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).

**Kind**: instance method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| duration | [<code>Duration</code>](#Duration) | 

<a name="Timestamp+checkedSub"></a>

### timestamp.checkedSub(duration) ⇒ [<code>Timestamp</code>](#Timestamp) \| <code>undefined</code>
Computes `self - duration`

Returns `null` if the operation leads to a timestamp not in the valid range for [RFC 3339](https://tools.ietf.org/html/rfc3339).

**Kind**: instance method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| duration | [<code>Duration</code>](#Duration) | 

<a name="Timestamp+toJSON"></a>

### timestamp.toJSON() ⇒ <code>any</code>
Serializes a `Timestamp` as a JSON object.

**Kind**: instance method of [<code>Timestamp</code>](#Timestamp)  
<a name="Timestamp.parse"></a>

### Timestamp.parse(input) ⇒ [<code>Timestamp</code>](#Timestamp)
Parses a `Timestamp` from the provided input string.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| input | <code>string</code> | 

<a name="Timestamp.nowUTC"></a>

### Timestamp.nowUTC() ⇒ [<code>Timestamp</code>](#Timestamp)
Creates a new `Timestamp` with the current date and time.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  
<a name="Timestamp.fromJSON"></a>

### Timestamp.fromJSON(json) ⇒ [<code>Timestamp</code>](#Timestamp)
Deserializes a `Timestamp` from a JSON object.

**Kind**: static method of [<code>Timestamp</code>](#Timestamp)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="VerificationMethod"></a>

## VerificationMethod
**Kind**: global class  

* [VerificationMethod](#VerificationMethod)
    * [new VerificationMethod(did, key_type, public_key, fragment)](#new_VerificationMethod_new)
    * _instance_
        * [.id](#VerificationMethod+id) ⇒ [<code>DIDUrl</code>](#DIDUrl)
        * [.controller](#VerificationMethod+controller) ⇒ [<code>DID</code>](#DID)
        * [.controller](#VerificationMethod+controller)
        * [.type](#VerificationMethod+type) ⇒ <code>string</code>
        * [.data](#VerificationMethod+data) ⇒ <code>any</code>
        * [.toJSON()](#VerificationMethod+toJSON) ⇒ <code>any</code>
    * _static_
        * [.newMerkleKey(digest, did, keys, fragment)](#VerificationMethod.newMerkleKey) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
        * [.fromJSON(value)](#VerificationMethod.fromJSON) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)

<a name="new_VerificationMethod_new"></a>

### new VerificationMethod(did, key_type, public_key, fragment)
Creates a new `VerificationMethod` object from the given `did` and
Base58-BTC encoded public key.


| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 
| key_type | <code>number</code> | 
| public_key | <code>string</code> | 
| fragment | <code>string</code> | 

<a name="VerificationMethod+id"></a>

### verificationMethod.id ⇒ [<code>DIDUrl</code>](#DIDUrl)
Returns the `id` `DIDUrl` of the `VerificationMethod` object.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+controller"></a>

### verificationMethod.controller ⇒ [<code>DID</code>](#DID)
Returns the `controller` `DID` of the `VerificationMethod` object.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+controller"></a>

### verificationMethod.controller
Returns the `controller` `DID` of the `VerificationMethod` object.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| did | [<code>DID</code>](#DID) | 

<a name="VerificationMethod+type"></a>

### verificationMethod.type ⇒ <code>string</code>
Returns the `VerificationMethod` type.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+data"></a>

### verificationMethod.data ⇒ <code>any</code>
Returns the `VerificationMethod` public key data.

**Kind**: instance property of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod+toJSON"></a>

### verificationMethod.toJSON() ⇒ <code>any</code>
Serializes a `VerificationMethod` object as a JSON object.

**Kind**: instance method of [<code>VerificationMethod</code>](#VerificationMethod)  
<a name="VerificationMethod.newMerkleKey"></a>

### VerificationMethod.newMerkleKey(digest, did, keys, fragment) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Creates a new `MerkleKeyCollection2021` method from the given key collection.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| digest | <code>number</code> | 
| did | [<code>DID</code>](#DID) | 
| keys | [<code>KeyCollection</code>](#KeyCollection) | 
| fragment | <code>string</code> | 

<a name="VerificationMethod.fromJSON"></a>

### VerificationMethod.fromJSON(value) ⇒ [<code>VerificationMethod</code>](#VerificationMethod)
Deserializes a `VerificationMethod` object from a JSON object.

**Kind**: static method of [<code>VerificationMethod</code>](#VerificationMethod)  

| Param | Type |
| --- | --- |
| value | <code>any</code> | 

<a name="VerifierOptions"></a>

## VerifierOptions
Holds additional signature verification options.
See `IVerifierOptions`.

**Kind**: global class  

* [VerifierOptions](#VerifierOptions)
    * [new VerifierOptions(options)](#new_VerifierOptions_new)
    * _instance_
        * [.toJSON()](#VerifierOptions+toJSON) ⇒ <code>any</code>
    * _static_
        * [.default()](#VerifierOptions.default) ⇒ [<code>VerifierOptions</code>](#VerifierOptions)
        * [.fromJSON(json)](#VerifierOptions.fromJSON) ⇒ [<code>VerifierOptions</code>](#VerifierOptions)

<a name="new_VerifierOptions_new"></a>

### new VerifierOptions(options)
Creates a new `VerifierOptions` from the given fields.

Throws an error if any of the options are invalid.


| Param | Type |
| --- | --- |
| options | <code>IVerifierOptions</code> | 

<a name="VerifierOptions+toJSON"></a>

### verifierOptions.toJSON() ⇒ <code>any</code>
Serializes a `VerifierOptions` as a JSON object.

**Kind**: instance method of [<code>VerifierOptions</code>](#VerifierOptions)  
<a name="VerifierOptions.default"></a>

### VerifierOptions.default() ⇒ [<code>VerifierOptions</code>](#VerifierOptions)
Creates a new `VerifierOptions` with default options.

**Kind**: static method of [<code>VerifierOptions</code>](#VerifierOptions)  
<a name="VerifierOptions.fromJSON"></a>

### VerifierOptions.fromJSON(json) ⇒ [<code>VerifierOptions</code>](#VerifierOptions)
Deserializes a `VerifierOptions` from a JSON object.

**Kind**: static method of [<code>VerifierOptions</code>](#VerifierOptions)  

| Param | Type |
| --- | --- |
| json | <code>any</code> | 

<a name="MethodRelationship"></a>

## MethodRelationship
**Kind**: global variable  
<a name="KeyType"></a>

## KeyType
**Kind**: global variable  
<a name="DIDMessageEncoding"></a>

## DIDMessageEncoding
**Kind**: global variable  
<a name="Digest"></a>

## Digest
**Kind**: global variable  
<a name="SubjectHolderRelationship"></a>

## SubjectHolderRelationship
Declares how credential subjects must relate to the presentation holder during validation.
See `PresentationValidationOptions::subject_holder_relationship`.

See also the [Subject-Holder Relationship](https://www.w3.org/TR/vc-data-model/#subject-holder-relationships) section of the specification.

**Kind**: global variable  
<a name="AlwaysSubject"></a>

## AlwaysSubject
The holder must always match the subject on all credentials, regardless of their [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property.
This variant is the default used if no other variant is specified when constructing a new
`PresentationValidationOptions`.

**Kind**: global variable  
<a name="SubjectOnNonTransferable"></a>

## SubjectOnNonTransferable
The holder must match the subject only for credentials where the [`nonTransferable`](https://www.w3.org/TR/vc-data-model/#nontransferable-property) property is `true`.

**Kind**: global variable  
<a name="Any"></a>

## Any
The holder is not required to have any kind of relationship to any credential subject.

**Kind**: global variable  
<a name="FailFast"></a>

## FailFast
Declares when validation should return if an error occurs.

**Kind**: global variable  
<a name="AllErrors"></a>

## AllErrors
Return all errors that occur during validation.

**Kind**: global variable  
<a name="FirstError"></a>

## FirstError
Return after the first error occurs.

**Kind**: global variable  
<a name="start"></a>

## start()
Initializes the console error panic hook for better error messages

**Kind**: global function  
