zip()
zip(range(3))
zip("a", "b")
zip("a", "b", *zip("c"))
zip(zip("a"), strict=False)
zip(zip("a", strict=True))

zip(range(3), strict=True)
zip("a", "b", strict=False)
zip("a", "b", "c", strict=True)