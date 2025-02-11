export interface PrompterStep {
  stepid: number
  prompter_uuid: string
  node_name: string
  cond_var: string
  stype: StepType
}


export enum SessionType {
  LONG = 'LONG',
  TEMP = "TEMP",
}
export interface NodeEdges {
  ChildsType: StepType
  Childs: PrompterStep[]
}

export interface AnyKV<T> {
  key: string
  value: T
  intro: string
}

export const SESSION_TYPES: AnyKV<SessionType>[] = [{
  key: SessionType.LONG,
  value: SessionType.LONG,
  intro: '长会话',
}, {
  key: SessionType.TEMP,
  value: SessionType.TEMP,
  intro: '临时会话',
}]

export interface AierSession {
  uuid: string
  prompter_uuid: string
  session_type: SessionType
  session_title: string
}

export enum MessageType {
  SYSTEM = 'SYSTEM',
  USER = 'USER',
  ASSISTANT = 'ASSISTANT',
}
export interface Step {
  uuid: string,
  content: string,
  value: string,
  ok: boolean,
  msg: string,
  time_secs: number,
}

export interface AierMessage {
  id: number
  session_uuid: string
  order: number
  itype: MessageType
  content: string
}

export interface Keyshut {
  id: number
  keyshut: string
  prompter_uuid: string
}

export interface Prompter {
  uuid: string
  title: string
  itype: SessionType
  content: string
  ptype: PrompterType

  model_name: string // 为空就是使用默认模型

  steps: PrompterStep[]

  edges: Edges[]
  nodes: Node[]
}


export interface Node {
  id: string
  type: string
  initialized: boolean
  position: {
    x: number
    y: number
  }
  data: {
    label: string
    data?: any
  }
  class?: string
}


export interface Edges {
  id: string
  type: string
  source: string
  target: string
  sourceX?: number
  sourceY?: number
  targetX?: number
  targetY?: number
  updatable?: boolean
  data?: {
    stepid?: number
    prompter_uuid?: string
    node_name?: string
    cond_var?: string
    stype?: string
  }
  label?: string
}

export enum PrompterType {
  // 正常的Promtper
  CHAT = 'CHAT', // 默认
  // 脚本
  PY_SCRIPT = 'PY_SCRIPT', // python脚本执行的Prompter，需要遵循严格模式
  // 多模态合成的Prompter
  MULTI_MODELS = 'MULTI_MODELS',
}

export const PrompterTypeArr = [
  PrompterType.CHAT,
  PrompterType.PY_SCRIPT,
  PrompterType.MULTI_MODELS,
]

export const PrompterTypeMap = {
  [PrompterType.CHAT]: {
    pType: PrompterType.CHAT,
    intro: '默认的Prompter，使用默认模型',
  },
  [PrompterType.PY_SCRIPT]: {
    pType: PrompterType.PY_SCRIPT,
    intro: 'python脚本执行的Prompter，需要遵循严格模式',
  },
  [PrompterType.MULTI_MODELS]: {
    pType: PrompterType.MULTI_MODELS,
    intro: '多模态合成的Prompter，由几个模型合成，使用前需要先创建其他的Prompter，然后由其他Promtper组成本Prompter',
  },
}

export function emptySession(): AierSession {
  return {
    uuid: '',
    prompter_uuid: '',
    session_type: SessionType.LONG,
    session_title: '',
  }
}

export function emptyMessage(): AierMessage {
  return {
    id: 0,
    session_uuid: '',
    order: 0,
    itype: MessageType.USER,
    content: '',
  }
}
export function emptyPrompter(): Prompter {
  return {
    uuid: '',
    title: '',
    itype: SessionType.TEMP,
    content: '',
    ptype: PrompterType.CHAT,
    edges: [],
    nodes: [],
    model_name: '',
    steps: [],
  }
}


export interface GptModel {
  model: string;
  url: string;
  api_token: string;
}

export interface GptConfig {
  models: GptModel[];
}

export interface GptMessage {
  role: string;
  content: string;
}

export enum StepType {
  NORMAL = 'NORMAL', // 普通的到步骤执行的Model
  SPEC_VAR = "SPEC_VAR", // 特殊的变量，只有 有此变量时，才会执行
}
export const StepTypeArr = [
  StepType.NORMAL,
  StepType.SPEC_VAR,
]

export function emptyStep(): PrompterStep {
  return {
    stepid: 0,
    prompter_uuid: '',
    node_name: '',
    cond_var: '',
    stype: StepType.NORMAL,
  }
}

