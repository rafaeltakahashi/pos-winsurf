package com.example.javapoc2.controller;

import com.example.javapoc2.dto.response.ProductStatsResponse;
import com.example.javapoc2.entity.Product;
import com.example.javapoc2.service.ProductService;
import lombok.RequiredArgsConstructor;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/products")
@RequiredArgsConstructor // Lombok annotation for constructor injection
public class ProductController {

    private final ProductService productService;

    /**
     * GET /products : Get all products with pagination.
     *
     * @param pageable the pagination information (e.g., ?page=0&size=20&sort=priceCents,asc)
     * @return the ResponseEntity with status 200 (OK) and the page of products in body.
     */
    @GetMapping
    public ResponseEntity<Page<Product>> getAllProducts(Pageable pageable) {
        Page<Product> page = productService.getAllProducts(pageable);
        return ResponseEntity.ok(page);
    }

    /**
     * POST /products/populate : Populates the database with 1000 random products.
     *
     * @return the ResponseEntity with status 200 (OK) if successful, or 500 (Internal Server Error) otherwise.
     */
    @PostMapping("/populate")
    public ResponseEntity<Void> populateProducts() {
        try {
            productService.populateProducts();
            return ResponseEntity.ok().build();
        } catch (Exception e) {
            // Log the exception (optional, depends on logging setup)
            // log.error("Error populating products", e);
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).build();
        }
    }

    @GetMapping("/by-category")
    public ResponseEntity<ProductStatsResponse> getProductStatsByCategory() {
        ProductStatsResponse stats = productService.getProductStats();
        return ResponseEntity.ok(stats);
    }
}
