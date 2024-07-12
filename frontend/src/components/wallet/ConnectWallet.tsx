"use client";

import { useWeb3Modal } from "@web3modal/wagmi/react";
import { useAccount } from "wagmi";

export function ConnectWallet() {
  const { open } = useWeb3Modal();

  return (
    <button
      className="bg-blue-900 text-white px-4 py-2 rounded-full font-medium"
      onClick={() => open()}
    >
      Connect Wallet
    </button>
  );
}

export default function Wallet() {
  const { status: walletStatus } = useAccount();

  return (
    <div className="flex justify-end">
      {walletStatus !== "connected" && (
        <div className="flex flex-row gap-x-2">
          <ConnectWallet />
        </div>
      )}
      {walletStatus === "connected" && (
        <div className="flex flex-row">
          <w3m-button balance="hide" size="sm" />
        </div>
      )}
    </div>
  );
}
