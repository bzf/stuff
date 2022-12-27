import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import type { AppProps } from "next/app";
import Link from "next/link";
import { useRouter } from "next/router";
import {
  moveAreaToPosition,
  moveProjectToArea,
  moveProjectToPosition,
  useAreas,
  useProjects,
} from "../stuff";
import { isEmpty } from "lodash";

import "../style.css";
import { faClock, faHeart } from "@fortawesome/free-regular-svg-icons";
import {
  faBox,
  faCog,
  faInbox,
  faPlus,
} from "@fortawesome/free-solid-svg-icons";
import { ReactSortable } from "react-sortablejs";

export default function MyApp({ Component, pageProps }: AppProps) {
  const projects = useProjects();
  const areas = useAreas();

  function handleAreaMove(event) {
    const { newIndex, item, to } = event;
    const { id: targetAreaId } = to;
    const { projectId, areaId } = item.dataset;

    moveProjectToPosition(projectId, newIndex);

    if (areaId !== targetAreaId) {
      moveProjectToArea(projectId, targetAreaId);
    }
  }

  function handleAreaMove(event) {
    const { newIndex, item } = event;
    const { areaId } = item.dataset;

    moveAreaToPosition(areaId, newIndex);
  }

  return (
    <div className="flex h-screen w-screen overflow-hidden">
      <div className="bg-gray-50 min-w-[255px] h-full py-4 px-2 flex justify-between flex-col">
        <div className="flex flex-col gap-3 flex-1 overflow-y-auto pb-8">
          <section className="w-full flex flex-col gap-1">
            <AppLink icon={faInbox} href="/inbox">
              Inbox
            </AppLink>
          </section>

          <section className="w-full flex flex-col gap-1">
            <AppLink icon={faClock} href="/today">
              Today
            </AppLink>

            <AppLink icon={faCog} href="/settings">
              Settings
            </AppLink>
          </section>

          <section className="w-full flex flex-col gap-1">
            <ReactSortable
              group="projects"
              animation={200}
              delay={2}
              list={projects.filter((p) => isEmpty(p.areaId))}
              setList={() => null}
              onEnd={handleAreaMove}
            >
              {projects
                .filter((p) => isEmpty(p.areaId))
                .map((project) => (
                  <AppLink
                    projectId={project.id}
                    key={project.id}
                    icon={faHeart}
                    href={`/projects/${project.id}`}
                    empty={_.isEmpty(project.name)}
                  >
                    {project.name || "New project"}
                  </AppLink>
                ))}
            </ReactSortable>
          </section>

          <ReactSortable
            className="flex flex-col gap-3"
            group="areas"
            animation={200}
            delay={2}
            list={areas}
            setList={() => null}
            onEnd={handleAreaMove}
          >
            {areas.map((area) => (
              <section
                key={area.id}
                data-area-id={area.id}
                className="w-full flex flex-col"
              >
                <AppLink
                  key={area.id}
                  areaId={area.id}
                  empty={isEmpty(area.name)}
                  icon={faBox}
                  href={`/areas/${area.id}`}
                >
                  {area.name || "New area"}
                </AppLink>

                <ReactSortable
                  group="projects"
                  animation={200}
                  id={area.id}
                  delay={2}
                  list={projects.filter((p) => p.areaId == area.id)}
                  setList={() => null}
                  onEnd={handleAreaMove}
                >
                  {projects
                    .filter((p) => p.areaId === area.id)
                    .map((project) => (
                      <AppLink
                        projectId={project.id}
                        areaId={area.id}
                        key={project.id}
                        icon={faHeart}
                        href={`/projects/${project.id}`}
                        empty={isEmpty(project.name)}
                      >
                        {project.name || "New project"}
                      </AppLink>
                    ))}
                </ReactSortable>
              </section>
            ))}
          </ReactSortable>
        </div>

        <div className="flex flex-col text-sm">
          <AppLink icon={faPlus} href={`/projects/new`}>
            New project
          </AppLink>

          <AppLink icon={faPlus} href={`/areas/new`}>
            New area
          </AppLink>
        </div>
      </div>

      <div className="px-8 flex-1 py-4 overflow-y-auto">
        <div className="max-w-[600px] mx-auto flex flex-col gap-4">
          <Component {...pageProps} />
        </div>
      </div>
    </div>
  );
}

function AppLink({ icon, href, children, empty, projectId, areaId }) {
  const router = useRouter();
  const classes = [
    "w-full",
    "block",
    "rounded-md",
    "h-6",
    "flex items-center px-3",
    "font-medium",
    "text-sm",
    "flex",
    "items-center",
    "gap-2",
  ];

  if (router.asPath === href) {
    classes.push("bg-black/[6%]");
  }

  if (empty) {
    classes.push("text-gray-400");
  }

  return (
    <Link href={href} legacyBehavior>
      <a
        className={classes.join(" ")}
        data-project-id={projectId}
        data-area-id={areaId}
      >
        <FontAwesomeIcon fixedWidth size="xs" icon={icon} />
        {children}
      </a>
    </Link>
  );
}
