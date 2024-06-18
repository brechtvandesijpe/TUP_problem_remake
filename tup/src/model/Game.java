package model;

/**
 * Represents a game in the tournament scheduling problem.
 * Each game is uniquely identified and associated with a specific round, home player, and out player.
 */

public class Game {
    public static int gameId; // Static counter to assign unique IDs to each game instance

    private final int id;
    private final int round;
    private final int homePlayerId;
    private final int outPlayerId;

    public Game(int round, int homePlayerId, int outPlayerId) {
        this.id = gameId++;
        this.round = round;
        this.homePlayerId = homePlayerId;
        this.outPlayerId = outPlayerId;
    }

    public int getRound() {
        return round;
    }

    public int getHomePlayerId() {
        return homePlayerId;
    }

    public int getOutPlayerId() {
        return outPlayerId;
    }

    @Override
    public String toString() {
        return "Game{" +
                "id=" + id +
                ", round=" + round +
                ", homePlayerId=" + homePlayerId +
                ", outPlayerId=" + outPlayerId +
                '}';
    }
}