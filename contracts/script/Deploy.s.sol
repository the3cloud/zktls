// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {ZkTLSGateway} from "../src/ZkTLSGateway.sol";

contract DeployScript is Script {
    ZkTLSGateway public zkTLSGateway;

    function run() public {
        vm.startBroadcast();

        zkTLSGateway = new ZkTLSGateway();

        console.log("ZkTLSGateway deployed at", address(zkTLSGateway));

        vm.stopBroadcast();
    }
}
