import Link from "next/link";

export default function Publishers() {
  return (
    <section className="py-12 px-4 sm:px-6 lg:px-8 ">
      <div className="mx-auto max-w-4xl rounded-3xl border border-sky-100 bg-sky-50 shadow-sm overflow-hidden">
        <div className="p-6 sm:p-10">
          <h2 className="text-3xl font-bold text-gray-900 mb-6">
            Why Partner with Us
          </h2>

          <div className="space-y-6">
            <div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">
                Enhance User Experience
              </h3>
              <p className="text-gray-600 leading-relaxed">
                Provide your users with{" "}
                <span className="text-sky-600 hover:text-sky-700 font-medium">
                  valuable insights
                </span>{" "}
                and nudge them in the right direction. Whether it’s alerting
                them about unclaimed airdrops, suggesting alternatives to avoid
                MEV sandwiching on a DEX, recommending cost-effective bridges,
                or advising on how to put idle ETH or USDC to good use on Aave,
                our platform delivers{" "}
                <span className="text-sky-600 hover:text-sky-700 font-medium">
                  meaningful recommendations
                </span>{" "}
                based on on-chain data.
              </p>
            </div>

            <div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">
                Add a New Revenue Source
              </h3>
              <p className="text-gray-600 leading-relaxed">
                You have users and plenty of empty space on your website.
                Instead of cluttering it with intrusive web2 ads, why not
                monetize it by working with other web3 dApps? Our platform
                allows you to{" "}
                <span className="text-sky-600 hover:text-sky-700 font-medium">
                  nudge your users towards beneficial actions and opportunities
                </span>{" "}
                , creating a win-win scenario. Enhance their experience with
                relevant messages while generating additional revenue.
              </p>
            </div>

            <div>
              <h3 className="text-xl font-semibold text-gray-900 mb-3">
                Maintain Complete Control
              </h3>
              <p className="text-gray-600 leading-relaxed">
                You have{" "}
                <span className="text-sky-600 hover:text-sky-700 font-medium">
                  100% control over who you choose to promote.
                </span>{" "}
                Customize the integration to fit your needs: call our API to
                design the experience your way, or integrate our soon-to-come
                SDK for even more flexibility.
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
