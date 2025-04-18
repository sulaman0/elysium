import { expect } from "chai";
import { AbiItem } from "web3-utils";

import ExplicitRevertReason from "../build/contracts/ExplicitRevertReason.json";
import {GAS, GAS_PRICE, GENESIS_ACCOUNT, GENESIS_ACCOUNT_PRIVATE_KEY} from "./config";
import { createAndFinalizeBlock, customRequest, describeWithFrontier } from "./util";

describeWithFrontier("Frontier RPC (Revert Reason)", (context) => {
	let contractAddress;

	const REVERT_W_MESSAGE_BYTECODE = ExplicitRevertReason.bytecode;

	const TEST_CONTRACT_ABI = ExplicitRevertReason.abi as AbiItem[];

	before("create the contract", async function () {
		this.timeout(15000);
		const tx = await context.web3.eth.accounts.signTransaction(
			{
				from: GENESIS_ACCOUNT,
				data: REVERT_W_MESSAGE_BYTECODE,
				value: "0x00",
				gasPrice: GAS_PRICE,
				gas: GAS,
			},
			GENESIS_ACCOUNT_PRIVATE_KEY
		);
		const r = await customRequest(context.web3, "eth_sendRawTransaction", [tx.rawTransaction]);
		await createAndFinalizeBlock(context.web3);
		const receipt = await context.web3.eth.getTransactionReceipt(r.result);
		contractAddress = receipt.contractAddress;
	});

	it("should fail with revert reason", async function () {
		const contract = new context.web3.eth.Contract(TEST_CONTRACT_ABI, contractAddress, {
			from: GENESIS_ACCOUNT,
			gasPrice: GAS_PRICE,
		});
		try {
			await contract.methods.max10(30).call();
		} catch (error) {
			expect(error.message).to.be.eq(
				"Returned error: VM Exception while processing transaction: revert Value must not be greater than 10."
			);
		}
	});
});
