"use client";

import { createWeb3Modal } from "@web3modal/wagmi/react";
import { WagmiProvider } from "wagmi";
import { mainnet } from "wagmi/chains";

import { config, projectId } from "../../wagmi.config";

if (!projectId) throw new Error("Project ID is not defined");

createWeb3Modal({
  wagmiConfig: config,
  projectId,
  allowUnsupportedChain: false,
  defaultChain: mainnet,
  featuredWalletIds: [
    "c57ca95b47569778a828d19178114f4db188b89b763c899ba0be274e97267d96", // Metamask
    "18388be9ac2d02726dbac9777c96efaac06d744b2f6d580fccdd4127a6d01fd1", // Rabby
    "fd20dc426fb37566d803205b19bbc1d4096b248ac04548e3cfb6b3a38bd033aa", // Coinbase Wallet
  ],
  excludeWalletIds: [
    "a797aa35c0fadbfc1a53e7f675162ed5226968b44a19ee3d24385c64d1d3c393",
  ],
  enableAnalytics: true,
  enableOnramp: true,
  themeVariables: {
    "--w3m-font-family": "Inter, sans-serif",
    "--w3m-accent": "#0284c7",
  },
});

export function Providers({ children }: { children: React.ReactNode }) {
  return <WagmiProvider config={config}>{children}</WagmiProvider>;
}
