"use client";

import { fetchNudge, Nudge } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";
import { useEffect } from "react";
import { toast } from "sonner";
import { useAccount } from "wagmi";

export default function Toast() {
  const { address } = useAccount();

  const {
    data: nudge,
    isLoading,
    isError,
    error,
  } = useQuery<Nudge, Error>({
    queryKey: ["nudge", address],
    queryFn: () => fetchNudge(address!),
  });

  const showToast = () => {
    toast("Psssst... got some alpha!", {
      description: nudge?.text,
      duration: 10000,
      action: {
        label: nudge?.cta,
        onClick: () =>
          window.open(
            "https://www.google.com",
            "_blank",
            "noopener,noreferrer"
          ),
      },
      dismissible: true,
    });
  };

  useEffect(() => {
    const timer = setTimeout(() => {
      showToast();
    }, 3000); // 3000 milliseconds = 3 seconds

    return () => clearTimeout(timer);
  }, []);

  return null;
}
