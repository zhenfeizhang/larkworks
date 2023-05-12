import hashlib

def CPAPKE_KeyGen(n, k, q, eta_1, eta_2, d_u, d_v):
    # 32 random bytes
    d = [randrange(255) for i in range(32)]
    
    # hash d with SHA3-512
    G = hashlib.sha3_512()
    G.update(bytes(d))
    tv = G.digest()
    rho = tv[:32]
    sigma = tv[32:]
    N = 0
    # create a zero k    
    A = Matrix(Rq, k, k)
    for i in range(k):
        for j in range(k):
            # TODO:
#             A[i,j] = Parse(XOF(rho, j, i))
            # for now just use a random elem
            A[i,j] = Rq.random_element()
        
    s = Matrix(Rq, k, 1)
    for i in range(k):
        tv_rho = bytearray(rho)
        tv_rho.append(N)
        tv_hash = hashlib.shake_256()
        tv_hash.update(tv_rho)
        s[i,0] = CBD(eta_1, tv_hash.digest(256))
        N += 1
    
    e = Matrix(Rq, k, 1)
    for i in range(k):
        tv_rho = bytearray(rho)
        tv_rho.append(N)
        tv_hash = hashlib.shake_256()
        tv_hash.update(tv_rho)
        e[i,0] = CBD(eta_1, tv_hash.digest(256))
        N += 1

# TODO: use NTTs for multiplication
#     s = NTT(s)
#     e = NTT(e)
    
    t = A*s + e
    sk = encode(12, s)
    assert(len(sk) == (12*k*n)/8)
    pk = encode(12, t) + list(rho)
    assert(len(pk) == (12*k*n)/8 + 32)
    return (pk, sk)
