

if __name__ == "__main__":
    print("main")
    import pyspark
    from delta import *

    builder = pyspark.sql.SparkSession.builder.appName("MyApp") \
        .config("spark.sql.extensions", "io.delta.sql.DeltaSparkSessionExtension") \
        .config("spark.sql.catalog.spark_catalog", "org.apache.spark.sql.delta.catalog.DeltaCatalog")

    spark = configure_spark_with_delta_pip(builder).getOrCreate()
    data = spark.range(0, 5)
    data.write.format("delta").mode("overwrite").save("./tmp/delta-table")
    data = spark.range(5, 10)
    data.show()
    data.write.format("delta").mode("overwrite").save("./tmp/delta-table")
