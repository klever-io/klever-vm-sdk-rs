#!/bin/bash

OPERATOR=../klever-go/bin/operator
WALLET_PEM=../klever-go/localWalletKey.pem
CONTRACT_FOLDER=./contracts/examples/deploy-delete
CONTRACT_DEPLOYER_FOLDER=${CONTRACT_FOLDER}/deployer
CONTRACT_INSTANCE_FOLDER=${CONTRACT_FOLDER}/instance
ABI_FILE=${CONTRACT_FOLDER}/instance/output/instance.abi.json

DEPLOYER_ADDRESS_FILE=deployer_address.txt
INSTANCE_ADDRESS_FILE=instance_address.txt

CONTRACT_DEPLOYER=${CONTRACT_DEPLOYER_FOLDER}/output/deployer.wasm
CONTRACT_INSTANCE=${CONTRACT_INSTANCE_FOLDER}/output/instance.wasm

DEPLOYER_ADDRESS=$(cat ${DEPLOYER_ADDRESS_FILE})
INSTANCE_ADDRESS=$(cat ${INSTANCE_ADDRESS_FILE})

hex_to_bech32() {
    local hex=$1
    echo $(${OPERATOR} --result-only helper convert-address ${hex})
}

function deploy_deployer() {
    echo "Deploying contract deployer..."
    # if the deployer address file exists, ask if should be deploy again
    if [ -f ${DEPLOYER_ADDRESS_FILE} ]; then
        echo "Deployer address file already exists."
        echo "Address: ${DEPLOYER_ADDRESS}"
        echo "Do you want to deploy again? (y/n) "
        read -r answer
        if [[ ! $answer =~ ^[Yy]$ ]]; then
            echo "Exiting..."
            exit 0
        fi
    fi
    ${OPERATOR} --key-file=${WALLET_PEM} \
        sc create \
        --wasm=${CONTRACT_DEPLOYER} \
        --readable \
        --await --sign --result-only \
        | jq -r '.logs.events[] | select(.identifier=="SCDeploy") | .address' > ${DEPLOYER_ADDRESS_FILE}
    DEPLOYER_ADDRESS=$(cat ${DEPLOYER_ADDRESS_FILE})
    echo "Deployer address: ${DEPLOYER_ADDRESS}"
}

function deploy_instance() {
    echo "Deploying contract instance..."
    # if the instance address file exists, ask if should be deploy again
    if [ -f ${INSTANCE_ADDRESS_FILE} ]; then
        echo "Instance address file already exists."
        echo "Address: ${INSTANCE_ADDRESS}"
        echo "Do you want to deploy again? (y/n) "
        read -r answer
        if [[ ! $answer =~ ^[Yy]$ ]]; then
            echo "Exiting..."
            exit 0
        fi
    fi
    ${OPERATOR} --key-file=${WALLET_PEM} \
        sc create \
        --wasm=${CONTRACT_INSTANCE} \
        --readable \
        --args=bi:0 \
        --await --sign --result-only \
        | jq -r '.logs.events[] | select(.identifier=="SCDeploy") | .address' > ${INSTANCE_ADDRESS_FILE}
    INSTANCE_ADDRESS=$(cat ${INSTANCE_ADDRESS_FILE})
    echo "Instance address: ${INSTANCE_ADDRESS}"
}

function new_instance() {
    echo "Creating new instance..."  
    INSTANCE_ADDRESS_NEW=$(
        ${OPERATOR} --key-file=${WALLET_PEM}  \
            sc invoke \
            ${DEPLOYER_ADDRESS} \
            deploy \
            --args=address:${INSTANCE_ADDRESS} \
            --await --sign --result-only \
            | jq -r '.logs.events[] | select(.identifier=="SCDeploy") | .address' 
    )
    echo "New instance address: ${INSTANCE_ADDRESS_NEW}"
}

function list_deployed() {
    echo "Deployed contracts:"
    echo "Deployer address: ${DEPLOYER_ADDRESS}"
    echo "Instance address: ${INSTANCE_ADDRESS}"
    RETURN_DATA_HEX=$(
        ${OPERATOR} --key-file=${WALLET_PEM} \
            sc invoke \
            ${DEPLOYER_ADDRESS} \
            getDeployedContracts \
            --await --sign --result-only \
            | jq -r '.logs.events[] | select(.identifier=="ReturnData") | .data[]'

    )
    # break values into array
    IFS=$'\n' read -r -d '' -a RETURN_DATA_HEX <<< "${RETURN_DATA_HEX}"
    
    # Convert each hex value to bech32 address
    ADDRESSES=()
    for hex in "${RETURN_DATA_HEX[@]}"; do
        bech32_address=$(hex_to_bech32 "${hex}")
        ADDRESSES+=("${bech32_address}")
    done

    # Print all addresses
    echo "Found deployed addresses:"
    for addr in "${ADDRESSES[@]}"; do
        echo "${addr}"
        get_instance_review ${addr}
    done
}

function get_instance_review() {
    local ADDR=$1
    VALUE=$(${OPERATOR} --key-file=${WALLET_PEM} \
        sc invoke \
        ${ADDR} \
        getReview \
        --await --sign --result-only \
        | jq -r '.logs.events[] | select(.identifier=="ReturnData") | .data[]'
    )
    # decode review
    echo Review: $(decode_review ${VALUE})
}

function decode_review() {
    local hex=$1
    # if hex is empty, return zero
    if [ -z "$hex" ]; then
        hex="00"
    fi
    # ./bin/operator sc parse-output hex getReview --abi=../klever-vm-sdk-rs-internal/contracts/examples/deploy-delete/instance/output/instance.abi.json --raw-output=01
    echo $(${OPERATOR} --result-only sc parse-output hex getReview --abi=${ABI_FILE} --raw-output=${hex})
}

function upgrade_instance() {
    local INSTANCE_INDEX=$1
    echo "Upgrading instance ${INSTANCE_INDEX}..."
    ${OPERATOR} --key-file=${WALLET_PEM} \
        sc invoke \
        ${DEPLOYER_ADDRESS} \
        upgradeDeployed \
        --args=usize:${INSTANCE_INDEX} \
        --args=address:${INSTANCE_ADDRESS} \
        --await --sign --result-only
    echo "Instance ${INSTANCE_INDEX} upgraded."
}

function delete_last() {
    echo "Deleting last instance..."
    ${OPERATOR} --key-file=${WALLET_PEM} \
        sc invoke \
        ${DEPLOYER_ADDRESS} \
        deleteLast \
        --await --sign --result-only
    echo "Last instance deleted."
}

main() {
    # if no arguments are passed, deploy the contract deployer and instance
    if [ $# -eq 0 ]; then
        deploy_deployer
        deploy_instance
        exit 0
    fi
    # case deploy-deployer and deploy-instance
    case $1 in
        deploy-deployer)
            deploy_deployer
            ;;
        deploy-instance)
            deploy_instance
            ;;
        new-instance)
            new_instance
            ;;
        list)
            list_deployed
            ;;
        review)
            if [ -z "$2" ]; then
                echo "Usage: $0 review <address>"
                exit 1
            fi
            get_instance_review "$2"
            ;;
        upgrade)
            if [ -z "$2" ]; then
                echo "Usage: $0 upgrade <instance_index>"
                exit 1
            fi
            upgrade_instance "$2"
            ;;
        delete)
            delete_last
            ;;
        *)
            echo "Usage: $0 [deploy-deployer|deploy-instance]"
            exit 1
            ;;
    esac    
}

main "$@"