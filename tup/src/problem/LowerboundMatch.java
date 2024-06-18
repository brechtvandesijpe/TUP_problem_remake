package problem;

import model.Game;
import model.Instance;
import subproblem.match.Match;

import java.util.Arrays;

import static main.Config.*;
import static model.Instance.getTravelDistanceBetween;
import static subproblem.match.MatchFactory.createMatchAlgorithm;

/**
 * This class is responsible for calculating the initial 2 round lower bounds
*/

public class LowerboundMatch {
    private final int[][] costArray;
    private final int MAX = 999999; 

    public LowerboundMatch() {
        this.costArray = new int[NUM_UMPIRES][NUM_UMPIRES];
    }

    /**
     * Calculates the matching cost for the given round.
     */

    public int calculateRoundMatching(int roundIndex) {
        // Calculate matching distance based on the result of cost array for the roundIndex
        generateCostArray(roundIndex);
        // generateCostArray(roundIndex + 1);
        return calculateMatchingDistance(solveAssignmentProblem(), roundIndex);
    }

    /**
     * Solves the assignment problem using a matching algorithm.
     */

    public int[][] solveAssignmentProblem() {
        Match match = createMatchAlgorithm(MATCH_TYPE);
        int[][] ret = match.getOptimalMatch(costArray);
        if (DEBUG_LOWERBOUND_MATCHER) {
            System.out.println("Return: " + Arrays.deepToString(ret));
        }
        return ret;
    }

    /**
     * Generates the cost array for the given round.
     */

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

    /**
     * Fills the cost array for the given round and umpire.
     */

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

    /**
     * Calculates the trav distance between two games.
     */

    public int calculateGameDistance(Game current, Game next) {
        int currentStadium = current.getHomePlayerId();
        int nextStadium = next.getHomePlayerId();
        return getTravelDistanceBetween(currentStadium, nextStadium);
    }

    /**
     * Retrieves the game at the specified round and umpire from branch start.
     */

    public Game getGame(int round, int umpireId) {
        int firstGameOfRound = round * NUM_UMPIRES;
        int gameId = firstGameOfRound + umpireId;
        return Instance.getGame(gameId);
    }

    /**
     * Calculates the matching distance based on the result of the assignment problem.
     */

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

    @Override
    public String toString() {
        return "LowerboundMatch{" + "costArray=" + Arrays.toString(costArray) + ", MAX=" + MAX + '}';
    }
}
