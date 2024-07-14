import Link from "next/link";
import Image from "next/image";

export default function About() {
  return (
    <section className="py-12 px-4 sm:px-6 lg:px-8 ">
      <div className="mx-auto max-w-4xl rounded-3xl border border-sky-100 bg-sky-50 shadow-sm overflow-hidden">
        <div className="p-6 sm:p-10">
          <h2 className="text-3xl font-bold text-gray-900 mb-6">
            What Is This?
          </h2>

          <div className="space-y-6 prose lg:prose-xl">
            <div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">
                $5k is a lot of money
              </h3>
              <p className="text-gray-600 leading-relaxed">
                Not long ago, I almost missed out on a $5k airdrop because I
                didn’t know I was eligible. I had not interacted in any way with
                this layer 2,{" "}
                <span className="underline">
                  so I didn’t bother checking my eligibility.
                </span>{" "}
                Turns out they decided to airdrop holders of an NFT collection,
                that I happened to own. By luck, a friend of mine knew I had an
                pengus, so he asked me if I had claimed, and that’s how I
                learned I was actually eligible.{" "}
                <span className="font-semibold">
                  It struck me that, had this friend not warned me, I would NOT
                  have claimed the airdrop.
                </span>
              </p>
              <div className="flex justify-center items-center w-full">
                <Image
                  src="/meme1.jpeg"
                  alt="What Is This?"
                  width={600}
                  height={400}
                />
              </div>
            </div>

            <div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">
                But why?
              </h3>
              <p className="text-gray-600 leading-relaxed">
                Later on that day, while voting on Snapshot, I was shocked that
                no frontend had warned me about my airdrop. After all, it’s all
                public information: the list of eligible wallets is not hidden.
                Moreover, I spend my day connected on different web3 apps that{" "}
                <span className="font-semibold">*know*</span> my address.{" "}
                <span className="font-semibold">
                  So what’s stopping those frontends to notify me whenever I’m
                  missing something as big as this?
                </span>{" "}
                How come Snapshot doesn’t show me “hey, come & claim your $5k”?
              </p>
            </div>

            <div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">
                Users need nudging
              </h3>
              <p className="text-gray-600 leading-relaxed">
                Well, turns out Dapps are not focused on looking at your onchain
                information and finding appropriate recommendations (nudges) for
                you. So that’s what we -{" "}
                <Link href="https://x.com/scottpiriou">
                  <span className="text-sky-600 font-medium">@scottpiriou</span>
                </Link>{" "}
                and{" "}
                <Link href="https://x.com/MaximeServais77">
                  <span className="text-sky-600 font-medium">
                    @MaximeServais77
                  </span>{" "}
                </Link>
                - set ourselves to do for this hackathon. Indeed it’s not only
                about claiming that airdrop that you are not aware of. It could
                also be{" "}
                <span className="font-semibold">
                  nudging users into lending their idle USDC on Aave
                </span>
                . Or{" "}
                <span className="font-semibold">
                  protecting themselves from sandwich attacks by using CowSwap.
                </span>{" "}
                Or even{" "}
                <span className="font-semibold">
                  voting on an important proposal on Snapshot!
                </span>
              </p>
              <div className="flex justify-center items-center w-full">
                <Image
                  src="/meme2.jpeg"
                  alt="What Is This?"
                  width={600}
                  height={400}
                />
              </div>
            </div>

            <div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">
                The demo
              </h3>
              <p className="text-gray-600 leading-relaxed">
                What you are looking at right now is a demo we built for the{" "}
                <Link href="https://ethglobal.com/events/brussels">
                  <span className="text-sky-600 font-medium">
                    EthGlobal Brussels 2024 hackathon
                  </span>
                </Link>
                . A dummy frontend (all static, buttons are disabled) that
                displays a funny nudge based on your connected address. Feel
                free to change wallet and see what the nudge is about. The nudge
                comes from a backend that uses{" "}
                <Link href="https://developers.zerion.io/reference/intro/getting-started">
                  <span className="text-sky-600 font-medium">Zerion’s API</span>
                </Link>{" "}
                to look at your onchain publicly available information and
                recommend something useful.
              </p>
            </div>

            <div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">
                Final words
              </h3>
              <p className="text-gray-600 leading-relaxed">
                The UI here is just an example of an integration. The integrator
                can choose how to display the actual text and Call To Action.
                Feel free to try out different things! If you have any
                questions, please reach out on telegram at{" "}
                <Link href="https://t.me/ttocsp">
                  <span className="text-sky-600 font-medium">@ttocsp</span>
                </Link>
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
