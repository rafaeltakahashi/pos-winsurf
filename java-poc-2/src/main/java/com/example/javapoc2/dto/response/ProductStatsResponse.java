package com.example.javapoc2.dto.response;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;

import java.util.List;

@Data
@NoArgsConstructor
@AllArgsConstructor
public class ProductStatsResponse {
    @JsonProperty("by_category_a")
    private List<CategoryAStat> byCategoryA;

    @JsonProperty("by_category_b")
    private List<CategoryBStat> byCategoryB;

    @JsonProperty("by_category_c")
    private List<CategoryCStat> byCategoryC;

    private TotalStat total;
}
