# Convert byte array to bits. Returns list of bits
def bytes_to_bits(byte_array):
    if not all(0 <= byte <= 255 for byte in byte_array):
        raise ValueError('Input must be a list of integers between 0 and 255.')

    bit_array = []
    for byte in byte_array:
        for i in range(8):
            bit = (byte >> i) & 1
            bit_array.append(bit)

    return bit_array


# Function to convert bit array to byte array
def bits_to_bytes(bits):
    n = len(bits)
    byte_array = []

    for i in range(0, n, 8):
        byte = 0
        for j in range(8):
            if i + j < n:
                byte |= bits[i + j] << (j)
        byte_array.append(byte)

    return byte_array


# on eta = {2,3} and B = (b_0, b_1, ..., b_{64*eta - 1})
# Centered binomial distribution
def CBD(eta, B):
    f = Rq.zero()
    bits = bytes_to_bits(B)
    for i in range(255):
        a = sum([bits[2*i*eta + j] for j in range(eta)])
        b = sum([bits[2*i*eta + eta + j] for j in range(eta)])
        f_i = a - b
        f += f_i * x**i
    return f


def compress(q, x, d):
    return ((2**d / q) * x ).round() % (2**d)

def decompress(q, x, d):
    return ((q / (2**d)) * x).round()


# Decodes the bytes into a single polynomial
# `Rq` should be in scope
def decode_poly(l, B):
#     assert(log(q, 2) > l)
    assert(len(B) == 32*l)
    
    bits = bytes_to_bits(B)
    f = Rq.zero()
    for i in range(256):
        f_i = sum([bits[i*l + j]*2**j for j in range(l)])
        f += f_i * x**i
    return f


# Encodes the bytes into a single polynomial
def encode_poly(l, f):
#     assert(log(q, 2) > l)
    
    bits = [0]*(256*l)
    for i, c in enumerate(f):
        # coefficients of f should be in {0, ..., 2^l - 1}
        assert(c < 2**l, "coefficient of f too large!")
        c_bits = Integer(c).bits()
        c_bits.reverse()
        # pad with zeros to match l. 
        # Includes the case when array is empty (i.e. c=0)
        while len(c_bits) < l:
            c_bits.append(0)
        
        for j in range(l):
            bits[j + i*l] = c_bits[j]
    return bits_to_bytes(bits)


# Encodes a vector of polynomials, where M: kx1 matrix
def encode(l, M):
    assert(M.ncols() == 1, "Unexpected input M: should be a kx1 matrix")
    ret = []
    for i in range(M.nrows()):
        f = M[i, 0]
        bits = encode_poly(l, f)
        ret += bits
    
    return ret

# Bit reversal of a 7-bit integer i
def br(i):
    tv = Integer(i).bits()
    while len(tv) < 7:
        tv.append(0)
    tv.reverse()
    out = 0
    for i, b in enumerate(tv):
        out += b << i
    return out
