<script setup lang=ts>
import Message from "../../components/Message.vue"
import { loadPrompter, loadMessages, loadSessions, saveMessage, saveSession } from "../../modules/promoter/prompter";
import { AierMessage, AierSession, Node, Prompter, PrompterType, SessionType, Step } from "../../views/types/aier";
import { computed, ref, watch } from "vue";
import { MessageType } from "../types/aier";
import { v4 as uuidv4 } from "uuid"
import { ElMessage } from "element-plus";
import { Channel } from "@tauri-apps/api/core";
import { Files } from "@element-plus/icons-vue";

const prompters = ref<Prompter[]>([])
const sessions = ref<AierSession[]>([])
const messages = ref<AierMessage[]>([])
const nodes = ref<Map<string, Node>>(new Map())
const loading = ref(false)

const promoterMap = ref<Map<string, Prompter>>(new Map())

async function refresh() {
  prompters.value = await loadPrompter()
  sessions.value = await loadSessions()
  console.log("sessions", sessions.value)
  console.log("usingSession.value.uuid", usingSession.value.uuid)

  if (usingSession.value.uuid != "") {
    messages.value = await loadMessages(usingSession.value.uuid)
    console.log("messages", messages.value)
  }
  console.log("prompters", prompters.value)
  promoterMap.value = new Map(prompters.value.map(p => [p.uuid, p]))
}

async function clean() {
  sessions.value = []

  usingSession.value = {
    uuid: "",
    prompter_uuid: "",
    session_type: SessionType.LONG,
    session_title: "",
  }
  messages.value = []

  sendingMsg.value = {
    session_uuid: "",
    order: 0,
    itype: MessageType.USER,
    content: "",
    id: 0,
  }
}

const usingSession = ref<AierSession>({
  uuid: "",
  prompter_uuid: "",
  session_type: SessionType.LONG,
  session_title: "",
})

const sendingMsg = ref<AierMessage>({
  id: 0,
  session_uuid: "",
  order: 0,
  itype: MessageType.USER,
  content: "",
})

const steps = ref<Step[]>([])

async function send() {
  steps.value = []

  if (sendingMsg.value.content == "") {
    return
  }

  if (usingSession.value.prompter_uuid == "") {
    if (prompters.value.length == 0) {
      ElMessage.error("Please create aier first")
      return
    }
    ElMessage.error("Please select aier")
    return
  }

  console.log("sendingMsg", sendingMsg.value)

  loading.value = true
  try {
    if (usingSession.value.uuid == "") {
      usingSession.value.uuid = uuidv4()
      // usingSession.value.prompter_uuid = prompters.value[0].uuid
      usingSession.value.session_title = sendingMsg.value.content.substring(0, 15)
      await saveSession(usingSession.value)
    }

    sendingMsg.value.session_uuid = usingSession.value.uuid

    sendingMsg.value.order = messages.value.length
    sendingMsg.value.itype = MessageType.USER

    const prompter = promoterMap.value.get(usingSession.value.prompter_uuid)!
    let chanl: Channel<Step> | undefined = undefined
    if (prompter.ptype == PrompterType.MULTI_MODELS) {
      chanl = new Channel<Step>()

      chanl.onmessage = (step) => {
        console.log("step", step)
        steps.value.push(step)
      }

    }
    await saveMessage(prompter, sendingMsg.value, chanl)

    messages.value = await loadMessages(usingSession.value.uuid)
    console.log("messages", messages.value)
    sendingMsg.value.content = ""
  } catch (error) {
    console.error(error)
    ElMessage.error("Send message failed")
  } finally {
    loading.value = false
  }
}
async function sendEvent(e: KeyboardEvent) {
  if (e.ctrlKey && e.key == "Enter") {
    await send()
  }
}

watch(usingSession, async () => {
  if (usingSession.value.uuid == "") {
    return
  }

  messages.value = await loadMessages(usingSession.value.uuid)
  sendingMsg.value.session_uuid = usingSession.value.uuid
  sendingMsg.value.order = messages.value.length

})

