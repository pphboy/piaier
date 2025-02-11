<script setup lang=ts>
import { computed, defineProps, onMounted, onUpdated } from "vue";
// @ts-ignore
import { marked } from "marked/marked.min.js";
import hljs from 'highlight.js';

// import "highlight.js/styles/github-dark.css";
import "highlight.js/styles/github.css";


interface MessageProps {
  direction: string
  message: string
}

const props = defineProps<MessageProps>()

const dt = computed({
  set() { },
  get() {
    const slr = props.direction == 'right'
    return {
      'from-left': !slr,
      'from-right': slr,
    }
  }
})

const css = computed({
  set() { },
  get() {
    const v = props.direction == 'right' ? "end" : "start"
    return {
      'justify-content': v,
    }
  }
})

const messageHtml = computed({
  set() { },
  get() {
    const h = marked(props.message, {
      highlight: (code: string, lang: string) => {
        console.log("lan", lang, code)
        const validLanguage = hljs.getLanguage(lang) ? lang : 'plaintext';
        return hljs.highlight(code, {
          language: "python",
        }).value;
      },
    })

    return h
  }
})

async function addClassHljs() {
  hljs.highlightAll()
}

onMounted(() => {
  addClassHljs()
})


onUpdated(() => {
  addClassHljs()
})

</script>

<template>
  <div style="display: flex;" :style="css">
    <section class="message ">
      <div class="nes-balloon" :class="dt">
        <div v-html="messageHtml"></div>
      </div>
    </section>
  </div>
</template>

<style scoped></style>