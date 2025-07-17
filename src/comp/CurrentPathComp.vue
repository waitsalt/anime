<script lang="ts" setup>
import { ref, computed, onMounted, onBeforeUnmount } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

// 组件状态
const indicator = ref<HTMLElement | null>(null);
const isDragging = ref(false);
const position = ref({ x: 20, y: 20 });
const dragOffset = ref({ x: 0, y: 0 });

// 计算当前路径
const currentPath = computed(() => route.path);

// 计算位置样式 - 调整为 CSSProperties 类型
const positionStyle = computed<Record<string, string>>(() => ({
    left: `${position.value.x}px`,
    top: `${position.value.y}px`,
    position: "fixed",
    zIndex: "9999",
}));

// 开始拖拽
const startDrag = (e: MouseEvent) => {
    isDragging.value = true;
    if (indicator.value) {
        const rect = indicator.value.getBoundingClientRect();
        dragOffset.value = {
            x: e.clientX - rect.left,
            y: e.clientY - rect.top,
        };
    }
    e.preventDefault();
};

// 处理拖拽
const handleDrag = (e: MouseEvent) => {
    if (!isDragging.value) return;

    let x = e.clientX - dragOffset.value.x;
    x = x > 0 ? x : 0;
    let y = e.clientY - dragOffset.value.y;
    y = y > 0 ? y : 0;

    position.value = {
        x: x,
        y: y,
    };

    // 保存位置到本地存储
    localStorage.setItem(
        "pathIndicatorPosition",
        JSON.stringify(position.value),
    );
};

// 停止拖拽
const stopDrag = () => {
    isDragging.value = false;
};

// 生命周期钩子
onMounted(() => {
    window.addEventListener("mousemove", handleDrag);
    window.addEventListener("mouseup", stopDrag);

    // 从本地存储加载位置
    const savedPos = localStorage.getItem("pathIndicatorPosition");
    if (savedPos) {
        position.value = JSON.parse(savedPos);
    }
});

onBeforeUnmount(() => {
    window.removeEventListener("mousemove", handleDrag);
    window.removeEventListener("mouseup", stopDrag);
});
</script>

<template>
    <div
        ref="indicator"
        :style="positionStyle"
        class="path-indicator"
        @mousedown="startDrag"
    >
        <div class="path-content">
            {{ currentPath }}
        </div>
    </div>
</template>

<style lang="css" scoped>
.path-indicator {
    cursor: move;
    user-select: none;
    background-color: rgba(0, 0, 0, 0.7);
    color: white;
    padding: 8px 12px;
    border-radius: 4px;
    font-family: monospace;
    font-size: 14px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
    transition: background-color 0.2s;
}

.path-indicator:hover {
    background-color: rgba(0, 0, 0, 0.8);
}

.path-content {
    white-space: nowrap;
}
</style>
