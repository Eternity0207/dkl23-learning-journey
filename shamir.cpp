#include <bits/stdc++.h>
using namespace std;

#define fastio ios::sync_with_stdio(false); cin.tie(nullptr);
#define all(x) (x).begin(), (x).end()
#define pb push_back
#define int long long
#define rep(i,a,b) for(int i = (a); i < (b); i++)

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

    int secret = 2324124;
    int n = 5, t = 3;

    vector<int> coeff(t);
    coeff[0] = secret;
    rep(i,1,t) coeff[i] = rand() % 1000;

    cout << "Polynomial: ";
    rep(i,0,t) {
        cout << coeff[i] << "*x^" << i;
        if(i != t-1) cout << " + ";
    }
    cout << endl;

    vector<pair<int,int>> shares;

    rep(i,1,n+1) {
        int x = i, y = 0, cur = 1;
        rep(j,0,t) {
            y = (y + coeff[j] * cur) % mod;
            cur = (cur * x) % mod;
        }
        shares.pb({x,y});
    }

    cout << "Shares:" << endl;
    for(auto &p : shares) {
        cout << "(" << p.first << ", " << p.second << ")" << endl;
    }

    int recovered = 0;

    cout << endl << "Reconstruction steps:" << endl;

    rep(i,0,t) {
        int xi = shares[i].first;
        int yi = shares[i].second;

        int num = 1, den = 1;

        rep(j,0,t) {
            if(i == j) continue;
            int xj = shares[j].first;
            num = (num * (-xj + mod)) % mod;
            den = (den * (xi - xj + mod)) % mod;
        }

        int li = (num * modinv(den)) % mod;

        cout << "Using share (" << xi << ", " << yi << ")" << endl;
        cout << "Lagrange coeff: " << li << endl;
        cout << "Contribution: " << (yi * li) % mod << endl;

        recovered = (recovered + yi * li) % mod;
    }

    cout << endl;
    cout << "Original: " << secret << endl;
    cout << "Recovered: " << recovered << endl;

    return 0;
}