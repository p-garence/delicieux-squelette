import { derived, get, writable, type Writable } from "svelte/store";
import type { AddRoom, Dessin, Room, RoomWithPass } from "./types";

import { addRoom, completeDessin, getDessins, requestDessin } from "./network";
import type { ToolAction } from "./dessiner/tools";

function localStorageStore<T>(key: string, initialValue: T): Writable<T> {
    const browser = typeof window !== 'undefined';
    let saved = initialValue;
    if (browser) {
        const item = localStorage.getItem(key);
        if (item) saved = JSON.parse(item);
    }
    const store = writable<T>(saved);
    if (browser) {
        store.subscribe((val) => localStorage.setItem(key, JSON.stringify(val)));
    }
    return store;
}

export interface InitAppState {
    allRooms: Room[]
}

export const allRooms = writable<Room[]>([]);
export const usersRooms: Writable<RoomWithPass[]> = localStorageStore("usersRooms", []);
export const selectedRoom: Writable<Room | null> = localStorageStore("selectedRoom", null);
export const currentDessin: Writable<Dessin | null> = localStorageStore("currentDessin", null);
export const undoBuffer = writable<Dessin[]>([])
export const redoBuffer = writable<Dessin[]>([])

selectedRoom.subscribe((_) => {
    undoBuffer.set([])
    redoBuffer.set([])
})

export const currentCells = derived(selectedRoom, async (room) => {
    if (!room) {
        return null
    }

    const dessins = await getDessins({ id: room.id, password: room.hashed_password }, fetch)

    if (!dessins.content) {
        console.log("Erreur pas de dessins")
        return null
    }

    return dessins.content
}
)

// export const worker = writable<Worker | null>();


export const joinRoom = async (room: Room) => {
    usersRooms.update((value) => {
        if (!value.find((elem) => elem.id === room.id)) {
            value.push(room)
        }
        return value
    })
    setSelectedRoom(room)
}

// export const quitRoom = async (room: Room) => {
//     usersRooms.update((value) => {
//         return value.filter((elem) => elem.id !== room.id)
//     })
//     setSelectedRoom(null)
// }

export const setSelectedRoom = (new_room: Room | null) => {
    selectedRoom.set(new_room)
}

export const request = async () => {
    const room = get(selectedRoom)
    if (room) {
        const response = await requestDessin({ id: room.id, password: room.hashed_password }, fetch)

        if (!response.content) {
            return
        }
        undoBuffer.set([])
        redoBuffer.set([])
        currentDessin.set(response.content)
    }
}

export const cancelDessin = async () => {
    currentDessin.set(null)
}

export const complete = async () => {
    const room = get(selectedRoom)
    const cell = get(currentDessin)

    if (room && cell) {
        const content = cell.selected_cell.content

        if (!content) {
            return
        }

        const dessin = await completeDessin({ id_room: room.id, dessin: content, key: cell.selected_cell.id, password: room.hashed_password }, fetch)

        if (dessin.error) {
            console.log("complete err :", dessin)
            return
        }

        currentDessin.set(null)
    }
}

export const createRoom = async (new_room: AddRoom) => {

    const result = await addRoom(new_room, fetch)

    if (!result.content) {
        return
    }

    const r: RoomWithPass = result.content
    r.admin = new_room.admin_password
    r.password = new_room.password

}

export const updateDessin = async (toolAction: ToolAction[], image_resolution) => {
    currentDessin.update((value) => {
        if (!value) {
            return value;
        }
        let content: number[];

        if (
            value.selected_cell.content &&
            value.selected_cell.content.length ===
            image_resolution * image_resolution * 4
        ) {
            content = value.selected_cell.content;
        } else {
            content = new Array(image_resolution * image_resolution * 4);
            content.fill(0);
        }

        toolAction.forEach((coo) => {
            let pos = (coo.y * image_resolution + coo.x) * 4; // position in buffer based on x and y
            if (pos < content.length) {
                // dessin.splice.apply([pos, 4, ...coo.value]);
                content[pos] = coo.value[0]; // some R value [0, 255]
                content[pos + 1] = coo.value[1]; // some G value
                content[pos + 2] = coo.value[2]; // some B value
                content[pos + 3] = coo.value[3]; // set alpha channel
            }
        });

        content = content.map((value) => (value === null ? 0 : value));

        value.selected_cell.content = content;

        return value;
    });
}