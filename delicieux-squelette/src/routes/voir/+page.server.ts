import { getRooms } from "../../components/network";

import { env } from '$env/dynamic/private';

/** @type {import('./$types').LayoutServerLoad} */
export async function load({ fetch }) {

    const allRooms = await getRooms(fetch, env.INTERNAL_API_URL)

    if (allRooms.error) {
        console.log("getRooms error : ", allRooms.error)
        return { allRooms: [], error : allRooms.error }
    }

    return { allRooms: allRooms.content };
}
