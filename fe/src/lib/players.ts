export enum Player {
    P1 = 1,
    Bot = -1,
    Empty = 0
}

export function getRandomPlayer() {
    return Math.random() > 0.5 ? Player.P1 : Player.Bot;
}
