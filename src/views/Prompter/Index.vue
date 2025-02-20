<script setup lang=ts>
import { ref } from 'vue';
import { loadPrompter, savePrompter, getModels, deletePrompter } from '../../modules/promoter/prompter';
import { GptConfig, Prompter, PrompterType, PrompterTypeMap, SESSION_TYPES } from '../types/aier';
import { ElMessage, ElMessageBox } from 'element-plus';
import { emptyPrompter } from '../types/aier';
import Chat from './Chat.vue'
import MultiPrompt from './MultiPrompt.vue'
const prompts = ref([] as Prompter[])
const editPrompter = ref(emptyPrompter())
const gptConfig = ref({} as GptConfig)

async function refresh() {
  prompts.value = await loadPrompter()
  console.log(prompts.value)
  getGptModels()
}

function clean() {
  editPrompter.value = emptyPrompter()
}

function edit(p: Prompter) {
  editPrompter.value = p
}

async function save() {
  console.log("editPrompter", editPrompter.value)
  if (!editPrompter.value.title) {
    ElMessage.error('aier title is required')
    return
  }
  if (!editPrompter.value.itype) {
    ElMessage.error('prompt type is required')
    return
  }

  switch (editPrompter.value.ptype) {
    case PrompterType.CHAT:
      if (!editPrompter.value.content) {
        ElMessage.error('prompt content is required')
        return
      }
      break
  }

  await savePrompter(editPrompter.value)
  clean()
  refresh()
  ElMessage.success('save success')
}

async function getGptModels() {
  gptConfig.value = await getModels()
  console.log(gptConfig.value)
  if (gptConfig.value.models.length > 0) {
    editPrompter.value.model_name = gptConfig.value.models[0].model
  }
}
async function remove(p: Prompter) {
  ElMessageBox.confirm('Are you sure you want to delete this prompter?', 'Confirm', {
    confirmButtonText: 'Delete',
    cancelButtonText: 'Cancel',
    type: 'warning',
  }).then(async () => {
    await deletePrompter(p.uuid)
    refresh()
  })
}

refresh()

</script>

<template>

  <el-row style="width:100%;" class="nes-container with-title is-centered">
    <div class="title">Prompter</div>
    <button class="nes-btn" style="position:absolute;top: 5px;right:10px;" @click="$router.push('/')">back</button>
    <el-col :span="24">
      <el-row>
        <el-col :span="24">
          <label for="default_select">prompter-type</label>
          <div class="nes-select " style="margin-bottom: 8px;">
            <select required id="default_select" v-model="editPrompter.ptype" placeholder="Prompt Type You Want...">
              <option v-for="v in PrompterTypeMap" :key="v.pType" :value="v.pType"> {{ v.pType }}
              </option>
            </select>
          </div>
        </el-col>
        <div>
          {{ PrompterTypeMap[editPrompter.ptype].intro }}
        </div>
        <el-col :span="24">
          <label for="name_field">title</label>
          <div class="nes-field" style="margin-bottom: 10px;">
            <input v-model="editPrompter.title" placeholder="name your aier" type="text" id="name_field"
              class="nes-input">
          </div>
        </el-col>
        <el-col :span="24">
          <label for="default_select">type</label>
          <div class="nes-select " style="margin-bottom: 8px;">
            <select required id="default_select" v-model="editPrompter.itype" placeholder="Prompt Type You Want...">
              <option v-for="v in SESSION_TYPES" :key="v.key" :value="v.key">{{ v.value }}</option>
            </select>
          </div>
        </el-col>
        <el-col :span="24" v-if="[PrompterType.CHAT, PrompterType.PY_SCRIPT].includes(editPrompter.ptype)">
          <Chat :editPrompter="editPrompter" :key="editPrompter.uuid" />
        </el-col>
        <el-col :span="24" v-if="editPrompter.ptype === PrompterType.MULTI_MODELS">
          <MultiPrompt v-model:edges_lines="editPrompter.edges" v-model:nodes="editPrompter.nodes" />
        </el-col>
      </el-row>
    </el-col>

    <el-col :span="24">
      <button class="nes-btn" @click="save" :disabled="!(editPrompter.title && editPrompter.itype)">{{
        editPrompter.uuid ? 'Update' : 'Save' }}</button>
    </el-col>
    <el-col :span="24">
      <div class="nes-table-responsive">
        <table class="nes-table is-bordered is-centered" style="width: 99%;">
          <thead>
            <tr>
              <th>uuid</th>
              <th>prompter-type</th>
              <th>session-type</th>
              <th>title</th>
              <th>model</th>
              <th>prompt-text</th>
              <th>operation</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="v in prompts" :key="v.uuid">
              <td width="20%">{{ v.uuid }}</td>
              <td width="20%">{{ v.ptype }}</td>
              <td width="20%">{{ v.itype }}</td>
              <td width="20%">{{ v.title }}</td>
              <td width="20%">{{ v.model_name }}</td>
              <td width="20%">{{ v.content.substring(0, 100) }}{{ v.content.length > 100 ? '...' : '' }}</td>
              <td width="20%">
                <div>
                  <button class="nes-btn" @click="edit(v)">Edit</button>
                  <button class="nes-btn is-error" @click="remove(v)">Delete</button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </el-col>

  </el-row>
</template>

<style scoped></style>