async function prompterChooseEvent() {
  const prompter = promoterMap.value.get(usingSession.value.prompter_uuid)!
  console.log("choose prompter", prompter)
  if (prompter.ptype == PrompterType.MULTI_MODELS) {
    prompter.nodes.forEach(v => {
      nodes.value.set(v.id, v)
    })
    console.log("nodesMap", nodes.value)
  }
}

refresh()

async function newSession() {
  await clean()
  await refresh()
}




</script>

<template>
  <el-row class="mh-over">
    <!-- <el-col :span="6" class="mh-over">
      <div class="nes-container with-title ">
        <p class="title">History</p>
        <div>
          <button v-for="s in sessions" type="button" class="nes-btn" style="width:100%;margin-bottom: 10px;"
            @click="intoSession(s)">{{ s.session_title }}</button>
        </div>
      </div>
    </el-col> -->
    <el-col :span="24" class="mh-over">
      <el-row>
        <el-col :span="24">
          <el-row>
            <el-col :span="24">
              <div class="nes-select" v-if="!usingSession.uuid">
                <select required id="default_select" v-model="usingSession.prompter_uuid" @change="prompterChooseEvent">
                  <option :value="usingSession.prompter_uuid" disabled selected hidden>Choose Aier</option>
                  <option v-for="p in prompters" :key="p.uuid" :value="p.uuid">{{ p.title }}</option>
                </select>
              </div>

            </el-col>
            <el-col :span="24">
              <div class="nes-select">
                <select required id="history_select" v-model="usingSession">
                  <option :value="usingSession" disabled selected hidden>Choose History</option>
                  <option v-for="s in sessions" :key="s.uuid" :value="s">{{ s.session_title }}</option>
                </select>
              </div>
            </el-col>

          </el-row>
        </el-col>
        <el-col :span="4">
        </el-col>

      </el-row>
      <br>
      <el-col :span="24">
        <el-row>
          <el-col :span="24">
            <section class="nes-container" style="min-height: 62vh;overflow-y: auto;">
              <section class="message-list">
                <Message v-if="!usingSession.uuid" message="New Session" direction="left"></Message>
                <div v-for="(m, key) in messages" :key="key">
                  <Message v-if="m.itype != MessageType.SYSTEM"
                    :direction="m.itype == MessageType.USER ? 'right' : 'left'" :message="m.content"></Message>
                </div>
              </section>
            </section>
          </el-col>
          <div style="height: 62vh;position:absolute;right: 10px;">
            <el-steps direction="vertical" :active="steps.length" finish-status="success">
              <el-step v-for="v in steps" :key="v.uuid" :title="nodes.get(v.uuid)?.data.label" />
            </el-steps>
          </div>
        </el-row>
      </el-col>
      <el-col :span="24">
        <el-row v-loading="loading" element-loading-text="Answering...">
          <el-col :span="24">
            <el-row>
              <el-col :span=24>
                <textarea @keydown="sendEvent" placeholder="Write Message for Aier, Ctrl+Enter to send"
                  id="textarea_field" class="nes-textarea" v-model="sendingMsg.content"></textarea>
              </el-col>
              <el-col :span="24">
                <div style="display: flex;justify-content: space-between;">
                  <div>
                    <button type="button" class="nes-btn" @click="$router.push('/prompter')">Prompt</button>
                    <button type="button" class="nes-btn" @click="newSession">NewSession</button>
                  </div>
                  <div>
                    <button type="button" class="nes-btn">
                      <el-icon>
                        <Files />
                      </el-icon>
                    </button>
                    <button type="button" @click="send" class="nes-btn">Send</button>
                  </div>
                </div>
              </el-col>
            </el-row>
          </el-col>
        </el-row>
      </el-col>
    </el-col>

  </el-row>
</template>

<style scoped></style>