# Request and Response Model

## Request and Template

Requests are allowed in two ways: original request body and template request body.

### Original Request Body

Origin Request Body has the following fields:

- RequestId: A id to identify a request.
- ProverId: Which prover will handle this request.
- Remote: The remote address, like: `domain:port` or `ip:port`.
- ServerName: The server name, this field will validate by SNI.
- EncryptedKey: The encrypted key. Some part of request can be encrypted by the key.
- MaxResponseSize: The maximum response size, the response can't exceed this size.
- requestTemplateHash: Must be `[0u8; 32]`.

After above fields, also have some repeated segment of request body:

- RequestId: A id to identify a request. 
- data: The data of request segment.
- isEncrypted: Whether this part of data is encrypted.

#### Request Hash

Request Hash is used to ensure a 1to1 mapping between requests and responses. The Guest program ensures that the Request Hash is calculated from the Request,
and also ensures that the Response is the result obtained after inputting the Request into the TLS protocol stack.

For Original Request Body, the Request Hash is calculated using the following rules:

```solidity
function computeOriginalRequestHash(
    string memory remote,
    string memory serverName,
    bytes memory encryptedKey,
    bytes[] memory data
) public pure returns (bytes32) {
    bytes32 request_hash = keccak256(
        abi.encode(remote, serverName, encryptedKey, data)
    );

    return request_hash;
}
```

### Template Request Body

Template Request Body has the following fields:

- RequestId: A id to identify a request.
- ProverId: Which prover will handle this request.
- Remote: The remote address, like: `domain:port` or `ip:port`.
- ServerName: The server name, this field will validate by SNI.
- EncryptedKey: The encrypted key. Some part of request can be encrypted by the key.
- MaxResponseSize: The maximum response size, the response can't exceed this size.
- requestTemplateHash: Which template will be used to handle this request.

After above fields, also have some repeated segment of request body:

- RequestId: A id to identify a request. 
- field: The field of request segment.
- value: The value of request segment.
- isEncrypted: Whether this part of data is encrypted.

#### Request Hash

For Template Request Body, the Request Hash is calculated using the following rules:

```solidity
function computeTemplateRequestHash(
    string memory remote,
    string memory serverName,
    bytes memory encryptedKey,
    bytes32 requestTemplateHash,
    uint64[] memory fieldOffsets,
    bytes[] memory fieldValues
) public pure returns (bytes32) {
    bytes32 request_hash = keccak256(
        abi.encode(
            remote,
            serverName,
            encryptedKey,
            requestTemplateHash,
            fieldOffsets,
            fieldValues
        )
    );

    return request_hash;
}
```
