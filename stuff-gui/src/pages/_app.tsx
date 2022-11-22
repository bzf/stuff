import type { AppProps } from "next/app";
import Link from "next/link";
import { useRouter } from "next/router";
import { useProjects } from "../stuff";

import "../style.css";

export default function MyApp({ Component, pageProps }: AppProps) {
  const projects = useProjects();

  return (
    <div className="flex h-screen w-screen">
      <div className="bg-gray-50 min-w-[255px] h-full py-4 px-2 flex flex-col gap-6">
        <section className="w-full flex flex-col gap-2">
          <AppLink href="/inbox">Inbox</AppLink>
          <AppLink href="/today">Today</AppLink>
        </section>

        <section className="w-full flex flex-col gap-2">
          {projects.map((project) => (
            <AppLink key={project.id} href={`/projects/${project.id}`}>
              {project.name}
            </AppLink>
          ))}
        </section>
      </div>

      <div className="px-8 flex-1 py-4">
        <div className="max-w-[600px] mx-auto flex flex-col gap-4">
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

  if (router.asPath === href) {
    classes.push("bg-black/[6%]");
  }

  return (
    <Link href={href} legacyBehavior>
      <a className={classes.join(" ")}>{children}</a>
    </Link>
  );
}
