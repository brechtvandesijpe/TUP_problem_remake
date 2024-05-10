package main;

import file.Reader;
import model.Game;
import model.Instance;

import java.io.File;
import java.io.IOException;
import java.lang.management.ManagementFactory;
import java.lang.management.ThreadMXBean;

import problem.Tree;

import static main.Config.*;

public class Benchmark {

    public static void main(String[] args) throws IOException {
        Benchmark benchmark = new Benchmark();
        benchmark.testInstance("resources/umps8.txt", 4, 2, 34311);
    }

    private void testInstance(String instanceFileName, int Q1, int Q2, int expectedValue) throws IOException {
        Game.gameId = 0;

        Config.FILE_NAME = instanceFileName;
        Config.Q1 = Q1;
        Config.Q2 = Q2;

        ThreadMXBean threadMXBean = ManagementFactory.getThreadMXBean();
        Instance instance = Reader.read(new File(instanceFileName));
        Tree tree = new Tree(instance, 0, NUM_ROUNDS - 1, false);
        long startExecution = threadMXBean.getCurrentThreadCpuTime();
        tree.startGlobalTraversal();
        long endExecution = threadMXBean.getCurrentThreadCpuTime();

        int upperbound = tree.getUpperbound();
        String solutionMessage = (upperbound == Integer.MAX_VALUE) ? "Infeasible" : String.valueOf(upperbound);

        if (upperbound == expectedValue) {
            System.out.println(green + "[PASSED]  " + reset + instanceFileName + " {" + Q1 + "," + Q2 + "} : Expected " + expectedValue + ", Actual " + solutionMessage + " - " + (endExecution - startExecution) / 1_000_000_000.0 + " sec");
        } else {
            System.out.println(red + "[FAILED]  " + reset + instanceFileName + " {" + Q1 + "," + Q2 + "} : Expected " + expectedValue + ", Actual " + solutionMessage + " - " + (endExecution - startExecution) / 1_000_000_000.0 + " sec");
        }

    }
}
