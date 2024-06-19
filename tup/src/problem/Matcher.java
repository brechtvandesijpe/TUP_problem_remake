package problem;

import main.Config;
import model.Game;
import model.Instance;
import subproblem.match.Match;

import java.util.BitSet;
import java.util.HashMap;
import java.util.Map;
import java.util.stream.IntStream;

import static main.Config.*;
import static model.Instance.*;
import static problem.Utility.blend;
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
    private int cacheHits = 0;
    private int cacheMisses = 0;
    private final Map<Integer, Integer> previouslySolved = new HashMap<>();

    public Matcher(Instance instance){
        this.instance = instance;
    }

    public int calculatePartialMatchingCost(BitSet vec, int subGraphSize, int roundIndex) {
        if(PRINT_HIT_RATIO) {
            printCacheHitRatio();
        }

        this.branchStart = NUM_UMPIRES * roundIndex;
        this.subGraphSize = subGraphSize;

        if (ENABLE_HASHING) {
            int partialMatchingDistance;
            int key = blend(roundIndex, vec);
            Integer cached = getValueFromCache(key);
            if (cached != null) {
                cacheHits++;
                if (DEBUG_MATCHER) {
                    System.out.println("Cache hit, " + cacheHits);
                }
                partialMatchingDistance = cached;
            } else {
                cacheMisses++;
                if (DEBUG_MATCHER) {
                    System.out.println("Cache miss, " + cacheMisses);
                }

                makeSubGraph(vec);
                //printSubGraph();
                partialMatchingDistance = calculateDistance();
                saveValueToCache(key, partialMatchingDistance);
            }
            return partialMatchingDistance;
        } else {
            // dont cache anything
            makeSubGraph(vec);
            return calculateDistance();
        }
    }


     /**
     * Constructs the subgraph for the current matching problem.
     */

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

      /**
     * Calculates the total travel distance for the current matching problem.
     */

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

    /**
     * Calculates the inter-game distance between two games, considering feasibility.
     */

    private int getInterGameDistance(int gameId, int gameId2) {
        Game g1 = getGame(branchStart + gameId);
        int nextBranchStart = branchStart + NUM_UMPIRES;
        Game g2 = getGame(nextBranchStart + gameId2);
        int interStadiumDist = getInterStadiumDistance(g1, g2);
        // no point continuing if is infeasible
        return isFeasible(g1, g2) ? interStadiumDist : INFEASIBLE;
    }

    public void printCacheHitRatio() {
        double hitRatio = (double) cacheHits / (cacheHits + cacheMisses);
        System.out.println(Config.red + "Hit Ratio of Cache: " + hitRatio + ". " + reset);
    }

    public double getHitRatio() {
        return (double) cacheHits / (cacheHits + cacheMisses);
    }

    public int getAccessCount() {
        return cacheHits + cacheMisses;
    }

    public void saveValueToCache(int key, int value) {
        previouslySolved.put(key, value);
    }

    public Integer getValueFromCache(int key) {
        return previouslySolved.get(key);
    }

    @Override
    public String toString() {
        return "Matcher{" +
                "instance=" + instance +
                '}';
    }
}
