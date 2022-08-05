import {expect} from "chai";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import {Context} from "./ctx";
import {
    addLiquidity, buyTokens,
    initialize, registerUser, stake, startRound
} from "./token-stacking-api";
import {transfer} from "./token";

chai.use(chaiAsPromised);

const ctx = new Context();

before(async () => {
    await ctx.setup();
});

describe("token-stacking", () => {
    it("Initialize", async () => {
        const roundDuration = 10;
        const registrationPrice = 100_000;
        await initialize(
            ctx,
            roundDuration,
            registrationPrice
        );

        const platform = await ctx.program.account.platform.fetch(ctx.platform);
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

        const user = await ctx.program.account.user.fetch(
            await ctx.user(ctx.users[0].publicKey)
        );
        expect(user.bump).to.gt(200);
        expect(user.bumpFctrVault).to.gt(200);
        expect(user.bumpBcdevVault).to.gt(200);
        expect(user.bumpReceipt).to.gt(200);
        expect(user.grantProgram).to.eql(true);
        expect(user.totalFctrAmount.toNumber()).to.eql(0);
        expect(user.authority).to.eql(ctx.users[0].publicKey);
    });

    it("Add liquidity", async () => {
        const amount = 100_000;
        const balanceBefore = await ctx.solVaultBalance();
        await addLiquidity(
            ctx,
            amount
        );

        const balanceAfter = await ctx.solVaultBalance();
        expect(balanceBefore).to.eql(balanceAfter - amount);
    });

    it("Buy tokens", async () => {
        const fctrAmountToBuy = 109;
        const balanceBefore = await ctx.solVaultBalance();
        const ftcrAmountBefore = (await ctx.program.account.platform.fetch(ctx.platform)).fctrTokenTotalAmount.toNumber();
        const expected_price = 1_000_000_000;
        await buyTokens(
            ctx,
            fctrAmountToBuy,
            ctx.users[0]
        );

        const ftcrAmountAfter = (await ctx.program.account.platform.fetch(ctx.platform)).fctrTokenTotalAmount.toNumber();

        const balanceAfter = await ctx.solVaultBalance();
        expect(balanceBefore).to.eql(balanceAfter - expected_price);
        expect(ftcrAmountAfter - ftcrAmountBefore).to.eql(fctrAmountToBuy);

        const user = await ctx.program.account.user.fetch(
            await ctx.user(ctx.users[0].publicKey)
        );
        const userFtcrAmount = await (await ctx.userFctrVault(user.authority)).amount(ctx);
        expect(userFtcrAmount).to.eql(fctrAmountToBuy);
    });

    it("Stake tokens", async () => {
        await startRound(ctx, false);
        await stake(ctx, ctx.users[0]);
    });
});
