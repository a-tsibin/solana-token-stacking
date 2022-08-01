# Solana token stacking
- [x] You can buy (and buy) FCTR-token for SOL.
      Minimum purchase = 10 FTCR
      Course 1:109, fixed.
      FCTR can be sold for SOL, at a rate of 101:1, also fixed.
      The user can only sell all of his FCTR at once.
      FCTR-token has decimals 12. Mint is owned by the program.
      BCDEV-token has decimals 18. Mint is owned by the program.
      BCDEV can be sold for SOL, at a rate of 11:1, the rate is fixed.
- [x] Only tokens purchased from the platform and/or transferred to “trust management” through the platform are considered legitimate. Tokens obtained by the user in another way(s) should not participate in staking.
- [x] The user must register on the platform.
- [x] It is assumed that registration on the platform will occur with some kind of non-deanonymizing verification, preventing the ability of one user to register many accounts. From the side of the solana program, this looks like waiting for an additional signature in the user creation instruction. (Paid registration?)
- [x] The user stakes the purchased FCTR on the platform and receives a BCDEV token as rewards ("APR" depends on the number of tokens trusted by the user and the number of tokens trusted by the user). The stake occurs immediately for all “legitimate” user FCTR tokens, without choosing the amount.
- [x] The stake can be increased, for example, if someone trusted the user their FCTR, or the user bought additional FCTR on the platform.
- [x] User unstakes (all at once) and claiming happens automatically. If that someone trusted the user their FCTR, after the unstake, the tokens are automatically returned to the owners. The user cannot unstake until the end of the round.
- [x] The user can transfer (trust) part of his FCTR to another user through the platform (he can also transfer simply through spl-token, but then he will not have return guarantees, and an increased "APR" for the staker).
- [x] Transferred tokens are automatically staked on behalf of a trusted person if this user has already staked his tokens.
- [ ] Otherwise, they simply go to the disposal of a trusted person.
- [x] If the user participates in the “trust program” (as a trustee or as a confidient), then he cannot (over-)buy and sell FCTR.
- [x] Owner (the server that owns the owner's private) can start the next round(s) of staking.
- [x] The platform owner can replenish the platform balance at any time.
- [x] Owner can withdraw sol from the platform if all users have sold their FCTR and BCDEV tokens, or more than two rounds have passed since the end of the last (final) round.
