import Toast from "@/components/nudges/Toast";
import Swap from "@/components/Swap";
import Image from "next/image";

export default async function Page() {
  return (
    <div className="absolute inset-0 flex flex-col justify-center items-center">
      <div className="z-10 w-full max-w-md px-4">
        <Swap />
      </div>
      <Toast />
      <div className="absolute inset-x-0 bottom-0 overflow-hidden">
        <div className="relative w-[200%] sm:w-full origin-bottom-left">
          <Image
            src="/background-cowswap-lightmode.svg"
            alt="CowSwap background"
            layout="responsive"
            width={1920}
            height={400}
            objectFit="cover"
            objectPosition="left bottom"
          />
        </div>
      </div>
    </div>
  );
}
