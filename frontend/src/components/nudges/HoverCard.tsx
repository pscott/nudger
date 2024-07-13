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
          <span className="text-sm">Psssst... got some alpha for you 👀</span>
        </HoverCardTrigger>
        <HoverCardContent className="text-sm">
          {nudge?.text}{" "}
          <a
            className="text-blue-900 font-medium"
            href={"https://google.com"}
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn more!
          </a>
        </HoverCardContent>
      </HoverCard>
    );
  }
  return null;
}
