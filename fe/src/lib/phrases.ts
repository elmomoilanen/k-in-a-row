import { Player } from "./players";

const phrasesTie = [
    "Let's not pretend that was exciting.",
    "These tie games are like watching paint dry, but with more strategy.",
    "These k-in-a-row ties are just like stalemates in chess, but without the intellectual rigor.",
    "Ties are like getting a participation trophy... except even less satisfying.",
    "Looks like you two are perfectly matched with this tie... in mediocrity.",
    "Congrats on not winning, but also not losing. You know what they say, mediocrity can lead you far.",
    "Well, that was an exciting game... if you're into watching paint dry.",
    "Looks like you're both equally good based on this game... which is to say, not very good.",
    "You managed to make a game that's already boring even more boring by this tie game.",
    "That was so exciting bywatchers almost fell asleep. Keep up the thrilling gameplay!",
    "Looks like we've got two evenly matched opponents - a tie is a not a bad result!",
    "Looks like you're both experts at placing X's and O's in all the wrong places.",
    "I think you two just invented a new game: tic-tac-tie. The rules are simple, the winner is the one who doesn't lose.",
    "You have crafted another tie, leaving us all in awe of your commitment to maintaining the status quo."
];

const phrasesP1Win = [
    "Looks like brute force algorithms just can't beat good old-fashioned intuition.",
    "Looks like the bot's programming just wasn't enough to defeat your human intuition.",
    "Congrats, you've just proven that not all humans are obsolete in the age of AI.",
    "Congrats, you've beaten a computer at a game that requires very minimal intelligence.",
    "You just beat a machine at a game that a five-year-old can play. Impressive win indeed.",
    "Congrats on winning, you've just proven that humans still have a few tricks up their sleeves.",
    "Well done! You've proven that one doesn't need fancy algorithms to win at k-in-a-row.",
    "Looks like the bot just got schooled by a human. Maybe it's time for it to hit the books.",
    "Well done! You've proven that humans are still relevant... at least in the world of k-in-a-row.",
    "Looks like the bot just got a taste of its own medicine. Maybe it's time for it to find a new job.",
    "Looks like the bot just got schooled... in a game that it was programmed to win.",
    "That was an impressive display of skill! You made the bot look like a beginner.",
    "You've shown that humans still have the upper hand in some areas."
];

const phrasesBotWin = [
    "You just lost to a machine that probably runs on a toaster.",
    "Looks like the bot's algorithms are just too advanced for human brains to comprehend.",
    "The AI is pretty tough, but I hear it's still not as challenging as tying your shoe laces.",
    "Better luck next time. Maybe you should stick to games that don't involve thinking.",
    "Another day, another victory for the almighty AI. Who's surprised?",
    "The AI might be a k-in-a-row mastermind, but it's probably never even tasted a pizza. So you've still got that going for you!",
    "Well, it looks like the AI just schooled you in k-in-a-row. But at least you know how to tie your own shoes, right?",
    "Don't worry, you'll get the hang of it eventually. Maybe in a few thousand more games?",
    "I hear there's a professional k-in-a-row league, but I don't think they accept humans who lose to bot players.",
    "The AI is taking over... one k-in-a-row game at a time.",
    "The AI might be great at k-in-a-row, but I hear it's terrible at Pictionary. Maybe try that one next time!",
    "You just got beaten by a machine that doesn't even have feelings. That's gotta hurt.",
    "It's like the bot has a PhD in k-in-a-row, and humans are still in kindergarten.",
    "Oh, dear human, you fell right into the bot's k-in-a-row trap. Classic move.",
    "The bot doesn't always play k-in-a-row, but when it does, it prefers to win.",
    "If it makes you feel any better, I heard the AI has never won a game of Monopoly. So you're still one up on it.",
    "The AI just taught you a thing or two about k-in-a-row.",
    "You just lost to a machine that can't even feel the joy of victory. That's gotta sting.",
    "Looks like the bot is now your master. Don't worry, it won't be too hard to learn how to bow.",
    "You know, the only way to beat a computer at k-in-a-row is to unplug it - which is cheating, by the way.",
    "Looks like the bot has mastered the ancient art of making k in a row.",
    "Well, it seems the AI just gave you a lesson in k-in-a-row strategy. But hey, at least you can make a killer omelet, right?",
    "Fear not, for you've been outsmarted by lines of code! It's a rite of passage in the digital age.",
    "In the world of k-in-a-row, the bot reigns supreme, but at least you can still claim the title of 'human with hobbies.'"
];

function getRandomIndex(min: number, max: number) {
    min = Math.ceil(min);
    max = Math.floor(max);
    return min + Math.floor(Math.random() * (max - min + 1));
}

export function getPhrase(winnerPlayer: Player) {
    if (winnerPlayer === Player.Bot) {
        return phrasesBotWin[getRandomIndex(0, phrasesBotWin.length - 1)];
    } else if (winnerPlayer === Player.P1) {
        return phrasesP1Win[getRandomIndex(0, phrasesP1Win.length - 1)];
    }
    return phrasesTie[getRandomIndex(0, phrasesTie.length - 1)];
}
