import { AierMessage, AierSession, GptConfig, Keyshut, Prompter, PrompterType, Step } from "../../views/types/aier";
import { Channel, invoke } from "@tauri-apps/api/core";


export async function savePrompter(pp: Prompter) {
  if (!pp.uuid) {
    pp.uuid = ""
  }

  const p = {
    ...pp,
    nodes: pp.nodes ? JSON.stringify(pp.nodes) : "",
    edges: pp.edges ? JSON.stringify(pp.edges) : ""
  }
  return await invoke('save_prompter', { prompter: p })
}

export async function loadPrompter(): Promise<Prompter[]> {
  const prompters = await invoke('get_prompters') as Array<any>
  const p = prompters.map((p: any) => {
    return {
      ...p,
      nodes: p.nodes ? JSON.parse(p.nodes) : [],
      edges: p.edges ? JSON.parse(p.edges) : []
    }
  })
  return p as Prompter[]
}


export async function deletePrompter(uuid: string): Promise<void> {
  return await invoke('delete_prompter', { uuid })
}


export async function loadMessages(session_uuid: string): Promise<AierMessage[]> {
  return await invoke('get_messages', { sessionUuid: session_uuid })
}



export async function saveMessage(prompter: Prompter, message: AierMessage, channel?: Channel<Step>): Promise<AierMessage> {
  const transPrompter = {
    ...prompter,
    nodes: JSON.stringify(prompter.nodes),
    edges: JSON.stringify(prompter.edges)
  }
  console.log("prompter", prompter)
  switch (prompter.ptype) {
    case PrompterType.CHAT:
      return await invoke('save_message', { prompter: transPrompter, messageItem: message })
    case PrompterType.PY_SCRIPT:
      return await invoke('handle_pyscript_prompter', { prompter: transPrompter, messageItem: message })
    case PrompterType.MULTI_MODELS:
      if (!channel) {
        channel = new Channel()
      }
      try {
        const res = await invoke('handle_multi_prompter', {
          prompter: transPrompter,
          message: message,
          channel: channel
        })
        console.log("multi model res", res)
        return res as AierMessage
      } catch (e) {
        console.log("multi model error", e)
        return message
      }
  }
}


// export async function deleteMessage(message_uuid: string): Promise<void> {
//   return await invoke('delete_message', { message_uuid })
// }

export async function loadSessions(): Promise<AierSession[]> {
  return await invoke('get_sessions')
}

export async function saveSession(session: AierSession): Promise<AierSession> {
  return await invoke('save_session', { session })
}

export async function loadKeyshuts(): Promise<Keyshut[]> {
  return await invoke('get_keyshuts')
}

export async function saveKeyshut(keyshut: Keyshut): Promise<Keyshut> {
  return await invoke('save_keyshut', { keyshut })
}

export async function deleteKeyshut(id: number): Promise<void> {
  return await invoke('delete_keyshut', { id })
}

export async function loadPrompterByUuid(uuid: string): Promise<Prompter> {
  return await invoke('get_prompter', { uuid })
}


export async function getModels(): Promise<GptConfig> {
  return await invoke('get_models')
}
