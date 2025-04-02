package com.example.javapoc2.dto.repository;

public interface ProductGroupResult {
    String getCategoryA();
    String getCategoryB();
    String getCategoryC();
    Long getCount();
    Double getValue(); // Use Double because SUM() can return null if no rows are aggregated
    Integer getGroupingId();
}
