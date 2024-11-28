// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IZkTLSGateway {
    event RequestTLSCallBegin(
        bytes32 indexed requestId,
        bytes32 indexed prover,
        bytes requestData,
        bytes responseTemplateData,
        bytes encryptedKey,
        uint256 maxResponseBytes
    );

    function deliveryResponse(bytes32 requestId, bytes32 requestHash, bytes calldata response, bytes calldata proofs)
        external;
}
