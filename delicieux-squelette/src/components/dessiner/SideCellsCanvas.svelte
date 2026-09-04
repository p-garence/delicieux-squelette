<script lang="ts">
  import { run } from 'svelte/legacy';

  import { currentDessin } from "../appState";

  let canvas: HTMLCanvasElement = $state();
  let context: CanvasRenderingContext2D | null = $state();

  import type { Dessin } from "../types";
  import { onMount } from "svelte";
  interface Props {
    image_resolution: number;
    pixel_size: number;
    children?: import('svelte').Snippet;
  }

  let { image_resolution, pixel_size, children }: Props = $props();




  function toBuffer(content: number[], res: number, pix_size: number) {
    let buffer = new Uint8ClampedArray(res * res * pix_size * pix_size * 4);

    content.forEach((_, content_index) => {
      if (content_index % 4 !== 0) {
        return;
      }
      let x = (content_index / 4) % res;
      let y = Math.floor(content_index / 4 / res);
      //console.log(x)

      for (let i = 0; i < pix_size; i++) {
        for (let j = 0; j < pix_size; j++) {
          let bufferPos =
            (x * pix_size + i + (j + y * pix_size) * res * pix_size) * 4;
          buffer[bufferPos] = content[content_index];
          buffer[bufferPos + 1] = content[content_index + 1];
          buffer[bufferPos + 2] = content[content_index + 2];
          buffer[bufferPos + 3] = content[content_index + 3];
        }
      }
    });
    return buffer;
  }
  let canvasHolderRef: HTMLDivElement = $state();

  onMount(async () => {
    if (!canvas) return;
    context = canvas.getContext("2d");
    if (!context) return;
  });
  let dxy = $derived(image_resolution * 0.25 * pixel_size);
  let w = $derived(image_resolution * pixel_size + dxy * 2);
  let h = $derived(w);
  let drawBuffer = $derived((
    current_dessin: Dessin | null,
    image_resolution: number,
    pixel_size: number
  ) => {
    if (!context || pixel_size == -1 || !current_dessin) {
      return;
    }
    canvasHolderRef.innerHTML = "";

    let x = current_dessin.selected_cell.x;
    let y = current_dessin.selected_cell.y;

    let neighbors = [
      [x - 1, y - 1],
      [x, y - 1],
      [x + 1, y - 1],
      [x - 1, y],
      [x + 1, y],
      [x - 1, y + 1],
      [x, y + 1],
      [x + 1, y + 1],
    ];

    for (let cell of current_dessin.side_cells) {
      
      if (!cell.content || !cell.done) {
        continue;
      }

      if (cell.content.length !== image_resolution * image_resolution * 4) {
        continue;
      }

      let i = neighbors.findIndex(
        (elem) => elem[0] === cell.x && elem[1] === cell.y
      );

      if (i !== -1) {
        neighbors.splice(i, 1);
      }

      let imageData = new ImageData(
        image_resolution * pixel_size,
        image_resolution * pixel_size
      );

      let content = toBuffer(cell.content, image_resolution, pixel_size);

      imageData.data.set(content);

      let x_p =
        dxy +
        image_resolution *
          pixel_size *
          (cell.x - current_dessin.selected_cell.x);
      let y_p =
        dxy +
        image_resolution *
          pixel_size *
          (cell.y - current_dessin.selected_cell.y);

      setTimeout(function () {
        context?.putImageData(imageData, x_p, y_p);
      }, 10);
    }


    for (let cell of neighbors) {
      let l = image_resolution * pixel_size;
      let diffX = x - cell[0];
      let diffY = y - cell[1];

      let child = document.createElement("div");

    
      child.style.width = (diffX == 0 ? l : dxy) + "px";
      child.style.height = (diffY == 0 ? l : dxy) + "px";
      child.style.top = (diffY == 0 ? dxy : diffY < 0 ? dxy + l : 0) + "px";
      child.style.left = (diffX == 0 ? dxy : diffX < 0 ? dxy + l : 0) + "px";
      child.style.backgroundColor = "#5d9281";
      child.style.position = "absolute";
      canvasHolderRef.appendChild(child);
    }
  });
  run(() => {
    drawBuffer($currentDessin, image_resolution, pixel_size);
  });
</script>

<div class="p-4">
  <div style="width: {w}px; height: {h}px">
    <div class="relative bg-[#709c8d]">
      <canvas
        bind:this={canvas}
        width={w}
        height={h}
        style="width: {w}px; height: {h}px;"
></canvas>

      <div bind:this={canvasHolderRef}></div>

      <div
        style="width: {image_resolution *
          pixel_size}px; height: {image_resolution *
          pixel_size}px; top: {dxy}px; left: {dxy}px"
        class="absolute"
      >
        {@render children?.()}
      </div>
    </div>
  </div>
</div>
