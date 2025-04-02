package com.example.productservice.service;

import com.example.productservice.model.Product;
import com.example.productservice.repository.ProductRepository;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.UUID;

@Service
public class ProductService {

    private final ProductRepository productRepository;

    public ProductService(ProductRepository productRepository) {
        this.productRepository = productRepository;
    }

    public List<Product> getAllProducts() {
        return productRepository.findAll();
    }

    public Product createProduct(Product product) {
        return productRepository.save(product);
    }

    public Product getProductById(UUID id) {
        return productRepository.findById(id).orElse(null);
    }

    public Product updateProduct(UUID id, Product newProduct) {
        return productRepository.findById(id).map(product -> {
            product.setCategoryA(newProduct.getCategoryA());
            product.setCategoryB(newProduct.getCategoryB());
            product.setCategoryC(newProduct.getCategoryC());
            product.setPriceInCents(newProduct.getPriceInCents());
            return productRepository.save(product);
        }).orElse(null);
    }

    public void deleteProduct(UUID id) {
        productRepository.deleteById(id);
    }
}
