import hashlib

hashlib.new('md5')
hashlib.new(name='md5', data=b'test')
hashlib.md5()

hashlib.new('md4', b'test')
hashlib.new('MD4', data=b'test')
hashlib.md4()

hashlib.new('sha1')
hashlib.new('sha1', data=b'test')
hashlib.sha1()

hashlib.new('sha', data=b'test')
hashlib.new(name='SHA', data=b'test')
hashlib.sha()

# usedforsecurity arg only availabe in Python 3.9+
hashlib.new('sha1', usedforsecurity=True)
hashlib.sha1(usedforsecurity=True)

# Test that plugin does not flag valid hash functions.
hashlib.new('sha256')
hashlib.sha256()

hashlib.new('SHA512')
hashlib.sha512()

# usedforsecurity arg only availabe in Python 3.9+
hashlib.new(name='sha1', usedforsecurity=False)
hashlib.sha1(usedforsecurity=False)
