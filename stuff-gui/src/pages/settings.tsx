import { useConfig } from "../stuff";
import PageTitle from "../components/PageTitle";

export default function () {
  const config = useConfig();

  return (
    <>
      <div>
        <div className="flex flex-col pb-4 px-3">
          <PageTitle title="Settings" />
        </div>

        <div className="flex flex-col gap-4 px-3">
          <div className="flex flex-col gap-1">
            <label className="text-sm font-medium">Client ID</label>
            <input
              type="text"
              className="bg-gray-50 text-sm block border px-2 py-1 rounded text-gray-400"
              disabled
              value={config.clientId ?? ""}
            />
          </div>

          <div className="flex flex-col gap-1">
            <label className="text-sm font-medium">Data directory</label>
            <input
              type="text"
              className="bg-gray-50 text-sm block border px-2 py-1 rounded text-gray-400"
              disabled
              value={config.dataDirectory ?? ""}
            />
          </div>
        </div>
      </div>
    </>
  );
}
