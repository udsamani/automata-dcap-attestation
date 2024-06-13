// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import "../contracts/PCCSRouter.sol";

contract DeployRouter is Script {
    uint256 deployerKey = uint256(vm.envBytes32("PRIVATE_KEY"));
    address enclaveIdDaoAddr = vm.envAddress("ENCLAVE_IDENTITY_DAO_PORTAL");
    address enclaveIdHelperAddr = vm.envAddress("ENCLAVE_IDENTITY_HELPER");
    address pckHelperAddr = vm.envAddress("X509_HELPER");
    address tcbDaoAddr = vm.envAddress("FMSPC_TCB_DAO_PORTAL");
    address tcbHelperAddr = vm.envAddress("FMSPC_TCB_HELPER");
    address crlHelperAddr = vm.envAddress("X509_CRL_HELPER");
    address pcsDaoAddr = vm.envAddress("PCS_DAO_PORTAL");

    function run() public {
        vm.startBroadcast(deployerKey);

        PCCSRouter router = new PCCSRouter(
            enclaveIdDaoAddr,
            tcbDaoAddr,
            pcsDaoAddr,
            pckHelperAddr,
            crlHelperAddr
        );
        console2.log("Deployed PCCSRouter to", address(router));

        vm.stopBroadcast();
    }
}