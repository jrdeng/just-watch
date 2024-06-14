<script setup lang="ts">
import { NFlex } from 'naive-ui';

interface Character {
    name: string,
    lv: number,
    hp: number,
    hp_max: number,
    mp: number,
    mp_max: number,
}

interface Props {
    character: Character | null;
}

// @ts-ignore
const props = defineProps<Props>();

</script>

<template>
    <n-flex vertical justify="center" v-if="character">
        <span class="center-text">{{ character.name }} ({{ character.lv }}级)</span>
        <span class="center-text">生命: {{ character.hp }}/{{ character.hp_max }}</span>
        <span class="center-text">魔力: {{ character.mp }}/{{ character.mp_max }}</span>
    </n-flex>
    <div class="center-text" v-else>
        空
    </div>
</template>

<style scoped>
.center-text {
    text-align: center;
}
</style>