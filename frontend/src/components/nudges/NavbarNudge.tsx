import { fetchNudge, INudge } from "@/lib/api";
import { useQuery } from "@tanstack/react-query";

export default function NavbarNudge({ address }: { address: `0x${string}` }) {
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
      <div className="text-md">
        💡 {nudge?.text}{" "}
        <a
          className="text-blue-900 font-medium underline"
          href={nudge?.cta_url}
          target="_blank"
          rel="noopener noreferrer"
        >
          {nudge?.cta_text}
        </a>
      </div>
    );
  }
  return null;
}
