import type { Metadata } from "next";
import { Toaster } from "@/components/ui/sonner";
import { Inter } from "next/font/google";
import Footer from "@/components/nav/Footer";
import Navbar from "@/components/nav/Navbar";
import { Providers } from "./providers";
import "@/styles/globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Nudger Demo",
  description: "Demo of Nudger",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <Providers>
          <div className="flex flex-col min-h-screen bg-[#65d9ff]">
            <Navbar />
            <main className="flex-grow relative">{children}</main>
            <Toaster />
            <Footer />
          </div>
        </Providers>
      </body>
    </html>
  );
}
