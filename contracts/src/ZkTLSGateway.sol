// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract ZkTLSGateway {
    event RequestTLSCallBegin(string url);

    event RequestTLSCallSegment(bytes data, bytes encrypted_key);
}
