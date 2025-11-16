# PoolPay: The Trustless Savings Protocol

PoolPay is a decentralized application (dApp) that modernizes and secures traditional Rotating Savings and Credit Associations (ROSCAs), commonly known as "Tandas," "Ajo," or "Chit Funds." PoolPay replaces fragile social trust with verifiably enforced digital collateral and an immutable escrow system built on smart contracts.

---

## 🚀 Vision & Core Problem

### The Problem
Traditional ROSCAs involve significant "Runner Risk": a member who is the organizer might disappear with our money, causing the pool to collapse and financially harming the team members.

### The PoolPay Solution
PoolPay eliminates this risk by holding the fund into a secured contract and the allocation of numbers are done in the contract, the contract also determines the cycle through some calculation which is determined by the details passed in durring pool creation.


## 🏗️ Architecture Overview

PoolPay follows a classic dApp architecture, separating immutable smart contract logic from the user interface.

| Component        | Technology                   | Role                                                                 |
|-----------------|------------------------------|----------------------------------------------------------------------|
| Smart Contracts | Solidity (EVM-compatible)    | Manages pool creation, deposits, withdrawal, and transparent payouts |
| Frontend        | React, TypeScript, Tailwind CSS | Handles wallet connection, real-time data display (via wagmi), and transaction signing |

---

## 🧠 Smart Contract Deep Dive (Solidity)

The smart contract layer secures funds and enforces rules.

### Key Contracts

| Contract Name      | Purpose                                                                 | Critical Functions                         |
|------------------|-------------------------------------------------------------------------|-------------------------------------------|
| PoolFactory.sol   | Factory contract to create and track active ROSCAPool instances, handle dApp configuration and fees | `createPool()`, `JoinPool()`          |
| ROSCAPool.sol     | Core logic for individual saving pools                                   | `joinGroup()`, `contribute()`, `withdrawAutomatically()` |


---

## 💻 Frontend Deep Dive (React/TypeScript)

The frontend provides an intuitive, transparent, and responsive user experience.

### Key Features

- **Pool Dashboard:** Personalized view of active pools, next contribution dates, and collateral status.  
- **Immutable Ledger:** Public log of all transactions pulled directly from contract events.  
- **Mobile-First Design:** Built with Tailwind CSS for a smooth experience on all devices.

### Data Flow

1. User connects wallet (MetaMask, WalletConnect).  
2. Frontend instantiates an `Wagmi` instance for smart contract interactions.  
3. Transaction and pool history data is retrieved   
4. Write operations are signed by the user's wallet and broadcast to the blockchain.

---

## 🛠️ Getting Started

Follow these steps to set up and run PoolPay locally.



### Installation
```bash
git clone https://github.com/your-repo/poolpay.git

```

#### 1. Smart Contract Deployment

```bash

cd poolpay/contracts

forge build
```
#### 2. Smart Contract Deployment
```bash

cd poolpay_frontend

npm install

npm run dev
```
