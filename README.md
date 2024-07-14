# nudger

This project has a backend and a frontend part to it for the demo:
- This blazing fast backend is written in Rust (axum / tokio / tower crates) and uses @zerion 's API to query the user's onchain data in order to find the most pertinent nudge to display (e.g, get the user's complex fungible portfolio and parse it to get specific positions in specific protocols, or getting specific NFT collection balance, or chain-specific tokens)

- Amongst others, the frontend, used for the demo, relies on:
    - Next.js: The React framework for building server-side rendered and static web applications.
    - Typescript: An open-source high-level programming language.
    - TailwindCSS: Utility-first CSS framework for styling.
    - Wagmi: React Hooks for Ethereum.
    - Viem: Low-level typescript interface for Ethereum.
    - Web3Modal: Web3 provider solution for all Wallets.

In the "real world" web3 frontend would be integrating our solution directly through an SDK (or our API for maximum control).
