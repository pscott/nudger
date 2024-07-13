"use client";

import { useDemoStore } from "@/store/DemoStore";
import { useAccount } from "wagmi";
import NavbarNudge from "./nudges/NavbarNudge";
import HoverCardNudge from "./nudges/HoverCard";

export default function Banner() {
  const { address } = useAccount();
  const { selectedDemo } = useDemoStore();

  return (
    <div className="absolute top-0 left-1/2 transform -translate-x-1/2 text-center mb-4">
      <p className="font-normal text-blue-950 font-sans">
        {address && selectedDemo === "navbar" && (
          <NavbarNudge address={address} />
        )}
        {address && selectedDemo === "hover-card" && (
          <HoverCardNudge address={address} />
        )}
      </p>
    </div>
  );
}
