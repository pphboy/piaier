<script setup lang="ts">
import { ref } from "vue";
import { loadPrompter } from "../../modules/promoter/prompter";
import { Prompter } from "../types/aier";
import { loadKeyshuts, saveKeyshut, deleteKeyshut } from "../../modules/promoter/prompter";
import { Keyshut } from "../types/aier";
import { ElMessage } from "element-plus";

const keyShutShow = ref<string>("")
const prompterList = ref<Prompter[]>([])
const prompterMap = ref<Map<string, Prompter>>(new Map())
const keyshutList = ref<Keyshut[]>([])
const editKeyshut = ref<Keyshut>(emptyKeyshut())

function emptyKeyshut(): Keyshut {
  return {
    id: 0,
    keyshut: "",
    prompter_uuid: "",
  }
}

// 管理快捷键
// 加载快捷键
// 重新注册快捷键
function handleKeyDown(event: KeyboardEvent) {
  console.log(event.key, event.ctrlKey, event.shiftKey, event.altKey)
  if (event.ctrlKey || event.shiftKey || event.altKey) {
    keyShutShow.value = `${event.ctrlKey ? "CommandOrControl+" : ""}${event.shiftKey ? "Shift+" : ""}${event.altKey ? "Alt+" : ""}${event.key.toUpperCase()}`
  }
}


async function addKeyshut() {
  if (keyShutShow.value === "") {
    return
  }
  editKeyshut.value.keyshut = keyShutShow.value
  try {
    await saveKeyshut(editKeyshut.value)
    ElMessage.success("Add Key Shortcut Success")
    refresh()
    clean()
  } catch (e) {
    ElMessage.error(e as string)
  }
}

async function removeKeyShut(id: number) {
  await deleteKeyshut(id)
  ElMessage.success("Delete Key Shortcut Success")
  refresh()
}


async function clean() {
  keyShutShow.value = ""
  editKeyshut.value = emptyKeyshut()
}

async function refresh() {
  prompterList.value = await loadPrompter()
  prompterMap.value = new Map(prompterList.value.map(prompter => [prompter.uuid, prompter]))
  keyshutList.value = await loadKeyshuts()


  console.log(prompterMap.value.size)
  if (prompterMap.value.size) {
    editKeyshut.value.prompter_uuid = prompterList.value[0].uuid
    console.log(editKeyshut.value)
  }
}

refresh()

</script>
<template>
  <div style="width: 100%;" class="nes-container">
    <el-row>
      <el-col :span="24">
        <button class="nes-btn" @click="$router.back()">Back</button>
        <button class="nes-btn" @click="addKeyshut">Add Key Shortcut</button>
      </el-col>
      <el-col :span="24">
        <el-row>
          <el-col :span="8">
            <input :value="keyShutShow" type="text" class="nes-input" @keydown="handleKeyDown" />
          </el-col>
          <el-col :span="8">
            <div class="nes-select">
              <select v-model="editKeyshut.prompter_uuid">
                <option v-for="prompter in prompterList" :value="prompter.uuid">{{ prompter.title }}</option>
              </select>
            </div>
          </el-col>

        </el-row>
      </el-col>
      <el-col :span="24">
        <div class="nes-table-responsive">
          <table class="nes-table is-bordered is-centered" style="width: 99%;">
            <thead>
              <tr>
                <th>Key Shortcut</th>
                <th>Prompter</th>
                <th>Action</th>
              </tr>
            </thead>
            <tbody>
              <tr v-for="keyshut in keyshutList">
                <td>{{ keyshut.keyshut }}</td>
                <td>{{ prompterMap.get(keyshut.prompter_uuid)?.title }}</td>
                <td>
                  <button class="nes-btn" @click="removeKeyShut(keyshut.id)">Delete</button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>

      </el-col>
    </el-row>
  </div>
</template>

<style scoped></style>
