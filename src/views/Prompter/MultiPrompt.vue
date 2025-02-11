<script setup lang="ts">
import { ref, watch } from 'vue'
import { EdgeChange, EdgeUpdateEvent, VueFlow, useVueFlow } from '@vue-flow/core'
import { Background } from '@vue-flow/background'
import { ControlButton, Controls } from '@vue-flow/controls'
import { MiniMap } from '@vue-flow/minimap'
import { Node as FlowNode, Edge as FlowEdge } from '@vue-flow/core'
import Icon from './Icon.vue'
import { CirclePlus } from '@element-plus/icons-vue'
import { StepTypeArr, emptyStep, StepType } from '../types/aier'
import { loadPrompter } from '../../modules/promoter/prompter'
import { ElMessage } from 'element-plus'
import { v4 as uuidv4 } from 'uuid'
import { Prompter, PrompterStep, Node as AierNode, Edges as AierEdges } from '../types/aier'

import '@vue-flow/core/dist/style.css'
import '@vue-flow/core/dist/theme-default.css'
import '@vue-flow/controls/dist/style.css'
import '@vue-flow/minimap/dist/style.css'

const props = defineProps<{
  edges_lines: AierEdges[]
  nodes: AierNode[]
}>()

const emit = defineEmits(['update:edges_lines', 'update:nodes'])


// import '@vue-flow/node-resizer/dist/style.css'
const dialogVisible = ref(false)
const promptList = ref([] as Prompter[])
const editingStep = ref(emptyStep())

const { onInit, onNodeDragStop, updateEdge, addEdges, setViewport, toObject, } = useVueFlow()

const nodes = ref<FlowNode[]>(props.nodes.length > 0 ? props.nodes : [
  {
    id: 'ENTRY',
    type: 'input',
    data: { label: 'Entry' },
    position: { x: 0, y: 0 },
    class: 'light',
  },
])
const edges = ref<FlowEdge[]>(props.edges_lines ? props.edges_lines : [])


const edgesMap = ref<Map<string, NodeEdges>>(new Map());
const nodesMap = ref<Map<string, PrompterStep>>(new Map());


interface NodeEdges {
  ChildsType: StepType
  Childs: PrompterStep[]
}

watch(edges, (newEdges) => {
  edgesMap.value = new Map()
  newEdges.forEach((edge: any) => {
    // console.log("edge", edge, edge.data)
    const ea = edgesMap.value.get(edge.source)
    if (ea) {
      edgesMap.value.set(edge.source, {
        ChildsType: ea.ChildsType,
        Childs: [...ea.Childs, edge.target]
      } as NodeEdges)
    } else {
      let ne = {
        Childs: [edge.target]
      } as NodeEdges
      if (edge.data?.stype) {
        ne.ChildsType = edge.data.stype
      }

      edgesMap.value.set(edge.source, ne)
    }
  })

})

watch(nodes, (newNodes) => {
  nodesMap.value = new Map()
  newNodes.forEach((node: any) => {
    nodesMap.value.set(node.id, node.data.data as PrompterStep)
  })
  emit('update:nodes', newNodes as unknown as AierNode[])
})

// our dark mode toggle flag
const dark = ref(false)


onInit((vueFlowInstance) => {
  // instance is the same as the return of `useVueFlow`
  vueFlowInstance.fitView()
})

function logNodes() {
  console.log(nodes.value)
}
onNodeDragStop(({ event, nodes: innerNodes, node }) => {
  console.log('Node Drag Stop', { event, nodes: innerNodes, node }, innerNodes)
  logNodes()

  const vueflow = toObject()
  nodes.value = vueflow.nodes
  edges.value = vueflow.edges
})

function updatePos() {
  nodes.value = nodes.value.map((node: any) => {
    return {
      ...node,
      position: {
        x: Math.random() * 400,
        y: Math.random() * 400,
      },
    }
  })
}


function logToObject() {
  console.log(toObject())
  const vueflow = toObject()
  nodes.value = vueflow.nodes
  edges.value = vueflow.edges

}


function resetTransform() {
  setViewport({ x: 0, y: 0, zoom: 1 })
}

function toggleDarkMode() {
  dark.value = !dark.value
}

function addNode() {
  dialogVisible.value = true
  // nodes.value.push({
  //   id: '7',
  //   data: { label: 'Node 7' },
  //   position: { x: 10, y: 10 },
  //   class: 'light',
  // })
}

async function connect(params: any) {
  console.log("connect", params)
  // 连接的时候，需要判断 source是否有一个 SPEC_VAR或者一个NORMAL
  // 有SPEC_VAR的才可以连接其他的SPEC_VAR，平行的结点只有SPEC_VAR才能明多个
  // 是NORMAL的，就只能有一个 TARGET结点
  // 总结：
  /**
   * 每个结点可能有 多个SPEC_VAR，或者单个NORMAL结点
   */
  // addEdges({
  //   id: 'e7-3',
  //   source: '7',
  //   target: '3',
  //   label: 'Node 7',
  // })
  if (!(await judgeEdges(params.source, params.target))) {
    return
  }

  const targetNode = nodesMap.value.get(params.target)!

  console.log("connect", targetNode)
  const eg = {
    id: `${params.source},${params.target}`,
    source: params.source,
    target: params.target,
    label: '',
    data: targetNode,
    updatable: true,
  }
  if (targetNode.stype === StepType.SPEC_VAR) {
    eg.label = targetNode.cond_var
  }

  console.log("eg", eg)

  addEdges(eg)
}

