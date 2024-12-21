use alloy::sol;

sol! {
    #[sol(rpc)]
    interface ZkTLSGateway {
        function deliverResponse(
            bytes calldata proof_,
            bytes32 proverId_,
            bytes32 responseId_,
            address client_,
            bytes32 dapp_,
            uint64 maxGasPrice_,
            uint64 gasLimit_,
            bytes calldata responses_
        ) external;

        function registerProver(bytes32 proverId_, address verifier_, address submitter_, address beneficiary_) external;
    }
}

sol! {
    #[sol(rpc)]
    #[sol(bytecode = "0x60c060405234801561001057600080fd5b5060405161057c38038061057c83398101604081905261002f91610045565b6001600160a01b0390911660805260a05261007f565b6000806040838503121561005857600080fd5b82516001600160a01b038116811461006f57600080fd5b6020939093015192949293505050565b60805160a0516104cc6100b0600039600081816056015261021c015260008181609001526101ef01526104cc6000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80631ddb5d291461005157806352a07fa31461008b578063b4b0686a146100ca578063b8e72af6146100eb575b600080fd5b6100787f000000000000000000000000000000000000000000000000000000000000000081565b6040519081526020015b60405180910390f35b6100b27f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610082565b6100dd6100d8366004610282565b610100565b6040516100829291906102ae565b6100fe6100f9366004610382565b6101d8565b005b6040805160018082528183019092526060918291906020808301908036833701905050915060008260008151811061013a5761013a6103f3565b6001600160a01b0392909216602092830291909101820152604080516001808252818301909252918281019080368337019050509050833a11156101a6576101828486610409565b81600081518110610195576101956103f3565b6020026020010181815250506101d0565b6101b03a86610409565b816000815181106101c3576101c36103f3565b6020026020010181815250505b935093915050565b60405163020a49e360e51b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906341493c609061024c907f000000000000000000000000000000000000000000000000000000000000000090889088908890889060040161045d565b60006040518083038186803b15801561026457600080fd5b505afa158015610278573d6000803e3d6000fd5b5050505050505050565b60008060006060848603121561029757600080fd5b505081359360208301359350604090920135919050565b6040808252835190820181905260009060208501906060840190835b818110156102f15783516001600160a01b03168352602093840193909201916001016102ca565b50508381036020808601919091528551808352918101925085019060005b8181101561032d57825184526020938401939092019160010161030f565b50919695505050505050565b60008083601f84011261034b57600080fd5b50813567ffffffffffffffff81111561036357600080fd5b60208301915083602082850101111561037b57600080fd5b9250929050565b6000806000806040858703121561039857600080fd5b843567ffffffffffffffff8111156103af57600080fd5b6103bb87828801610339565b909550935050602085013567ffffffffffffffff8111156103db57600080fd5b6103e787828801610339565b95989497509550505050565b634e487b7160e01b600052603260045260246000fd5b808202811582820484141761042e57634e487b7160e01b600052601160045260246000fd5b92915050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b858152606060208201526000610477606083018688610434565b828103604084015261048a818587610434565b9897505050505050505056fea264697066735822122029326a40b178bfe66fc450bdb29384fdc3aafb99b1c9899ea7d66cb1d562425a64736f6c634300081c0033")]
    contract Sp1Verifier {
        constructor(address sp1Verifier_, bytes32 programVKey_) {}
    }
}

