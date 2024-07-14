import { createPublicClient, createWalletClient } from "viem";
import { createConfig, http } from "wagmi";
import { mainnet, Chain } from "wagmi/chains";
import { coinbaseWallet, injected, walletConnect } from "wagmi/connectors";

declare module "wagmi" {
  interface Register {
    config: typeof config;
  }
}

export const projectId = process.env.NEXT_PUBLIC_PROJECT_ID;

if (!projectId) throw new Error("Project ID is not defined");

const metadata = {
  name: "Web3Modal",
  description: "Web3Modal Nudger",
  url: process.env.NEXT_PUBLIC_APP_URL as string, // origin must match your domain & subdomain
  icons: ["https://avatars.githubusercontent.com/u/37784886"],
};

const chains: [Chain, ...Chain[]] = [mainnet];

// Create wagmiConfig
export const config = createConfig({
  chains: chains,
  transports: {
    [mainnet.id]: http(),
  },
  connectors: [
    walletConnect({ projectId, metadata, showQrModal: false }),
    injected({ shimDisconnect: true }),
    coinbaseWallet({
      appName: metadata.name,
      appLogoUrl: metadata.icons[0],
      preference: "all",
    }),
  ],
  ssr: true,
});
