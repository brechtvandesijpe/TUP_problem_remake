package problem;

import model.Instance;

public class Matcher {

    private Instance instance;

    public Matcher(Instance instance){
        this.instance = instance;
    }

    public void matchRound() {
        // todo
    }

    public void solveSubProblem() {
        // todo
    }

    public Instance getInstance() {
        return instance;
    }

    @Override
    public String toString() {
        return "Matcher{" +
                "instance=" + instance +
                '}';
    }
}
