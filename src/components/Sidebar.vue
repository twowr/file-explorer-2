<script setup>

const emit = defineEmits(["back"])

</script>

<template>
    <div>
        <button class="backButton" @click="emit('back')">back</button>
    </div>
</template>