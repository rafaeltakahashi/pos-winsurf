package com.example.javapoc2.service;

import com.example.javapoc2.dto.response.ProductStatsResponse;
import org.junit.jupiter.api.Test;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.boot.test.context.SpringBootTest;
import org.springframework.transaction.annotation.Transactional;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration tests for the ProductService, interacting with the real database.
 */
@SpringBootTest // Loads the full Spring application context
@Transactional // Rolls back database changes after each test
class ProductServiceIT {

    @Autowired // Injects the actual ProductService bean
    private ProductService productService;

    @Test
    void getProductStats_runsSuccessfullyAgainstDatabase() {
        // Arrange: Ensure the table exists and has data (populateProducts handles DDL if needed)
        // This step might take a moment as it inserts 1000 records.
        assertDoesNotThrow(() -> productService.populateProducts(),
                "Populating products should not throw an exception.");

        // Act & Assert: Call the method under test and verify it doesn't throw an exception
        ProductStatsResponse response = null;
        try {
            response = productService.getProductStats();
        } catch (Exception e) {
            fail("productService.getProductStats() threw an exception: " + e.getMessage(), e);
        }

        // Assert: Basic check that the response object was created
        assertNotNull(response, "The response from getProductStats should not be null.");

        // Optional: Add more detailed assertions here if needed, e.g., check total count > 0
        assertNotNull(response.getTotal(), "Total stats should not be null");
        // Since we populated, we expect some results
        assertTrue(response.getTotal().getCount() > 0, "Total count should be greater than 0 after populating.");
        // Also check that category lists are not null (though they might be empty depending on generated data)
        assertNotNull(response.getByCategoryA(), "Category A list should not be null");
        assertNotNull(response.getByCategoryB(), "Category B list should not be null");
        assertNotNull(response.getByCategoryC(), "Category C list should not be null");

        System.out.println("Integration test getProductStats_runsSuccessfullyAgainstDatabase completed successfully.");
    }
}
