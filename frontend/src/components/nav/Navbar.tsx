"use client";

import SelectDemo from "../SelectDemo";
import Wallet from "../wallet/ConnectWallet";

import Link from "next/link";

export const items = [
  {
    name: "What is this?",
    url: "/what-is-this",
  },
];

export default function Navbar() {
  return (
    <nav className="p-4">
      <div className="bg-[#a3e6fc] max-w-7xl mx-auto p-2 rounded-full flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <Link href="/">
            <svg
              className="ml-4 h-8 w-8 text-blue-900"
              viewBox="0 0 24 24"
              fill="currentColor"
            >
              <path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5" />
            </svg>
          </Link>
          <div className="hidden md:block">
            <SelectDemo />
          </div>
        </div>

        <div className="flex items-center space-x-4">
          {/* Desktop menu */}
          {items?.map((item) => (
            <Link
              key={item.name}
              href={item.url}
              className="px-4 py-2 rounded-full text-md font-medium transition-colors duration-200 text-gray-700 hover:bg-white hover:text-sky-600"
            >
              {item.name}
            </Link>
          ))}

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
