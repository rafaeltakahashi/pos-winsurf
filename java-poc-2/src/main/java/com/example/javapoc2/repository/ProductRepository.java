package com.example.javapoc2.repository;

import com.example.javapoc2.dto.repository.ProductGroupResult;
import com.example.javapoc2.entity.Product;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.stereotype.Repository;

import java.util.List;
import java.util.UUID;

@Repository
public interface ProductRepository extends JpaRepository<Product, UUID> {

    @Query(value = """
        SELECT
            categorya as categoryA,
            categoryb as categoryB,
            categoryc as categoryC,
            count(*) as count,
            sum(price_cents) as value,
            GROUPING(categorya, categoryb, categoryc) as groupingId
        FROM
            products
        GROUP BY
            GROUPING SETS (
                (categorya),
                (categoryb),
                (categoryc),
                ()
            )
        """, nativeQuery = true)
    List<ProductGroupResult> findProductStatsByGroupingSets();
}
