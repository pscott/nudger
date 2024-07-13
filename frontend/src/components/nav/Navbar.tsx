"use client";

import { useDemoStore } from "@/store/DemoStore";
import SelectDemo from "../SelectDemo";
import Wallet from "../wallet/ConnectWallet";

import { useAccount } from "wagmi";
import HoverCardNudge from "../nudges/HoverCard";
import NavbarNudge from "../nudges/NavbarNudge";

export default function Navbar() {
  const { address } = useAccount();
  const { selectedDemo } = useDemoStore();

  return (
    <nav className="p-4">
      <div className="bg-[#a3e6fc] max-w-7xl mx-auto p-2 rounded-full flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <svg
            className="ml-4 h-8 w-8 text-blue-900"
            viewBox="0 0 24 24"
            fill="currentColor"
          >
            <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
          </svg>
          <div className="hidden md:block">
            <SelectDemo />
          </div>
        </div>

        <div className="flex items-center space-x-4">
          {address && selectedDemo === "navbar" && (
            <NavbarNudge address={address} />
          )}
          {address && selectedDemo === "hover-card" && (
            <HoverCardNudge address={address} />
          )}
          <div className="hidden md:block">
            <div className="flex items-center  rounded-full px-3 py-1">
              <svg
                className="h-5 w-5 text-blue-600 mr-2"
                viewBox="0 0 24 24"
                fill="currentColor"
              >
                <path d="M11.944 17.97L4.58 13.62 11.943 24l7.37-10.38-7.372 4.35h.003zM12.056 0L4.69 12.223l7.365 4.354 7.365-4.35L12.056 0z" />
              </svg>
              <span className="text-blue-900 font-medium">Ethereum</span>
              <svg
                className="h-4 w-4 text-blue-900 ml-1"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </div>
          </div>
          <Wallet />
        </div>
      </div>
    </nav>
  );
}
