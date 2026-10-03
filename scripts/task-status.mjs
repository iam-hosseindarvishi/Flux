#!/usr/bin/env node
import fs from "node:fs";
import path from "node:path";

const root = process.cwd();
const registryPath = path.join(root, "tasks.json");
const allowed = new Set(["pending", "in_progress", "blocked", "completed"]);
const [, , command, id] = process.argv;
const registry = JSON.parse(fs.readFileSync(registryPath, "utf8"));
const task = registry.tasks.find((item) => item.id === id);

function fail(message) { console.error(`Error: ${message}`); process.exit(1); }
function taskData(item) {
  return JSON.parse(fs.readFileSync(path.join(root, item.file), "utf8"));
}
function save(item, status) {
  const data = taskData(item);
  data.status = status;
  data.updated_at = new Date().toISOString();
  fs.writeFileSync(path.join(root, item.file), JSON.stringify(data, null, 2) + "\n");
  item.status = status;
  registry.updated_at = data.updated_at;
  fs.writeFileSync(registryPath, JSON.stringify(registry, null, 2) + "\n");
  console.log(`${item.id} ${item.title}: ${status}`);
}

if (command === "show") {
  for (const item of registry.tasks) console.log(`${item.id} [${item.status}] ${item.title}`);
  process.exit(0);
}
if (!task) fail("Task ID not found. Use: node scripts/task-status.mjs show");
const next = { start: "in_progress", block: "blocked", complete: "completed" }[command];
if (!next) fail("Usage: start|block|complete|show <task-id>");
if (next === "completed") {
  const incomplete = task.dependencies.filter((dep) => registry.tasks.find((x) => x.id === dep)?.status !== "completed");
  if (incomplete.length) fail(`Dependencies are not complete: ${incomplete.join(", ")}`);
}
save(task, next);
