# Deposit KDA Pool

## Overview

The Deposit KDA Pool contract is designed to facilitate deposits into your asset’s kda pool. By default, it is only possible for the owner, pool admin, and any address that has the Deposit function to be able to make deposits.

To allow anyone to deposit into your pool, you must enable the address of the contract that was deployed to have deposit function permissions. To do this, you must make an [Asset Trigger](https://docs.klever.org/contract-details#asset-trigger) transaction of type AddRoles.

## Workflow

1. **Deploy Contract**:
   - Deploy Contract passing your kda pool in the params.

2. **Deposit**:
   - If your contract is deployed and you give the Deposit Role permission to your contract, anyone can deposit KLV and your token to the contract! Just call the function <em>deposit</em>