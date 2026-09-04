<script lang="ts">
  import { ChevronLeft } from "lucide-svelte";
  import { ChevronRight } from "lucide-svelte";
  import { currentDessin, redoBuffer, undoBuffer } from "../appState";
  import { clone } from "$lib/hash";

  const redo = () => {
    if ($redoBuffer.length != 0) {
      currentDessin.update((dessin) => {
        undoBuffer.update((buff) => {
          buff.push(clone(dessin));
          return buff;
        });
        return $redoBuffer[$redoBuffer.length - 1];
      });
      redoBuffer.update((buff) => {
        buff.splice(buff.length - 1);
        return buff;
      });
    }
  };
  const undo = () => {
    if ($undoBuffer.length != 0) {
      currentDessin.update((dessin) => {
        redoBuffer.update((buff) => {
          buff.push(clone(dessin));
          return buff;
        });
        return $undoBuffer[$undoBuffer.length - 1];
      });
      undoBuffer.update((buff) => {
        buff.splice(buff.length - 1);
        return buff;
      });
    }
  };
</script>

<div class="w-16 flex flex-row items-center justify-center p-0.5">
  <button
    onclick={undo}
    class={"w-7.5 h-7.5 " +
      ($undoBuffer.length !== 0 ? "" : "opacity-20")}><ChevronLeft /></button
  >
  <button
    onclick={redo}
    class={"w-7.5 h-7.5 " +
      ($redoBuffer.length !== 0 ? "" : "opacity-20")}><ChevronRight /></button
  >
</div>
