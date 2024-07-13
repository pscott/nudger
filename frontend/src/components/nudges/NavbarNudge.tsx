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
      <div className="text-sm">
        {nudge?.text} <span className="mr-1">→</span>{" "}
        <a
          className="text-blue-900 font-medium"
          href={"https://google.com"}
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn more!
        </a>
      </div>
    );
  }
  return null;
}
