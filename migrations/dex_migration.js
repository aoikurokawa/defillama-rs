const { deployProxy } = require('@openzeppelin/truffle-upgrades');

const MaticDex = artifacts.require('MaticDex');
const MaticNftFractionsRepository = artifacts.require('MaticNftFractionsRepository');

module.exports = async function (deployer, network) {
    let maticNftFractionsRepositoryInstance;
    let maticDex;

    if (network === 'maticMumbai') {
        maticDex = await deployProxy(MaticDex, [], { deployer });
        maticNftFractionsRepositoryInstance = await MaticNftFractionsRepository.deployed();
        await maticDex.setNftFractionsRepository(maticNftFractionsRepositoryInstance.address);
    }
};