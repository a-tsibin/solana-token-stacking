import {expect} from "chai";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import {Context} from "./ctx";
import {
    initialize
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
        const registrationPrice = 200_000;
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
        expect(platform.authority).to.eql(ctx.platformAuthority.publicKey);
        expect(platform.roundDuration).to.eql(roundDuration);
        expect(platform.registrationPrice).to.eql(registrationPrice);
        expect(platform.isFinal).to.eql(false);
        expect(platform.fctrTokenTotalAmount).to.eql(0);
        expect(platform.bcdevTokenTotalAmount).to.eql(0);
        expect(platform.roundStart).to.eql(0);
    });
});
