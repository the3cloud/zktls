// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IZkTLSGateway {
    /// @notice Event emitted when a TLS call begins.
    /// @param requestId The ID of the request.
    /// @param prover The address of the prover.
    /// @param requestTemplateHash The hash of the request template. If this field is 0x0, it means the request template is not specified.
    /// @param remote The remote address.
    /// @param serverName The server name.
    /// @param encryptedKey The encrypted key.
    /// @param maxResponseSize The maximum response size.
    event RequestTLSCallBegin(
        bytes32 indexed requestId,
        bytes32 indexed prover,
        bytes32 requestTemplateHash,
        bytes32 responseTemplateHash,
        string remote,
        string serverName,
        bytes encryptedKey,
        uint256 maxResponseSize
    );

    /// @notice Event emitted when a TLS call template field is received.
    /// @param requestId The ID of the request.
    /// @param field The field name.
    /// @param value The field value.
    /// @param isEncrypted Whether the value is encrypted.
    event RequestTLSCallTemplateField(bytes32 indexed requestId, uint64 indexed field, bytes value, bool isEncrypted);

    /// @notice Function to deliver a TLS call response.
    /// @param requestId The ID of the request.
    /// @param requestHash The hash of the request.
    /// @param responseData The response data.
    function deliveryResponse(
        bytes32 requestId,
        bytes32 requestHash,
        bytes calldata responseData,
        bytes calldata proof
    ) external;
}
