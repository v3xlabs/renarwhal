use alloy_sol_types::sol;

// 1.ultrabulk.eth -- 0x7Ff29Bd08AF26495EeB96cb5D80f1813C0410917
sol! {
    #[sol(rpc)]
    contract UltraBulkContract {
        #[derive(Debug)]
        function renewAll(
            string[] calldata names,
            uint256 duration,
            uint256 price
        ) external payable {}

        #[derive(Debug)]
        function multiCommit(
            bytes32[] calldata commitments
        ) external payable {}

        #[derive(Debug)]
        function multiRegisterWithAddress(
            string[] calldata names,
            address[] calldata owners,
            uint256 duration,
            bytes32 secret,
            address resolver,
            address addr
        ) external payable {}

        #[derive(Debug)]
        function multiRegister(
            string[] calldata names,
            address[] calldata owners,
            uint256 duration,
            bytes32 secret,
            address resolver
        ) external payable {}
    }
}
