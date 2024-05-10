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

    public static Game getGame(int gameId) {
        return games[gameId];
    }
}
