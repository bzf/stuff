import { useRouter } from "next/router";
import PageTitle from "../../components/PageTitle";
import { useProject } from "../../stuff";

export default function Project() {
  const { projectId } = useRouter().query;
  const project = useProject(projectId);

  if (project === undefined) {
    return <div>loading</div>;
  }

  return (
    <>
      <PageTitle title={project.name} />
    </>
  );
}
