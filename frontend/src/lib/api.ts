export interface Nudge {
  protocol: string;
  text: string;
  cta: string;
  targets: string[];
}

export async function fetchNudge(address: `0x${string}`): Promise<Nudge> {
  try {
    const res = await fetch(
      `${process.env.NEXT_PUBLIC_BACKEND_URL}/get-nudge`,
      {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({
          target: address,
        }),
      }
    );

    if (!res.ok) {
      throw new Error(`HTTP error! status: ${res.status}`);
    }

    return await res.json();
  } catch (error) {
    console.error("Failed to fetch data:", error);
    throw error;
  }
}
