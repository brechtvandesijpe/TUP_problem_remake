package model;

import java.util.Arrays;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.IntStream;

import static main.Config.*;

public class Instance {
    private static int[][] distances;
    private static int[][] opponents;
    public static int[][] roundStadium;
    private static Game[] games = new Game[0];

    public Instance(int[][] opponents, int[][] distances) {
        NUM_TEAMS = distances.length;
        NUM_UMPIRES = distances.length / 2;
        NUM_ROUNDS = distances.length * 2 - 2;
        NUM_GAMES = NUM_ROUNDS * distances.length / 2;
        Instance.distances = distances;
        Instance.opponents = opponents;
        roundStadium = new int[NUM_ROUNDS][distances.length];
        IntStream.range(0, NUM_ROUNDS).forEach(i -> Arrays.fill(roundStadium[i], UNASSIGNED));
        games = new Game[NUM_GAMES];
        addGamesForRounds();
    }

    public void addGamesForRounds() {
        AtomicInteger idx = new AtomicInteger(0);
        IntStream.range(0, NUM_ROUNDS).forEach(ri -> IntStream.range(0, NUM_TEAMS).filter(playerId -> isHomeGame(ri, playerId)).forEach(pi -> {
            int gi = idx.getAndIncrement();
            games[gi] = new Game(ri, pi, opponents[ri][pi] - 1);
            roundStadium[ri][pi] = gi;
        }));
    }

    public static boolean isHomeGame(int round, int playerId) {
        return opponents[round][playerId] > 0;
    }

    public static int getInterStadiumDistance(int previousGameId, int currentGameId) {
        return distances[games[currentGameId].getHomePlayerId()][games[previousGameId].getHomePlayerId()];
    }

    public static int getInterStadiumDistance(Game previousGame, Game currentGame) {
        return distances[previousGame.getHomePlayerId()][currentGame.getHomePlayerId()];
    }

    public static int determineGameForPlayer(int roundIndex, int playerId) {
        int gameNumber;
        if (isHomeGame(roundIndex, playerId)) {
            gameNumber = Instance.roundStadium[roundIndex][playerId];
        } else {
            gameNumber = Instance.roundStadium[roundIndex][getOpponentOfPlayerInRound(playerId, roundIndex) * (-1) - 1];
        }
        return gameNumber - ((gameNumber / NUM_UMPIRES) * NUM_UMPIRES);
    }

    public static boolean isFeasible(Game current, Game next) {
        // C4: An umpire crew must wait q1-1 rounds before revisiting a player's stadium
        boolean q1Conflict = hasStadiumCountConflict(current, next);
        // C1: An umpire crew must wait q2-1 rounds before officiating the same player again
        boolean q2Conflict = hasPlayerConflict(current, next);
        return !q1Conflict && !q2Conflict;
    }

    private static boolean hasPlayerConflict(Game current, Game next) {
        int homePlayerId1 = current.getHomePlayerId();
        int outPlayerId1 = current.getOutPlayerId();
        int homePlayerId2 = next.getHomePlayerId();
        int outPlayerId2 = next.getOutPlayerId();
        return homePlayerId1 == homePlayerId2 || outPlayerId1 == outPlayerId2 || homePlayerId1 == outPlayerId2 || outPlayerId1 == homePlayerId2;
    }

    private static boolean hasStadiumCountConflict(Game current, Game next) {
        return current.getHomePlayerId() == next.getHomePlayerId();
    }

    public static int getOpponentOfPlayerInRound(int playerId, int round) {
        return opponents[round][playerId];
    }

    public static int getTravelDistanceBetween(int stadium1, int stadium2) {
        return distances[stadium1][stadium2];
    }

    public static Game getGame(int gameId) {
        return games[gameId];
    }
}
