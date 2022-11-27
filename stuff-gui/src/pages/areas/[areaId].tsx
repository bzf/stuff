import { useRouter } from "next/router";
import _ from "lodash";
import PageTitle from "../../components/PageTitle";
import { useArea, useTasks } from "../../stuff";

export default function Area() {
  const { areaId } = useRouter().query;
  const area = useArea(areaId);
  const tasks = useTasks();

  if (tasks === undefined || area === undefined) {
    return <div>loading</div>;
  }

  return (
    <>
      <div>
        <div className="pb-2">
          <div className="flex flex-col pb-4 px-3">
            <PageTitle title={area.name} />
          </div>
        </div>
      </div>
    </>
  );
}
