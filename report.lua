done = function(summary, latency, requests)
  io.write(string.format("report %d %d\n", latency:percentile(90), requests:percentile(90)))
end
