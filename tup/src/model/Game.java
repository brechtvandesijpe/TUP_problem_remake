package model;

public class Game {
    public static int gameId;

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