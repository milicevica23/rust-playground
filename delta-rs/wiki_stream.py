

if __name__ == "__main__":
    print("main")
    import pyspark
    from delta import *

    builder = pyspark.sql.SparkSession.builder.appName("MyApp").config("spark.local.dir", "./tmp/spark-temp") \
        .config("spark.sql.extensions", "io.delta.sql.DeltaSparkSessionExtension") \
        .config("spark.sql.catalog.spark_catalog", "org.apache.spark.sql.delta.catalog.DeltaCatalog")

    spark = configure_spark_with_delta_pip(builder).getOrCreate()
    data = spark.range(0, 5)
    data.write.format("delta").save("./tmp/delta-table")
