<script lang="ts">
  import { onMount } from "svelte";
  import InfiniteCanvas from "ef-infinite-canvas";
  import { selectedRoom } from "../appState";
  import type { Cell } from "../types";
  import { _ } from "svelte-i18n";
  import { ArrowLeft, ArrowRight, ArrowDown, ArrowUp, Plus, Minus } from "lucide-svelte";

  interface Props {
    width: number;
    height: number;
    cells: Cell[];
  }

  let { width, height, cells }: Props = $props();

  let worker: Worker | null;

  // FIX 1 : On enlève le $state() !
  // Dans Svelte 5, bind:this marche très bien avec un 'let' classique.
  // Cela empêche le Proxy Svelte de faire planter ef-infinite-canvas.
  let canvas: HTMLCanvasElement;
  let context: CanvasRenderingContext2D | null;

  // FIX 2 : On passe ces variables en $state pour supprimer les avertissements de compilation
  let offsetX = $state(0);
  let offsetY = $state(0);
  let scale = $state(1);

  let middlex = $derived((width / 2) + offsetX);
  let middley = $derived((height / 2) + offsetY);

  // FIX 3 : Compatible SSR. On l'initialise à false, et on vérifiera au montage.
  let is_safari = $state(false);

  const move = (x: number, y: number) => {
    if (!context || !canvas) return;
    context.clearRect(0, 0, canvas.width, canvas.height);
    
    offsetX += x;
    offsetY += y;
    
    worker?.postMessage({
      res: $selectedRoom?.resolution,
      cells: JSON.parse(JSON.stringify(cells)),
      wait: false,
    });
  };

  function scaleImageData(imageData: ImageData, scale: number) {
    if (!context) return imageData;
    var scaled = context.createImageData(
      imageData.width * scale,
      imageData.height * scale,
    );

    for (var row = 0; row < imageData.height; row++) {
      for (var col = 0; col < imageData.width; col++) {
        var sourcePixel = [
          imageData.data[(row * imageData.width + col) * 4 + 0],
          imageData.data[(row * imageData.width + col) * 4 + 1],
          imageData.data[(row * imageData.width + col) * 4 + 2],
          imageData.data[(row * imageData.width + col) * 4 + 3],
        ];
        for (var y = 0; y < scale; y++) {
          var destRow = row * scale + y;
          for (var x = 0; x < scale; x++) {
            var destCol = col * scale + x;
            for (var i = 0; i < 4; i++) {
              scaled.data[(destRow * scaled.width + destCol) * 4 + i] =
                sourcePixel[i];
            }
          }
        }
      }
    }
    return scaled;
  }

  const zoom = (value: boolean) => {
    if (value) {
      scale += 1;
    } else {
      if (scale <= 1) return;
      scale -= 1;
    }
    if (!context || !canvas) return;
    context.clearRect(0, 0, canvas.width, canvas.height);
    worker?.postMessage({
      res: $selectedRoom?.resolution,
      cells: JSON.parse(JSON.stringify(cells)),
      wait: false,
    });
  };

  const loadWorker = async () => {
    const SyncWorker = await import("$lib/worker.ts?worker");
    worker = new SyncWorker.default();

    worker.onmessage = async (e: MessageEvent<any>) => {
      if (e.data.message) {
        console.log("worker: ", e.data.message);
      }
      
      if (e.data.imageData) {
        let data = e.data.imageData;
        
        if (is_safari) {
          data = scaleImageData(e.data.imageData, scale);
        }

        try {
          // LA CORRECTION EST ICI :
          // On transforme les pixels bruts en "ImageBitmap" (ultra performant)
          const bitmap = await window.createImageBitmap(data);
          
          // Et on utilise drawImage au lieu de putImageData !
          context?.drawImage(
            bitmap,
            e.data.x * scale + middlex,
            e.data.y * scale + middley
          );
        } catch (err) {
          console.error("Erreur de dessin :", err);
        }
      }
    };

    if ($selectedRoom?.resolution) {
      worker.postMessage({
        res: $selectedRoom?.resolution,
        cells: JSON.parse(JSON.stringify(cells)),
        wait: true,
      });
    } else {
      console.log("error drawing : no room selected");
    }
  };

  onMount(() => {
    // La vérification du navigateur se fait ici (côté client uniquement)
    const is_chrome = navigator.userAgent.indexOf("Chrome") > -1;
    is_safari = navigator.userAgent.indexOf("Safari") > -1 && !is_chrome;

    if (!canvas) return;

    if (is_safari) {
      context = canvas.getContext("2d");
    } else {
      // Le canvas brut est passé à la librairie, le crash disparaît !
      const infinite_canvas = new InfiniteCanvas(canvas);
      infinite_canvas.greedyGestureHandling = true;
      context = infinite_canvas.getContext("2d");
    }

    if (!context) return;
    context.shadowBlur = 0;
    context.imageSmoothingEnabled = false;

    if (window.Worker) {
      loadWorker();

      return () => {
        worker?.terminate();
      };
    } else {
      console.log("no worker capabilities");
    }
  });
</script>

<div style="width: {width}px; height: {height}px;">
  {#if is_safari}
    <div
      class="absolute top-0 bottom-0 right-0 flex justify-center items-center"
    >
      <div class="base rounded-md w-22">
        <div class="no-text-border text-xs pb-0 p-2">
          {$_("safari_bug")}
        </div>
        <div class="flex flex-row gap-2 m-2 text-[#134705]">
          <button
            onclick={() => {
              zoom(true);
            }}
            class="h-8 w-8 bg-dessin rounded-md"
          >
            <Plus />
          </button>
          <button
            onclick={() => {
              zoom(false);
            }}
            class="h-8 w-8 bg-dessin rounded-md"
          >
            <Minus />
          </button>
        </div>

        <div class="flex flex-row gap-1 m-1 text-[#134705]">
          <div class="h-6 w-6"></div>
          <button
            onclick={() => {
              move(0, 50);
            }}
            class="h-6 w-6 bg-dessin rounded-md"
          >
            <ArrowUp />
          </button>
          <div class="h-6 w-6"></div>
        </div>
        <div class="flex flex-row gap-1 m-1 mb-2 text-[#134705]">
          <button
            onclick={() => {
              move(50, 0);
            }}
            class="h-6 w-6 bg-dessin rounded-md"
          >
            <ArrowLeft />
          </button>
          <button
            onclick={() => {
              move(0, -50);
            }}
            class="h-6 w-6 bg-dessin rounded-md"
          >
            <ArrowDown />
          </button>
          <button
            onclick={() => {
              move(-50, 0);
            }}
            class="h-6 w-6 bg-dessin rounded-md"
          >
            <ArrowRight />
          </button>
        </div>
      </div>
    </div>
  {/if}

  <canvas
    bind:this={canvas}
    {width}
    {height}
    style="width: {width}px; height: {height}px; background-color: #709c8d;"
  ></canvas>
</div>
