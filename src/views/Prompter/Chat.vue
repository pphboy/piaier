<script setup lang="ts">
import { ref, watch } from 'vue'
import { GptConfig, Prompter, } from "../types/aier"
import { getModels } from '../../modules/promoter/prompter'

const gptConfig = ref({} as GptConfig)

const props = defineProps<{
  editPrompter: Prompter
}>()

async function refresh() {
  gptConfig.value = await getModels()
}

const editPrompter = ref(props.editPrompter)

watch(props.editPrompter, () => {
  editPrompter.value = props.editPrompter
  console.log(editPrompter.value)
  refresh()
})


console.log(editPrompter.value)

refresh()

</script>
<template>

  <el-col :span="24">
    <label for="default_select">model</label>
    <div class="nes-select " style="margin-bottom: 8px;">
      <select required id="default_select" v-model="editPrompter.model_name" placeholder="Prompt Type You Want...">
        <option v-for="v in gptConfig.models" :key="v.model" :value="v.model"> {{ v.model }}
        </option>
      </select>
    </div>
  </el-col>
  <el-col :span="24">
    <textarea v-model="editPrompter.content" placeholder="Prompt Content" id="textarea_field"
      class="nes-textarea"></textarea>
  </el-col>
</template>
<style scoped></style>