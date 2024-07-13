"use client";

import { fetchNudge, INudge } from "@/lib/api";
import { useAccount } from "wagmi";
import { useQuery } from "@tanstack/react-query";
import { useDemoStore } from "@/store/DemoStore";

import { toast } from "sonner";

export default function ToastNudge() {
  const { address } = useAccount();
  const { selectedDemo } = useDemoStore();

  if (!address || selectedDemo !== "toaster") {
    return null;
  }

  return <Nudge address={address} />;
}

export function Nudge({ address }: { address: `0x${string}` }) {
  const {
    data: nudge,
    isLoading,
    isError,
    error,
  } = useQuery<INudge, Error>({
    queryKey: ["nudge", address],
    queryFn: () => fetchNudge(address),
  });

  if (isLoading) return null;
  if (isError) {
    return null;
  }

  if (nudge) {
    console.log("Nudge: Showing toast"); // Debug log
    toast("Psssst... got some alpha for you 👀", {
      description: nudge.text,
      duration: 10000,
      action: {
        label: nudge.cta_text || "Do it",
        onClick: () =>
          window.open(
            nudge.cta_url || "https://www.google.com",
            "_blank",
            "noopener,noreferrer"
          ),
      },
      dismissible: true,
    });
  }

  return null;
}
