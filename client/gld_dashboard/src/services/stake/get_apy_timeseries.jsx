const get_apy_timeseries = async (actor, starting_day, limit) => {
  const result = await actor.get_apy_timeseries({
    starting_day,
    limit: [limit],
  });
  return result;
};

export default get_apy_timeseries;
