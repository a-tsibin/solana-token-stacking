import {expect} from "chai";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import {Context} from "./ctx";
import {
    addLiquidity, buyTokens, claimTokens, grantTokens,
    initialize, registerUser, sellBcdevTokens, sellFctrTokens, stake, startRound, unstake, withdraw
} from "./token-stacking-api";
import {transfer} from "./token";
import {sleep} from "./utils";
import {Keypair} from "@solana/web3.js";

chai.use(chaiAsPromised);

const ctx = new Context();

before(async () => {
    await ctx.setup();
});

describe("token-stacking", () => {
    it("Initialize", async () => {
        const roundDuration = 3;
        const registrationPrice = 100_000;
        await initialize(ctx, roundDuration, registrationPrice);

        const platform = await ctx.platformAcc();
        expect(platform.bump).to.gt(200);
        expect(platform.bumpSolVault).to.gt(200);
        expect(platform.bumpFctrMint).to.gt(200);
        expect(platform.bumpBcdevMint).to.gt(200);
        expect(platform.bumpFctrTokenVault).to.gt(200);
        expect(platform.authority.toString()).to.eql(ctx.platformAuthority.publicKey.toString());
        expect(platform.roundDuration.toNumber()).to.eql(roundDuration);
        expect(platform.registrationPrice.toNumber()).to.eql(registrationPrice);
        expect(platform.isFinal).to.eql(false);
        expect(platform.fctrTokenTotalAmount.toNumber()).to.eql(0);
        expect(platform.bcdevTokenTotalAmount.toNumber()).to.eql(0);
        expect(platform.roundStart.toNumber()).to.eql(0);
    });

    it("Register user", async () => {
        const promises = [];
        for (let i = 0; i < ctx.users.length; i++) {
            promises.push(registerUser(ctx, ctx.users[i], true));
        }
        await Promise.all(promises);

        const user = await ctx.userAcc(ctx.users[0].publicKey);
        expect(user.bump).to.gt(200);
        expect(user.bumpFctrVault).to.gt(200);
        expect(user.bumpBcdevVault).to.gt(200);
        expect(user.bumpReceipt).to.gt(200);
        expect(user.grantProgram).to.eql(true);
        expect(user.userFctrAmount.toNumber()).to.eql(0);
        expect(user.authority).to.eql(ctx.users[0].publicKey);

        const receipt = await ctx.receiptAcc(user.authority);
        expect(receipt.isValid).to.eql(false);
    });

    it("Add liquidity", async () => {
        const amount = 100_000;
        const balanceBefore = await ctx.solVaultBalance();
        await addLiquidity(ctx, amount);

        const balanceAfter = await ctx.solVaultBalance();
        expect(balanceBefore).to.eql(balanceAfter - amount);
    });

    it("Buy tokens", async () => {
        const lamports = 10;
        const balanceBefore = await ctx.solVaultBalance();
        const ftcrAmountBefore = (await ctx.platformAcc()).fctrTokenTotalAmount.toNumber();
        const expectedFctrCount = 1_090_000;
        await buyTokens(ctx, lamports, ctx.users[0]);

        const ftcrAmountAfter = (await ctx.platformAcc()).fctrTokenTotalAmount.toNumber();

        const balanceAfter = await ctx.solVaultBalance();
        expect(balanceBefore).to.eql(balanceAfter - lamports);
        expect(ftcrAmountAfter - ftcrAmountBefore).to.eql(expectedFctrCount);

        const platform = await ctx.program.account.platform.fetch(ctx.platform);
        expect(platform.fctrTokenTotalAmount.toNumber()).to.eql(expectedFctrCount);

        const user = await ctx.userAcc(ctx.users[0].publicKey);
        const userFtcrAmount = await (await ctx.userFctrVault(user.authority)).amount(ctx);
        expect(userFtcrAmount).to.eql(expectedFctrCount);
        expect(user.userFctrAmount.toNumber()).to.eql(expectedFctrCount);
    });

    it("Can't stake before round starts", async () => {
        await expect(stake(ctx, ctx.users[0])).to.be.rejected;
    });

    it("Stake tokens without grantors", async () => {
        const userFctrAmountBefore = await (await ctx.userFctrVault(ctx.users[0].publicKey)).amount(ctx);
        await startRound(ctx, false);
        await stake(ctx, ctx.users[0]);

        const user = await ctx.userAcc(ctx.users[0].publicKey);
        const userFtcrAmountWhileStake = await (await ctx.userFctrVault(user.authority)).amount(ctx);
        expect(userFtcrAmountWhileStake).to.eql(0);

        await expect(unstake(ctx, ctx.users[0])).to.be.rejected;

        await sleep(4000);

        await unstake(ctx, ctx.users[0]);

        const userFctrAmountAfter = await (await ctx.userFctrVault(ctx.users[0].publicKey)).amount(ctx);
        expect(userFctrAmountAfter).to.eql(userFctrAmountBefore);

        const userBcdevAmount = await (await ctx.userBcdevVault(ctx.users[0].publicKey)).amount(ctx);
        expect(userBcdevAmount).to.gt(0);

        const platform = await ctx.platformAcc();
        expect(platform.bcdevTokenTotalAmount.toNumber()).to.eql(userBcdevAmount)
    });

    it("Withdraw failed", async () => {
        await expect(withdraw(ctx, ctx.platformAuthority)).to.be.rejected;
    });

    it("Sell tokens", async () => {
        const userFctrAmountBefore = await (await ctx.userFctrVault(ctx.users[0].publicKey)).amount(ctx);
        const userBcdevAmountBefore = await (await ctx.userBcdevVault(ctx.users[0].publicKey)).amount(ctx);
        let platform = await ctx.platformAcc();
        const platformFctrAmountBefore = platform.fctrTokenTotalAmount;
        const platformBcdevAmountBefore = platform.bcdevTokenTotalAmount;

        await sellFctrTokens(ctx, ctx.users[0]);
        await sellBcdevTokens(ctx, userBcdevAmountBefore, ctx.users[0]);

        const user = await ctx.userAcc(ctx.users[0].publicKey);
        expect(user.userFctrAmount.toNumber()).to.eql(0);
        const userFtcrAmount = await (await ctx.userFctrVault(user.authority)).amount(ctx);
        expect(userFtcrAmount).to.eql(0);
        const userBcdevAmount = await (await ctx.userBcdevVault(user.authority)).amount(ctx);
        expect(userBcdevAmount).to.eql(0);

        platform = await ctx.platformAcc();
        expect(platform.fctrTokenTotalAmount.toNumber()).to.eql(platformFctrAmountBefore.toNumber() - userFctrAmountBefore);
        expect(platform.bcdevTokenTotalAmount.toNumber()).to.eql(platformBcdevAmountBefore.toNumber() - userBcdevAmountBefore);
    });

    it("Withdraw", async () => {
        await withdraw(ctx, ctx.platformAuthority);

        const balance = await ctx.solVaultBalance();
        expect(balance).to.eql(0);

        await addLiquidity(ctx, 1_000_000_000);
        await buyTokens(ctx, 100_000, ctx.users[0]);
    });

    it("Grant tokens", async () => {
        const platformFctrAmountBefore = await (await ctx.fctrVault()).amount(ctx);
        const lamports = 50_000;
        for (let i = 1; i < 5; i++) {
            await buyTokens(ctx, lamports, ctx.users[i]);
        }
        const grantorFtcrAmountBefore = await (await ctx.userFctrVault(ctx.users[1].publicKey)).amount(ctx);
        const grantAmount = grantorFtcrAmountBefore / 2;

        for (let i = 1; i < 5; i++) {
            await grantTokens(ctx, grantAmount, ctx.users[0].publicKey, ctx.users[i]);
        }

        const receipt = await ctx.receiptAcc(ctx.users[0].publicKey);
        expect(receipt.nextRoundGrantors.length).to.eql(4);
        expect(receipt.grantorsHistory.length).to.eql(4);

        const grantorFtcrAmountAfter = await (await ctx.userFctrVault(ctx.users[1].publicKey)).amount(ctx);
        expect(grantorFtcrAmountAfter).to.eql(grantorFtcrAmountBefore - grantAmount);

        const grantorReceipt = await ctx.receiptAcc(ctx.users[1].publicKey);
        expect(grantorReceipt.apr).to.eql(0.02);

        const platformFctrAmountAfter = await (await ctx.fctrVault()).amount(ctx);
        expect(platformFctrAmountAfter - platformFctrAmountBefore).to.eql(grantAmount * 4);
    });

    it("Can't accept more than 4 grants", async () => {
        await buyTokens(ctx, 50_000, ctx.users[5]);
        await expect(grantTokens(ctx, 50_000, ctx.users[0].publicKey, ctx.users[5])).to.be.rejected;
    });

    it("Stake tokens with grantors(before round)", async () => {
        const bcdevAmountBefore = (await ctx.platformAcc()).bcdevTokenTotalAmount.toNumber();
        await startRound(ctx, false);
        await stake(ctx, ctx.users[0]);

        let receipt = await ctx.receiptAcc(ctx.users[0].publicKey);
        expect(receipt.nextRoundGrantors.length).to.eql(0);
        expect(receipt.grantors.length).to.eql(4);

        await claimTokens(ctx, ctx.users[0].publicKey, ctx.users[4]);
        receipt = await ctx.receiptAcc(ctx.users[0].publicKey);
        expect(receipt.nextRoundGrantors.length).to.eql(0);
        expect(receipt.grantors.length).to.eql(3);

        await sleep(4000);

        await unstake(ctx, ctx.users[0]);

        const bcdevAmountAfter = (await ctx.platformAcc()).bcdevTokenTotalAmount.toNumber();
        const reward = bcdevAmountAfter - bcdevAmountBefore;

        const confidantBcdev = await (await ctx.userBcdevVault(ctx.users[0].publicKey)).amount(ctx);
        expect(Math.abs((reward / 2) - confidantBcdev)).to.lt(1);

        const grantorReward = reward / (2 * 3);
        for (let i = 1; i < 4; i++) {
            const user = await ctx.userAcc(ctx.users[i].publicKey);
            const userFctrAmount = await (await ctx.userFctrVault(ctx.users[i].publicKey)).amount(ctx);
            expect(userFctrAmount).to.eql(user.userFctrAmount.toNumber());
            const userBcdevAmount = await (await ctx.userBcdevVault(ctx.users[i].publicKey)).amount(ctx);
            expect(Math.abs(grantorReward - userBcdevAmount)).to.lt(1);
        }
    });

    it("Stake tokens with grantors(while round is started)", async () => {
        const bcdevAmountBefore = (await ctx.platformAcc()).bcdevTokenTotalAmount.toNumber();
        const grantorFtcrAmountBefore = await (await ctx.userFctrVault(ctx.users[2].publicKey)).amount(ctx);
        const grantAmount = grantorFtcrAmountBefore / 2;

        await startRound(ctx, true);
        await stake(ctx, ctx.users[0]);

        await grantTokens(ctx, grantAmount, ctx.users[0].publicKey, ctx.users[1]);
        let receipt = await ctx.receiptAcc(ctx.users[0].publicKey);
        expect(receipt.grantors.length).to.eql(1);

        await sleep(1000);
        await grantTokens(ctx, grantAmount, ctx.users[0].publicKey, ctx.users[2]);
        receipt = await ctx.receiptAcc(ctx.users[0].publicKey);
        expect(receipt.grantors.length).to.eql(2);

        await sleep(3000);

        await unstake(ctx, ctx.users[0]);

        const bcdevAmountAfter = (await ctx.platformAcc()).bcdevTokenTotalAmount.toNumber();
        const reward = bcdevAmountAfter - bcdevAmountBefore;

        const confidantBcdev = await (await ctx.userBcdevVault(ctx.users[0].publicKey)).amount(ctx);
        expect(Math.abs((reward / 2) - confidantBcdev)).to.lt(1);

        const user1BcdevAmount = await (await ctx.userBcdevVault(ctx.users[1].publicKey)).amount(ctx);
        const user2BcdevAmount = await (await ctx.userBcdevVault(ctx.users[2].publicKey)).amount(ctx);
        expect(user1BcdevAmount).to.gt(user2BcdevAmount);
    });
});
