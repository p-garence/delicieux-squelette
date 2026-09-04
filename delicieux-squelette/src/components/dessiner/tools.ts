
import { Pen } from 'lucide-svelte'
import { Eraser } from 'lucide-svelte'
import { get, writable } from 'svelte/store';



export const selectedToolIndex = writable<number>(0);
export const toolSize = writable<number>(2);
export const toolColor = writable<string>("#50d71e");

export type ToolAction = {
    x: number,
    y: number,
    value: Uint8ClampedArray
}
export type Tool = {
    name: string,
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    icon: any,
    action: (x: number, y: number, image_resolution: number) => ToolAction[]
}

export const toolsList: Tool[] = [
    {
        name: "dessiner",
        icon: Pen,
        action: (x: number, y: number, image_resolution: number) => {

            const color = get(toolColor)
            const tool_size = get(toolSize)
            const rgba = hexToRgbA(color)
            if (!rgba) {
                console.log("not rgba : ", color)
                return []
            }

            const value = new Uint8ClampedArray(4);
            value[0] = rgba[0]
            value[1] = rgba[1]
            value[2] = rgba[2]
            value[3] = rgba[3]

            // console.log(image_resolution, x)
            const positions: ToolAction[] = []
            switch (tool_size) {
                case 4:
                    if (x > 1) positions.push({ x: x - 2, y, value });
                    if (x < image_resolution - 2) positions.push({ x: x + 2, y, value });
                    if (y > 1) positions.push({ x, y: y - 2, value });
                    if (y < image_resolution - 2) positions.push({ x, y: y + 2, value });
                // eslint-disable-next-line no-fallthrough
                case 3:
                    if (x > 0 && y > 0) positions.push({ x: x - 1, y: y - 1, value });
                    if (x < image_resolution - 1 && y < image_resolution - 1) positions.push({ x: x + 1, y: y + 1, value });
                    if (x < image_resolution - 1 && y > 0) positions.push({ x: x + 1, y: y - 1, value });
                    if (x > 0 && y < image_resolution - 1) positions.push({ x: x - 1, y: y + 1, value });
                // eslint-disable-next-line no-fallthrough
                case 2:
                    if (x > 0) positions.push({ x: x - 1, y, value });
                    if (x < image_resolution - 1) positions.push({ x: x + 1, y, value });
                    if (y > 0) positions.push({ x, y: y - 1, value });
                    if (y < image_resolution - 1) positions.push({ x, y: y + 1, value });
                // eslint-disable-next-line no-fallthrough
                case 1:
                    positions.push({ x, y, value });
            }
            // console.log(positions)
            return positions

        }
    },
    {
        name: "effacer",
        icon: Eraser,
        action: (x: number, y: number, image_resolution: number) => {
            const value = new Uint8ClampedArray(4);
            const tool_size = get(toolSize)

            const positions: ToolAction[] = []
            switch (tool_size) {
                case 4:
                    if (x > 1) positions.push({ x: x - 2, y, value });
                    if (x < image_resolution - 2) positions.push({ x: x + 2, y, value });
                    if (y > 0) positions.push({ x, y: y - 2, value });
                    if (y !== image_resolution - 2) positions.push({ x, y: y + 2, value });
                // eslint-disable-next-line no-fallthrough
                case 3:
                    if (x !== 0 && y !== 0) positions.push({ x: x - 1, y: y - 1, value });
                    if (x !== image_resolution - 1 && y !== image_resolution - 1) positions.push({ x: x + 1, y: y + 1, value });
                    if (x !== 0 && y !== image_resolution - 1) positions.push({ x: x + 1, y: y - 1, value });
                    if (x !== image_resolution - 1 && y !== 0) positions.push({ x: x - 1, y: y + 1, value });
                // eslint-disable-next-line no-fallthrough
                case 2:
                    if (x !== 0) positions.push({ x: x - 1, y, value });
                    if (x !== image_resolution - 1) positions.push({ x: x + 1, y, value });
                    if (y !== 0) positions.push({ x, y: y - 1, value });
                    if (y !== image_resolution - 1) positions.push({ x, y: y + 1, value });
                // eslint-disable-next-line no-fallthrough
                case 1:
                    positions.push({ x, y, value });
            }

            return positions

        }
    },
    // {
    //     name: "remplir",
    //     icon: FaFill,
    //     action: (x: number, y: number, image_resolution: number, tool_size: number) => {
    //         let value = new Uint8ClampedArray(4);
    //         value[0] = 255

    //         let positions: { x: number, y: number, value: Uint8ClampedArray }[] = []
    //         switch (tool_size) {
    //             case 1:
    //                 positions.push({ x, y, value });
    //             case 2:
    //                 positions.push({ x, y, value });
    //                 if (x !== 0) positions.push({ x: x - 1, y, value });
    //                 if (x !== image_resolution) positions.push({ x: x + 1, y, value });
    //                 if (y !== 0) positions.push({ x, y: y - 1, value });
    //                 if (y !== image_resolution) positions.push({ x, y: y + 1, value });
    //                 break;
    //         }

    //         return positions

    //     }
    // }
]


function hexToRgbA(hex: string) {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let c: any;
    if (/^#([A-Fa-f0-9]{3}){1,2}$/.test(hex)) {
        c = hex.substring(1).split('');
        if (c.length == 3) {
            c = [c[0], c[0], c[1], c[1], c[2], c[2]];
        }
        c = '0x' + c.join('');
        return [(c >> 16) & 255, (c >> 8) & 255, c & 255, 255]
        // return 'rgba('+[(c>>16)&255, (c>>8)&255, c&255].join(',')+',1)';
    }
    return;
}
