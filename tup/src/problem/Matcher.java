package problem;

import model.Game;
import model.Instance;
import subproblem.match.Match;

import java.util.BitSet;
import java.util.stream.IntStream;

import static main.Config.*;
import static model.Instance.*;
import static subproblem.match.MatchFactory.createMatchAlgorithm;


/**
 * 1. A subgraph is derived containing:
 * • the games of round r − 1 of umpires not yet allocated in round r,
 * • the games of round r with allocations pending,
 * • the edges connecting games of these two sets.
 * 2. A matching problem defined by the subgraph is solved.
 */

public class Matcher {

    private final Instance instance;
    private int branchStart;
    private int[][] subGraph;
    private int subGraphSize = 0;
    private final int[][] freeIndices = new int[NUM_UMPIRES][2];
    private final int INFEASIBLE = 9999999;

    public Matcher(Instance instance){
        this.instance = instance;
    }

    public int calculatePartialMatchingCost(BitSet vec, int subGraphSize, int roundIndex) {
        this.branchStart = NUM_UMPIRES * roundIndex;
        this.subGraphSize = subGraphSize;
        makeSubGraph(vec);
        return calculateDistance();
    }

    public void makeSubGraph(BitSet vec) {
        int[][] freeIndices = createFreeIndices(vec);
        subGraph = IntStream.range(0, subGraphSize).mapToObj(y -> IntStream.range(0, subGraphSize).map(x -> getInterGameDistance(freeIndices[y][0], freeIndices[x][1])).toArray()).toArray(int[][]::new);
    }

    public int[][] createFreeIndices(BitSet vec) {
        final int[] counter = new int[2];

        // Iterate over each bit position from 0 to 2 * NUM_UMPIRES - 1.
        IntStream.range(0, 2 * NUM_UMPIRES)
                // Filter zeroes
                .filter(bit -> !vec.get(bit))
                // For each filtered bit position, update the freeIndices array and increment the corresponding counter.
                .forEach(bit -> {
                    int temp = bit < NUM_UMPIRES ? 0 : 1;
                    freeIndices[counter[temp]][temp] = bit - temp * NUM_UMPIRES;
                    counter[temp]++;
                });
        return freeIndices;
    }

    public int calculateDistance() {
        Match matchAlgorithm = createMatchAlgorithm(MATCH_TYPE);
        int result = 0;
        for (int[] res : matchAlgorithm.getOptimalMatch(subGraph)) {
            int stadium1 = getGame(freeIndices[res[0]][0] + branchStart).getHomePlayerId();
            int stadium2 = getGame(NUM_UMPIRES + branchStart + freeIndices[res[1]][1]).getHomePlayerId();
            int interDist = getTravelDistanceBetween(stadium2, stadium1);
            //System.out.println("stadium1: " + stadium1 + ", stadium2: " + stadium2 + ", interDistance: " + interDist);
            result = result + interDist;
            //System.out.println("result: " + result);
        }
        if (DEBUG_MATCHER) {
            System.out.println("Total distance based on subgraph: " + result);
        }

        return result;
    }

    private int getInterGameDistance(int gameId, int gameId2) {
        Game g1 = getGame(branchStart + gameId);
        int nextBranchStart = branchStart + NUM_UMPIRES;
        Game g2 = getGame(nextBranchStart + gameId2);
        int interStadiumDist = getInterStadiumDistance(g1, g2);
        return isFeasible(g1, g2) ? interStadiumDist : INFEASIBLE;
    }

    @Override
    public String toString() {
        return "Matcher{" +
                "instance=" + instance +
                '}';
    }
}
