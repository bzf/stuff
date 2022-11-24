import { markTaskAsComplete, markTaskAsIncomplete } from "../stuff";

export default function TaskItem({ task }) {
  function handleToggle() {
    if (!!task.completedAt) {
      markTaskAsIncomplete(task.id);
    } else {
      markTaskAsComplete(task.id);
    }
  }

  return (
    <div className="py-1 flex gap-3 justify-start items-start">
      <span>
        <input
          type="checkbox"
          onChange={handleToggle}
          checked={!!task.completedAt}
        />
      </span>

      <div>
        <span className="font-medium text-md text-gray-700">{task.title}</span>
      </div>
    </div>
  );
}
