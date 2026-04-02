#include <bits/stdc++.h>
using namespace std;

#define fastio ios::sync_with_stdio(false); cin.tie(nullptr);
#define int long long

int mod = 1000000007;

int power(int a, int b) {
    int res = 1;
    while(b) {
        if(b & 1) res = (res * a) % mod;
        a = (a * a) % mod;
        b >>= 1;
    }
    return res;
}

int modinv(int a) {
    return power(a, mod - 2);
}

int32_t main() {
    fastio;

    int secret = 1234;
    int n = 5, t = 3;

    vector<int> coeff(t);
    coeff[0] = secret;
    for(int i = 1; i < t; i++) coeff[i] = rand() % 1000;

    vector<pair<int,int>> shares;

    for(int i = 1; i <= n; i++) {
        int x = i, y = 0, cur = 1;
        for(int j = 0; j < t; j++) {
            y = (y + coeff[j] * cur) % mod;
            cur = (cur * x) % mod;
        }
        shares.push_back({x, y});
    }

    int recovered = 0;

    for(int i = 0; i < t; i++) {
        int xi = shares[i].first;
        int yi = shares[i].second;

        int num = 1, den = 1;

        for(int j = 0; j < t; j++) {
            if(i == j) continue;
            int xj = shares[j].first;
            num = (num * (-xj + mod)) % mod;
            den = (den * (xi - xj + mod)) % mod;
        }

        int li = (num * modinv(den)) % mod;
        recovered = (recovered + yi * li) % mod;
    }

    cout << "Original: " << secret << endl;
    cout << "Recovered: " << recovered << endl;

    return 0;
}