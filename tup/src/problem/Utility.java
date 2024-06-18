package problem;

import java.util.BitSet;
import java.util.stream.IntStream;

import static main.Config.NUM_UMPIRES;


/**
 * Provides utility functions for hashing.
 */

public class Utility {
    
    // values don't matter as long as the prime is high enough. 
    // make sure no collisions occur.
    private static final int PRIME = 31;
    private static final int MULTIPLIER = 1223;
    private static final int SET = 1231;
    // private static final int SET = 17;
    // private static final int SET = 89;
    private static final int UNSET = 1237;
    // private static final int UNSET = 7;

    /**
     * Calculates the vector representation of umpire assignments for a given round.
     *
     * @param umpire                The number of umpires.
     * @param currentRoundIndex     The index of the current round.
     * @param umpireScheduleByRound The schedule of umpire assignments by round.
     * @return The vector representation of umpire assignments for the current round.
     */

    public static BitSet calculateVec(int umpire, int currentRoundIndex, int[][] umpireScheduleByRound) {
        return IntStream.rangeClosed(0, umpire).mapToObj(umpireId -> {
            int previousRound = currentRoundIndex - 1;
            int previousGameId = Math.floorMod(umpireScheduleByRound[umpireId][previousRound], NUM_UMPIRES);
            int nextGameId = Math.floorMod(umpireScheduleByRound[umpireId][currentRoundIndex], NUM_UMPIRES);
            BitSet vec = new BitSet(2 * NUM_UMPIRES);
            vec.set(previousGameId);
            vec.set(NUM_UMPIRES + nextGameId);
            return vec;
        }).reduce(new BitSet(2 * NUM_UMPIRES), (acc, next) -> {
            acc.or(next);
            return acc;
        });
    }

    /**
     * Computes a blended digest value using a multiplicative mixing function.
     * It combines the round number and the hash value of a BitSet.
     *
     * @param round The current round number.
     * @param vec   The BitSet to be hashed.
     * @return The blended digest value.
     */

    public static int blend(int round, BitSet vec) {
        int digest = 1;
        digest = mixMultiplicative(mixMultiplicative(digest, round), hashBitSet(vec));
        return digest;
    }

    /**
     * Computes the hash value of a BitSet using a prime number-based hashing function.
     *
     * @param vec The BitSet to be hashed.
     * @return The hash value of the BitSet.
     */

    public static int hashBitSet(BitSet vec) {
        int digest = 1;
        for (int b = 0; b < vec.length(); b++) {
            digest = PRIME * digest + (vec.get(b) ? SET : UNSET);
            //System.out.println("digest: " + digest);
        }
        return digest;
    }

    /**
     * Mixes a digest value with a given integer value using a multiplicative mixing function.
     *
     * @param digest The current digest value.
     * @param value  The value to be mixed.
     * @return The mixed digest value.
     */

    public static int mixMultiplicative(int digest, int value) {
        return MULTIPLIER * digest + value;
    }

}
