n = 256
q = 3329

Z.<x1> = ZZ[]
Zq.<x2> = GF(q)[]

R.<y> = QuotientRing(Z, Z.ideal(x1^n + 1))
Rq.<x> = QuotientRing(Zq, Zq.ideal(x2^n + 1))

load("utils.sage")
load("Kyber.sage")

(pk, sk) = CPAPKE_KeyGen(256, 2, 3329, 3, 2, 10, 4)
print("Public key is: ", pk)
print("Secret key is: ", sk)
