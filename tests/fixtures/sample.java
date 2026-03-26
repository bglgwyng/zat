package com.example;

import java.util.List;

public class Service {
    public String name;
    private String secret;
    protected int value;

    public Service(String name) {
        this.name = name;
        this.secret = "";
        this.value = 0;
    }

    public String getName() {
        return this.name;
    }

    private void internal() {
        // hidden
    }

    protected void reset() {
        this.value = 0;
    }
}

public interface Reader {
    int read(byte[] buffer);
    void close();
}

public enum Color {
    RED,
    GREEN,
    BLUE;
}

public abstract class Base {
    public abstract void run();
}
