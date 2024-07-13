"use client";

import { useDemoStore } from "@/store/DemoStore";
import { useAccount } from "wagmi";
import NavbarNudge from "./nudges/NavbarNudge";
import HoverCardNudge from "./nudges/HoverCard";

export default function Banner() {
  const { address } = useAccount();
  const { selectedDemo } = useDemoStore();

  return (
    <>
      {address && selectedDemo === "navbar" && (
        <NavbarNudge address={address} />
      )}
      {address && selectedDemo === "hover-card" && (
        <HoverCardNudge address={address} />
      )}
    </>
  );
}
