import _ from "lodash";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faNoteSticky } from "@fortawesome/free-regular-svg-icons";
import { markTaskAsComplete, markTaskAsIncomplete } from "../stuff";
import { useState } from "react";
import dayjs from "dayjs";

export default function TaskItem({ task, ...rest }) {
  const [showNotes, setShowNotes] = useState(false);

  function handleToggle() {
    if (!!task.completedAt) {
      markTaskAsIncomplete(task.id);
    } else {
      markTaskAsComplete(task.id);
    }
  }

  return (
    <div
      {...rest}
      data-task-id={task.id}
      data-heading-id={task.projectHeadingId}
      className="py-1 flex gap-3 justify-start items-start cursor-default select-none"
    >
      <span>
        <input
          type="checkbox"
          onChange={handleToggle}
          checked={!!task.completedAt}
        />
      </span>

      <div className="flex flex-col gap-1 w-full">
        <div className="flex w-full gap-2 items-center">
          {task.deferDate ? <DeferDateLabel date={task.deferDate} /> : null}

          <div className="flex items-center gap-2">
            <span className="font-medium text-md text-gray-700">
              {task.title}
            </span>
            {_.isEmpty(task.description) ? null : (
              <button
                onClick={() => setShowNotes(!showNotes)}
                className="text-gray-400 hover:text-gray-500 cursor-default"
              >
                <FontAwesomeIcon icon={faNoteSticky} />
              </button>
            )}
          </div>
        </div>

        {showNotes ? (
          <div className="text-sm text-gray-700">{task.description}</div>
        ) : null}
      </div>
    </div>
  );
}

function DeferDateLabel({ date }) {
  const dateString = dayjs(date).format("D MMM");

  return (
    <time className="text-sm bg-gray-200 px-2 rounded-lg font-semibold text-gray-500">
      {dateString}
    </time>
  );
}
