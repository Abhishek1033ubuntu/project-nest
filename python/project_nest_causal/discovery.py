import numpy as np
import scipy.linalg
from scipy.optimize import minimize

class NOTEARSCausalDiscovery:
    def __init__(self, rho_max: float = 1e16, lambda1: float = 0.1):
        self.rho_max = rho_max
        self.lambda1 = lambda1

    @staticmethod
    def _h(W: np.ndarray) -> float:
        d = W.shape[0]
        E = scipy.linalg.expm(W * W)
        return np.trace(E) - d

    def fit(self, X: np.ndarray, max_iter: int = 100, threshold: float = 0.3) -> np.ndarray:
        n, d = X.shape
        W_est = np.zeros((d, d))
        rho = 1.0
        alpha = 0.0
        h_val = np.inf

        def _loss(w_flat):
            W = w_flat.reshape((d, d))
            h = self._h(W)
            diff = X - X @ W
            loss_val = 0.5 / n * np.sum(diff ** 2)
            reg_val = self.lambda1 * np.sum(np.abs(W))
            return loss_val + reg_val + alpha * h + 0.5 * rho * (h ** 2)

        for _ in range(max_iter):
            if rho > self.rho_max or h_val < 1e-8:
                break
            res = minimize(_loss, W_est.flatten(), method='L-BFGS-B')
            W_est = res.x.reshape((d, d))
            h_val = self._h(W_est)
            alpha += rho * h_val
            rho *= 10.0

        W_est[np.abs(W_est) < threshold] = 0.0
        np.fill_diagonal(W_est, 0.0)
        return W_est
