package problem;

import model.Game;
import model.Instance;
import java.util.Arrays;
import static main.Config.*;
import static model.Instance.getTravelDistanceBetween;


public class LowerboundMatch {
    private final int[][] costArray;
    private final Instance instance;
    private final int MAX = 999999;

    public LowerboundMatch(Instance instance) {
        this.instance = instance;
        this.costArray = new int[NUM_UMPIRES][NUM_UMPIRES];
    }

    public void generateCostArray(int roundIndex) {
        Arrays.stream(costArray).forEach(row -> Arrays.fill(row, MAX));
        fillCostArray(costArray, roundIndex, 0);

        if (DEBUG_COSTARRAY) {
            System.out.println("Costarray round: " + roundIndex);
            for (int[] ints : costArray) {
                for (int j = 0; j < costArray.length; j++) {
                    if (ints[j] >= MAX) {
                        System.out.print("INF ");
                    } else {
                        System.out.print(ints[j] + " ");
                    }
                }
                System.out.println();
            }
        }
    }

    public void fillCostArray(int[][] costArray, int roundIndex, int umpireId) {
        if (umpireId >= NUM_UMPIRES) {
            return;
        }
        Game current = getGame(roundIndex, umpireId);
        for (int nextUmpireId = 0; nextUmpireId < NUM_UMPIRES; nextUmpireId++) {
            int nextRound = roundIndex + 1;
            Game next = getGame(nextRound, nextUmpireId);
            costArray[umpireId][nextUmpireId] = calculateGameDistance(current, next);
        }
        int nextUmpireId = umpireId + 1;
        fillCostArray(costArray, roundIndex, nextUmpireId);
    }

    public int calculateGameDistance(Game current, Game next) {
        int currentStadium = current.getHomePlayerId();
        int nextStadium = next.getHomePlayerId();
        return getTravelDistanceBetween(currentStadium, nextStadium);
    }

    public Game getGame(int round, int umpireId) {
        int firstGameOfRound = round * NUM_UMPIRES;
        int gameId = firstGameOfRound + umpireId;
        return Instance.getGame(gameId);
    }

    public int calculateMatchingDistance(int[][] result, int currentRoundIndex) {
        return Arrays.stream(result).mapToInt(assignment -> {
            Game current = getGame(currentRoundIndex, assignment[0]);
            int nextRound = currentRoundIndex + 1;
            Game next = getGame(nextRound, assignment[1]);
            int interGameDistance = calculateGameDistance(current, next);
            if (DEBUG_LOWERBOUND_MATCHER) {
                System.out.println("Inter game distance: {" + current + "," + next + "}" + "= " + interGameDistance);
            }
            return interGameDistance;
        }).sum();
    }

}
