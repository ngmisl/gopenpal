import { useState, useEffect } from "react";
import { useWaterStore } from "../store/water";

export function WaterTracker({ compact = false }: { compact?: boolean }) {
  const [amount, setAmount] = useState("");
  const { stats, setStats } = useWaterStore();

  // Mock data for demo
  useEffect(() => {
    setStats({
      today_ml: 1200,
      goal_ml: 2000,
      percentage: 60,
      entries_today: 4,
      last_log: new Date().toISOString(),
    });
  }, [setStats]);

  const quickAmounts = [250, 500, 750, 1000];

  if (compact) {
    return (
      <div className="bg-white rounded-lg shadow-md p-4">
        <h3 className="font-semibold mb-2 flex items-center">
          <span className="text-xl mr-2">💧</span>
          Water Today
        </h3>
        {stats && (
          <>
            <div className="text-2xl font-bold text-blue-600">{stats.today_ml}ml</div>
            <div className="text-sm text-gray-500">of {stats.goal_ml}ml goal</div>
            <div className="mt-2 bg-gray-200 rounded-full h-2">
              <div
                className="bg-blue-500 h-2 rounded-full transition-all"
                style={{ width: `${Math.min(stats.percentage, 100)}%` }}
              ></div>
            </div>
          </>
        )}
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* Stats */}
      {stats && (
        <div className="grid grid-cols-2 gap-4">
          <div className="bg-blue-50 rounded-lg p-4">
            <div className="text-sm text-gray-600">Today</div>
            <div className="text-3xl font-bold text-blue-600">{stats.today_ml}ml</div>
          </div>
          <div className="bg-blue-50 rounded-lg p-4">
            <div className="text-sm text-gray-600">Goal</div>
            <div className="text-3xl font-bold text-blue-600">{stats.goal_ml}ml</div>
          </div>
        </div>
      )}

      {/* Progress */}
      {stats && (
        <div>
          <div className="flex justify-between mb-2">
            <span className="text-sm font-medium">Progress</span>
            <span className="text-sm text-gray-600">{stats.percentage.toFixed(0)}%</span>
          </div>
          <div className="bg-gray-200 rounded-full h-4">
            <div
              className="bg-gradient-to-r from-blue-400 to-blue-600 h-4 rounded-full transition-all"
              style={{ width: `${Math.min(stats.percentage, 100)}%` }}
            ></div>
          </div>
        </div>
      )}

      {/* Log Water */}
      <div>
        <h3 className="font-semibold mb-3">Log Water</h3>
        <div className="grid grid-cols-4 gap-2 mb-3">
          {quickAmounts.map((amt) => (
            <button
              key={amt}
              onClick={() => setAmount(amt.toString())}
              className="px-3 py-2 bg-blue-100 hover:bg-blue-200 rounded-lg text-sm font-medium transition-colors"
            >
              {amt}ml
            </button>
          ))}
        </div>
        <div className="flex space-x-2">
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            placeholder="Amount (ml)"
            className="flex-1 px-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <button
            onClick={() => {
              if (amount && stats) {
                const newTotal = stats.today_ml + Number(amount);
                setStats({
                  ...stats,
                  today_ml: newTotal,
                  percentage: (newTotal / stats.goal_ml) * 100,
                  entries_today: stats.entries_today + 1,
                  last_log: new Date().toISOString(),
                });
                setAmount("");
              }
            }}
            className="px-6 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 transition-colors"
          >
            Log
          </button>
        </div>
      </div>
    </div>
  );
}
