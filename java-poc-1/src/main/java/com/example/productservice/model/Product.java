package com.example.productservice.model;


import org.springframework.data.annotation.Id;

import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;

import java.util.UUID;

@Entity
public class Product {

    @Id
    @GeneratedValue
    private UUID id;

    private String categoryA;
    private String categoryB;
    private String categoryC;

    // price stored internally in cents
    private int priceInCents;

    public Product() {}

    // Getters and setters
    public UUID getId() {
        return id;
    }

    public void setId(UUID id) {
        this.id = id;
    }

    public String getCategoryA() {
        return categoryA;
    }

    public void setCategoryA(String categoryA) {
        this.categoryA = categoryA;
    }

    public String getCategoryB() {
        return categoryB;
    }

    public void setCategoryB(String categoryB) {
        this.categoryB = categoryB;
    }

    public String getCategoryC() {
        return categoryC;
    }

    public void setCategoryC(String categoryC) {
        this.categoryC = categoryC;
    }

    // Internal getter and setter for price in cents
    public int getPriceInCents() {
        return priceInCents;
    }

    public void setPriceInCents(int priceInCents) {
        this.priceInCents = priceInCents;
    }

    // Getter for price as double (dollars)
    public double getPrice() {
        return priceInCents / 100.0;
    }

    // Setter that accepts price in dollars and converts to cents
    public void setPrice(double price) {
        this.priceInCents = (int) Math.round(price * 100);
    }
}
