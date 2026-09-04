import { writable } from 'svelte/store';
import type { Dessin } from '../types';
import type { ToolAction } from './tools';

export function createCanvasData(image_resolution: number, pixel_size: number, current_dessin: Dessin | null) {
	// console.log("creating buffer	")
	let buffer = new Uint8ClampedArray(
		image_resolution * image_resolution * pixel_size * pixel_size * 4
	);

	if (current_dessin && current_dessin.selected_cell.content) {
		current_dessin.selected_cell.content.forEach((_, content_index) => {
			if (content_index % 4 !== 0) {
				return;
			}
			let x = (content_index / 4) % image_resolution;
			let y = Math.floor(content_index / 4 / image_resolution);
			for (let i = 0; i < pixel_size; i++) {
				for (let j = 0; j < pixel_size; j++) {
					let bufferPos =
						(x * pixel_size +
							i +
							(j + y * pixel_size) * image_resolution * pixel_size) *
						4;
					if (current_dessin.selected_cell.content) {
						buffer[bufferPos] =
							current_dessin.selected_cell.content[content_index];
						buffer[bufferPos + 1] =
							current_dessin.selected_cell.content[content_index + 1];
						buffer[bufferPos + 2] =
							current_dessin.selected_cell.content[content_index + 2];
						buffer[bufferPos + 3] =
							current_dessin.selected_cell.content[content_index + 3];
					}
				}
			}
		});
	}

	const { subscribe, update } = writable(buffer);

	return {
		subscribe,
		applyToolActions: (toolAction: ToolAction[]) =>
			update((buffer) => {
				toolAction.forEach((action) => {
					for (let i = 0; i < pixel_size; i++) {
						for (let j = 0; j < pixel_size; j++) {
							let bufferPos =
								(action.x * pixel_size +
									i +
									(j + action.y * pixel_size) * image_resolution * pixel_size) *
								4;
							buffer[bufferPos] = action.value[0];
							buffer[bufferPos + 1] = action.value[1];
							buffer[bufferPos + 2] = action.value[2];
							buffer[bufferPos + 3] = action.value[3];
						}
					}
				});
				return buffer
			}),
		draw: (context: CanvasRenderingContext2D | null) => {
			if (context) {
				let imageData = context.createImageData(
					image_resolution * pixel_size,
					image_resolution * pixel_size
				);
				imageData.data.set(buffer);
				context.putImageData(imageData, 0, 0);
			}
		}


	};
}