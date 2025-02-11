<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import { AierMessage, MessageType, AierSession, emptyMessage, emptySession, Prompter, emptyPrompter } from '../types/aier';
import { v4 as uuidv4 } from 'uuid';
import { saveSession, loadPrompterByUuid, saveMessage, loadMessages } from '../../modules/promoter/prompter';
import { useRoute } from 'vue-router';
import Message from '../../components/Message.vue';
const route = useRoute()
const puuid = route.query.puuid as string

const editorMessage = ref<AierMessage>(emptyMessage())
const usingSession = ref<AierSession>(emptySession())
const thisPrompter = ref<Prompter>(emptyPrompter())
const loading = ref<boolean>(false)
const messageList = ref<AierMessage[]>([])


async function initSession() {
  if (usingSession.value.uuid == "") {
    usingSession.value.uuid = uuidv4()
    usingSession.value.prompter_uuid = puuid
    usingSession.value.session_title = editorMessage.value.content.substring(0, 15)
    await saveSession(usingSession.value)
  }
}

async function sendMsg() {
  loading.value = true
  initSession()
  editorMessage.value.session_uuid = usingSession.value.uuid
  editorMessage.value.order = messageList.value.length + 1
  await saveMessage(thisPrompter.value, editorMessage.value)
  console.log(editorMessage.value)
  refreshMessages()
  await clean()
  loading.value = false
}

async function refresh() {
  thisPrompter.value = await loadPrompterByUuid(puuid)
  console.log("thisPrompter", thisPrompter.value)
}


async function refreshMessages() {
  messageList.value = await loadMessages(usingSession.value.uuid)
  console.log("messageList", messageList.value)

}

async function clean() {
  editorMessage.value.content = ""
  await focusInput()
}

async function focusInput() {
  document.getElementById('input-text')?.focus()
}

async function keyDown(e: KeyboardEvent) {
  if (e.key == "Enter" && e.ctrlKey) {
    sendMsg()
  }
}

const messageListCmp = computed(() => {
  return messageList.value.filter(m => m.itype == MessageType.ASSISTANT).sort((a, b) => b.order - a.order)
})

refresh()
onMounted(async () => {
  focusInput()
})

</script>
<template>
  <el-row style="width:100%;" v-loading="loading" element-loading-text="Loading...">
    <el-col :span="24">
      <textarea id="input-text" placeholder="Write Message for Aier, Ctrl+Enter to send" class="nes-input" type="text"
        v-model="editorMessage.content" @keyup="keyDown"></textarea>
    </el-col>
    <el-col :span="24">
      <div class="nes-container" v-if="messageListCmp.length">
        <div class="message-list">
          <div v-for=" (m, key) in messageListCmp" :key="key">
            <Message direction="left" :message="m.content" />
          </div>
        </div>
      </div>
    </el-col>
  </el-row>
</template>

<style scoped></style>
