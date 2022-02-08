const { deployProxy } = require('@openzeppelin/truffle-upgrades');

const MaticNftFractionsRepository = artifacts.require('MaticNftFractionsRepository');

module.exports = async function (deployer, network) {
    if (network === 'maticMumbai') {
        await deployProxy(MaticNftFractionsRepository, ["URI"], { deployer });
    }
};