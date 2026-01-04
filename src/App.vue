<script setup lang="ts">
import { ref, nextTick, watch, onMounted } from 'vue';
import { writeTextFile, readTextFile, exists, mkdir } from '@tauri-apps/plugin-fs';
import { appDataDir, BaseDirectory } from '@tauri-apps/api/path';

const DATA_FILE = 'tasks.json';

// Define the structure of a task
interface Task {
  id: number;
  text: string;
  completed: boolean;
}

// Reactive state for the list of tasks - starts empty now
const tasks = ref<Task[]>([]);

// --- Persistence Functions ---
const saveTasks = async () => {
  try {
    await mkdir(DATA_FILE, { baseDir: BaseDirectory.AppData });
  } catch {
    // Directory already exists, ignore
  }
  await writeTextFile(DATA_FILE, JSON.stringify(tasks.value), { baseDir: BaseDirectory.AppData });
};

const loadTasks = async () => {
  try {
    const content = await readTextFile(DATA_FILE, { baseDir: BaseDirectory.AppData });
    const loadedTasks = JSON.parse(content);
    if (Array.isArray(loadedTasks)) {
      tasks.value = loadedTasks;
    }
  } catch {
    // File doesn't exist yet, start fresh
    tasks.value = [];
  }
};

// Watch for changes in tasks and save them
watch(tasks, saveTasks, { deep: true });

// Load tasks when the component is mounted
onMounted(loadTasks);


// Reactive state for the new task input
const newTaskText = ref('');

// --- Editing State ---
const editingTask = ref<Task | null>(null);
const beforeEditText = ref('');

// Function to add a new task
const addTask = () => {
  const text = newTaskText.value.trim();
  if (text) {
    tasks.value.unshift({
      id: Date.now(),
      text,
      completed: false,
    });
    newTaskText.value = '';
  }
};

// --- Editing Functions ---
const editTask = async (task: Task) => {
  beforeEditText.value = task.text;
  editingTask.value = task;
  await nextTick();
  // Focus the input element
  const input = document.getElementById(`edit-${task.id}`);
  input?.focus();
};

const doneEdit = (task: Task) => {
  if (!editingTask.value) return;
  task.text = task.text.trim();
  editingTask.value = null;
  if (!task.text) {
    deleteTask(task.id);
  }
};

const cancelEdit = (task: Task) => {
  if (!editingTask.value) return;
  task.text = beforeEditText.value;
  editingTask.value = null;
};

const deleteTask = (id: number) => {
  tasks.value = tasks.value.filter((t) => t.id !== id);
};
</script>

<template>
  <div class="container" data-tauri-drag-region>
    <div class="task-list">
      <div v-for="task in tasks" :key="task.id" class="task-item" :class="{ completed: task.completed, editing: task === editingTask }">
        <input type="checkbox" v-model="task.completed" />
        <span v-if="task !== editingTask" @dblclick="editTask(task)">{{ task.text }}</span>
        <input
          v-else
          type="text"
          :id="`edit-${task.id}`"
          v-model="task.text"
          @blur="doneEdit(task)"
          @keydown.enter="doneEdit(task)"
          @keydown.esc="cancelEdit(task)"
        />
      </div>
    </div>
    <div class="input-area">
      <input
        type="text"
        placeholder="> 在此输入新任务，按回车添加..."
        v-model="newTaskText"
        @keydown.enter="addTask"
      />
    </div>
  </div>
</template>

<style>
/* Global styles for transparency */
html,
body {
  background-color: transparent;
  margin: 0;
  padding: 0;
}

:root {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica,
    Arial, sans-serif;
  font-size: 14px;
  color: #f0f0f0;
  text-shadow: 0 0 5px rgba(0, 0, 0, 0.4);
}

/* App container */
.container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  box-sizing: border-box;
  padding: 15px;
  overflow: hidden;
  border-radius: 10px;
}

/* Scrollbar styling */
.task-list::-webkit-scrollbar {
  width: 6px;
}
.task-list::-webkit-scrollbar-track {
  background: transparent;
}
.task-list::-webkit-scrollbar-thumb {
  background: rgba(255, 255, 255, 0.2);
  border-radius: 3px;
}
.task-list::-webkit-scrollbar-thumb:hover {
  background: rgba(255, 255, 255, 0.3);
}

/* Area for the list of tasks */
.task-list {
  flex-grow: 1;
  overflow-y: auto;
  padding-right: 5px; /* space for scrollbar */
}

.task-item {
  display: flex;
  align-items: center;
  padding: 8px 4px;
  font-size: 16px;
  border-radius: 5px;
  transition: background-color 0.2s ease-in-out;
}

.task-item:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.task-item span {
  cursor: pointer;
  flex-grow: 1;
}

.task-item.completed span {
  text-decoration: line-through;
  color: rgba(255, 255, 255, 0.4);
}

/* Custom Checkbox */
.task-item input[type='checkbox'] {
  appearance: none;
  -webkit-appearance: none;
  min-width: 18px;
  width: 18px;
  height: 18px;
  border: 1px solid rgba(255, 255, 255, 0.4);
  border-radius: 5px;
  margin-right: 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  outline: none;
  transition: background-color 0.2s, border-color 0.2s;
}

.task-item input[type='checkbox']:checked {
  background-color: rgba(255, 255, 255, 0.2);
  border-color: rgba(255, 255, 255, 0.5);
}

.task-item input[type='checkbox']:checked::after {
  content: '✔';
  font-size: 12px;
  color: rgba(255, 255, 255, 0.8);
}

/* Editing styles */
.task-item input[type='text'] {
  width: 100%;
  padding: 4px 6px;
  font-size: 16px;
  font-family: inherit;
  border: 1px solid #747bff;
  border-radius: 5px;
  background-color: rgba(0, 0, 0, 0.3);
  color: #fff;
  outline: none;
}
.task-item.editing {
  padding: 5px 2px; /* Adjust padding for editing */
}

/* Bottom input area */
.input-area {
  flex-shrink: 0;
  padding-top: 10px;
}

.input-area input {
  width: 100%;
  padding: 12px;
  box-sizing: border-box;
  border: 1px solid rgba(255, 255, 255, 0.2);
  background-color: rgba(0, 0, 0, 0.3);
  color: #fff;
  border-radius: 8px;
  outline: none;
  transition: background-color 0.2s, border-color 0.2s;
}

.input-area input:focus {
  background-color: rgba(0, 0, 0, 0.4);
  border-color: rgba(255, 255, 255, 0.4);
}

.input-area input::placeholder {
  color: rgba(255, 255, 255, 0.4);
}
</style>