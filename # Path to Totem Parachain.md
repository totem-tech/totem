# Path to Totem Parachain.

---

### This document is designed for the technical team working on Totem Live Accounting.

It also provides for the working steps for integrating the Totem Live Accounting Parachain on Polkadot Live Network. However, all projects also require a testnet to try out upgrades and the like and therefore we have two Parachains that have been developed:

### 1. Totem Lego Parachain

**_This is the parachain that will be connected to a local testnet version of Polkadot Rococo Test Network_**. 

The intention is to apply all changes to this network ahead of upgrading the Totem WAPEX network which will be live on Westend. Of course all issues will be noted and fixed prior to building and compiling the WAPEX upgrade code.

### 1. Totem Wapex Parachain

**_This is the parachain that will be connected to the Polkadot Westend Test Network_**. 

The intention is to apply all changes to this network ahead of upgrading the Totem KAPEX network which will be live on Polkadot. Of course all issues will be noted and fixed prior to building and compiling the KAPEX upgrade code.

For the moment the Wapex network is a facsimilie of Totem KAPEX and this should always be the case after an upgrade of KAPEX. However Wapex will always be ahead of KAPEX as features and products are added.

> Wapex is a port of the Totem Meccano Canary Blockchain Network that was used to construct the UI. Meccano is still in operation, but will be deprecated over time.

Although Wapex contains all the ported code from Meccano already, it is not fully activated as explained below.

---

### 2. Totem KAPEX Parachain

**_This is the parachain that will be connected to the Polkadot Live Network_** 

It will be used in the initial stages of the Crowdloan and Parachain Auctions. KAPEX is for all intents and purposes a live network, but currently has reduced functionality.

The reason why the functionality is currently reduced, is primarily for the safety for the Polkadot Network. Once KAPEX is safely onboarded as a Parachain, new features will be added, first on the Wapex Network and when working and finalised a KAPEX build will occur to upgrade KAPEX on Polkadot.

The first features added will align functionality with Totem Meccano in a step-by step fashion. The SuDo user for the network will therefore not be deprecated throughout the Roadmap until the MainNet is ready to launch.

---

> Both these chains are available for inspection in our [Gitlab Code Repository](https://gitlab.com/totem-tech)

---

### Steps to implementation
The following is a table of high-level steps required to get from where the project is now, to Parachain on Polkadot and eventually MainNet.

The MainNet detailed steps will be published closer to the time.

| Done  | Steps    | Description                                                | Comment                                                                                 |
|--|----------|-----------------------------------------------------------------|-----------------------------------------------------------------------------------------|
|Done  | Step 0   | Preparation of the Lego Network and genesis file.               |                                                                                     |
|Done  | Step 1   | Preparation of the Wapex Network and genesis file.              |                                                                                     |
|Done  | Step 2   | Preparation of the KAPEX Genesis file.                          |                                                                                     |
|Done  | Step 3   | Branding images created                                         |                                                                                     |
|Done  | Step 4   | Documentation site upgraded.                                    |                                                                                     |
|Done  | Step 5   | Build Lego WASM Binary for Rococo.                              |                                                                                     |
|Done  | Step 6   | Build Wapex WASM Binary for Westend.                            |                                                                                     |
|Done  | Step 7   | Build aKapex WASM Binary for Polkadot.                          |                                                                                     |
|Done  | Step 8   | Generate authority and Bootnode keys Lego                       |                                                                                     |
|Done  | Step 9   | Generate authority and Bootnode keys Wapex                      |                                                                                     |
|Done  | Step 10  | Generate authority and Bootnode keys Kapex                      |                                                                                     |
|[ ]   | Step 11  | Pull requests into PolkadotJS Apps for technicals and branding. |                                                                                     |
|[ ]   | Step 12  | Website upgrade                                                 |                                                                                     |
|[ ]   | Step 13  | Run Totem Lego in the Polkadot Rococo environment.              | If it is producing blocks we are ready to submit the WASM binary to the Polkadot Local Network in Step 14 |
|[ ]   | Step 14  | Apply the WASM Binary to Rococo.                                |                                                                                     |
|[ ]   | Step 15  | Run Totem Wapex in the Polkadot Westend environment.            | If it is producing blocks we are ready to submit the WASM binary to the Westend Network in Step 16 |
|[ ]   | Step 16  | Apply the WASM Binary to Westend.                               |                                                                                     |
|[ ]   | Step 17  | Apply the WASM Binary to Polkadot.                              |                                                                                     |
|[ ]   | Step 18  | Begin Parachain Crowdloan & Auction                             | Once a parachain slot is won then distribution of awards can be made in Step 20     |
|[ ]   | Step 19  | Assuming Parachain slot won, put Meccano into Maintenance Mode. | Maintenance Mode means that no more actions are possible on the network. It will still be kept live in order to read the account balances for migration to KAPEX. |
|[ ]   | Step 20  | Distribution of Coins to Crowdloan Participants                 |                                                                                     |
|[ ]   | Step 21  | Unlocking the Balance Transfers                                 |                                                                                     |
|[ ]   | Step 22  | Upgrading the Wapex Network with the first features             |                                                                                     |
|[ ]   | Step 23  | Testing                                                         | This should occur both in the UI and PolkadotJS apps.                               |
|[ ]   | Step 24  | Build and apply to KAPEX                                        |                                                                                     |
|[ ]   | Step 25  | Repeat from Step 11 to 13 until Meccano can be replaced.        |                                                                                     |
|[ ]   | Step 26  | Begin Meccano deprecation                                       | Meccano will be taken offline as soon as possible.                                  |
|[ ]   | Step 27  | Repeat from Step 11 to 13 until end of Roadmap                  |                                                                                     |
|[ ]   | Step 28  | Launch MainNet and remove SuDo user                             |