async function judgeEdges(source: string, target: string): Promise<boolean> {
  let sourceEdges = edgesMap.value.get(source)
  if (sourceEdges?.ChildsType == StepType.NORMAL) {
    ElMessage.error('source node is a NORMAL, can not connect to other nodes')
    return false
  }

  const targetNode = nodesMap.value.get(target)
  if (!targetNode) {
    ElMessage.error('connect node isn\'t found')
    return false
  }

  console.log("sourceEdges", sourceEdges)
  console.log("targetNode", targetNode)
  if (sourceEdges?.ChildsType == StepType.SPEC_VAR && targetNode.stype != StepType.SPEC_VAR) {
    ElMessage.error('source node is a SPEC_VAR, can only connect to other SPEC_VAR')
    return false
  }
  return true
}

async function refreshPromptList() {
  const res = await loadPrompter()
  promptList.value = res as Prompter[]
}


async function refresh() {
  await refreshPromptList()
}

refresh()

async function addPrompterNode() {
  console.log(editingStep.value)
  if (!editingStep.value.prompter_uuid) {
    ElMessage.error('prompter is required')
    return
  }
  if (editingStep.value.stype === StepType.SPEC_VAR && !editingStep.value.cond_var) {
    ElMessage.error('cond var is required')
    return
  }
  if (!editingStep.value.node_name) {
    ElMessage.error('node name is required')
    return
  }

  const condStr = `(${editingStep.value.cond_var})`

  const nid = uuidv4()
  nodes.value.push({
    id: nid,
    data: { label: `${nid.substring(0, 4)}:${editingStep.value.node_name}[${editingStep.value.stype.substring(0, 1)}]${editingStep.value.stype === StepType.SPEC_VAR ? condStr : ''}`, data: editingStep.value },
    position: { x: -10, y: -10 },
    class: 'light',
  })
  dialogVisible.value = false

}


async function onEdgeUpdate(event: EdgeUpdateEvent) {
  console.log("onEdgeUpdate", event, event.connection.source, event.connection.target)

  if (!(await judgeEdges(event.connection.source, event.connection.target))) {
    return
  }


  let edge = event.edge
  let len = edges.value.length
  console.log("real edge", edge, JSON.parse(JSON.stringify(edges.value)))
  // 更新之    需要将原来的edge删除
  edges.value = edges.value.filter((e) => e.id !== edge.id)

  console.log("update edge", edge, edges.value)

  if (len === edges.value.length) {
    updateEdge(edge, event.connection)
  }

}

// 添加连接线时，会触发
async function onEdgesChange(edgesChange: EdgeChange[]) {
  console.log("onEdgesChange", edgesChange)
  const vueflow = toObject()
  console.log("vueflow", vueflow.edges, edges.value)
  emit('update:edges_lines', vueflow.edges as unknown as AierEdges[])
}


</script>

<template>
  <VueFlow fit-view-on-init @edge-update="onEdgeUpdate" @edges-change="onEdgesChange" @connect="connect" :nodes="nodes"
    :edges="edges" :class="{ dark }" class="basic-flow" :default-viewport="{ zoom: 0.9 }" :min-zoom="0.2" :max-zoom="4"
    style="width:100%;height: 70vh;border: 1px solid #ccc;border-radius: 5px;">
    <Background pattern-color="#aaa" :gap="16" />

    <MiniMap />

    <Controls position="top-left">
      <ControlButton title="Reset Transform" @click="resetTransform">
        <Icon name="reset" />
      </ControlButton>

      <ControlButton title="Shuffle Node Positions" @click="updatePos">
        <Icon name="update" />
      </ControlButton>

      <ControlButton title="Toggle Dark Mode" @click="toggleDarkMode">
        <Icon v-if="dark" name="sun" />
        <Icon v-else name="moon" />
      </ControlButton>

      <ControlButton title="Log `toObject`" @click="logToObject">
        <Icon name="log" />
      </ControlButton>

      <ControlButton title="Add Node" @click="addNode">
        <el-icon>
          <CirclePlus />
        </el-icon>
      </ControlButton>
    </Controls>
  </VueFlow>
  {{ editingStep }}
  <el-dialog v-model="dialogVisible" title="Multi Prompt Step" width="30%">
    <el-form label-width="100px">
      <el-form-item label="step type">
        <el-select v-model="editingStep.stype" placeholder="choose step type">
          <el-option v-for="item in StepTypeArr" :key="item" :label="item" :value="item" />
        </el-select>
      </el-form-item>
      <el-form-item label="node name">
        <el-input v-model="editingStep.node_name" placeholder="introduce your node effect" />
      </el-form-item>
      <el-form-item label="prompter">
        <el-select v-model="editingStep.prompter_uuid" placeholder="choose prompter">
          <el-option v-for="item in promptList" :key="item.uuid" :label="item.title" :value="item.uuid" />
        </el-select>
      </el-form-item>
      <el-form-item label="cond var" v-if="editingStep.stype === StepType.SPEC_VAR">
        <el-input v-model="editingStep.cond_var"
          placeholder="input cond var, the var returned by your chosen prompter" />
      </el-form-item>

      <el-form-item label="operation">
        <el-button type="primary" @click="addPrompterNode">add step</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
</template>
