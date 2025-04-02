package com.example.javapoc2.service;

import com.example.javapoc2.dto.repository.ProductGroupResult;
import com.example.javapoc2.dto.response.ProductStatsResponse;
import com.example.javapoc2.repository.ProductRepository;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.extension.ExtendWith;
import org.mockito.InjectMocks;
import org.mockito.Mock;
import org.mockito.junit.jupiter.MockitoExtension;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;
import static org.mockito.Mockito.verify;
import static org.mockito.Mockito.when;

@ExtendWith(MockitoExtension.class) // Integrates Mockito with JUnit 5
class ProductServiceTest {

    @Mock // Creates a mock instance of ProductRepository
    private ProductRepository productRepository;

    @InjectMocks // Creates an instance of ProductService and injects the mocks (@Mock) into it
    private ProductService productService;

    // Test helper: Mock implementation of ProductGroupResult
    private static class MockProductGroupResult implements ProductGroupResult {
        private final String categoryA;
        private final String categoryB;
        private final String categoryC;
        private final Long count;
        private final Double value;
        private final Integer groupingId;

        MockProductGroupResult(String a, String b, String c, Long count, Double value, Integer groupingId) {
            this.categoryA = a;
            this.categoryB = b;
            this.categoryC = c;
            this.count = count;
            this.value = value;
            this.groupingId = groupingId;
        }

        @Override public String getCategoryA() { return categoryA; }
        @Override public String getCategoryB() { return categoryB; }
        @Override public String getCategoryC() { return categoryC; }
        @Override public Long getCount() { return count; }
        @Override public Double getValue() { return value; }

        @Override
        public Integer getGroupingId() {
            return this.groupingId;
        }
    }


    @Test
    void getProductStats_shouldReturnAggregatedStats() {
        // Arrange: Define the mock data the repository should return
        List<ProductGroupResult> mockResults = Arrays.asList(
            new MockProductGroupResult("A", null, null, 10L, 100.50, 3), // Grouped by categoryA (id=3)
            new MockProductGroupResult("B", null, null, 5L, 55.25, 3), // Another Category A (id=3)
            new MockProductGroupResult(null, "X", null, 20L, 210.75, 5), // Grouped by categoryB (id=5)
            new MockProductGroupResult(null, null, "P", 15L, 150.00, 6), // Grouped by categoryC (id=6)
            new MockProductGroupResult(null, null, null, 50L, 516.50, 7) // Grand Total (id=7)
        );
        when(productRepository.findProductStatsByGroupingSets()).thenReturn(mockResults);

        // Act: Call the method under test
        ProductStatsResponse response = productService.getProductStats();

        // Assert: Verify the results
        assertNotNull(response, "Response should not be null");
        assertNotNull(response.getByCategoryA(), "Category A stats should not be null");
        assertNotNull(response.getByCategoryB(), "Category B stats should not be null");
        assertNotNull(response.getByCategoryC(), "Category C stats should not be null");
        assertNotNull(response.getTotal(), "Total stats should not be null");

        // Basic count verification (more detailed checks could be added)
        assertEquals(2, response.getByCategoryA().size(), "Should have 2 Category A stats");
        assertEquals(1, response.getByCategoryB().size(), "Should have 1 Category B stat");
        assertEquals(1, response.getByCategoryC().size(), "Should have 1 Category C stat");

        // Verify total stats (based on the specific mock data for the grand total)
        assertEquals(50L, response.getTotal().getCount(), "Total count mismatch");
        assertEquals(516.50, response.getTotal().getValue(), 0.001, "Total value mismatch");


        // Verify that the repository method was called exactly once
        verify(productRepository).findProductStatsByGroupingSets();
    }

    @Test
    void getProductStats_whenRepositoryReturnsEmpty_shouldReturnEmptyStats() {
        // Arrange: Define empty results from the repository
        when(productRepository.findProductStatsByGroupingSets()).thenReturn(Collections.emptyList());

        // Act: Call the method under test
        ProductStatsResponse response = productService.getProductStats();

        // Assert: Verify the response contains empty lists and zero totals
        assertNotNull(response, "Response should not be null");
        assertTrue(response.getByCategoryA().isEmpty(), "Category A stats should be empty");
        assertTrue(response.getByCategoryB().isEmpty(), "Category B stats should be empty");
        assertTrue(response.getByCategoryC().isEmpty(), "Category C stats should be empty");
        assertNotNull(response.getTotal(), "Total stats should not be null");
        assertEquals(0L, response.getTotal().getCount(), "Total count should be 0");
        assertEquals(0.0, response.getTotal().getValue(), 0.001, "Total value should be 0.0");

        // Verify that the repository method was called
        verify(productRepository).findProductStatsByGroupingSets();
    }
}
