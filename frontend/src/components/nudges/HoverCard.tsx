import { fetchNudge, INudge } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card";

export default function HoverCardNudge({
  address,
}: {
  address: `0x${string}`;
}) {
  const {
    data: nudge,
    isLoading,
    isError,
  } = useQuery<INudge, Error>({
    queryKey: ["nudge", address],
    queryFn: () => fetchNudge(address),
  });

  if (isLoading) return null;
  else if (isError) return null;
  else if (nudge) {
    return (
      <HoverCard>
        <HoverCardTrigger>
          <span className="text-md">
            Psssst... got some alpha for you 👀. Scratch me!
          </span>
        </HoverCardTrigger>
        <HoverCardContent className="text-sm rounded-2xl">
          💡 {nudge?.text}{" "}
          <a
            className="text-blue-900 font-medium underline"
            href={nudge?.cta_url}
            target="_blank"
            rel="noopener noreferrer"
          >
            {nudge?.cta_text}
          </a>
        </HoverCardContent>
      </HoverCard>
    );
  }
  return null;
}