sol! {
    #[sol(rpc)]
    #[sol(bytecode = "0x60c060405234801561001057600080fd5b506040516105d93803806105d983398101604081905261002f91610045565b6001600160a01b0390911660805260a05261007f565b6000806040838503121561005857600080fd5b82516001600160a01b038116811461006f57600080fd5b6020939093015192949293505050565b60805160a0516105296100b06000396000818160d001526102770152600081816056015261024601526105296000f3fe608060405234801561001057600080fd5b506004361061004c5760003560e01c80635c9770c514610051578063b4b0686a14610095578063b8e72af6146100b6578063ef3f7dd5146100cb575b600080fd5b6100787f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b0390911681526020015b60405180910390f35b6100a86100a33660046102d8565b610100565b60405161008c929190610304565b6100c96100c43660046103d8565b6101d8565b005b6100f27f000000000000000000000000000000000000000000000000000000000000000081565b60405190815260200161008c565b6040805160018082528183019092526060918291906020808301908036833701905050915060008260008151811061013a5761013a610449565b6001600160a01b0392909216602092830291909101820152604080516001808252818301909252918281019080368337019050509050833a11156101a657610182848661045f565b8160008151811061019557610195610449565b6020026020010181815250506101d0565b6101b03a8661045f565b816000815181106101c3576101c3610449565b6020026020010181815250505b935093915050565b6000600285856040516101ec92919061048a565b602060405180830381855afa158015610209573d6000803e3d6000fd5b5050506040513d601f19601f8201168201806040525081019061022c919061049a565b60405163ab750e7560e01b81529091506001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063ab750e75906102a190869086907f00000000000000000000000000000000000000000000000000000000000000009087906004016104b3565b60006040518083038186803b1580156102b957600080fd5b505afa1580156102cd573d6000803e3d6000fd5b505050505050505050565b6000806000606084860312156102ed57600080fd5b505081359360208301359350604090920135919050565b6040808252835190820181905260009060208501906060840190835b818110156103475783516001600160a01b0316835260209384019390920191600101610320565b50508381036020808601919091528551808352918101925085019060005b81811015610383578251845260209384019390920191600101610365565b50919695505050505050565b60008083601f8401126103a157600080fd5b50813567ffffffffffffffff8111156103b957600080fd5b6020830191508360208285010111156103d157600080fd5b9250929050565b600080600080604085870312156103ee57600080fd5b843567ffffffffffffffff81111561040557600080fd5b6104118782880161038f565b909550935050602085013567ffffffffffffffff81111561043157600080fd5b61043d8782880161038f565b95989497509550505050565b634e487b7160e01b600052603260045260246000fd5b808202811582820484141761048457634e487b7160e01b600052601160045260246000fd5b92915050565b8183823760009101908152919050565b6000602082840312156104ac57600080fd5b5051919050565b606081528360608201528385608083013760006080858301015260006080601f19601f87011683010190508360208301528260408301529594505050505056fea2646970667358221220aa1d3260494d86991de3c8cbbeb10ea0cc2a603c078f4627d038f046c269349064736f6c634300081c0033")]
    contract R0Verifier {
        constructor(address risc0Verifier_, bytes32 imageId_) {}
    }
}

sol! {
    #[sol(rpc)]
    #[sol(bytecode = "0x608060405260008055348015601357600080fd5b50610380806100236000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80630bcf730314610046578063b4b0686a14610062578063b8e72af614610083575b600080fd5b61004f60005481565b6040519081526020015b60405180910390f35b610075610070366004610198565b610098565b6040516100599291906101c4565b610096610091366004610298565b610170565b005b604080516001808252818301909252606091829190602080830190803683370190505091506000826000815181106100d2576100d2610309565b6001600160a01b0392909216602092830291909101820152604080516001808252818301909252918281019080368337019050509050833a111561013e5761011a848661031f565b8160008151811061012d5761012d610309565b602002602001018181525050610168565b6101483a8661031f565b8160008151811061015b5761015b610309565b6020026020010181815250505b935093915050565b6000548114610192576040516309bde33960e01b815260040160405180910390fd5b50505050565b6000806000606084860312156101ad57600080fd5b505081359360208301359350604090920135919050565b6040808252835190820181905260009060208501906060840190835b818110156102075783516001600160a01b03168352602093840193909201916001016101e0565b50508381036020808601919091528551808352918101925085019060005b81811015610243578251845260209384019390920191600101610225565b50919695505050505050565b60008083601f84011261026157600080fd5b50813567ffffffffffffffff81111561027957600080fd5b60208301915083602082850101111561029157600080fd5b9250929050565b600080600080604085870312156102ae57600080fd5b843567ffffffffffffffff8111156102c557600080fd5b6102d18782880161024f565b909550935050602085013567ffffffffffffffff8111156102f157600080fd5b6102fd8782880161024f565b95989497509550505050565b634e487b7160e01b600052603260045260246000fd5b808202811582820484141761034457634e487b7160e01b600052601160045260246000fd5b9291505056fea26469706673582212203a29a788bd43d2859db7b457a923ee6795f17d233d6e9e2ddf34a8e10435c5a264736f6c634300081c0033")]
    contract MockVerifier {
        constructor() {}
    }
}
