package com.example.javapoc2.dto.response;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

@Data
@NoArgsConstructor
@AllArgsConstructor
public class CategoryAStat {
    @JsonProperty("category_a")
    private String categoryA;
    private long count;
    private double value;
}
