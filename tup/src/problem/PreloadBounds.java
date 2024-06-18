package problem;

import java.util.HashMap;
import java.util.Map;

/*
 * 
 *  Unused.
 */
public class PreloadBounds {
    // todo: preload bounds om betere bounds te bekomen
    private Map<String, Integer> lowerbounds;

    public PreloadBounds() {
        this.lowerbounds = new HashMap<>();
    }

    public void setLowerBound(String instanceName) {
        int lowerbound = getLBOfInstance(instanceName);
    }

    public int getLBOfInstance(String instanceName) {
        return -1;
    }

}
