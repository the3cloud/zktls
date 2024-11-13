// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

library RequestHashLib {
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

    function computeTemplateRequestHash(
        string memory remote,
        string memory serverName,
        bytes memory encryptedKey,
        bytes32 requestTemplateHash,
        uint64[] memory fields,
        bytes[] memory values
    ) public pure returns (bytes32) {
        bytes32 request_hash = keccak256(
            abi.encode(
                remote,
                serverName,
                encryptedKey,
                requestTemplateHash,
                fields,
                values
            )
        );

        return request_hash;
    }
}
