export type Room = {
    id: number,
    name: string,
    rules?: string,
    is_public: boolean,
    colors: string[]
    resolution: number,
    number_of_dessin?: number,
    password_protected: boolean,
    hashed_password?: string,
    hashed_admin?: string,
}

export interface NetworkResponse<T> {
    error?: Error,
    content?: T
}

export interface Error {
    message: string,
    body?: any
}


export interface RoomWithPass extends Room {
    admin?: string,
    password?: string
}

export type AddRoom = {
    name: string,
    rules?: string,
    is_public: boolean,
    password_protected: boolean,
    colors: string[]
    resolution: number,
    password?: string
    admin_password: string,
}

export type ChangeRoom = {
    rules?: string,
}


export type Cell = {
    id: number,
    x: number,
    y: number,
    done: boolean,
    room_id: number,
    content: number[] | null,
    last_request?: {
        secs_since_epoch: number
    },
}

export type Dessin =
    {
        key: number,
        side_cells: Cell[]
        selected_cell: Cell
    }


