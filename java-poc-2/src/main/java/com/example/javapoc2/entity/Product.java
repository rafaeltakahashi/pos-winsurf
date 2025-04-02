package com.example.javapoc2.entity;

import jakarta.persistence.*;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.AllArgsConstructor;

import java.util.UUID;

@Entity
@Table(name = "products") // Specify table name
@Data // Lombok annotation for getters, setters, toString, equals, hashCode
@NoArgsConstructor // Lombok annotation for no-args constructor
@AllArgsConstructor // Lombok annotation for all-args constructor
public class Product {

    @Id
    @GeneratedValue(strategy = GenerationType.AUTO)
    private UUID id;

    private String categoryA;
    private String categoryB;
    private String categoryC;

    @Column(name = "price_cents")
    private Integer priceCents; // Store price as integer cents

    // Custom getter for price as double
    public Double getPrice() {
        if (this.priceCents == null) {
            return null;
        }
        return this.priceCents / 100.0;
    }

    // Custom setter for price as double (optional, but good practice)
    public void setPrice(Double price) {
        if (price == null) {
            this.priceCents = null;
        } else {
            this.priceCents = (int) Math.round(price * 100);
        }
    }
}
