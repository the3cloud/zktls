// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./IZkTLSGateway.sol";

contract ZkTLSGateway is IZkTLSGateway {
    uint256 public nonce;

    function requestTLSCallTemplate(
        bytes32 requestTemplateHash,
        bytes32 responseTemplateHash,
        string calldata remote,
        string calldata serverName,
        bytes calldata encrypted_key,
        uint64[] calldata fields,
        bytes[] calldata values
    ) public {
        require(
            fields.length == values.length,
            "Fields and values must have the same length"
        );

        bytes32 requestId = keccak256(abi.encode(msg.sender, nonce++));

        emit RequestTLSCallBegin(
            requestId,
            0x0,
            requestTemplateHash,
            responseTemplateHash,
            remote,
            serverName,
            encrypted_key,
            500000
        );

        for (uint256 i = 0; i < fields.length; i++) {
            emit RequestTLSCallTemplateField(
                requestId,
                fields[i],
                values[i],
                i % 2 == 0
            );
        }
    }

    function deliveryResponse(
        bytes32 requestId,
        bytes32 requestHash,
        bytes calldata responseData,
        bytes calldata proof
    ) public {}
}
