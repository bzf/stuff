import type { AppProps } from "next/app";
import Link from "next/link";
import { useRouter } from "next/router";

import "../style.css";

export default function MyApp({ Component, pageProps }: AppProps) {
  return (
    <div className="flex h-screen w-screen">
      <div className="bg-gray-100 min-w-[255px] h-full py-4">
        <section className="w-full px-2 flex flex-col gap-2">
          <AppLink href="/inbox">Inbox</AppLink>
          <AppLink href="/today">Today</AppLink>
        </section>
      </div>

      <div className="px-8 flex-1 py-4">
        <div className="max-w-[600px] mx-auto">
          <Component {...pageProps} />
        </div>
      </div>
    </div>
  );
}

function AppLink({ href, children }) {
  const router = useRouter();
  const classes = [
    "w-full",
    "block",
    "rounded",
    "h-8",
    "flex items-center px-3",
    "font-medium",
  ];

  if (router.pathname === href) {
    classes.push("bg-black/10");
  }

  return (
    <Link href={href} legacyBehavior>
      <a className={classes.join(" ")}>{children}</a>
    </Link>
  );
}
