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

- RequestId:A id to identify a request. 
- data: The data of request segment.
- isEncrypted: Whether this part of data is encrypted.

### Template Request Body
