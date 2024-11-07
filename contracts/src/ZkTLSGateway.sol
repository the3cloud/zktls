// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "./IZkTLSGateway.sol";

contract ZkTLSGateway is IZkTLSGateway {
    uint256 public nonce;

    function requestTLSCall(
        string calldata remote,
        string calldata serverName,
        bytes calldata encrypted_key,
        bytes[] calldata data
    ) public {
        bytes32 requestId = keccak256(abi.encode(msg.sender, nonce++));

        emit RequestTLSCallBegin(
            requestId,
            0x0,
            0x0,
            0x0,
            remote,
            serverName,
            encrypted_key,
            500000
        );

        for (uint256 i = 0; i < data.length; i++) {
            bool is_encrypted = i % 2 == 0;
            emit RequestTLSCallSegment(requestId, data[i], !is_encrypted);
        }
    }

    function requestTLSCallTemplate(
        bytes32 requestTemplateHash,
        bytes32 responseTemplateHash,
        string calldata remote,
        string calldata serverName,
        bytes calldata encrypted_key,
        bytes32[] calldata fields,
        bytes[] calldata values
    ) public {
        require(
            fields.length == values.length,
            "Fields and values must have the same length"
        );

        bytes32 requestId = keccak256(abi.encode(msg.sender, nonce++));

        emit RequestTLSCallBegin(
            requestId,
            requestTemplateHash,
            0x0,
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
        bytes calldata responseData
    ) public {}
}
