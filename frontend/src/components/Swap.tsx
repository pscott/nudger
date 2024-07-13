"use client";

import { useWeb3Modal } from "@web3modal/wagmi/react";
import { useAccount } from "wagmi";

export default function Swap() {
  const { open } = useWeb3Modal();
  const { status: walletStatus } = useAccount();

  return (
    <div className="bg-white rounded-3xl shadow-lg p-6 w-[360px] mx-auto">
      <div className="flex justify-between items-center mb-6">
        <div className="flex space-x-1 bg-gray-100 rounded-full p-1">
          <button className="font-semibold text-blue-800 bg-white rounded-full px-4 py-2 transition-colors">
            Swap
          </button>
          <button className="text-gray-500 px-4 py-2 rounded-full hover:bg-white hover:font-bold hover:text-blue-500 transition-colors">
            Limit
          </button>
          <button className="text-gray-500 px-4 py-2 rounded-full hover:bg-white hover:font-bold hover:text-blue-500 transition-colors">
            TWAP
          </button>
        </div>
        <button className="text-gray-500 hover:text-gray-700 transition-colors">
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
            />
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
            />
          </svg>
        </button>
      </div>
      {/* WETH input */}
      <div className="bg-gray-100 rounded-2xl p-4 mb-2">
        <div className="flex justify-between items-center">
          <button className="flex items-center bg-white rounded-full px-3 py-2 hover:bg-gray-50 transition-colors">
            <svg
              className="h-5 w-5 text-blue-500 mr-2"
              viewBox="0 0 24 24"
              fill="currentColor"
            >
              <path d="M11.944 17.97L4.58 13.62 11.943 24l7.37-10.38-7.372 4.35h.003zM12.056 0L4.69 12.223l7.365 4.354 7.365-4.35L12.056 0z" />
            </svg>
            <span className="font-semibold text-gray-800">WETH</span>
            <svg
              className="h-5 w-5 text-gray-400 ml-1"
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
          </button>
          <span className="text-xl font-bold text-gray-400">0.0</span>
        </div>
      </div>
      {/* Arrow */}
      <div className="flex justify-center my-2">
        <svg
          className="h-5 w-5 text-gray-400"
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path
            strokeLinecap="round"
            strokeLinejoin="round"
            strokeWidth={2}
            d="M19 14l-7 7m0 0l-7-7m7 7V3"
          />
        </svg>
      </div>
      {/* Select a token input */}
      <div className="bg-gray-100 rounded-2xl p-4 mb-6">
        <div className="flex justify-between items-center">
          <button className="flex items-center bg-blue-900 text-white rounded-full px-3 py-2 hover:bg-blue-600 transition-colors">
            <span className="font-semibold">Select a token</span>
            <svg
              className="h-5 w-5 ml-1"
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
          </button>
          <span className="text-xl font-bold text-gray-400">0.0</span>
        </div>
      </div>
      {walletStatus === "connected" && (
        <button className="w-full bg-blue-900 text-white py-4 rounded-2xl font-semibold text-lg hover:bg-blue-600 transition-colors">
          Swap
        </button>
      )}
      {walletStatus !== "connected" && (
        <button
          className="w-full bg-blue-900 text-white py-4 rounded-2xl font-semibold text-lg hover:bg-blue-600 transition-colors"
          onClick={() => open()}
        >
          Connect Wallet
        </button>
      )}
    </div>
  );
}
