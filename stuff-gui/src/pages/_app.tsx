import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import type { AppProps } from "next/app";
import Link from "next/link";
import { useRouter } from "next/router";
import { useProjects } from "../stuff";

import "../style.css";
import { faHeart } from "@fortawesome/free-regular-svg-icons";

export default function MyApp({ Component, pageProps }: AppProps) {
  const projects = useProjects();

  return (
    <div className="flex h-screen w-screen overflow-hidden">
      <div className="bg-gray-50 min-w-[255px] h-full py-4 px-2 flex flex-col gap-6 overflow-y-auto">
        <section className="w-full flex flex-col gap-2">
          <AppLink icon={faHeart} href="/inbox">
            Inbox
          </AppLink>
          <AppLink icon={faHeart} href="/today">
            Today
          </AppLink>
        </section>

        <section className="w-full flex flex-col gap-2">
          {projects.map((project) => (
            <AppLink
              key={project.id}
              icon={faHeart}
              href={`/projects/${project.id}`}
            >
              {project.name}
            </AppLink>
          ))}
        </section>
      </div>

      <div className="px-8 flex-1 py-4 overflow-y-auto">
        <div className="max-w-[600px] mx-auto flex flex-col gap-4">
          <Component {...pageProps} />
        </div>
      </div>
    </div>
  );
}

function AppLink({ icon, href, children }) {
  const router = useRouter();
  const classes = [
    "w-full",
    "block",
    "rounded-lg",
    "h-8",
    "flex items-center px-3",
    "font-medium",
    "flex",
    "items-center",
    "gap-2",
  ];

  if (router.asPath === href) {
    classes.push("bg-black/[6%]");
  }

  return (
    <Link href={href} legacyBehavior>
      <a className={classes.join(" ")}>
        <FontAwesomeIcon fixedWidth size="xs" icon={icon} />
        {children}
      </a>
    </Link>
  );
}
