<script setup lang="ts">
import Main from "./components/Main.vue";
import { NDialogProvider } from 'naive-ui';
</script>

<template>
  <n-dialog-provider>
    <Main />
  </n-dialog-provider>
</template>
