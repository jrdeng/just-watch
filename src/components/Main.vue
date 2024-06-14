<script setup lang="ts">
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event';
import { NFlex, NButton, NInput, NSelect, NGrid, NGi, useDialog } from 'naive-ui';
import Character from "./Character.vue";

// presenting AppState in Rust
const hwnd = ref("");
const addr = ref("59075C");
const encoding = ref("BIG5");
const started = ref(false);
const characters = ref(new Map<number, any>());

// UI properties
const button_text = ref("开始");
const f1_dialog_displaying = ref(false);
const encoding_options = [
  {
    label: "GB18030",
    value: 'GB18030'
  },
  {
    label: "BIG5",
    value: 'BIG5'
  }
];

const dialog = useDialog();

const button_clicked = async () => {
  if (started.value) {
    await invoke("stop_monitoring");
  } else {
    await invoke("start_monitoring", { addr: addr.value, encoding: encoding.value });
  }
};

// event listeners
const setupEventListener = () => {
  listen('hwnd_changed', (event) => {
    console.log('hwnd_changed:', event.payload);
    // @ts-ignore
    hwnd.value = event.payload;
  });

  listen('started_changed', (event) => {
    console.log('started_changed:', event.payload);
    // @ts-ignore
    started.value = event.payload;
    if (started.value) {
      button_text.value = "停止";
    } else {
      button_text.value = "开始";
    }
  });
  listen('characters_changed', (event) => {
    console.log('characters_changed: payload=', event.payload);
    try {
      // @ts-ignore
      const parsedCharacters = JSON.parse(event.payload);
      console.log("parsedCharacters:", parsedCharacters);
      characters.value = parsedCharacters as Map<number, any>;
    } catch (e) {
      console.error('Failed to parse characters:', e);
    }
  });
};

const handleKeyDown = async (event: any) => {
  // console.log(event.key);
  if (event.key === 'F1') {
    if (started.value) {
      if (f1_dialog_displaying.value) {
        return;
      }
      f1_dialog_displaying.value = true;
      dialog.error({
        title: '无法切换窗口',
        content: '正在运行中，如要切换窗口，需要先停止',
        positiveText: '好的',
        maskClosable: false,
        onPositiveClick: () => {
          f1_dialog_displaying.value = false;
        },
        onEsc: () => {
          f1_dialog_displaying.value = false;
        },
        onClose: () => {
          // console.log("onClose...");
          f1_dialog_displaying.value = false;
        }
      });
    } else {
      await invoke("get_window_handle");;
    }
  }
};

onMounted(() => {
  console.log("mounted");
  window.addEventListener('keydown', handleKeyDown);
  setupEventListener();
})

onUnmounted(() => {
  console.log("unmounted");
  window.removeEventListener('keydown', handleKeyDown);
})

function get_grid_item_class(n: number) {
  if (n < 10) {
    return "enemy";
  } else {
    return "teammate";
  }
}

const validateHex = (value: string) => {
  const hexRegex = /^[0-9a-fA-F]*$/; // 正则表达式，只允许输入 0-9 和 a-f（大小写不敏感）
  if (!hexRegex.test(value)) {
    // 如果输入不符合要求，可以做一些处理，例如清空输入框或者给出提示
    addr.value = value.replace(/[^0-9a-fA-F]/g, ''); // 过滤掉非法字符
  }
};

</script>

<template>
  <n-flex vertical style="margin: 5px">
    <n-flex>
      <div class="center-text">窗口:</div>
      <n-input v-model:value="hwnd" placeholder="按F1获取鼠标处窗口句柄" readonly="true" style="width: 120px" />
      <div class="center-text">地址:</div>
      <n-input v-model:value="addr" placeholder="内存地址" style="width: 120px" :disabled="started" @input="validateHex" />
      <div class="center-text">编码:</div>
      <n-select v-model:value="encoding" :options="encoding_options" :disabled="started" style="width: 120px" />
      <n-button @click="button_clicked" :disabled="!hwnd || !addr">{{ button_text }}</n-button>
    </n-flex>

    <n-grid cols="5" x-gap="5" y-gap="5">
      <n-gi v-for="n in 20">
        <Character v-if="(characters as any)[n - 1]" :class="get_grid_item_class(n - 1)" :character="(characters as any)[n - 1]"></Character>
        <div class="center-text" v-else :class="get_grid_item_class(n - 1)">空</div>
      </n-gi>
    </n-grid>
  </n-flex>
</template>

<style scoped>
.enemy {
  background-color: rgb(236, 162, 162);
  height: 120px;
}

.teammate {
  background-color: rgb(143, 232, 179);
  height: 120px;
}

.center-text {
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
